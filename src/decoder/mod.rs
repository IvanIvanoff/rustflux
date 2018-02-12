extern crate serde_json;
use self::serde_json::{Error, Value};
use errors::RustfluxError;

/// Accepts the JSON returned from influxdb and converts
/// it to line protocol, suitable for sending
pub fn json_to_line_protocol(json_str: &str) -> Result<Value, RustfluxError> {
    let val = json_from_str(json_str)?;

    Ok(val)
}

pub fn json_to_line_protocol_file(json_str: &str, _file: &str) -> Result<(), RustfluxError> {
    let val = json_to_line_protocol(json_str)?;

    Ok(())
}

pub fn json_strings_to_list(json_str: &str) -> Result<Vec<String>, RustfluxError> {
    let mut result: Vec<String> = Vec::new();

    let json = json_from_str(json_str)?;

    if let Some(vec) = json["results"][0]["series"][0]["values"].as_array() {
        for elem in vec.iter() {
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

fn json_from_str(json_str: &str) -> Result<Value, RustfluxError> {
    match serde_json::from_str(json_str) {
        Ok(val) => Ok(val),
        Err(_) => Err(RustfluxError::JsonDecode(String::from(
            "Cannot decode json",
        ))),
    }
}
