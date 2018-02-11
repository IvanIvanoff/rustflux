extern crate serde_json;
use self::serde_json::{Error, Value};

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

pub fn json_to_line_protocol_file(json: String, _file: String) {
    let _val = json_to_line_protocol(json);
}

pub fn json_strings_to_list(json_str: &str) -> Result<Vec<String>, Error> {
    let mut result: Vec<String> = Vec::new();

    let json: Value = serde_json::from_str(&json_str)?;
    match &json["results"][0]["series"][0]["values"].as_array() {
        &Some(vec) => for elem in vec.iter() {
            match elem {
                &Value::Array(ref arr) => {
                    // TODO: FIX
                    let s = arr.first().unwrap().as_str().unwrap();
                    result.push(String::from(s));
                }

                _ => {}
            }
        },
        &None => {}
    }

    Ok(result)
}
