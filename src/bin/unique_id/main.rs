use gossip_glomers::common::actor::{Action, Actor};
use gossip_glomers::common::error::Result;
use gossip_glomers::common::message::NodeId;
use gossip_glomers::common::runner::run_actor;
use gossip_glomers::common::this_node::ThisNode;

use crate::message::{GenerateRequest, GenerateResponseValue};

mod message;

struct UniqueIdActor {
    node_id: NodeId,
    counter: u64,
}

impl Actor for UniqueIdActor {
    type Req = GenerateRequest;
    type Resp = GenerateResponseValue;

    fn new(this_node: &ThisNode) -> Self {
        UniqueIdActor {
            node_id: this_node.node_id.clone(),
            counter: 0,
        }
    }

    fn on_request(&mut self, request: Self::Req) -> Vec<Action<Self::Resp>> {
        let new_id = format!("{}_{}", self.node_id.0, self.counter);
        self.counter += 1;
        vec![
            Action::reply(request.msg_id, GenerateResponseValue { id: new_id })
        ]
    }
}

fn main() -> Result<()> {
    run_actor::<UniqueIdActor>()
}
