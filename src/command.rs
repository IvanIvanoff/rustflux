use std::str::FromStr;
use std::fmt;
use errors::RustfluxError;

pub enum Command {
    IgnoreEmptyLine,
    Connect(String),
    Use(String),
    DownloadMeasurement(String),
    DownloadDatabase,
    UploadMeasurement(String),
    ShowDatabases,
    ShowMeasurements,
    ShowTagsMeasurement(String),
    Unknown(String),
    Help,
    Info,
}

fn is_same_command(commands: Vec<&str>, input: &[&str]) -> bool {
    if input.len() >= commands.len() {
        input.iter().zip(commands).all(|(s1, s2)| s1 == &s2)
    } else {
        false
    }
}

impl FromStr for Command {
    type Err = RustfluxError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let words = line.split_whitespace().collect::<Vec<&str>>();

        if words.is_empty() {
            Ok(Command::IgnoreEmptyLine)
        } else if words.len() == 1 && is_same_command(vec!["info"], &words) {
            Ok(Command::Info)
        } else if words.len() == 1 && is_same_command(vec!["help"], &words) {
            Ok(Command::Help)
        } else if words.len() == 2 && is_same_command(vec!["connect"], &words) {
            Ok(Command::Connect(String::from(words[1])))
        } else if words.len() == 2 && is_same_command(vec!["use"], &words) {
            Ok(Command::Use(String::from(words[1])))
        } else if words.len() == 2 && is_same_command(vec!["show", "databases"], &words) {
            Ok(Command::ShowDatabases)
        } else if words.len() == 2 && is_same_command(vec!["show", "measurements"], &words) {
            Ok(Command::ShowMeasurements)
        } else if words.len() == 3 && is_same_command(vec!["download", "measurement"], &words) {
            Ok(Command::DownloadMeasurement(String::from(words[2])))
        } else if words.len() == 2 && is_same_command(vec!["download", "database"], &words) {
            Ok(Command::DownloadDatabase)
        } else if words.len() == 2 && is_same_command(vec!["upload"], &words) {
            Ok(Command::UploadMeasurement(String::from(words[1])))
        } else if words.len() == 5 && is_same_command(vec!["show", "tag", "keys", "from"], &words) {
            Ok(Command::ShowTagsMeasurement(String::from(words[4])))
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
        match *self {
            Command::IgnoreEmptyLine => Ok(()),
            Command::Connect(ref host) => write!(f, "Connecting to host {}", host),
            Command::Use(ref db) => write!(f, "Use {}", db),
            Command::DownloadMeasurement(ref measurement) => {
                write!(f, "Download measurement {}", measurement)
            }
            Command::DownloadDatabase => write!(f, "Download database"),
            Command::UploadMeasurement(ref measurement) => {
                write!(f, "Upload measurement {}", measurement)
            }
            Command::ShowDatabases => write!(f, "Showing the databases"),
            Command::ShowMeasurements => write!(f, "Showing the measurements"),
            Command::ShowTagsMeasurement(ref file_name) => write!(
                f,
                "Showing the tags from measurement from file {}",
                file_name
            ),
            Command::Unknown(ref line) => write!(f, "Ignoring unknown command - {}", line),
            Command::Help => write!(f, "Showing help"),
            Command::Info => write!(f, "Showing info"),
        }
    }
}
