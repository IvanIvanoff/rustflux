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
            download_measurement(context, &measurement)?
        }

        Ok(Command::ShowDatabases) => show_databases(context)?,

        Ok(Command::ShowMeasurements) => show_measurements(context)?,

        Ok(Command::ShowTagsMeasurement(measurement)) => {
            show_tags_from_measurement(context, &measurement)?
        }

        Ok(Command::Unknown(_)) => println!("{}", command.unwrap()),

        Err(err) => println!("Error exectuing '{:?}'", err),
    }

    Ok(())
}

// Helper functions

fn get_databases(context: &mut Context) -> Result<Vec<String>, RustfluxError> {
    let mut result: Vec<String> = Vec::new();

    let query = queries::show_databases(&context.host);
    let databases = http_client::get(&query)?;

    let db_list = decoder::json_strings_to_list(&databases)?;
    for db_name in &db_list {
        result.push(db_name.to_string());
    }
    Ok(result)
}

fn show_databases(context: &mut Context) -> Result<(), RustfluxError> {
    for measurement in get_databases(context)? {
        println!("{}", measurement)
    }
    Ok(())
}

fn get_measurements(context: &mut Context) -> Result<Vec<String>, RustfluxError> {
    let mut result: Vec<String> = Vec::new();

    let query = queries::show_measurements(&context.host, &context.database);
    let measurements = http_client::get(&query)?;
    let measurement_list = decoder::json_strings_to_list(&measurements)?;

    for measurement in &measurement_list {
        result.push(measurement.to_string());
    }
    Ok(result)
}

fn show_measurements(context: &mut Context) -> Result<(), RustfluxError> {
    for measurement in get_measurements(context)? {
        println!("{}", measurement)
    }
    Ok(())
}

fn get_tags_from_measurement(
    context: &mut Context,
    measurement: &str,
) -> Result<Vec<String>, RustfluxError> {
    let mut result: Vec<String> = Vec::new();
    let query = queries::show_tags_from_measurement(&context.host, &context.database, measurement);
    let tags = http_client::get(&query)?;

    let tag_list = decoder::json_strings_to_list(&tags)?;
    for tag in &tag_list {
        result.push(tag.to_string());
    }
    Ok(result)
}

fn show_tags_from_measurement(
    context: &mut Context,
    measurement: &str,
) -> Result<(), RustfluxError> {
    for tag in get_tags_from_measurement(context, measurement)? {
        println!("{}", tag);
    }

    Ok(())
}

fn download_measurement(context: &mut Context, measurement: &str) -> Result<(), RustfluxError> {
    let query = queries::measurement(&context.host, &context.database, measurement);
    let measurement = http_client::get(&query)?;

    decoder::json_to_line_protocol(&measurement);

    Ok(())
}
