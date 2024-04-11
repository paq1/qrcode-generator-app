pub trait QrcodeGenerator<QrCode> {
    fn create_qrcode(&self, message: &str) -> QrCode;
}
