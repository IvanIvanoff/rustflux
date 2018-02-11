extern crate reqwest;
use errors::RustfluxError;

///
pub fn get(url: &str) -> Result<String, RustfluxError> {
    match get_internal(url) {
        Ok(body) => Ok(body),
        Err(_) => Err(RustfluxError::GetRequest(String::from(
            "Error executing GET request",
        ))),
    }
}

pub fn post(url: &str) -> Result<String, RustfluxError> {
    match post_internal(url) {
        Ok(body) => Ok(body),
        Err(_) => Err(RustfluxError::GetRequest(String::from(
            "Error executing GET request",
        ))),
    }
}

///
fn post_internal(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.post(url).send()?;

    match response.status().is_success() {
        true => Ok(String::from("Success")),
        false => Ok(String::from("Request not successful    ")),
    }
}

// Helper functions
fn get_internal(url: &str) -> Result<String, reqwest::Error> {
    let mut resp = reqwest::get(url)?;
    let body = resp.text()?;
    Ok(body)
}
