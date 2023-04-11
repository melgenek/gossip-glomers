use std::collections::{BinaryHeap, BTreeSet};
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
    seen_messages: BTreeSet<i64>,
    batched_messages: BinaryHeap<Record<i64>>,
}

impl BroadcastActor {
    fn observe_message(&mut self, message: i64, now: Instant) {
        if self.seen_messages.insert(message) {
            self.batched_messages.push(Record { timestamp: now, value: message });
        } else {
            debug!("Made a circle {:?}", message);
        }
    }

    fn get_broadcast_message(&mut self, now: Instant) -> Vec<RunnerAction<BroadcastMessage, ()>> {
        let timestamp = self.batched_messages.peek().map(|Record { timestamp, .. }| timestamp).unwrap_or(&now).clone();
        if now.duration_since(timestamp) >= SINGLE_MESSAGE_DELAY
            || self.batched_messages.len() >= self.batch_size {
            debug!("Batch is full {:?}", self.batched_messages.len() >= self.batch_size);
            let messages: Vec<i64> = std::mem::replace(&mut self.batched_messages, BinaryHeap::new()).into_iter().map(|Record { value, .. }| value).collect();
            // todo add to in-flight
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
            vec![set_timer(duration_until_expiration, ())]
        }
    }
}

impl Actor for BroadcastActor {
    type Msg = BroadcastMessage;
    type TimerKey = ();

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
            seen_messages: BTreeSet::new(),
            batched_messages: BinaryHeap::new(),
        })
    }

    fn on_request(&mut self, request: Message<Self::Msg>, now: Instant) -> Result<Vec<RunnerAction<Self::Msg, Self::TimerKey>>> {
        let (body, address) = request.body_and_address();
        match body {
            BroadcastMessage::Broadcast { message: MessageValue::Single(message) } => {
                self.observe_message(message, now);
                let mut messages = self.get_broadcast_message(now);
                messages.push(reply(address, BroadcastMessage::BroadcastOk));
                Ok(messages)
            }
            BroadcastMessage::Broadcast { message: MessageValue::Batch(messages) } => {
                for message in messages {
                    self.observe_message(message, now);
                }
                Ok(self.get_broadcast_message(now))
            }
            BroadcastMessage::Read => {
                Ok(vec![reply(address, BroadcastMessage::ReadOk { messages: self.seen_messages.clone() })])
            }
            BroadcastMessage::Topology { .. } => {
                Ok(vec![reply(address, BroadcastMessage::TopologyOk)])
            }
            BroadcastMessage::BroadcastOk => Err(UnexpectedMessage("BroadcastOk".to_string())),
            BroadcastMessage::TopologyOk => Err(UnexpectedMessage("TopologyOk".to_string())),
            BroadcastMessage::ReadOk { .. } => Err(UnexpectedMessage("ReadOk".to_string()))
        }
    }

    fn on_timeout(&mut self, _timer_key: Self::TimerKey, now: Instant) -> Result<Vec<RunnerAction<Self::Msg, Self::TimerKey>>> {
        Ok(self.get_broadcast_message(now))
    }
}

fn main() -> Result<()> {
    run_actor::<BroadcastActor>()
}
