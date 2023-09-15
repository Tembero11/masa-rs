use crate::game_server::{GameServer, GameServerCommand};

mod game_server;

fn main() {
  println!("Hello from Masa!");

  game_server::run_cmd(String::from("java"), Vec::from([String::from("--version")]), String::from("."));

  // Testing...
  let test_server = GameServer {
    name: String::from("testi_servu"),
    dir: String::from("test_server"),
    cmd: GameServerCommand {
      cmd: String::from("java"),
      args: Vec::from([
        String::from("-Xms1024M"),
        String::from("-Xmx1024M"),
        String::from("-jar"),
        String::from("server.jar"),
        String::from("nogui"),
      ])
    },
    id: None,
  };

  game_server::start_server(test_server);
}
