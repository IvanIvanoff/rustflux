use std::str::FromStr;
use std::fmt;

pub enum CommandError {
    ParseError
}

pub enum Command {
    Connect(String),
    ShowDatabases,
    ShowMeasurements(String),
    ShowTags(String),
    ShowTagsMeasurement(String,String),
    Unknown(String),
    Help,
    Quit,
}

fn is_same_command(command: &str, input: &str) -> bool {
    let s1 = String::from(command).to_lowercase();
    let s2 = String::from(input).to_lowercase();

    if s1 == s2 {
        true
    } else {
        false
    }
}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let words = line.split_whitespace().collect::<Vec<&str>>();

        Ok(Command::Unknown(String::from(line)))
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
            &Command::Connect(ref host) => {
                write!(f, "Connecting to host {}", host)
            },
            &Command::ShowDatabases => {
                write!(f, "Showing the databases")
            },
            &Command::ShowMeasurements(ref database) => {
                write!(f, "Showing the measurements in the database {}", database)
            },
            &Command::ShowMeasurements(ref database) => {
                write!(f, "Showing the measurements in the database {}", database)
            },
            &Command::ShowTags(ref database) => {
                write!(f, "Showing the tags in the database {}", database)
            },
            &Command::ShowTagsMeasurement(ref database, ref measurement) => {
                write!(f, "Showing the measurements from {} in the database {}", measurement, database)
            },
            &Command::Unknown(ref line) => {
                write!(f, "Ignoring unknown command - {}", line)
            },
            &Command::Help => {
                write!(f, "Showing help")
            },
            &Command::Quit => {
                write!(f, "Quit")
            }
        }
    }
}