use image::{ImageBuffer, Luma};
use qrcode::QrCode;

use crate::core::services::can_save::CanBeSaveInFile;
use crate::core::services::image_service::ImageService;
use crate::core::services::qrcode_generator::QrcodeGenerator;
use crate::models::QrcodeAppError;

pub struct QrcodeGeneratorImpl;

impl QrcodeGenerator<QrCode> for QrcodeGeneratorImpl {
    fn create_qrcode(&self, message: &str) -> QrCode {
        QrCode::new(message).unwrap()
    }
}

pub struct ImageServiceImpl;

impl ImageService<QrCode, ImageBuffer<Luma<u8>, Vec<u8>>> for ImageServiceImpl {
    fn create_qrcode_image(&self, qrcode: &QrCode) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        qrcode.render::<Luma<u8>>().build()
    }
}

impl CanBeSaveInFile for ImageBuffer<Luma<u8>, Vec<u8>> {
    fn save_in_file(&self, path: &str) -> Result<(), QrcodeAppError> {
        self
            .save(path)
            .map_err(|err| QrcodeAppError { message: err.to_string() })
    }
}