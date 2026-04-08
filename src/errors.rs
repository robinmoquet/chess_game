use std::fmt;

#[derive(Debug)]
pub struct ActionError {
    pub message: &'static str,
}

impl ActionError {
    pub fn new(message: &'static str) -> Self {
        ActionError { message }
    }
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action err: {}", self.message)
    }
}
