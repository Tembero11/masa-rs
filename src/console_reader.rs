use crate::events::{server_ready::ServerReadyEvent};

pub enum GameServerEventKind {
  ServerReadyEvent{event: ServerReadyEvent, stdout: String},
}


pub fn parse(stdout: String) {
  ServerReadyEvent::parse(stdout);
}