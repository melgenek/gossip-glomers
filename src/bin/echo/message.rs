use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "echo")]
pub enum EchoMessage {
    #[serde(rename = "echo")]
    Echo {
        echo: String,
    },
    #[serde(rename = "echo_ok")]
    EchoOk {
        echo: String,
    },
}

#[cfg(test)]
mod tests {
    use gossip_glomers::common::error::Result;
    use gossip_glomers::common::message::{MessageId, NodeId};
    use gossip_glomers::common::message::message::{Message, MessageAddress};
    use crate::message::EchoMessage;

    #[test]
    fn should_deserialize_init() -> Result<()> {
        let str = r#"{"id":0,"src":"c0","dest":"n0","body":{"type":"echo","echo":"text","msg_id":1}}"#;

        let result: Message<EchoMessage> = serde_json::from_str(&str)?;

        assert_eq!(result, Message::new_request(MessageAddress {
            src: NodeId("c0".to_string()),
            dest: NodeId("n0".to_string()),
            msg_id: MessageId(1),
        }, EchoMessage::Echo {
            echo: "text".to_string(),
        }));
        Ok(())
    }

    #[test]
    fn should_serialize_init_ok() -> Result<()> {
        let expected = r#"{"src":"n0","dest":"c0","body":{"in_reply_to":1,"type":"echo_ok","echo":"text"}}"#;

        let result = serde_json::to_string(&Message::new_reply(MessageAddress {
            src: NodeId("c0".to_string()),
            dest: NodeId("n0".to_string()),
            msg_id: MessageId(1),
        }.to_reply_address(), EchoMessage::EchoOk {
            echo: "text".to_string()
        }))?;
        assert_eq!(result, expected);
        Ok(())
    }
}
