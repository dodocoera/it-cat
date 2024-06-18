use anyhow::Result;
use base64::{engine::general_purpose, Engine};
use clap::Parser;
use it_cat::{qrcode::unicode_generate, read_from_stdin_or_parameter, Base64Command, Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::QrCode{content} => {
            let qrcode = unicode_generate(content)?;
            println!("{qrcode}")
        },
        Commands::Base64 { command } => {
            match &command {
                Base64Command::Encode { content } => {
                    let data = read_from_stdin_or_parameter(content);
                    // https://github.com/PowerShell/PowerShell/issues/5974
                    // powershell will add \r\n for each line read, disgusting behavior
                    let res = general_purpose::STANDARD.encode(data.trim());
                    print!("{res}");
                },
                Base64Command::Decode { content } => {
                    let data = read_from_stdin_or_parameter(content);
                    let res = general_purpose::STANDARD.decode(data.trim())?;
                    print!("{}", String::from_utf8(res)?);
                },
            }
        },


    }
    Ok(())
}
