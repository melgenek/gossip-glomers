use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::common::message::{MessageId, NodeId};
use crate::common::this_node::ThisNode;

pub enum Action<A> {
    Reply {
        msg_id: MessageId,
        value: A,
    },
    AskForReply {
        dest: NodeId,
        value: A,
    },
    SendAndForget {
        dest: NodeId,
        value: A,
    },
}

impl<A> Action<A> {
    pub fn reply(msg_id: MessageId, value: A) -> Action<A> {
        Action::Reply { msg_id, value }
    }
}

pub trait Actor {
    type Req: Debug + DeserializeOwned;
    type Resp: Debug + Serialize;

    fn new(this_node: &ThisNode) -> Self;

    fn on_request(&mut self, request: Self::Req) -> Vec<Action<Self::Resp>>;
}
