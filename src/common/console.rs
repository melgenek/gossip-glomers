use std::{io, thread};
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvError, RecvTimeoutError, Sender};
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::common::error::Error;

use super::error::Result;

pub struct Console {
    stdin_receiver: Receiver<String>,
    stdout_sender: Sender<Vec<u8>>,
}

impl Console {
    pub fn new() -> Console {
        let stdin_receiver = spawn_stdin_channel();
        let stdout_sender = spawn_stdout_channel();

        Console {
            stdin_receiver,
            stdout_sender,
        }
    }

    pub fn read<A>(&self, timeout: Duration) -> Result<Option<A>>
        where A: DeserializeOwned {
        match self.stdin_receiver.recv_timeout(timeout) {
            Ok(line) => {
                Ok(Some(serde_json::from_str(&line)?))
            }
            Err(RecvTimeoutError::Timeout) => Ok(None),
            Err(RecvTimeoutError::Disconnected) => Err(Error::Console("Read channel disconnected".to_string())),
        }
    }

    pub fn read_blocking<A>(&self) -> Result<A>
        where A: DeserializeOwned {
        match self.stdin_receiver.recv() {
            Ok(line) => {
                Ok(serde_json::from_str(&line)?)
            }
            Err(RecvError) => Err(Error::Console("Read channel disconnected".to_string())),
        }
    }

    pub fn write<A>(&self, response: &A) -> Result<()>
        where A: Serialize {
        let bytes = serde_json::to_vec(response)?;
        match self.stdout_sender.send(bytes) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Console(format!("Write channel error: {:?}", e)))
        }
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (sender, receiver) = mpsc::channel::<String>();
    let _ = thread::spawn(move ||
        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("Could not read a line");
            sender.send(line).expect("Could not send a line");
        }
    );
    receiver
}

fn spawn_stdout_channel() -> Sender<Vec<u8>> {
    let (sender, receiver) = mpsc::channel::<Vec<u8>>();
    let _ = thread::spawn(move || {
        for bytes in receiver {
            let mut out_lock = io::stdout().lock();
            out_lock.write_all(bytes.as_slice()).unwrap();
            out_lock.write_all(&[b'\n']).unwrap();
        }
    });
    sender
}
