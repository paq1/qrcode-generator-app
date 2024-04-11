use crate::core::services::can_save::CanBeSaveInFile;
use crate::models::QrcodeAppError;

pub trait ImageService<QrCode, Image>
where
    Image: CanBeSaveInFile
{
    fn create_qrcode_image(&self, qrcode: &QrCode) -> Image;

    fn save_image(&self, image: &Image, path: &str) -> Result<(), QrcodeAppError> {
        image.save_in_file(path)
    }
}
