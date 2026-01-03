#![allow(unused)]
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
        args::Command::Encode(a) => commands::encode(a),
        args::Command::Decode(a) => commands::decode(a),
        args::Command::Remove(a) => commands::remove(a),
        args::Command::Print(a) => commands::print(a),
    }
}
