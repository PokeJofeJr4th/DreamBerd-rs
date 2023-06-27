#![warn(clippy::pedantic, clippy::nursery)]

use std::{error::Error, fs, path::PathBuf};

use clap::Parser;

mod lexer;
mod parser;
mod types;

#[derive(Parser)]
struct Args {
    /// path to the source file
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let path = PathBuf::from(args.path);
    let file = format!("{{{}}}", fs::read_to_string(path)?);
    let tokens = lexer::tokenize(&file)?;
    println!("{tokens:?}");
    let parsed = parser::parse(tokens)?;
    println!("{parsed:?}");
    Ok(())
}
