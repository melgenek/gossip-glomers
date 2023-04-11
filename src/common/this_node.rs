use std::cell::RefCell;

use crate::common::message::MessageId;
use crate::common::message::message::MessageAddress;

use super::message::NodeId;

#[derive(Debug)]
pub struct ThisNode {
    pub node_id: NodeId,
    pub node_ids: Vec<NodeId>,
    outbound_message_id: RefCell<MessageId>,
}

impl ThisNode {
    pub fn new(node_id: NodeId, node_ids: Vec<NodeId>) -> ThisNode {
        ThisNode {
            node_id,
            node_ids,
            outbound_message_id: RefCell::new(MessageId(1)),
        }
    }

    pub fn new_destination_address(&self, dest: NodeId) -> MessageAddress {
        let msg_id = self.outbound_message_id.replace_with(|value| value.inc());
        MessageAddress {
            src: self.node_id.clone(),
            dest,
            msg_id,
        }
    }
}
