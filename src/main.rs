use anyhow::Result;
use clap::Parser;
use it_cat::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::QrCode(qrcode_args) => {
            let qrcode = qrcode_args.unicode_generate()?;
            println!("{qrcode}")
        }
    }
    Ok(())
}
