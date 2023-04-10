use std::ops::Add;
use std::time::{Duration, Instant};

use log::{debug, trace};
use stderrlog::{ColorChoice, LogLevelNum, Timestamp};

use crate::common::actor::Actor;
use crate::common::console::Console;
use crate::common::error::Error::UnexpectedMessage;
use crate::common::message::init::InitMessage;
use crate::common::message::message::{Message, MessageAddress};
use crate::common::timer::Timer;

use super::error::Result;
use super::this_node::ThisNode;

pub fn run_actor<A>() -> Result<()>
    where A: Actor {
    stderrlog::new()
        .verbosity(LogLevelNum::Debug)
        .timestamp(Timestamp::Microsecond)
        .color(ColorChoice::Always)
        .init()
        .unwrap();

    let console = Console::new();
    let this_node = init(&console)?;
    let mut actor = A::new(this_node);

    let mut timer: Timer<A::TimerKey> = Timer::new();

    loop {
        let now = Instant::now();
        trace!("Now: '{:?}'", now);
        let mut iteration_actions: Vec<RunnerAction<A::Msg, A::TimerKey>> = vec![];

        let expired_timers = timer.remove_expired_timers(now);
        for expired_timer in expired_timers {
            trace!("Got expired timer: '{:?}'", expired_timer);
            iteration_actions.extend(actor.on_timer(expired_timer)?);
        }

        let duration_until_next_timer = timer.duration_until_next_timer(now);

        if let Some(message) = console.read::<Message<A::Msg>>(duration_until_next_timer)? {
            debug!("Got message: '{:?}'", message);
            iteration_actions.extend(actor.on_request(message)?);
        }

        for action in iteration_actions {
            match action {
                RunnerAction::SendMessage(message) => {
                    debug!("Writing message: '{:?}'", message);
                    console.write(&message)?;
                }
                RunnerAction::SetTimer { period, timer_key } => {
                    debug!("Adding timer. Period: '{:?}', key: '{:?}'", period, timer_key);
                    timer.add_timer(now.add(period), timer_key);
                }
            }
        }
    }
}

fn init(console: &Console) -> Result<ThisNode> {
    let message: Message<InitMessage> = console.read_blocking()?;
    debug!("Got init request: '{:?}'", message);

    let (body, address) = message.body_and_address();
    match body {
        InitMessage::Init { node_id, node_ids } => {
            let init_response = Message::new_reply(address.to_reply_address(), InitMessage::InitOk);
            debug!("Writing init response: '{:?}'", init_response);
            console.write(&init_response)?;
            Ok(ThisNode::new(node_id, node_ids))
        }
        InitMessage::InitOk => Err(UnexpectedMessage("InitOk".to_string()))
    }
}


pub enum RunnerAction<A, B> {
    SendMessage(Message<A>),
    SetTimer {
        period: Duration,
        timer_key: B,
    },
}


pub fn reply<A, B>(request_address: MessageAddress, value: A) -> RunnerAction<A, B> {
    RunnerAction::SendMessage(Message::new_reply(request_address.to_reply_address(), value))
}

pub fn send<A, B>(address: MessageAddress, value: A) -> RunnerAction<A, B> {
    RunnerAction::SendMessage(Message::new_request(address, value))
}

pub fn set_timer<A, B>(period: Duration, timer_key: B) -> RunnerAction<A, B> {
    RunnerAction::SetTimer { period, timer_key }
}
