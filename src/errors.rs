use std::fmt;

pub enum RustfluxError {
    InputParse(String),
    JsonDecode(String),
    GetRequest(String),
    PostRequest(String),
    IOError(String),
}

impl fmt::Display for RustfluxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RustfluxError::InputParse(ref error) => write!(f, "Error parsing input: {}", error),
            RustfluxError::JsonDecode(ref error) => write!(f, "Error decoding json: {}", error),
            RustfluxError::GetRequest(ref error) => {
                write!(f, "Error sending GET request: {}", error)
            }
            RustfluxError::PostRequest(ref error) => {
                write!(f, "Error sending POST request: {}", error)
            }
            RustfluxError::IOError(ref error) => write!(f, "IO error: {}", error),
        }
    }
}

impl fmt::Debug for RustfluxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
