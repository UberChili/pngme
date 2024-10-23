use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File path
    #[arg(short, long)]
    pub filepath: String,

    /// Chunk type
    #[arg(short, long)]
    pub chunk_type: String,

    /// Message
    #[arg(short, long, default_value_t = String::from("Hello"))]
    pub message: String,

    /// Output file (optional)
    #[arg(short, long, default_value_t = String::from("output.png"))]
    pub out_file: String,
}

pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

pub struct EncodeArgs {
    filepath: String,
    chunk_type: String,
    message: String,
    out_file: String,
}

pub struct DecodeArgs {
    filepath: String,
    chunk_type: String,
}

pub struct RemoveArgs {
    filepath: String,
    chunk_type: String,
}

pub struct PrintArgs {
    filepath: String,
}
