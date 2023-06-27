use lazy_regex::regex;
use std::iter::Peekable;

use crate::types::{SResult, Syntax, Token};

pub fn parse(tokens: Vec<Token>) -> SResult<Syntax> {
    let mut tokens = tokens.into_iter().peekable();
    let mut syntax = Vec::new();
    while tokens.peek().is_some() {
        syntax.push(inner_parse(&mut tokens)?);
    }
    Ok(Syntax::Block(syntax))
}

fn inner_parse<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> SResult<Syntax> {
    match tokens.next() {
        Some(Token::Ident(id)) => {
            if regex!("^f?u?n?c?t?i?o?n?$").is_match(&id) {
                todo!("This is a function");
            } else if regex!("^Reg(ular)?[eE]x(pression|p)?$").is_match(&id) {
                todo!("This is a regex");
            }
            todo!()
        }
        Some(Token::LSquirrely) => todo!(),
        Some(other) => Err(format!("Unexpected token `{other:?}`")),
        None => Err(String::from("Unexpected End of File")),
    }
}
