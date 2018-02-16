use std::str::FromStr;
use std::fmt;
use errors::RustfluxError;

#[derive(PartialEq, Debug)]
pub enum Command {
    IgnoreEmptyLine,
    Connect(String),
    Use(String),
    DownloadMeasurement(String),
    UploadMeasurement(String),
    DownloadDatabase,
    UploadDatabase(String),
    ShowDatabases,
    ShowMeasurements,
    ShowTagsMeasurement(String),
    DropDatabase(String),
    DropMeasurement(String),
    Unknown(String),
    Help,
    Info,
}

/// Check if the given command represented as a vector of strings is equal to the
/// input, represented as an array of strings
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
        } else if words.len() == 2 && is_same_command(vec!["upload"], &words) {
            Ok(Command::UploadMeasurement(String::from(words[1])))
        } else if words.len() == 2 && is_same_command(vec!["download", "database"], &words) {
            Ok(Command::DownloadDatabase)
        } else if words.len() == 3 && is_same_command(vec!["upload", "database"], &words) {
            Ok(Command::UploadDatabase(String::from(words[2])))
        } else if words.len() == 5 && is_same_command(vec!["show", "tag", "keys", "from"], &words) {
            Ok(Command::ShowTagsMeasurement(String::from(words[4])))
        } else if words.len() == 3 && is_same_command(vec!["drop", "database"], &words) {
            Ok(Command::DropDatabase(String::from(words[2])))
        } else if words.len() == 3 && is_same_command(vec!["drop", "measurement"], &words) {
            Ok(Command::DropMeasurement(String::from(words[2])))
        } else {
            Ok(Command::Unknown(String::from(line)))
        }
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
            Command::UploadDatabase(ref database) => write!(f, "Upload database {}", database),
            Command::ShowDatabases => write!(f, "Showing the databases"),
            Command::ShowMeasurements => write!(f, "Showing the measurements"),
            Command::ShowTagsMeasurement(ref file_name) => write!(
                f,
                "Showing the tags from measurement from file {}",
                file_name
            ),
            Command::DropDatabase(ref database) => write!(f, "Drop database {}", database),
            Command::DropMeasurement(ref measurement) => {
                write!(f, "Drop measurement {}", measurement)
            }
            Command::Unknown(ref line) => write!(f, "Ignoring unknown command - {}", line),
            Command::Help => write!(f, "Showing help"),
            Command::Info => write!(f, "Showing info"),
        }
    }
}

// TESTS

#[test]
fn test_command_parsing() {
    assert_eq!(Command::IgnoreEmptyLine, Command::from_str("").unwrap());
    assert_eq!(
        Command::Unknown(String::from(
            "xastduadjhhhhqhqhqhzzzz sauqywuqtugznnnsisiskq as"
        )),
        Command::from_str("xastduadjhhhhqhqhqhzzzz sauqywuqtugznnnsisiskq as").unwrap()
    );
    assert_eq!(
        Command::DropDatabase("test_db".to_string()),
        Command::from_str("drop database test_db").unwrap()
    );
    assert_eq!(
        Command::DropMeasurement("test_ms".to_string()),
        Command::from_str("drop measurement test_ms").unwrap()
    );
    assert_eq!(
        Command::Connect("host".to_string()),
        Command::from_str("connect host").unwrap()
    );
    assert_eq!(
        Command::Use("db_name".to_string()),
        Command::from_str("use db_name").unwrap()
    );
    assert_eq!(
        Command::DropMeasurement("test_ms".to_string()),
        Command::from_str("drop measurement test_ms").unwrap()
    );
    assert_eq!(
        Command::DownloadMeasurement("test_ms".to_string()),
        Command::from_str("download measurement test_ms").unwrap()
    );
    assert_eq!(
        Command::ShowDatabases,
        Command::from_str("show databases").unwrap()
    );
    assert_eq!(
        Command::ShowMeasurements,
        Command::from_str("show measurements").unwrap()
    );
    assert_eq!(
        Command::DropMeasurement("test_ms".to_string()),
        Command::from_str("drop measurement test_ms").unwrap()
    );
    assert_eq!(
        Command::ShowTagsMeasurement("test_ms".to_string()),
        Command::from_str("show tag keys from test_ms").unwrap()
    );
    assert_eq!(Command::Help, Command::from_str("help").unwrap());
    assert_eq!(Command::Info, Command::from_str("info").unwrap());
}
