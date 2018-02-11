use context::Context;
use command::{Command, CommandError};
use std::str::FromStr;
use queries;
use http_client;

///
pub fn execute(context: &mut Context, line: String) {
  let command = Command::from_str(&line);

  match command {
    Ok(Command::Help) | Ok(Command::Info) =>
      println!("{}", command.unwrap()),

    Ok(Command::Connect(host)) =>
      context.host = host,

    Ok(Command::Use(database)) =>
      context.using = database,

    Ok(Command::ShowDatabases) => {
      let query = queries::show_databases(&context.host);
      match http_client::get(query) {
        Ok(body) =>
          println!("{}", body),
        Err(err) =>
          println!("Error - {}", err)
      }
    },

    Ok(Command::ShowMeasurements) =>
      println!("Showing measurements"),

    Ok(Command::ShowTags) =>
      println!("Showing tags"),

    Ok(Command::ShowTagsMeasurement(_measurement)) =>
      println!("Show tags from measurement"),

    Ok(Command::Unknown(_)) =>
      println!("{}", command.unwrap()),

    Err(err) =>
      println!("Error exectuing {:?}", err),
  }
}
