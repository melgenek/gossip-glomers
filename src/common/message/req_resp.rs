use serde::{Deserialize, Serialize};
use crate::common::message::MessageId;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Request<A> {
    pub msg_id: MessageId,
    #[serde(flatten)]
    pub value: A,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Response<A> {
    pub in_reply_to: MessageId,
    #[serde(flatten)]
    pub value: A,
}
