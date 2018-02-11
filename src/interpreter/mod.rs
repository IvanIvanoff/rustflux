use command::{Command, CommandError};
use std::str::FromStr;

///
pub fn execute(line: String) {
    match Command::from_str(&line) {
        Ok(command) => println!("{}", command),
        Err(err) => println!("Error exectuing {:?}", err),
    }
}
