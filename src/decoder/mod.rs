extern crate serde_json;
use self::serde_json::{Error, Value};
use std::fs::File;

pub fn string_to_json(json: String) -> Result<Value, Error> {
    let val = serde_json::from_str(&json);
    val
}

/// Accepts the JSON returned from influxdb and converts
/// it to line protocol, suitable for sending
pub fn json_to_line_protocol(json: String) -> Result<Value, Error> {
    let val: Result<Value, serde_json::Error> = serde_json::from_str(&json);
    val
}

pub fn json_to_line_protocol_file(json: String, file: String) {
    let val = json_to_line_protocol(json);
}
