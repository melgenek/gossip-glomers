use std::io::{self, Write};

use log::debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use stderrlog::{ColorChoice, LogLevelNum, Timestamp};

use crate::common::actor::{Action, Actor};

use super::error::Result;
use super::message::init::{INIT_RESPONSE_VALUE_INSTANCE, InitRequest, InitRequestValue};
use super::message::Message;
use super::message::req_resp::{Request, Response};
use super::this_node::ThisNode;

pub fn run_actor<A>() -> Result<()>
    where A: Actor {
    stderrlog::new()
        .verbosity(LogLevelNum::Trace)
        .timestamp(Timestamp::Microsecond)
        .color(ColorChoice::Always)
        .init()
        .unwrap();

    let this_node = init()?;
    let mut actor = A::new(&this_node);

    loop {
        let request: Message<A::Req> = read_request()?;
        debug!("Got request: '{:?}'", request);

        let actions = actor.on_request(request.body);

        for action in actions {
            match action {
                Action::Reply { msg_id, value } => {
                    let response = Message {
                        src: this_node.node_id.clone(),
                        dest: request.src.clone(),
                        body: Response {
                            in_reply_to: msg_id,
                            value,
                        },
                    };
                    debug!("Writing reply: '{:?}'", response);
                    write_response(&response)?;
                }
                Action::AskForReply { .. } => {}
                Action::SendAndForget { .. } => {}
            }
        }
    }
}

fn init() -> Result<ThisNode> {
    let message: Message<InitRequest> = read_request()?;
    debug!("Got init request: '{:?}'", message);
    match message.body {
        Request { msg_id, value: InitRequestValue { node_id, node_ids } } => {
            let this_node = ThisNode {
                node_id,
                node_ids,
            };

            let init_response = Message {
                src: this_node.node_id.clone(),
                dest: message.src,
                body: Response {
                    in_reply_to: msg_id,
                    value: INIT_RESPONSE_VALUE_INSTANCE,
                },
            };

            debug!("Writing init response: '{:?}'", init_response);
            write_response(&init_response)?;

            Ok(this_node)
        }
    }
}

fn read_request<A>() -> Result<Message<A>>
    where A: DeserializeOwned {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line)?;
    let message: Message<A> = serde_json::from_str(&line)?;
    Ok(message)
}

fn write_response<A>(response: &Message<A>) -> Result<()> where A: Serialize {
    let response_bytes = serde_json::to_vec(&response)?;
    let mut out_lock = io::stdout().lock();
    out_lock.write_all(response_bytes.as_slice())?;
    out_lock.write_all(&[b'\n'])?;
    Ok(())
}
