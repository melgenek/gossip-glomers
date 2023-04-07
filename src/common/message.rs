use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NodeId(pub String);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "init")]
pub struct InitRequest {
    pub msg_id: u64,
    pub node_id: NodeId,
    pub node_ids: Vec<NodeId>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "init_ok")]
pub struct InitResponse {
    pub in_reply_to: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Message<A> {
    pub src: NodeId,
    pub dest: NodeId,
    pub body: A,
}

impl<A> Message<A> {}


#[cfg(test)]
mod tests {
    use crate::common::error::Result;
    use crate::common::message::{InitRequest, InitResponse, Message, NodeId};

    #[test]
    fn should_deserialize_init() -> Result<()> {
        let str = r#"{"id":0,"src":"c0","dest":"n0","body":{"type":"init","node_id":"n0","node_ids":["n0"],"msg_id":1}}"#;

        let result: Message<InitRequest> = serde_json::from_str(&str)?;

        assert_eq!(result, Message {
            src: NodeId("c0".to_string()),
            dest: NodeId("n0".to_string()),
            body: InitRequest {
                msg_id: 1,
                node_id: NodeId("n0".to_string()),
                node_ids: vec![NodeId("n0".to_string())],
            },
        });
        Ok(())
    }

    #[test]
    fn should_serialize_init_ok() -> Result<()> {
        let expected = r#"{"src":"n0","dest":"c0","body":{"type":"init_ok","in_reply_to":1}}"#;

        let result = serde_json::to_string(&Message {
            src: NodeId("n0".to_string()),
            dest: NodeId("c0".to_string()),
            body: InitResponse {
                in_reply_to: 1,
            },
        })?;

        assert_eq!(result, expected);
        Ok(())
    }
}

