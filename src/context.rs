pub struct Context {
  pub host: String,
  pub using: String,
}

#[derive(Debug)]
pub enum ContextError {
  NotValid,
}

impl Context {
  pub fn new() -> Result<Context, ContextError> {
    Ok( Self {
      host: String::from(""),
      using: String::from(""),
    })
  }
}