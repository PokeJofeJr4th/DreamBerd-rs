#![warn(clippy::pedantic, clippy::nursery)]

use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use interpreter::inner_interpret;
use types::{rc_mut_new, Pointer, RcMut, State, Syntax};

mod interpreter;
mod lexer;
mod parser;
#[cfg(test)]
mod tests;
mod types;

macro_rules! input {
    ($msg: expr) => {{
        use std::io::Write;
        print!($msg);
        std::io::stdout().flush().unwrap();
        let mut response: String = String::new();
        std::io::stdin().read_line(&mut response).unwrap();
        response.trim().to_owned()
    }};
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    sub_command: SubcommandArg,
}

#[derive(Subcommand)]
enum SubcommandArg {
    Run {
        /// path to the source file
        path: String,
    },
    Repl {
        /// path to the source file (optional)
        path: Option<String>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.sub_command {
        SubcommandArg::Run { path } => {
            let _result = interpreter::interpret(&file_to_syntax(&PathBuf::from(path))?)?;
            // println!("{result:?}");
        }
        SubcommandArg::Repl { path } => {
            let state = rc_mut_new(State::new());
            if let Some(path) = path {
                let syn = file_to_syntax(&PathBuf::from(path))?;
                let Syntax::Block(statements) = syn else { panic!() };
                for statement in statements {
                    inner_interpret(&statement, state.clone())?;
                }
                // println!("{result}");
                // println!("{state:?}");
            }
            loop {
                let input = input!(">>> ");
                if input.is_empty() {
                    return Ok(());
                }
                let result = run_input(&input, state.clone());
                match result {
                    Ok(ptr) => {
                        if ptr != state.borrow().undefined {
                            println!("{ptr}");
                        }
                    }
                    Err(err) => println!("Error: {err}"),
                }
            }
        }
    }
    Ok(())
}

fn run_input(input: &str, context: RcMut<State>) -> Result<Pointer, Box<dyn Error>> {
    Ok(inner_interpret(
        &parser::parse(lexer::tokenize(&format!("{{{input}}}"))?)?,
        context,
    )?)
}

fn file_to_syntax(path: &Path) -> Result<Syntax, Box<dyn Error>> {
    let file = format!("{{{}}}", fs::read_to_string(path)?);
    let tokens = lexer::tokenize(&file)?;
    // println!("{tokens:?}");
    parser::parse(tokens).map_err(Into::into)
}
