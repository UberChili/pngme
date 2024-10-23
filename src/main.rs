use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = commands::Args::parse();

    println!("{:?}", args);

    let filepath = args.filepath;
    let chunk_type = args.chunk_type;
    let message = args.message;
    let output_file = args.out_file;

    Ok(())
}
