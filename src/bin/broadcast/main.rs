use std::collections::{BinaryHeap, BTreeSet, HashMap};
use std::ops::Add;
use std::time::{Duration, Instant};

use log::debug;

use gossip_glomers::common::actor::Actor;
use gossip_glomers::common::error::{Error, Result};
use gossip_glomers::common::error::Error::UnexpectedMessage;
use gossip_glomers::common::message::message::Message;
use gossip_glomers::common::message::NodeId;
use gossip_glomers::common::record::Record;
use gossip_glomers::common::runner::{reply, run_actor, RunnerAction, send, set_timer};
use gossip_glomers::common::this_node::ThisNode;

use crate::message::{BroadcastMessage, MessageValue};

mod message;

const MAX_ACK_DELAY: Duration = Duration::from_millis(4000);

/// Max broadcast propagation duration: = 24(nodes-1) / 4(NEXT_NODES) * 140ms (max latency) ~= 6 * 140ms = 840ms
/// Target mean propagation duration: 1s
/// Target max propagation duration: 2s
/// Period to max batch: 2s - 840ms ~= 1160ms
const NEXT_NODES: usize = 4;
const TARGET_OPS_PER_BROADCAST: usize = 20;
const SINGLE_MESSAGE_DELAY: Duration = Duration::from_millis(1000);

/// Max broadcast propagation duration: = 24(nodes-1) / 8(NEXT_NODES) * 140ms (max latency) ~= 3 * 140ms = 420ms
/// Target mean propagation duration: 400ms
/// Target max propagation duration: 600ms
/// Period to max batch: 600ms - 420ms = 180ms
/// /// How many nodes to notify during broadcast
// const NEXT_NODES: usize = 8;
// const TARGET_OPS_PER_BROADCAST: usize = 30;
// const SINGLE_MESSAGE_DELAY: Duration = Duration::from_millis(80);

struct BroadcastActor {
    batch_size: usize,
    this_node: ThisNode,
    next_nodes: Vec<NodeId>,
    seen_messages: HashMap<i64, BTreeSet<NodeId>>,
    batched_messages: BinaryHeap<Record<i64>>,
}

impl BroadcastActor {
    fn observe_message(&mut self, node_id: NodeId, message: i64, now: Instant) -> Vec<RunnerAction<BroadcastMessage, TimerKey>> {
        let prev_nodes = self.seen_messages.entry(message).or_default();
        let prev_nodes_count = prev_nodes.len();
        prev_nodes.insert(node_id);
        if prev_nodes_count == 0 {
            self.batched_messages.push(Record { timestamp: now, value: message });
            vec![set_timer(MAX_ACK_DELAY, TimerKey::CheckAck(message))]
        } else {
            vec![]
        }
    }

    fn get_broadcast_message(&mut self, now: Instant) -> Vec<RunnerAction<BroadcastMessage, TimerKey>> {
        let timestamp = self.batched_messages.peek().map(|Record { timestamp, .. }| timestamp).unwrap_or(&now).clone();
        if now.duration_since(timestamp) >= SINGLE_MESSAGE_DELAY
            || self.batched_messages.len() >= self.batch_size {
            let messages: Vec<i64> = std::mem::replace(&mut self.batched_messages, BinaryHeap::new()).into_iter().map(|Record { value, .. }| value).collect();
            self.next_nodes
                .iter()
                .map(|node_id| {
                    send(
                        self.this_node.new_destination_address(node_id.clone()),
                        BroadcastMessage::Broadcast { message: MessageValue::Batch(messages.clone()) },
                    )
                })
                .collect()
        } else {
            let duration_until_expiration = now.duration_since(timestamp.add(SINGLE_MESSAGE_DELAY)).add(Duration::from_millis(1));
            vec![set_timer(duration_until_expiration, TimerKey::SendBatch)]
        }
    }
}

#[derive(Debug)]
enum TimerKey {
    SendBatch,
    CheckAck(i64),
}

impl Actor for BroadcastActor {
    type Msg = BroadcastMessage;
    type TimerKey = TimerKey;

    fn new(this_node: ThisNode) -> Result<Self> {
        let batch_size = ((this_node.node_ids.len() as f64) * (NEXT_NODES as f64) / (TARGET_OPS_PER_BROADCAST as f64)).ceil() as usize;
        debug!("Batch size {:?}", batch_size);

        let mut all_nodes = this_node.node_ids.clone();
        all_nodes.sort();
        let this_node_idx = all_nodes.binary_search(&this_node.node_id)
            .map_err(|_| Error::UnexpectedError(format!("Could not find node_id:{:?} in nodes_ids:{:?}", this_node.node_id, this_node.node_ids)))?;
        let bigger_nodes = all_nodes.split_off(this_node_idx);
        let next_nodes: Vec<NodeId> = bigger_nodes.into_iter().chain(all_nodes.into_iter()).skip(1).take(NEXT_NODES).collect();
        debug!("Next nodes: '{:?}'", next_nodes);

        Ok(BroadcastActor {
            batch_size,
            this_node,
            next_nodes,
            seen_messages: HashMap::new(),
            batched_messages: BinaryHeap::new(),
        })
    }

    fn on_request(&mut self, request: Message<Self::Msg>, now: Instant) -> Result<Vec<RunnerAction<Self::Msg, Self::TimerKey>>> {
        let (body, address) = request.body_and_address();
        match body {
            BroadcastMessage::Broadcast { message: MessageValue::Single(message) } => {
                let mut responses = self.observe_message(address.src.clone(), message, now);
                responses.extend(self.get_broadcast_message(now));
                responses.push(reply(address, BroadcastMessage::BroadcastOk));
                Ok(responses)
            }
            BroadcastMessage::Broadcast { message: MessageValue::Batch(messages) } => {
                let mut responses: Vec<_> = messages
                    .into_iter()
                    .flat_map(|message| self.observe_message(address.src.clone(), message, now))
                    .collect();
                responses.extend(self.get_broadcast_message(now));
                Ok(responses)
            }
            BroadcastMessage::Read => {
                let messages = self.seen_messages.clone().into_keys().collect();
                Ok(vec![reply(address, BroadcastMessage::ReadOk { messages })])
            }
            BroadcastMessage::Topology { .. } => {
                Ok(vec![reply(address, BroadcastMessage::TopologyOk)])
            }
            BroadcastMessage::BroadcastOk => Err(UnexpectedMessage("BroadcastOk".to_string())),
            BroadcastMessage::TopologyOk => Err(UnexpectedMessage("TopologyOk".to_string())),
            BroadcastMessage::ReadOk { .. } => Err(UnexpectedMessage("ReadOk".to_string()))
        }
    }

    fn on_timeout(&mut self, timer_key: Self::TimerKey, now: Instant) -> Result<Vec<RunnerAction<Self::Msg, Self::TimerKey>>> {
        match timer_key {
            TimerKey::SendBatch => Ok(self.get_broadcast_message(now)),
            TimerKey::CheckAck(message) => {
                let node_ids = self.seen_messages.get(&message).ok_or(Error::UnexpectedError(format!("There must be nodes for the message {:?}", message)))?;
                if node_ids.len() < NEXT_NODES {
                    self.batched_messages.push(Record { timestamp: now, value: message });
                    let mut responses = self.get_broadcast_message(now);
                    responses.push(set_timer(MAX_ACK_DELAY, TimerKey::CheckAck(message)));
                    Ok(responses)
                } else {
                    Ok(vec![])
                }
            }
        }
    }
}

fn main() -> Result<()> {
    run_actor::<BroadcastActor>()
}
