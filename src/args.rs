use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "pngme")]
#[command(about = "Hide secret messages in PNG files", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}
#[derive(Parser, Debug)]
pub struct EncodeArgs {
    pub path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output: Option<PathBuf>,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    pub path: PathBuf,
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    pub path: PathBuf,
    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    pub path: PathBuf,
}
