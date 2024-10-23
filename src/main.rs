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
            let filename = encode_args.filepath;
            let chunk_type = encode_args.chunk_type.as_str();
            let message = encode_args.message;
            let out_filepath = encode_args.out_file;
            println!("{} {} {} {}", filename, chunk_type, message, out_filepath);
        }
        args::PngMeArgs::Decode(decode_args) => {
            let filename = decode_args.filepath;
            let chunk_type = decode_args.chunk_type.as_str();
            println!("{} {} ", filename, chunk_type);
        }
        args::PngMeArgs::Remove(remove_args) => {
            let filename = remove_args.filepath;
            let chunk_type = remove_args.chunk_type.as_str();
            println!("{} {} ", filename, chunk_type);
        }
        args::PngMeArgs::Print(print_args) => {
            let filename = print_args.filepath;
            println!("{}", filename);
        }
    }

    Ok(())
}
