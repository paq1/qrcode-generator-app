use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct QrcodeAppError {
    pub message: String
}

impl Display for QrcodeAppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "QrcodeAppError : {}", self.message)
    }
}

impl Error for QrcodeAppError {}