mod message;

use gossip_glomers::common::message::Message;
use gossip_glomers::common::runner::Runner;
use gossip_glomers::common::error::Result;
use crate::message::{EchoRequest, EchoResponse};

fn main() -> Result<()> {
    let runner: Runner<EchoRequest, EchoResponse> = Runner::new();

    runner.run(|_, message| {
        match message.body {
            EchoRequest { msg_id, echo } => {
                Message {
                    src: message.dest,
                    dest: message.src,
                    body: EchoResponse { in_reply_to: msg_id, echo },
                }
            }
        }
    })
}
