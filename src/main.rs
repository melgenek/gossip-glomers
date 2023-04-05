use std::io::{self, BufReader, Write};
use std::marker::PhantomData;
use std::net::Shutdown::Read;

use log::error;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::error::{CommonError, CommonResult};

mod error;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct NodeId(String);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "init")]
struct InitRequest {
    msg_id: u64,
    node_id: NodeId,
    node_ids: Vec<NodeId>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "init_ok")]
struct InitResponse {
    in_reply_to: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "echo")]
struct EchoRequest {
    msg_id: u64,
    echo: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "echo_ok")]
struct EchoResponse {
    in_reply_to: u64,
    echo: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "generate")]
struct GenerateRequest {
    msg_id: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename = "generate_ok")]
struct GenerateResponse {
    in_reply_to: u64,
    id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Message<A> {
    src: NodeId,
    dest: NodeId,
    body: A,
}

fn read_request<A>() -> CommonResult<Message<A>>
    where A: DeserializeOwned {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line)?;
    let message: Message<A> = serde_json::from_str(&line)?;
    Ok(message)
}

fn write_response<A>(response: &Message<A>) -> CommonResult<()>
    where A: Serialize {
    let response_bytes = serde_json::to_vec(&response)?;
    let mut out_lock = io::stdout().lock();
    out_lock.write_all(response_bytes.as_slice())?;
    out_lock.write_all(&[b'\n'])?;
    Ok(())
}

#[derive(Debug)]
struct ThisNode {
    node_id: NodeId,
    node_ids: Vec<NodeId>,
}

impl ThisNode {
    fn init() -> CommonResult<ThisNode> {
        let message: Message<InitRequest> = read_request()?;
        match message.body {
            InitRequest { msg_id, node_id, node_ids } => {
                let this_node = ThisNode {
                    node_id,
                    node_ids,
                };
                write_response(&Message {
                    src: this_node.node_id.clone(),
                    dest: message.src,
                    body: InitResponse { in_reply_to: msg_id },
                })?;
                Ok(this_node)
            }
        }
    }
}

fn main() -> CommonResult<()> {
    env_logger::init();

    let this_node = ThisNode::init()?;
    error!("This node {:?}", this_node);

    // loop {
    //     let request: Message<EchoRequest> = read_request()?;
    //     error!("Got request: '{:?}'", request);
    //
    //     write_response(&Message {
    //         src: request.dest.clone(),
    //         dest: request.src,
    //         body: EchoResponse { in_reply_to: request.body.msg_id, echo: request.body.echo },
    //     })?;
    // }

    let mut counter: u64 = 1;

    loop {
        let request: Message<GenerateRequest> = read_request()?;
        error!("Got request: '{:?}'", request);

        let new_id = format!("{}_{}", this_node.node_id.0, counter);
        counter += 1;
        write_response(&Message {
            src: request.dest.clone(),
            dest: request.src,
            body: GenerateResponse { in_reply_to: request.body.msg_id, id: new_id },
        })?;
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::{InitRequest, InitResponse, Message, NodeId};
    use crate::error::{CommonError, CommonResult};

    #[test]
    fn should_deserialize_init() -> CommonResult<()> {
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
    fn should_serialize_init_ok() -> CommonResult<()> {
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

