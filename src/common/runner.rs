use std::cmp::max;
use std::ops::{Add};
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

const MINIMUM_READ_DURATION: Duration = Duration::from_millis(1000);

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
    let mut actor = A::new(this_node)?;

    let mut timer: Timer<A::TimerKey> = Timer::new();

    loop {
        let now = Instant::now();
        let expired_timers = timer.remove_expired_timers(now);
        for expired_timer in expired_timers {
            trace!("Got expired timer: '{:?}'", expired_timer);
            let now = Instant::now();
            for action in actor.on_timeout(expired_timer, now)? {
                execute_action::<A>(&console, &mut timer, now, action)?;
            }
        }

        let now = Instant::now();
        let duration_until_next_timer = timer.duration_until_next_timer(now);
        trace!("Duration until next timer: '{:?}'", duration_until_next_timer);
        if let Some(message) = console.read::<Message<A::Msg>>(max(duration_until_next_timer, MINIMUM_READ_DURATION))? {
            debug!("Got message: '{:?}'", message);
            let now = Instant::now();
            for action in actor.on_request(message, now)? {
                execute_action::<A>(&console, &mut timer, now, action)?;
            }
        }
    }
}

fn execute_action<A>(console: &Console,
                     timer: &mut Timer<A::TimerKey>,
                     now: Instant,
                     action: RunnerAction<A::Msg, A::TimerKey>) -> Result<()>
    where A: Actor {
    match action {
        RunnerAction::SendMessage(message) => {
            debug!("Writing message: '{:?}'", message);
            console.write(&message)?;
        }
        RunnerAction::SetTimer { delay, timer_key } => {
            trace!("Adding timer. Delay: '{:?}', key: '{:?}'", delay, timer_key);
            timer.add_timer(now.add(delay), timer_key);
        }
    }
    Ok(())
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
        delay: Duration,
        timer_key: B,
    },
}

pub fn reply<A, B>(request_address: MessageAddress, value: A) -> RunnerAction<A, B> {
    RunnerAction::SendMessage(Message::new_reply(request_address.to_reply_address(), value))
}

pub fn send<A, B>(address: MessageAddress, value: A) -> RunnerAction<A, B> {
    RunnerAction::SendMessage(Message::new_request(address, value))
}

pub fn set_timer<A, B>(delay: Duration, timer_key: B) -> RunnerAction<A, B> {
    RunnerAction::SetTimer { delay, timer_key }
}
