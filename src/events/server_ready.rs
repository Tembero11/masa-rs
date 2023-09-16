pub struct ServerReadyEvent {

}

impl ServerReadyEvent {
  pub fn parse(stdout: String) -> Option<ServerReadyEvent> {
    // TODO: Regex
    None
  }
}