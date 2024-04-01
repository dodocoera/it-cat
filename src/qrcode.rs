use anyhow::{Context, Result};
use clap::Args;
use qrcode::{render::unicode, QrCode};

#[derive(Args)]
pub struct QrCodeArgs {
    /// 二维码内容
    content: String,
}

impl QrCodeArgs {
    pub fn unicode_generate(&self) -> Result<String> {
        let qrcode = QrCode::new(&self.content)
            .context("Attempt to create QR code failed")?
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();
        Ok(qrcode)
    }
}
