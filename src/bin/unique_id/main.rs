mod message;

use gossip_glomers::common::message::Message;
use gossip_glomers::common::runner::Runner;
use gossip_glomers::common::error::Result;
use crate::message::{GenerateRequest, GenerateResponse};

fn main() -> Result<()> {
    let runner: Runner<GenerateRequest, GenerateResponse> = Runner::new();

    let mut counter: u64 = 1;
    runner.run(|this_node, message| {
        match message.body {
            GenerateRequest { msg_id } => {
                let new_id = format!("{}_{}", this_node.node_id.0, counter);
                counter += 1;
                Message {
                    src: message.dest,
                    dest: message.src,
                    body: GenerateResponse { in_reply_to: msg_id, id: new_id },
                }
            }
        }
    })
}
