use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cmd_to_run = args::Args::parse().command.as_str();

    match cmd_to_run {
        "encode" => {
            let args = args::EncodeArgs::parse();
            println!("{:?}", args);
        }
        "decode" => {
            let args = args::DecodeArgs::parse();
            println!("{:?}", args);
        }
        "remove" => {
            let args = args::RemoveArgs::parse();
            println!("{:?}", args);
        }
        "print" => {
            let args = args::PrintArgs::parse();
            println!("{:?}", args);
        }
    }

    Ok(())
}
