use std::time::Instant;
use gossip_glomers::common::actor::Actor;
use gossip_glomers::common::error::Error::UnexpectedMessage;
use gossip_glomers::common::error::Result;
use gossip_glomers::common::message::message::Message;
use gossip_glomers::common::runner::{reply, run_actor, RunnerAction};
use gossip_glomers::common::this_node::ThisNode;

use crate::message::GenerateMessage;

mod message;

struct UniqueIdActor {
    this_node: ThisNode,
    counter: u64,
}

impl Actor for UniqueIdActor {
    type Msg = GenerateMessage;
    type TimerKey = ();

    fn new(this_node: ThisNode) -> Result<Self> {
        Ok(UniqueIdActor {
            this_node,
            counter: 0,
        })
    }

    fn on_request(&mut self, request: Message<Self::Msg>, _now: Instant) -> Result<Vec<RunnerAction<Self::Msg, Self::TimerKey>>> {
        let (body, address) = request.body_and_address();
        match body {
            GenerateMessage::Generate => {
                let id = format!("{}_{}", self.this_node.node_id, self.counter);
                self.counter += 1;
                Ok(vec![
                    reply(address, GenerateMessage::GenerateOk { id })
                ])
            }
            GenerateMessage::GenerateOk { .. } => Err(UnexpectedMessage("GenerateOk".to_string()))
        }
    }
}

fn main() -> Result<()> {
    run_actor::<UniqueIdActor>()
}
