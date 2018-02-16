extern crate reqwest;
use errors::RustfluxError;

use std::fs::File;

///
pub fn get(url: &str) -> Result<String, RustfluxError> {
    match get_internal(url) {
        Ok(body) => Ok(body),
        Err(err) => Err(RustfluxError::GetRequest(format!(
            "Error executing GET request: {:?}",
            err
        ))),
    }
}

pub fn post(url: &str) -> Result<String, RustfluxError> {
    match post_internal(url) {
        Ok(res) => Ok(res),
        Err(err) => Err(RustfluxError::PostRequest(format!(
            "Error executing POST request: {:?}",
            err
        ))),
    }
}

pub fn post_file(url: &str, file_name: &str) -> Result<String, RustfluxError> {
    match post_file_internal(url, file_name) {
        Ok(res) => Ok(res),
        Err(err) => Err(RustfluxError::PostRequest(format!(
            "Error executing POST request: {:?}",
            err
        ))),
    }
}

// Helper functions

fn post_file_internal(url: &str, file_name: &str) -> Result<String, reqwest::Error> {
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

fn post_internal(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client.post(url).body("").send()?;

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
