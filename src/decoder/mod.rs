extern crate serde_json;

use self::serde_json::{Error, Value};
use errors::RustfluxError;
use std::str::FromStr;
use chrono::prelude::*;

use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;

/// Accepts the JSON returned from influxdb and converts
/// it to line protocol, suitable for sending
pub fn json_to_line_protocol_file(
    measurement_str: &str,
    measurement_name: &str,
    tags: &Vec<String>,
) -> Result<String, RustfluxError> {
    let mut line_protocol: Vec<String> = Vec::new();

    let json = json_from_str(measurement_str)?;
    let columns = extract_column_names(&json)?;

    let mut column_tag: Vec<(String, bool)> = Vec::new();
    for column in columns.iter() {
        if tags.contains(column) {
            column_tag.push((column.to_string(), true));
        } else {
            column_tag.push((column.to_string(), false));
        }
    }

    let values = json["results"][0]["series"][0]["values"].as_array().ok_or(
        RustfluxError::JsonDecode(String::from("Cannot decode json")),
    )?;

    for value in values.iter() {
        if let Value::Array(ref arr) = *value {
            let mut tag_set = String::new();
            let mut field_set = String::new();

            let nanoseconds = extract_time_nanoseconds(arr);

            for (elem, &(ref column_name, ref is_tag)) in
                arr.iter().skip(1).zip(column_tag.iter().skip(1))
            {
                let mut key_value = String::new();

                match elem {
                    &Value::String(ref s) => {
                        let val = elem.as_str().unwrap();
                        key_value = format!(",{}=\"{}\"", column_name, val);
                    }

                    &Value::Number(ref num) => {
                        if num.is_f64() {
                            let num = num.as_f64().unwrap();
                            key_value = format!(",{}={}f", column_name, num);
                        } else if num.is_i64() {
                            let num = num.as_i64().unwrap();
                            key_value = format!(",{}={}i", column_name, num);
                        }
                    }

                    _ => {}
                }

                if *is_tag {
                    tag_set.push_str(&key_value);
                } else {
                    field_set.push_str(&key_value);
                }
            }

            field_set.remove(0);
            let line = format!(
                "{}{} {} {}",
                measurement_name, tag_set, field_set, nanoseconds
            );
            line_protocol.push(line);
        }
    }

    let file_name = save_file_to_disk(measurement_name, &line_protocol)?;
    Ok(file_name)
}

pub fn json_strings_to_list(json_str: &str) -> Result<Vec<String>, RustfluxError> {
    let mut result: Vec<String> = Vec::new();

    let json = json_from_str(json_str)?;

    if let Some(values) = json["results"][0]["series"][0]["values"].as_array() {
        for elem in values.iter() {
            if let Value::Array(ref arr) = *elem {
                // TODO: FIX
                let s = arr.first().unwrap().as_str().unwrap();
                result.push(String::from(s));
            }
        }
    }

    Ok(result)
}

// Helper functions

fn save_file_to_disk(
    measurement_name: &str,
    line_protocol: &Vec<String>,
) -> Result<String, RustfluxError> {
    let _ = match fs::create_dir_all("/tmp/.rustflux") {
        Ok(_) => {}
        Err(_) => {
            return Err(RustfluxError::IOError(String::from(
                "Cannot create /tmp/.rustflux",
            )))
        }
    };

    let utc = Utc::now().timestamp();
    let file_name = format!("/tmp/.rustflux/{}_{}", measurement_name, utc);
    {
        let path = Path::new(&file_name);

        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(RustfluxError::IOError(String::from(format!(
                    "Cannot create file for measurement: {}",
                    err
                ))));
            }
        };

        println!(
            "Saving the measurement {} to a file {}",
            measurement_name, file_name
        );

        for line in line_protocol.iter() {
            file.write(line.as_bytes());
            file.write("\n".as_bytes());
        }
    }
    Ok(file_name)
}

fn extract_column_names(json: &Value) -> Result<Vec<String>, RustfluxError> {
    let mut result: Vec<String> = Vec::new();

    if let Some(columns) = json["results"][0]["series"][0]["columns"].as_array() {
        for elem in columns.iter() {
            if let Value::String(ref s) = *elem {
                result.push(s.to_string());
            }
        }
    }

    Ok(result)
}

fn extract_time_nanoseconds(array: &Vec<Value>) -> i64 {
    let time_str = array.first().unwrap().as_str().unwrap();
    let time: DateTime<Utc> = DateTime::from_str(time_str).unwrap();
    let nanoseconds: i64 = DateTime::timestamp(&time) * 1_000_000_000; // make nanoseconds
    nanoseconds
}

fn json_from_str(json_str: &str) -> Result<Value, RustfluxError> {
    match serde_json::from_str(json_str) {
        Ok(val) => Ok(val),
        Err(_) => Err(RustfluxError::JsonDecode(String::from(
            "Cannot decode json",
        ))),
    }
}
