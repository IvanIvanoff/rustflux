extern crate reqwest;

///
pub fn get(url: &str) -> Result<String, reqwest::Error> {
    let mut resp = reqwest::get(url)?;
    let body = resp.text()?;
    Ok(body)
}

///
pub fn post(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let _body = client.post(url).send()?;
    Ok(String::from("sent"))
}
