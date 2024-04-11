use crate::models::QrcodeAppError;

pub trait CanBeSaveInFile {
    fn save_in_file(&self, path: &str) -> Result<(), QrcodeAppError>;
}