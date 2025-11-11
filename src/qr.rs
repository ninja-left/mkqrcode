e qrcode::QrCode;
use image::{Luma, ImageBuffer};
use std::io::Cursor;

pub fn generate_qr_png(data: &str) -> Vec<u8> {
     let code = QrCode::new(data.as_bytes()).expect("Failed to create QR code");
    let image = code.render::<Luma<u8>>().build();

    let mut buffer = Cursor::new(Vec::new());
    image.write_to(&mut buffer, image::ImageFormat::Png).unwrap();
    buffer.into_inner()
}
