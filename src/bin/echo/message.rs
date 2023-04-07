use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "echo")]
pub struct EchoRequest {
    pub msg_id: u64,
    pub echo: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "echo_ok")]
pub struct EchoResponse {
    pub in_reply_to: u64,
    pub echo: String,
}


#[cfg(test)]
mod tests {
    use gossip_glomers::common::error::Result;
    use gossip_glomers::common::message::{Message, NodeId};

    use crate::message::{EchoRequest, EchoResponse};

    #[test]
    fn should_deserialize_init() -> Result<()> {
        let str = r#"{"id":0,"src":"c0","dest":"n0","body":{"type":"echo","echo":"text","msg_id":1}}"#;

        let result: Message<EchoRequest> = serde_json::from_str(&str)?;

        assert_eq!(result, Message {
            src: NodeId("c0".to_string()),
            dest: NodeId("n0".to_string()),
            body: EchoRequest {
                msg_id: 1,
                echo: "text".to_string(),
            },
        });
        Ok(())
    }

    #[test]
    fn should_serialize_init_ok() -> Result<()> {
        let expected = r#"{"src":"n0","dest":"c0","body":{"type":"echo_ok","in_reply_to":1,"echo":"text"}}"#;

        let result = serde_json::to_string(&Message {
            src: NodeId("n0".to_string()),
            dest: NodeId("c0".to_string()),
            body: EchoResponse {
                in_reply_to: 1,
                echo: "text".to_string(),
            },
        })?;

        assert_eq!(result, expected);
        Ok(())
    }
}
