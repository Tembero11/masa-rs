use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use crate::console_reader::{self, parse};

#[derive(Clone, Debug)]
pub struct GameServerCommand {
  pub cmd: String,
  pub args: Vec<String>,
}
impl GameServerCommand {
  pub fn get_full_cmd(self) -> String {
    format!("{} {}", self.cmd, self.args.join(" "))
  }
}

#[derive(Clone, Debug)]
pub struct GameServer {
  pub name: String,
  pub cmd: GameServerCommand,
  pub dir: String,
  // Null if not connected to database yet
  pub id: Option<i32>,
  pub state: ServerState
}
impl GameServer {
    pub fn get_full_cmd(self) -> String {
      self.cmd.get_full_cmd()
    }
}

#[derive(Clone, Debug)]
pub struct ServerState {
  pub is_ready: bool,
}
impl ServerState {
  pub fn init() -> ServerState {
    ServerState { is_ready: false }
  }
}

pub fn start_server(game_server: &mut GameServer) {
  let full_cmd_str = game_server.clone().get_full_cmd();
  println!("{}", full_cmd_str);

  let mut cmd = Command::new(&game_server.cmd.cmd)
        .current_dir(&game_server.dir)
        .args(&game_server.cmd.args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Server failed to start!");

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for wrapped_line in stdout_lines {
          let line = wrapped_line.unwrap();
            println!("[{} Server Output] {}", game_server.name, line);
            let event = console_reader::parse(line, game_server);
            println!("{:?}", event);
        }
    }

    cmd.wait().unwrap();
}

pub fn run_cmd(cmd_str: String, args: Vec<String>, cwd: String) {
  let mut cmd = Command::new(cmd_str)
        .current_dir(cwd)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Server failed to start!");

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            println!("Read: {:?}", line.unwrap());
        }
    }

    cmd.wait().unwrap();
}