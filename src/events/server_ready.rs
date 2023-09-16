use crate::game_server::ServerState;
use regex::Regex;

#[derive(Debug)]
pub struct ServerReadyEvent {
  // TODO: Add time
}

impl ServerReadyEvent {
  pub fn parse(stdout: &str, state: &mut ServerState) -> Option<ServerReadyEvent> {
    if state.is_ready {
      return None
    }
    // Test event with regex
    let re = Regex::new("Done \\(.{1,}\\)\\!").unwrap();

    let is_match = re.is_match(stdout);
    
    if is_match {
      state.is_ready = true;
      return Some(ServerReadyEvent {});
    }

    None
  }
}