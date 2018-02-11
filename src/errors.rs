#[derive(Debug)]
pub enum RustfluxError {
    InputParse(String),
    JsonDecode(String),
    GetRequest(String),
    PostRequest(String),
}
