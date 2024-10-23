use std::fs;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::{Error, Result};

/// Encodes a message into a PNG file and saves the Result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let filename = args.filepath;
    let chunk_type: [u8; 4] = args.chunk_type.as_bytes().try_into()?;
    let message = args.message;
    let out_filepath = args.out_file;

    let file: Vec<u8> = fs::read(filename)?;

    let ct = ChunkType::try_from(chunk_type);

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    todo!()
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    todo!()
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let filename = args.filepath;

    println!("Attempting to open {}", filename);

    let file: Vec<u8> = fs::read(filename)?;

    //let chunks = Chunk::try_from(file.as_slice());

    let png = Png::try_from(file.as_slice())?;

    for chunk in png.chunks() {
        println!("{}", chunk);
    }

    Ok(())
}
