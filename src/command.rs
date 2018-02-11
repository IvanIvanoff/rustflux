use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
pub enum CommandError {
    ParseError,
}

pub enum Command {
    Connect(String),
    Use(String),
    ShowDatabases,
    ShowMeasurements,
    ShowTags,
    ShowTagsMeasurement(String),
    Unknown(String),
    Help,
		Info,
}

fn is_same_command(commands: Vec<&str>, input: &Vec<&str>) -> bool {
    if input.len() >= commands.len() {
        input.iter().zip(commands).all(|(s1, s2)| s1 == &s2)
    } else {
        false
    }
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        if words.len() == 2 && is_same_command(vec!["connect"], &words) {
            Ok(Command::Connect(String::from(words[1])))
        } else if words.len() == 2 && is_same_command(vec!["show", "databases"], &words) {
            Ok(Command::ShowDatabases)
        } else if words.len() == 2 && is_same_command(vec!["show", "measurements"], &words) {
            Ok(Command::ShowMeasurements)
				} else if words.len() == 2 && is_same_command(vec!["show", "tags"], &words) {
					Ok(Command::ShowTags)
				} else if words.len() == 3 && is_same_command(vec!["show", "tags"], &words) {
						Ok(Command::ShowTagsMeasurement(String::from(words[3])))
				} else {
            Ok(Command::Unknown(String::from(line)))
        }
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Command::Connect(ref host) => write!(f, "Connecting to host {}", host),
            &Command::Use(ref db) => write!(f, "Use {}", db),
            &Command::ShowDatabases => write!(f, "Showing the databases"),
            &Command::ShowMeasurements => write!(f, "Showing the measurements"),
            &Command::ShowTags => write!(f, "Showing the tags in the database"),
            &Command::ShowTagsMeasurement(ref measurement) => {
                write!(f, "Showing the tags from measurement {}", measurement)
            }
            &Command::Unknown(ref line) => write!(f, "Ignoring unknown command - {}", line),
            &Command::Help => write!(f, "Showing help"),
						&Command::Info => write!(f, "Showing info"),
        }
    }
}
