pub struct Context {
    pub host: String,
    pub database: String,
}

#[derive(Debug)]
pub enum ContextError {
}

impl Context {
    pub fn new() -> Result<Context, ContextError> {
        Ok(Self {
            host: String::from("http://localhost:8086"),
            database: String::from(""),
        })
    }
}
