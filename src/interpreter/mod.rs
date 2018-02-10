use command::Command;

///
pub fn execute(line: String) {
  println!("GOT {}", line);
}

fn parse_command(line: &str) -> Command {
  Command::Unknown(String::from("hahaha"))
}