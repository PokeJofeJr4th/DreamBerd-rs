#![warn(clippy::pedantic, clippy::nursery)]

use std::alloc::System;

#[global_allocator]
static A: System = System;

use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use dialoguer::Confirm;

use clap::{Parser, Subcommand};
use interpreter::inner_interpret;
use types::{rc_mut_new, Pointer, RcMut, State, Syntax};

mod interpreter;
mod lexer;
mod parser;
#[cfg(test)]
mod tests;
mod types;

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
            println!("\x1b[93mRepl - DreamBerd-rs\x1b[0m");
            //
            let state = rc_mut_new(State::new());
            if let Some(path) = path {
                let syn = file_to_syntax(&PathBuf::from(path))?;
                let statements = match syn {
                    Syntax::Block(statements) => statements,
                    other => vec![other],
                };
                for statement in statements {
                    inner_interpret(&statement, state.clone())?;
                }
                // println!("{result}");
                // println!("{state:?}");
            }

            let path = "history";

            let mut rl = Editor::<()>::new();
            if rl.load_history(path).is_err() {
                println!("No hist");
            }

            loop {
                //
                let readline = rl.readline(">>> ");
                match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str());
                        //
                        if line.is_empty() {
                            return Ok(());
                        }
                        let result = run_input(&line, state.clone());
                        match result {
                            Ok(ptr) => {
                                if ptr != state.borrow().undefined {
                                    println!("{ptr:?}");
                                }
                            }
                            Err(err) => println!("Error: {err}"),
                        }
                    },
                    Err(ReadlineError::Interrupted) => {
                        // Bye bye! - awesome
                        println!("\nCTRL-C");
                        let confirmation = Confirm::new()
                            .with_prompt("Do you want to leave the repl?")
                            .interact()
                            .unwrap();
                        if confirmation {
                            println!("{}", "Leaving");
                            break
                        }
                    },
                    Err(ReadlineError::Eof) => {
                        println!("CTRL-D");
                        break
                    },
                    Err(err) => {
                        println!("Error: {:?}", err);
                        break
                    }
                }
                rl.save_history(path).unwrap();
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
