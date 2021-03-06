extern crate serde_json;

use self::serde_json::Value;
use errors::RustfluxError;
use std::str::FromStr;
use chrono::prelude::*;

/// Accepts the JSON returned from influxdb and converts
/// it to line protocol, suitable for sending
pub fn json_to_line_protocol(
    measurement_str: &str,
    measurement_name: &str,
    tags: &[String],
) -> Result<Vec<String>, RustfluxError> {
    let mut line_protocol: Vec<String> = Vec::new();

    let json = json_from_str(measurement_str)?;
    let columns = extract_column_names(&json)?;

    let mut column_tag: Vec<(String, bool)> = Vec::new();
    for column in &columns {
        if tags.contains(column) {
            column_tag.push((column.to_string(), true));
        } else {
            column_tag.push((column.to_string(), false));
        }
    }

    let values = json["results"][0]["series"][0]["values"]
        .as_array()
        .ok_or_else(|| RustfluxError::JsonDecode(String::from("Cannot decode json")))?;

    for value in values.iter() {
        if let Value::Array(ref arr) = *value {
            let mut tag_set = String::new();
            let mut field_set = String::new();

            let nanoseconds = extract_time_nanoseconds(arr);

            for (elem, &(ref column_name, ref is_tag)) in
                arr.iter().skip(1).zip(column_tag.iter().skip(1))
            {
                let mut key_value = String::new();

                match *elem {
                    Value::String(ref val) => {
                        if *is_tag {
                            key_value = format!(",{}={}", column_name, val);
                        } else {
                            key_value = format!(",{}=\"{}\"", column_name, val);
                        }
                    }

                    Value::Number(ref num) => {
                        if num.is_f64() {
                            let num = num.as_f64().unwrap();
                            key_value = format!(",{}={}", column_name, num);
                        } else if num.is_i64() {
                            let num = num.as_i64().unwrap();
                            // Currently there is no query to get the field type.
                            // As a safe alternative write all numeric data as float
                            key_value = format!(",{}={}", column_name, num);
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

    Ok(line_protocol)
}

/// Expects an influxdb response containing arrays of single strings. That are the results
/// of `show databases` and `show measurements`
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

fn extract_time_nanoseconds(array: &[Value]) -> i64 {
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

// TESTS

#[test]
fn test_json_from_str_fail() {
    assert_eq!(
        Err(RustfluxError::JsonDecode(String::from(
            "Cannot decode json"
        ))),
        json_from_str("}[}[asdaj not valid!!!! json asdasds")
    );
}

#[test]
fn test_json_from_str() {
    let s = r###"{
    "results": [
        {
            "statement_id": 0,
            "series": [
                {
                    "name": "SAN",
                    "columns": [
                        "time",
                        "block_number",
                        "from_addr",
                        "to_addr",
                        "transaction_type",
                        "trx_value"
                    ],
                    "values": [
                        [
                            "2017-07-04T12:12:08Z",
                            3972817,
                            "0xc58f14af29ec15bbbf2734fe7f4fe8bc4448d38f",
                            "0x6dd5a9f47cfbc44c04a0a4452f0ba792ebfbcc9a",
                            "in",
                            5
                        ],
                        [
                            "2017-07-12T08:03:11Z",
                            4011221,
                            "0xda2cf810c5718135247628689d84f94c61b41d6a",
                            "0x6dd5a9f47cfbc44c04a0a4452f0ba792ebfbcc9a",
                            "in",
                            45000
                        ]
                    ]
                }
              ]
            }
        ]
    }
    "###;

    assert!(json_from_str(s).is_ok());
}

#[test]
fn test_extract_column_names() {
    let s = r###"{
    "results": [
        {
            "statement_id": 0,
            "series": [
                {
                    "name": "SAN",
                    "columns": [
                        "time",
                        "block_number",
                        "from_addr",
                        "to_addr",
                        "transaction_type",
                        "trx_value"
                    ],
                    "values": [
                        [
                            "2017-07-04T12:12:08Z",
                            3972817,
                            "0xc58f14af29ec15bbbf2734fe7f4fe8bc4448d38f",
                            "0x6dd5a9f47cfbc44c04a0a4452f0ba792ebfbcc9a",
                            "in",
                            5
                        ],
                        [
                            "2017-07-12T08:03:11Z",
                            4011221,
                            "0xda2cf810c5718135247628689d84f94c61b41d6a",
                            "0x6dd5a9f47cfbc44c04a0a4452f0ba792ebfbcc9a",
                            "in",
                            45000
                        ]
                    ]
                }
              ]
            }
        ]
    }
    "###;

    let json = json_from_str(s).unwrap();
    let column_names = extract_column_names(&json).unwrap();
    assert_eq!(
        vec![
            "time",
            "block_number",
            "from_addr",
            "to_addr",
            "transaction_type",
            "trx_value",
        ],
        column_names
    );
}

#[test]
fn test_json_to_line_protocol() {
    let s = r###"{
    "results": [
        {
            "statement_id": 0,
            "series": [
                {
                    "name": "SAN",
                    "columns": [
                        "time",
                        "block_number",
                        "from_addr",
                        "to_addr",
                        "transaction_type",
                        "trx_value"
                    ],
                    "values": [
                        [
                            "2017-07-04T12:12:08Z",
                            3972817,
                            "0xc58f14af29ec15bbbf2734fe7f4fe8bc4448d38f",
                            "0x6dd5a9f47cfbc44c04a0a4452f0ba792ebfbcc9a",
                            "in",
                            5
                        ],
                        [
                            "2017-07-12T08:03:11Z",
                            4011221,
                            "0xda2cf810c5718135247628689d84f94c61b41d6a",
                            "0x6dd5a9f47cfbc44c04a0a4452f0ba792ebfbcc9a",
                            "in",
                            45000
                        ]
                    ]
                }
              ]
            }
        ]
    }
    "###;

    let line_protocol = json_to_line_protocol(&s, "SAN", &["transaction_type".to_string()]);
    assert_eq!(
    vec![
    "SAN,transaction_type=in block_number=3972817,from_addr=\"0xc58f14af29ec15bbbf2734fe7f4fe8bc4448d38f\",to_addr=\"0x6dd5a9f47cfbc44c04a0a4452f0ba792ebfbcc9a\",trx_value=5 1499170328000000000",
    "SAN,transaction_type=in block_number=4011221,from_addr=\"0xda2cf810c5718135247628689d84f94c61b41d6a\",to_addr=\"0x6dd5a9f47cfbc44c04a0a4452f0ba792ebfbcc9a\",trx_value=45000 1499846591000000000"],
    line_protocol.unwrap());
}
