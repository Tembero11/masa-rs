use crate::{events::server_ready::ServerReadyEvent, game_server::GameServer};

#[derive(Debug)]
pub enum GameServerEventKind {
  Unknown,
  ServerReadyEvent{event: ServerReadyEvent, stdout: String},
}


pub fn parse(stdout: String, server: &mut GameServer) -> GameServerEventKind {
  let ready_event = ServerReadyEvent::parse(&stdout, &mut server.state);

  if ready_event.is_some() {
    return GameServerEventKind::ServerReadyEvent { event: ready_event.unwrap(), stdout }
  }

  GameServerEventKind::Unknown
}