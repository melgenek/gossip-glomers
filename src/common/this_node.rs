use super::message::NodeId;

#[derive(Debug)]
pub struct ThisNode {
    pub node_id: NodeId,
    pub node_ids: Vec<NodeId>,
}

impl ThisNode {}
