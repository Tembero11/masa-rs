use crate::game_server::{GameServer, GameServerCommand};

mod game_server;

fn main() {
  println!("Hello from Masa!");

  game_server::run_cmd(String::from("java"), Vec::from([String::from("--version")]));

  let test_server = GameServer {
    name: String::from("testi_servu"),
    cmd: GameServerCommand {
      cmd: String::from("java"),
      args: Vec::from([String::from("--version")])
    },
    id: None,
  };

  game_server::start_server(test_server);
  
}
