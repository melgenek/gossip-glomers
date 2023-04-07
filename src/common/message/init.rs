use serde::{Deserialize, Serialize};

use crate::common::message::NodeId;
use crate::common::message::req_resp::Request;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "init")]
pub struct InitRequestValue {
    pub node_id: NodeId,
    pub node_ids: Vec<NodeId>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "init_ok")]
pub struct InitResponseValue {
    #[serde(flatten)]
    n: (),
}

pub const INIT_RESPONSE_VALUE_INSTANCE: InitResponseValue = InitResponseValue { n: () };

pub type InitRequest = Request<InitRequestValue>;

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::common::error::Result;
    use crate::common::message::{Message, MessageId, NodeId};
    use crate::common::message::init::{INIT_RESPONSE_VALUE_INSTANCE, InitRequest, InitRequestValue, InitResponseValue};
    use crate::common::message::req_resp::{Request, Response};

    #[test]
    fn should_deserialize_init() -> Result<()> {
        let str = r#"{"id":0,"src":"c0","dest":"n0","body":{"type":"init","node_id":"n0","node_ids":["n0"],"msg_id":1}}"#;

        let result: Message<InitRequest> = serde_json::from_str(&str)?;

        assert_eq!(result, Message {
            src: NodeId("c0".to_string()),
            dest: NodeId("n0".to_string()),
            body: Request {
                msg_id: MessageId(1),
                value: InitRequestValue {
                    node_id: NodeId("n0".to_string()),
                    node_ids: vec![NodeId("n0".to_string())],
                },
            },
        });

        Ok(())
    }

    #[test]
    fn should_serialize_init_ok() -> Result<()> {
        let expected = r#"{"src":"n0","dest":"c0","body":{"in_reply_to":1,"type":"init_ok"}}"#;

        let result = serde_json::to_string(&Message {
            src: NodeId("n0".to_string()),
            dest: NodeId("c0".to_string()),
            body: Response {
                in_reply_to: MessageId(1),
                value: INIT_RESPONSE_VALUE_INSTANCE,
            },
        })?;

        assert_eq!(result, expected);
        Ok(())
    }
}

