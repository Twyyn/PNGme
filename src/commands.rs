use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::png::{Chunk, ChunkType, Png};
use crate::{Error, Result};

pub fn encode(args: EncodeArgs) -> Result<()> {
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;

    let mut png = Png::from_file(&args.path)?;

    let chunk = Chunk::new(chunk_type, args.message.into_bytes());
    png.append_chunk(chunk);

    let output_path = args.output.as_ref().unwrap_or(&args.path);
    png.to_file(output_path)?;

    println!("Encoded message into chunk '{}'", args.chunk_type);
    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    let png = Png::from_file(&args.path)?;

    let chunk = png
        .chunk_by_type(&args.chunk_type)
        .ok_or(Error::from("Chunk type not found"))?;

    let message = chunk.data_as_string()?;
    println!("{}", message);

    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(&args.path)?;

    png.remove_first_chunk(&args.chunk_type)?;
    png.to_file(&args.path)?;

    println!("Removed chunk '{}'", args.chunk_type);
    Ok(())
}

pub fn print(args: PrintArgs) -> Result<()> {
    let png = Png::from_file(&args.path)?;
    println!("{}", png);
    Ok(())
}
