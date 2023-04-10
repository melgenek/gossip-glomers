use std::collections::{BTreeSet, HashMap};
use std::time::Duration;
use log::debug;

use gossip_glomers::common::actor::Actor;
use gossip_glomers::common::error::Error::UnexpectedMessage;
use gossip_glomers::common::error::Result;
use gossip_glomers::common::message::{MessageId, NodeId};
use gossip_glomers::common::message::message::Message;
use gossip_glomers::common::runner::{reply, run_actor, RunnerAction, send, set_timer};
use gossip_glomers::common::this_node::ThisNode;

use crate::message::BroadcastMessage;

mod message;

struct BroadcastActor {
    this_node: ThisNode,
    topology: HashMap<NodeId, BTreeSet<NodeId>>,
    seen_messages: BTreeSet<i64>,
    in_flight_broadcasts: HashMap<NodeId, HashMap<MessageId, i64>>,
}

impl BroadcastActor {
    fn get_adjacent_nodes(&self) -> &BTreeSet<NodeId> {
        &self.topology[&self.this_node.node_id]
    }
}

impl Actor for BroadcastActor {
    type Msg = BroadcastMessage;
    type TimerKey = (NodeId, MessageId);

    fn new(this_node: ThisNode) -> Self {
        let total_topology: Vec<(NodeId, BTreeSet<NodeId>)> = this_node.node_ids.iter().map(|node_id| {
            let mut adjacent_nodes = this_node.node_ids.clone();
            adjacent_nodes.remove(node_id);
            (node_id.clone(), adjacent_nodes)
        }).collect();
        BroadcastActor {
            this_node,
            topology: HashMap::from_iter(total_topology),
            seen_messages: BTreeSet::new(),
            in_flight_broadcasts: HashMap::new(),
        }
    }

    fn on_request(&mut self, request: Message<Self::Msg>) -> Result<Vec<RunnerAction<Self::Msg, Self::TimerKey>>> {
        let (body, address) = request.body_and_address();
        match body {
            BroadcastMessage::Topology { topology } => {
                self.topology = topology;
                Ok(vec![reply(address, BroadcastMessage::TopologyOk)])
            }
            BroadcastMessage::Broadcast { message } => {
                let is_message_new = self.seen_messages.insert(message);

                let mut messages = if is_message_new {
                    let actions: Vec<RunnerAction<Self::Msg, Self::TimerKey>> = self.get_adjacent_nodes().iter()
                        .flat_map(|node_id| {
                            if node_id != &address.src {
                                let dest_addr = self.this_node.new_destination_address(node_id.clone());
                                vec![
                                    set_timer(Duration::from_millis(1000), (dest_addr.dest.clone(), dest_addr.msg_id.clone())),
                                    send(dest_addr, BroadcastMessage::Broadcast { message }),
                                ]
                            } else {
                                vec![]
                            }
                        })
                        .collect();

                    for action in actions.iter() {
                        match action {
                            RunnerAction::SendMessage(msg) => {
                                let address = msg.address();
                                self.in_flight_broadcasts.entry(address.dest).or_default().insert(address.msg_id, message);
                            }
                            RunnerAction::SetTimer { .. } => {}
                        }
                    }

                    actions
                } else {
                    vec![]
                };
                messages.push(reply(address, BroadcastMessage::BroadcastOk));
                Ok(messages)
            }
            BroadcastMessage::BroadcastOk => {
                self.in_flight_broadcasts.entry(address.src).or_default().remove(&address.msg_id);
                Ok(vec![])
            }
            BroadcastMessage::Read => {
                Ok(vec![reply(address, BroadcastMessage::ReadOk { messages: self.seen_messages.clone() })])
            }
            BroadcastMessage::TopologyOk => Err(UnexpectedMessage("TopologyOk".to_string())),
            BroadcastMessage::ReadOk { .. } => Err(UnexpectedMessage("ReadOk".to_string()))
        }
    }

    fn on_timer(&mut self, timer_key: Self::TimerKey) -> Result<Vec<RunnerAction<Self::Msg, Self::TimerKey>>> {
        let (node_id, msg_id) = timer_key;

        let node_msgs = self.in_flight_broadcasts.entry(node_id.clone()).or_default();
        if let Some(message) = node_msgs.remove(&msg_id) {
            debug!("Resending the message '{:?}' to node '{:?}'", message, node_id);
            let dest_addr = self.this_node.new_destination_address(node_id);
            node_msgs.insert(dest_addr.msg_id.clone(), message);
            Ok(vec![
                set_timer(Duration::from_millis(1000), (dest_addr.dest.clone(), dest_addr.msg_id.clone())),
                send(dest_addr, BroadcastMessage::Broadcast { message }),
            ])
        } else {
            debug!("No need to resend a message with id '{:?}' to node '{:?}'", msg_id, node_id);
            Ok(vec![])
        }
    }
}

fn main() -> Result<()> {
    run_actor::<BroadcastActor>()
}
