use context::Context;
use command::Command;
use std::str::FromStr;
use queries;
use http_client;
use decoder;
use errors::RustfluxError;

///
pub fn execute(context: &mut Context, line: &str) -> Result<(), RustfluxError> {
    let command = Command::from_str(line);

    match command {
        Ok(Command::IgnoreEmptyLine) | Ok(Command::Help) | Ok(Command::Info) => {
            println!("{}", command.unwrap())
        }

        Ok(Command::Connect(host)) => context.host = host,

        Ok(Command::Use(database)) => context.database = database,

        Ok(Command::DownloadMeasurement(measurement)) => {
            download_measurement(context, &measurement)
        }

        Ok(Command::ShowDatabases) => show_databases(context),

        Ok(Command::ShowMeasurements) => show_measurements(context)?,

        Ok(Command::ShowTagsMeasurement(measurement)) => {
            show_tags_from_measurement(context, &measurement)
        }

        Ok(Command::Unknown(_)) => println!("{}", command.unwrap()),

        Err(err) => println!("Error exectuing '{:?}'", err),
    }

    Ok(())
}

// Helper functions

fn show_databases(context: &mut Context) {
    let query = queries::show_databases(&context.host);
    match http_client::get(&query) {
        Ok(databases) => match decoder::json_strings_to_list(&databases) {
            Ok(db_list) => for db_name in &db_list {
                println!("{}", db_name)
            },
            Err(err) => println!("Error decoding databases json response: {:?}", err),
        },

        Err(err) => println!("Error showing databases: {}", err),
    }
}

fn get_measurements(context: &mut Context) -> Result<Vec<String>, RustfluxError> {
    let mut result: Vec<String> = Vec::new();

    let query = queries::show_measurements(&context.host, &context.database);

    match http_client::get(&query) {
        Ok(measurements) => match decoder::json_strings_to_list(&measurements) {
            Ok(measurement_list) => {
                for measurement in &measurement_list {
                    result.push(measurement.to_string());
                }
                Ok(result)
            }
            Err(_) => Err(RustfluxError::JsonDecode(String::from(
                "Cannot decode json measurement",
            ))),
        },

        Err(err) => {
            let error = format!(
                "Error showing measurements from {}: {}",
                &context.database, err,
            );
            Err(RustfluxError::GetRequest(error))
        }
    }
}

fn show_measurements(context: &mut Context) -> Result<(), RustfluxError> {
    for measurement in get_measurements(context)? {
        println!("{}", measurement)
    }
    Ok(())
}

fn show_tags_from_measurement(context: &mut Context, measurement: &str) {
    let query = queries::show_tags_from_measurement(&context.host, &context.database, measurement);

    match http_client::get(&query) {
        Ok(tags) => match decoder::json_strings_to_list(&tags) {
            Ok(tag_list) => for tag in &tag_list {
                println!("{}", tag)
            },
            Err(err) => println!(
                "Error decoding tag keys json from {}: {:?}",
                &context.database, err
            ),
        },

        Err(err) => println!(
            "Error showing tags from {}/{}: {}",
            &context.database, measurement, err
        ),
    }
}

fn download_measurement(context: &mut Context, measurement: &str) {
    let _query = queries::measurement(&context.host, &context.database, measurement);
}
