pub mod init;
pub mod message;

use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(untagged)]
#[serde(from = "&str")]
pub enum NodeId {
    Server(String),
    Client(String),
}

impl Display for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            NodeId::Server(v) => v,
            NodeId::Client(v) => v
        })
    }
}

impl From<&str> for NodeId {
    fn from(value: &str) -> Self {
        if value.starts_with("c") {
            NodeId::Client(value.to_string())
        } else {
            NodeId::Server(value.to_string())
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageId(pub u64);

impl MessageId {
    pub fn inc(&self) -> MessageId {
        MessageId(self.0 + 1)
    }
}

