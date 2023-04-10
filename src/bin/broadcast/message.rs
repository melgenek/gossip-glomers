use std::collections::{BTreeSet, HashMap};

use serde::{Deserialize, Serialize};

use gossip_glomers::common::message::NodeId;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum BroadcastMessage {
    #[serde(rename = "topology")]
    Topology {
        topology: HashMap<NodeId, BTreeSet<NodeId>>,
    },
    #[serde(rename = "topology_ok")]
    TopologyOk,
    #[serde(rename = "broadcast")]
    Broadcast {
        message: i64,
    },
    #[serde(rename = "broadcast_ok")]
    BroadcastOk,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "read_ok")]
    ReadOk {
        messages: BTreeSet<i64>,
    },
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, HashMap};

    use gossip_glomers::common::error::Result;
    use gossip_glomers::common::message::{MessageId, NodeId};
    use gossip_glomers::common::message::message::{MessageAddress, Message};

    use crate::message::BroadcastMessage;

    #[test]
    fn should_deserialize_topology() -> Result<()> {
        let str = r#"{"id":0,"src":"c0","dest":"n0","body":{
          "msg_id": 1,
          "type": "topology",
          "topology": {
            "n1": ["n2", "n3"],
            "n2": ["n1"],
            "n3": ["n1"]
          }
        }}"#;

        let result: Message<BroadcastMessage> = serde_json::from_str(&str)?;

        assert_eq!(result, Message::new_request(MessageAddress {
            src: NodeId("c0".to_string()),
            dest: NodeId("n0".to_string()),
            msg_id: MessageId(1),
        }, BroadcastMessage::Topology {
            topology: HashMap::from([
                (NodeId(
                    "n1".to_string()), BTreeSet::from([NodeId("n2".to_string()), NodeId("n3".to_string())])),
                (NodeId("n2".to_string()), BTreeSet::from([NodeId("n1".to_string())])),
                (NodeId("n3".to_string()), BTreeSet::from([NodeId("n1".to_string())]))
            ])
        }));
        Ok(())
    }

    #[test]
    fn should_deserialize_broadcast() -> Result<()> {
        let str = r#"{"id":0,"src":"c0","dest":"n0","body":{"msg_id": 1,"type": "broadcast","message": 1000}}"#;

        let result: Message<BroadcastMessage> = serde_json::from_str(&str)?;

        assert_eq!(result, Message::new_request(MessageAddress {
            src: NodeId("c0".to_string()),
            dest: NodeId("n0".to_string()),
            msg_id: MessageId(1),
        }, BroadcastMessage::Broadcast {
            message: 1000
        }));
        Ok(())
    }

    #[test]
    fn should_serialize_broadcast() -> Result<()> {
        let expected = r#"{"src":"c0","dest":"n0","body":{"msg_id":1,"type":"broadcast","message":1000}}"#;

        let result = serde_json::to_string(&Message::new_request(MessageAddress {
            src: NodeId("c0".to_string()),
            dest: NodeId("n0".to_string()),
            msg_id: MessageId(1),
        }, BroadcastMessage::Broadcast {
            message: 1000
        }))?;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_read() -> Result<()> {
        let str = r#"{"id":0,"src":"c0","dest":"n0","body":{
          "msg_id": 1,
          "type": "read"
        }}"#;

        let result: Message<BroadcastMessage> = serde_json::from_str(&str)?;

        assert_eq!(result, Message::new_request(MessageAddress {
            src: NodeId("c0".to_string()),
            dest: NodeId("n0".to_string()),
            msg_id: MessageId(1),
        }, BroadcastMessage::Read));
        Ok(())
    }

    #[test]
    fn should_serialize_topology_ok() -> Result<()> {
        let expected = r#"{"src":"n0","dest":"c0","body":{"in_reply_to":1,"type":"topology_ok"}}"#;

        let result = serde_json::to_string(&Message::new_reply(MessageAddress {
            src: NodeId("c0".to_string()),
            dest: NodeId("n0".to_string()),
            msg_id: MessageId(1),
        }.to_reply_address(), BroadcastMessage::TopologyOk))?;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn should_deserialize_broadcast_ok() -> Result<()> {
        let str = r#"{"src":"n0","dest":"c0","body":{"in_reply_to":1,"type":"broadcast_ok"}}"#;

        let result: Message<BroadcastMessage> = serde_json::from_str(&str)?;

        assert_eq!(result, Message::new_reply(MessageAddress {
            src: NodeId("c0".to_string()),
            dest: NodeId("n0".to_string()),
            msg_id: MessageId(1),
        }.to_reply_address(), BroadcastMessage::BroadcastOk));
        Ok(())
    }

    #[test]
    fn should_serialize_broadcast_ok() -> Result<()> {
        let expected = r#"{"src":"n0","dest":"c0","body":{"in_reply_to":1,"type":"broadcast_ok"}}"#;

        let result = serde_json::to_string(&Message::new_reply(MessageAddress {
            src: NodeId("c0".to_string()),
            dest: NodeId("n0".to_string()),
            msg_id: MessageId(1),
        }.to_reply_address(), BroadcastMessage::BroadcastOk))?;

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn should_serialize_read_ok() -> Result<()> {
        let expected = r#"{"src":"n0","dest":"c0","body":{"in_reply_to":1,"type":"read_ok","messages":[1,8,25,72]}}"#;

        let result = serde_json::to_string(&Message::new_reply(MessageAddress {
            src: NodeId("c0".to_string()),
            dest: NodeId("n0".to_string()),
            msg_id: MessageId(1),
        }.to_reply_address(), BroadcastMessage::ReadOk {
            messages: BTreeSet::from([1, 8, 72, 25])
        }))?;

        assert_eq!(result, expected);
        Ok(())
    }
}
