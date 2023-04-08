use gossip_glomers::common::actor::{Action, Actor};
use gossip_glomers::common::error::Result;
use gossip_glomers::common::runner::run_actor;
use gossip_glomers::common::this_node::ThisNode;

use crate::message::{EchoRequest, EchoResponseValue};

mod message;

struct EchoActor;

impl Actor for EchoActor {
    type Req = EchoRequest;
    type Resp = EchoResponseValue;

    fn new(_: &ThisNode) -> Self {
        EchoActor
    }

    fn on_request(&mut self, request: Self::Req) -> Vec<Action<Self::Resp>> {
        vec![
            Action::reply(request.msg_id, EchoResponseValue { echo: request.value.echo })
        ]
    }
}

fn main() -> Result<()> {
    run_actor::<EchoActor>()
}
