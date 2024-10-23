use std::fs;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;

/// Encodes a message into a PNG file and saves the Result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let filename = args.filepath;
    let chunk_type = args.chunk_type;
    let message = args.message;
    let out_filename = args.out_file;

    let file = fs::read(&filename.as_str())?;

    let mut png = Png::try_from(file.as_slice())?;

    let chunk_type = ChunkType::from_str(&chunk_type)?;
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

    png.append_chunk(chunk);

    fs::write(out_filename, png.as_bytes())?;

    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let filename = args.filepath;
    let chunk_type = args.chunk_type;

    let file = fs::read(&filename.as_str())?;

    let png = Png::try_from(file.as_slice())?;

    let chunk = png
        .chunks()
        .iter()
        .find(|c| c.chunk_type().to_string() == chunk_type);

    if let Some(chunk) = chunk {
        let message = String::from_utf8(chunk.chunk_data().to_vec())?;
        println!("Message: {}", message);
    } else {
        println!(
            "Error: No chunk of type '{}' was found in the file",
            chunk_type
        );
    }

    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let filename = args.filepath;

    let file: Vec<u8> = fs::read(&filename)?;
    let mut png = Png::try_from(file.as_slice())?;

    // Attempting to remove chunk
    let chunk_type = args.chunk_type;

    // Check if chunk exists before trying to remove it
    let chunk_exists = png
        .chunks()
        .iter()
        .any(|c| c.chunk_type().to_string() == chunk_type);

    if !chunk_exists {
        println!(
            "Warning: No chunk of type '{}' was found in the file",
            chunk_type
        );
        return Ok(());
    }

    png.remove_first_chunk(chunk_type.as_str())?;

    // Write the complete PNG data back to file
    fs::write(filename, png.as_bytes())?;

    println!(
        "Successfully removed chunk of type '{}' from the file",
        chunk_type
    );
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let filename = args.filepath;
    let file: Vec<u8> = fs::read(filename)?;
    let png = Png::try_from(file.as_slice())?;

    for chunk in png.chunks() {
        println!("{}", chunk);
    }

    Ok(())
}
