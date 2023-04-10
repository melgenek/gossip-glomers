use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::common::message::message::Message;
use crate::common::runner::RunnerAction;
use crate::common::this_node::ThisNode;

use super::error::Result;

pub trait Actor {
    type Msg: Debug + DeserializeOwned + Serialize;
    type TimerKey: Debug;

    fn new(this_node: ThisNode) -> Self;

    fn on_request(&mut self, request: Message<Self::Msg>) -> Result<Vec<RunnerAction<Self::Msg, Self::TimerKey>>>;

    fn on_timer(&mut self, _timer_key: Self::TimerKey) -> Result<Vec<RunnerAction<Self::Msg, Self::TimerKey>>> {
        Ok(vec![])
    }
}
