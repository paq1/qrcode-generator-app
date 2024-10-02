use crate::app::services::{ImageServiceImpl, QrcodeGeneratorImpl};
use crate::core::services::can_save::CanBeSaveInFile;
use crate::core::services::image_service::ImageService;
use crate::core::services::qrcode_generator::QrcodeGenerator;

mod app;
mod core;
mod models;

fn main() {
    let qrcode_service = QrcodeGeneratorImpl;
    let image_service = ImageServiceImpl;

    let qrcode = qrcode_service.create_qrcode("https://docs.google.com/forms/d/e/1FAIpQLSekr3Ki82k3FYMTAm_USFHoEACchpkJMDV5Cj_lGiYE1PSsfA/viewform");
    let image = image_service.create_qrcode_image(&qrcode);

    image.save_in_file("target/main.qrcode.png").unwrap();
}