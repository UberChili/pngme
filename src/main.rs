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
            //let filename = encode_args.filepath;
            //let chunk_type = encode_args.chunk_type.as_str();
            //let message = encode_args.message;
            //let out_filepath = encode_args.out_file;

            commands::encode(encode_args);
        }
        args::PngMeArgs::Decode(decode_args) => {
            //let filename = decode_args.filepath;
            //let chunk_type = decode_args.chunk_type.as_str();
            commands::decode(decode_args);
        }
        args::PngMeArgs::Remove(remove_args) => {
            //let filename = remove_args.filepath;
            //let chunk_type = remove_args.chunk_type.as_str();
            commands::remove(remove_args);
        }
        args::PngMeArgs::Print(print_args) => {
            //let filename = print_args.filepath;
            commands::print_chunks(print_args);
        }
    }

    Ok(())
}
