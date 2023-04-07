pub mod init;
pub mod req_resp;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NodeId(pub String);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MessageId(pub u64);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Message<A> {
    pub src: NodeId,
    pub dest: NodeId,
    pub body: A,
}

