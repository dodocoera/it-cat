pub mod qrcode;
use clap::{Parser, Subcommand};
use qrcode::QrCodeArgs;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 二维码生成
    QrCode(QrCodeArgs),
}
