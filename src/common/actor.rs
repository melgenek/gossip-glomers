use std::fmt::Debug;
use std::time::Instant;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::common::message::message::Message;
use crate::common::runner::RunnerAction;
use crate::common::this_node::ThisNode;

use super::error::Result;

pub trait Actor
    where Self: Sized {
    type Msg: Debug + DeserializeOwned + Serialize;
    type TimerKey: Debug;

    fn new(this_node: ThisNode) -> Result<Self>;

    fn on_request(&mut self, request: Message<Self::Msg>, now: Instant) -> Result<Vec<RunnerAction<Self::Msg, Self::TimerKey>>>;

    fn on_timeout(&mut self, _timer_key: Self::TimerKey, _now: Instant) -> Result<Vec<RunnerAction<Self::Msg, Self::TimerKey>>> {
        Ok(vec![])
    }
}
