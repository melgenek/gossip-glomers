use serde::{Deserialize, Serialize};
use gossip_glomers::common::message::req_resp::{Request, Response};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "echo")]
pub struct EchoRequestValue {
    pub echo: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "echo_ok")]
pub struct EchoResponseValue {
    pub echo: String,
}

pub type EchoRequest = Request<EchoRequestValue>;
pub type EchoResponse = Response<EchoResponseValue>;

#[cfg(test)]
mod tests {
    use gossip_glomers::common::error::Result;
    use gossip_glomers::common::message::{Message, MessageId, NodeId};
    use gossip_glomers::common::message::req_resp::{Request, Response};

    use crate::message::{EchoRequest, EchoRequestValue, EchoResponse, EchoResponseValue};

    #[test]
    fn should_deserialize_init() -> Result<()> {
        let str = r#"{"id":0,"src":"c0","dest":"n0","body":{"type":"echo","echo":"text","msg_id":1}}"#;

        let result: Message<EchoRequest> = serde_json::from_str(&str)?;

        assert_eq!(result, Message {
            src: NodeId("c0".to_string()),
            dest: NodeId("n0".to_string()),
            body: Request {
                msg_id: MessageId(1),
                value: EchoRequestValue {
                    echo: "text".to_string(),
                },
            },
        });
        Ok(())
    }

    #[test]
    fn should_serialize_init_ok() -> Result<()> {
        let expected = r#"{"src":"n0","dest":"c0","body":{"in_reply_to":1,"type":"echo_ok","echo":"text"}}"#;

        let result = serde_json::to_string(&Message {
            src: NodeId("n0".to_string()),
            dest: NodeId("c0".to_string()),
            body: Response {
                in_reply_to: MessageId(1),
                value: EchoResponseValue {
                    echo: "text".to_string()
                },
            },
        })?;

        assert_eq!(result, expected);
        Ok(())
    }
}
