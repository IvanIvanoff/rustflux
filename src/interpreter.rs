use context::Context;
use command::Command;
use std::str::FromStr;
use queries;
use http_client;
use converter;
use errors::RustfluxError;
use filesystem;
use chrono::prelude::*;

/// Gets the current context and command as inputs and executes the command
pub fn execute(context: &mut Context, line: &str) -> Result<(), RustfluxError> {
    let command = Command::from_str(line);

    match command {
        Ok(Command::IgnoreEmptyLine) | Ok(Command::Help) => println!("{}", command.unwrap()),

        Ok(Command::Connect(host)) => context.host = host,

        Ok(Command::Use(database)) => context.database = database,

        Ok(Command::DownloadMeasurement(measurement)) => {
            download_measurement(context, &measurement)?
        }

        Ok(Command::UploadMeasurement(measurement_file)) => {
            upload_measurement(context, &measurement_file)?
        }

        Ok(Command::DownloadDatabase(database_name)) => download_database(context, &database_name)?,

        Ok(Command::UploadDatabase(database_dir)) => upload_database(context, &database_dir)?,

        Ok(Command::ShowDatabases) => show_databases(context)?,

        Ok(Command::ShowMeasurements) => show_measurements(context)?,

        Ok(Command::ShowTagsMeasurement(measurement)) => {
            show_tags_from_measurement(context, &measurement)?
        }

        Ok(Command::DropDatabase(database)) => drop_database(context, &database)?,

        Ok(Command::DropMeasurement(measurement)) => drop_measurement(context, &measurement)?,

        Ok(Command::Info) => println!(
            "Connectect to host: {} \n Using database: {}",
            &context.host, &context.database
        ),

        Ok(Command::Unknown(_)) => println!("{}", command.unwrap()),

        Err(err) => {
            println!("Error exectuing a command. Reason: '{:?}'", err);
            panic!("Error executing a command")
        }
    }

    Ok(())
}

// Helper functions

fn get_databases(context: &mut Context) -> Result<Vec<String>, RustfluxError> {
    let mut result: Vec<String> = Vec::new();

    let query = queries::show_databases(&context.host);
    let databases = http_client::get(&query)?;

    let db_list = converter::json_strings_to_list(&databases)?;
    for db_name in &db_list {
        result.push(db_name.to_string())
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
    let measurement_list = converter::json_strings_to_list(&measurements)?;

    for measurement in &measurement_list {
        result.push(measurement.to_string())
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

    let tag_list = converter::json_strings_to_list(&tags)?;
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

fn download_measurement(
    context: &mut Context,
    measurement_name: &str,
) -> Result<(), RustfluxError> {
    let query = queries::measurement(&context.host, &context.database, measurement_name);
    let tags = get_tags_from_measurement(context, measurement_name)?;
    let measurement_json = http_client::get(&query)?;

    let line_protocol =
        converter::json_to_line_protocol(&measurement_json, measurement_name, &tags)?;

    let file_name =
        filesystem::save_file_to_disk("/tmp/.rustflux", measurement_name, &line_protocol)?;

    println!(
        "Saving the measurement {} to file {}",
        measurement_name, file_name
    );

    Ok(())
}

fn upload_measurement(context: &mut Context, file_name: &str) -> Result<(), RustfluxError> {
    let url = queries::write(&context.host, &context.database);
    let post_status = http_client::post_file(&url, file_name)?;
    println!("{}", post_status);

    Ok(())
}

fn download_database(context: &mut Context, database: &str) -> Result<(), RustfluxError> {
    let utc = Utc::now().timestamp();
    let dir_name = format!("/tmp/.rustflux/{}_{}", &database, utc);

    for measurement_name in get_measurements(context)? {
        let query = queries::measurement(&context.host, &database, &measurement_name);
        let tags = get_tags_from_measurement(context, &measurement_name)?;
        let measurement_json = http_client::get(&query)?;

        let line_protocol =
            converter::json_to_line_protocol(&measurement_json, &measurement_name, &tags)?;

        let _file_name =
            filesystem::save_file_to_disk(&dir_name, &measurement_name, &line_protocol)?;
    }

    println!("Saving the database {} to folder {}", &database, &dir_name);

    Ok(())
}

fn upload_database(context: &mut Context, database_dir: &str) -> Result<(), RustfluxError> {
    let db_name = database_dir.split("/").collect::<Vec<&str>>();
    let db_name = String::from(*db_name.last().unwrap());
    let db_name = db_name.split("_").collect::<Vec<&str>>();
    let db_name = String::from(*db_name.first().unwrap());

    context.database = db_name.clone();

    let url = queries::create_db(&context.host, &context.database);

    let _ = http_client::post(&url)?;
    for file_name in filesystem::files_in_dir(&database_dir)? {
        let _post_status = upload_measurement(context, &file_name)?;
    }

    println!("Uploaded database from {}", &database_dir);

    Ok(())
}

fn drop_database(context: &mut Context, database: &str) -> Result<(), RustfluxError> {
    let url = queries::drop_db(&context.host, &database);
    let post_status = http_client::post(&url)?;
    println!("{}", post_status);

    Ok(())
}

fn drop_measurement(context: &mut Context, measurement: &str) -> Result<(), RustfluxError> {
    let url = queries::drop_measurement(&context.host, &context.database, &measurement);
    let post_status = http_client::post(&url)?;
    println!("{}", post_status);

    Ok(())
}
