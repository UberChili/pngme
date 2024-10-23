use core::panic;

use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = args::Args::parse();

    match args.command.to_lowercase().as_str() {
        "encode" => {
            let sub_args = args::EncodeArgs::parse();
            let filepath = sub_args.filepath;
            let chunk_type = sub_args.chunk_type.as_str();
            let msg = sub_args.message;
            let out_filepath = sub_args.out_file;

            println!("{} {} {} {}", filepath, chunk_type, msg, out_filepath);
        }
        "decode" => {
            println!("This would decode")
        }
        "remove" => {
            println!("This would remove")
        }
        "print" => {
            println!("This would just print")
        }
        _ => {
            return Err("Wrong arguments".into());
        }
    }

    Ok(())
}
