use errors::RustfluxError;

pub struct Context {
    pub host: String,
    pub database: String,
}

impl Context {
    pub fn new() -> Result<Context, RustfluxError> {
        Ok(Self {
            host: String::from("http://localhost:8086"),
            database: String::from(""),
        })
    }
}
