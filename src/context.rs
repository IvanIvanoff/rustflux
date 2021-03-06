use errors::RustfluxError;
use std::collections::HashMap;

pub struct Context {
    pub host: String,
    pub database: String,
    pub bindings: HashMap<String, String>,
}

impl Context {
    pub fn new() -> Result<Context, RustfluxError> {
        Ok(Self {
            host: String::from("http://localhost:8086"),
            database: String::from(""),
            bindings: HashMap::new(),
        })
    }
}
