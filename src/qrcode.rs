use anyhow::{Context, Result};
use qrcode::{render::unicode, QrCode};

pub fn unicode_generate(content: &str) -> Result<String> {
    let qrcode = QrCode::new(content)
        .context("Attempt to create QR code failed")?
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    Ok(qrcode)

}