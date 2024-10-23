use clap::{CommandFactory, Parser};

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
            let filename = encode_args.filepath;
            let chunk_type = encode_args.chunk_type.as_str();
            let message = encode_args.message;
            let out_filepath = encode_args.out_file;
            println!("{} {} {} {}", filename, chunk_type, message, out_filepath);
        }
        args::PngMeArgs::Decode(decode_args) => {
            todo!()
        }
        args::PngMeArgs::Remove(remove_args) => {
            todo!()
        }
        args::PngMeArgs::Print(print_args) => {
            todo!()
        }
        _ => {
            return Err("Incorrect arguments".into());
        }
    }

    Ok(())
}
