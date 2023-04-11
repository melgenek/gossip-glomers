use serde::{Deserialize, Serialize};

use crate::common::message::NodeId;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum InitMessage {
    #[serde(rename = "init")]
    Init {
        node_id: NodeId,
        node_ids: Vec<NodeId>,
    },
    #[serde(rename = "init_ok")]
    InitOk,
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use crate::common::error::Result;
    use crate::common::message::{MessageId, NodeId};
    use crate::common::message::init::{InitMessage};
    use crate::common::message::message::{Message, MessageAddress};

    #[test]
    fn should_deserialize_init() -> Result<()> {
        let str = r#"{"id":0,"src":"c0","dest":"n0","body":{"type":"init","node_id":"n0","node_ids":["n0"],"msg_id":1}}"#;

        let result: Message<InitMessage> = serde_json::from_str(&str)?;

        assert_eq!(result, Message::new_request(MessageAddress {
            src: NodeId::from("c0"),
            dest: NodeId::from("n0"),
            msg_id: MessageId(1),
        }, InitMessage::Init {
            node_id: NodeId::from("n0"),
            node_ids: vec![NodeId::from("n0")],
        }));

        Ok(())
    }

    #[test]
    fn should_serialize_init_ok() -> Result<()> {
        let expected = r#"{"src":"n0","dest":"c0","body":{"in_reply_to":1,"type":"init_ok"}}"#;

        let result = serde_json::to_string(&Message::new_reply(MessageAddress {
            src: NodeId::from("c0"),
            dest: NodeId::from("n0"),
            msg_id: MessageId(1),
        }.to_reply_address(), InitMessage::InitOk))?;

        assert_eq!(result, expected);
        Ok(())
    }
}

