mod message;

use gossip_glomers::common::message::Message;
use gossip_glomers::common::runner::Runner;
use gossip_glomers::common::error::Result;
use gossip_glomers::common::message::req_resp::{Request, Response};
use crate::message::{EchoRequest, EchoRequestValue, EchoResponse, EchoResponseValue};

fn main() -> Result<()> {
    let runner: Runner<EchoRequest, EchoResponse> = Runner::new();

    runner.run(|_, message| {
        match message.body {
            Request { msg_id, value: EchoRequestValue { echo } } => {
                Message {
                    src: message.dest,
                    dest: message.src,
                    body: Response { in_reply_to: msg_id, value: EchoResponseValue { echo } },
                }
            }
        }
    })
}
