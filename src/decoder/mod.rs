extern crate serde_json;
use self::serde_json::{Error, Value};
use errors::RustfluxError;

pub fn string_to_json(json_str: &str) -> Result<Value, Error> {
    serde_json::from_str(json_str)
}

/// Accepts the JSON returned from influxdb and converts
/// it to line protocol, suitable for sending
pub fn json_to_line_protocol(json_str: &str) -> Result<Value, RustfluxError> {
    match serde_json::from_str(json_str) {
        Ok(val) => Ok(val),
        Err(_) => Err(RustfluxError::JsonDecode(String::from(
            "Cannot decode json",
        ))),
    }
}

pub fn json_to_line_protocol_file(json_str: &str, _file: &str) {
    let _val = json_to_line_protocol(json_str);
}

pub fn json_strings_to_list(json_str: &str) -> Result<Vec<String>, RustfluxError> {
    let mut result: Vec<String> = Vec::new();

    let json: Value = match serde_json::from_str(json_str) {
        Ok(val) => val,
        Err(_) => {
            return Err(RustfluxError::JsonDecode(String::from(
                "Cannot decode json",
            )))
        }
    };

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
