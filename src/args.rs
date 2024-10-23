use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub command: PngMeArgs,
}

#[derive(Parser, Debug)]
pub enum PngMeArgs {
    /// Encode a message in a PNG file
    Encode(EncodeArgs),
    /// Decode a message from a PNG file
    Decode(DecodeArgs),
    /// Remove a chunk from a PNG file
    Remove(RemoveArgs),
    /// Print a PNG file
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct EncodeArgs {
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

#[derive(Parser, Debug)]
#[command(version)]
pub struct DecodeArgs {
    /// File path
    #[arg(short, long)]
    pub filepath: String,

    /// Chunk type
    #[arg(short, long)]
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct RemoveArgs {
    /// File path
    #[arg(short, long)]
    pub filepath: String,

    /// Chunk type
    #[arg(short, long)]
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct PrintArgs {
    /// File path
    #[arg(short, long)]
    pub filepath: String,
}
