pub mod qrcode;
use std::io::{self, IsTerminal, Read};

use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 二维码生成
    QrCode {
        content: String
    },
    /// base64 
    Base64{
        #[command(subcommand)]
        command: Base64Command,
    }
}

#[derive(Subcommand)]
pub enum Base64Command {
    /// base64 encode
    Encode{
        content: Option<String>
    },

    /// base64 decode
    Decode {
        content: Option<String>
    }
}

pub fn read_from_stdin_or_parameter(content: &Option<String>) -> String {
    let data: String;
    if let Some(content) = content {
        data = content.into();
    } else {
        if io::stdin().is_terminal() {
            eprintln!("{}", Cli::command().render_long_help());
            std::process::exit(-1);
        }
        let mut content = String::new();
        io::stdin().lock().read_to_string(&mut content).expect("read from stdin failed.");
        data = content;
    }
    data
}
