extern crate reqwest;
use errors::RustfluxError;

use std::fs::File;

///
pub fn get(url: &str) -> Result<String, RustfluxError> {
    match get_internal(url) {
        Ok(body) => Ok(body),
        Err(_) => Err(RustfluxError::GetRequest(String::from(
            "Error executing GET request",
        ))),
    }
}

pub fn post(url: &str, file_name: &str) -> Result<String, RustfluxError> {
    match post_internal(url, file_name) {
        Ok(res) => Ok(res),
        Err(_) => Err(RustfluxError::PostRequest(String::from(
            "Error executing GET request",
        ))),
    }
}

// Helper functions

fn post_internal(url: &str, file_name: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let file = File::open(file_name).unwrap();

    let response = client.post(url).body(file).send()?;

    if response.status().is_success() {
        Ok(String::from("POST request successful"))
    } else {
        let error = format!("POST request not successful: {:?}", response.status());
        Ok(error)
    }
}

fn get_internal(url: &str) -> Result<String, reqwest::Error> {
    let mut resp = reqwest::get(url)?;
    let body = resp.text()?;
    Ok(body)
}
