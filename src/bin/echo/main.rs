use gossip_glomers::common::actor::Actor;
use gossip_glomers::common::error::Error::UnexpectedMessage;
use gossip_glomers::common::error::Result;
use gossip_glomers::common::message::message::Message;
use gossip_glomers::common::runner::{reply, run_actor, RunnerAction};
use gossip_glomers::common::this_node::ThisNode;

use crate::message::EchoMessage;

mod message;

struct EchoActor;

impl Actor for EchoActor {
    type Msg = EchoMessage;
    type TimerKey = ();

    fn new(_: ThisNode) -> Self {
        EchoActor
    }

    fn on_request(&mut self, request: Message<Self::Msg>) -> Result<Vec<RunnerAction<Self::Msg, Self::TimerKey>>> {
        let (body, address) = request.body_and_address();
        match body {
            EchoMessage::Echo { echo } => {
                Ok(vec![
                    reply(address, EchoMessage::EchoOk { echo })
                ])
            }
            EchoMessage::EchoOk { .. } => Err(UnexpectedMessage("EchoOk".to_string()))
        }
    }
}

fn main() -> Result<()> {
    run_actor::<EchoActor>()
}
