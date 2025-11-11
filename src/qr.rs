use qrcode::QrCode;
use image::Luma;
use std::io::Cursor;

/// Generates a QR code image as bytes (PNG format)
pub fn generate_qr_png(data: &str) -> Vec<u8> {
    let code = QrCode::new(data).expect("Failed to generate QR code");

    // Render to an image buffer (Luma grayscale)
    let image = code.render::<Luma<u8>>().build();

    // Encode to PNG in memory
    let mut buffer = Cursor::new(Vec::new());
    image
        .write_to(&mut buffer, image::ImageFormat::Png)
        .expect("Failed to write PNG");
    buffer.into_inner()
}

