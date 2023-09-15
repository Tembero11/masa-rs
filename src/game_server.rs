use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};

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
  pub id: Option<i32>
}
impl GameServer {
    pub fn get_full_cmd(self) -> String {
      self.cmd.get_full_cmd()
    }
}

pub fn start_server(game_server: GameServer) {
  let full_cmd_str = game_server.clone().get_full_cmd();
  println!("{}", full_cmd_str);

  run_cmd(game_server.cmd.cmd, game_server.cmd.args, game_server.dir);
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