use std::fmt::Debug;
use std::io::{self, Write};
use std::marker::PhantomData;

use log::debug;
use serde::de::DeserializeOwned;
use serde::Serialize;
use stderrlog::{ColorChoice, LogLevelNum, Timestamp};

use super::error::Result;
use super::message::{InitRequest, InitResponse};
use super::message::Message;
use super::this_node::ThisNode;

pub struct Runner<Req, Resp> {
    req: PhantomData<Req>,
    resp: PhantomData<Resp>,
}

impl<Req, Resp> Runner<Req, Resp> {
    pub fn new() -> Runner<Req, Resp> {
        stderrlog::new()
            .verbosity(LogLevelNum::Trace)
            .timestamp(Timestamp::Microsecond)
            .color(ColorChoice::Always)
            .init()
            .unwrap();
        Runner { req: PhantomData, resp: PhantomData }
    }

    pub fn run<F>(&self, mut f: F) -> Result<()>
        where F: FnMut(&ThisNode, Message<Req>) -> Message<Resp>,
              Req: Debug + DeserializeOwned,
              Resp: Debug + Serialize
    {
        let this_node = init()?;

        loop {
            let request: Message<Req> = read_request()?;
            debug!("Got request: '{:?}'", request);

            let response = f(&this_node, request);

            debug!("Writing response: '{:?}'", response);
            write_response(&response)?;
        }
    }
}

fn init() -> Result<ThisNode> {
    let message: Message<InitRequest> = read_request()?;
    debug!("Got init request: '{:?}'", message);
    match message.body {
        InitRequest { msg_id, node_id, node_ids } => {
            let this_node = ThisNode {
                node_id,
                node_ids,
            };
            let init_response = Message {
                src: this_node.node_id.clone(),
                dest: message.src,
                body: InitResponse { in_reply_to: msg_id },
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
