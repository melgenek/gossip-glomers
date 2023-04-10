use serde::{Deserialize, Serialize};
use crate::common::message::{MessageId, NodeId};

pub struct ReplyAddress {
    src: NodeId,
    dest: NodeId,
    in_reply_to: MessageId,
}

#[derive(Debug)]
pub struct MessageAddress {
    pub src: NodeId,
    pub dest: NodeId,
    pub msg_id: MessageId,
}

impl MessageAddress {
    pub fn to_reply_address(self) -> ReplyAddress {
        ReplyAddress {
            src: self.dest,
            dest: self.src,
            in_reply_to: self.msg_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
enum MessageBody<A> {
    Request {
        msg_id: MessageId,
        #[serde(flatten)]
        value: A,
    },
    Reply {
        in_reply_to: MessageId,
        #[serde(flatten)]
        value: A,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Message<A> {
    src: NodeId,
    dest: NodeId,
    body: MessageBody<A>,
}

impl<A> Message<A> {
    pub fn new_request(address: MessageAddress, value: A) -> Message<A> {
        Message {
            src: address.src,
            dest: address.dest,
            body: MessageBody::Request {
                msg_id: address.msg_id,
                value,
            },
        }
    }

    pub fn new_reply(address: ReplyAddress, value: A) -> Message<A> {
        Message {
            src: address.src,
            dest: address.dest,
            body: MessageBody::Reply {
                in_reply_to: address.in_reply_to,
                value,
            },
        }
    }

    pub fn body_and_address(self) -> (A, MessageAddress) {
        let Message { src, dest, body } = self;
        let (msg_id, value) = match body {
            MessageBody::Request { msg_id, value } => (msg_id, value),
            MessageBody::Reply { in_reply_to, value } => (in_reply_to, value)
        };
        (value, MessageAddress {
            src,
            dest,
            msg_id,
        })
    }

    pub fn address(&self) -> MessageAddress {
        let Message { src, dest, body } = self;
        let msg_id = match body {
            MessageBody::Request { msg_id, .. } => msg_id,
            MessageBody::Reply { in_reply_to, .. } => in_reply_to
        };
        MessageAddress {
            src: src.clone(),
            dest: dest.clone(),
            msg_id: msg_id.clone(),
        }
    }
}
