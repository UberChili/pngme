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
    match args.command {
        args::PngMeArgs::Encode(encode_args) => {
            let _ = commands::encode(encode_args);
        }
        args::PngMeArgs::Decode(decode_args) => {
            let _ = commands::decode(decode_args);
        }
        args::PngMeArgs::Remove(remove_args) => {
            let _ = commands::remove(remove_args);
        }
        args::PngMeArgs::Print(print_args) => {
            let _ = commands::print_chunks(print_args);
        }
    }

    Ok(())
}
