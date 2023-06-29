use std::iter::Peekable;

use crate::types::prelude::*;

mod grouping;

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
        Some(Token::String(str)) => Ok(Syntax::String(str)),
        Some(Token::Ident(id)) => {
            consume_whitespace(tokens);
            if id == "const" || id == "var" {
                let Some(Token::Ident(second)) = tokens.next() else {
                    return Err(String::from("Expected `const` or `var` after `{id}`"))
                };
                let var_type = match (id.as_ref(), second.as_ref()) {
                    ("var", "var") => VarType::VarVar,
                    ("var", "const") => VarType::VarConst,
                    ("const", "var") => VarType::ConstVar,
                    ("const", "const") => VarType::ConstConst,
                    ("var" | "const", _) => {
                        return Err(format!(
                            "Expected `const` or `var` after `{id}`, not `{second}`"
                        ))
                    }
                    _ => unreachable!(),
                };
                consume_whitespace(tokens);
                let Some(Token::Ident(varname)) = tokens.next() else {
                    return Err(format!("Expected a variable name after `{id} {second}`"))
                };
                consume_whitespace(tokens);
                let value = match tokens.next() {
                    Some(Token::Bang(_)) => Syntax::Ident(String::new()),
                    Some(Token::Equal(1)) => {
                        consume_whitespace(tokens);
                        inner_parse(tokens)?
                    }
                    other => {
                        return Err(format!(
                            "Expected `!` or `=` after variable name, got `{other:?}`"
                        ))
                    }
                };
                consume_bang(tokens);
                Ok(Syntax::Declare(var_type, varname, Box::new(value)))
            } else {
                match tokens.peek() {
                    Some(Token::LParen) => {
                        tokens.next();
                        consume_whitespace(tokens);
                        let mut args_buf = Vec::new();
                        while let Some(tok) = tokens.peek() {
                            match tok {
                                Token::Comma => {
                                    tokens.next();
                                    consume_whitespace(tokens);
                                }
                                Token::RParen => {
                                    tokens.next();
                                    break;
                                }
                                _ => args_buf.push(parse_group(tokens)?),
                            }
                        }
                        consume_bang(tokens);
                        Ok(Syntax::Function(id, args_buf))
                    }
                    // get the value of the variable
                    _ => Ok(Syntax::Ident(id)),
                }
            }
        }
        Some(Token::LSquirrely) => {
            let mut statements_buf = Vec::new();
            while let Some(tok) = tokens.peek() {
                if tok == &Token::RSquirrely {
                    break;
                }
                statements_buf.push(inner_parse(tokens)?);
            }
            if tokens.next() == Some(Token::RSquirrely) {
                Ok(Syntax::Block(statements_buf))
            } else {
                Err(String::from("Expected `}`"))
            }
        }
        Some(Token::Space(_)) => inner_parse(tokens),
        Some(Token::LParen) => {
            let syn = parse_group(tokens)?;
            if tokens.next() == Some(Token::RParen) {
                Ok(syn)
            } else {
                Err(String::from("Expected `)`"))
            }
        }
        Some(other) => Err(format!("Unexpected token `{other:?}`")),
        None => Err(String::from("Unexpected End of File")),
    }
}

fn parse_group<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> SResult<Syntax> {
    let mut groups_buf = Vec::new();
    let tail;
    loop {
        let left = inner_parse(tokens)?;
        let mut spc = if let Some(&Token::Space(spc)) = tokens.peek() {
            tokens.next();
            spc
        } else {
            0
        };
        let op = match tokens.peek() {
            Some(&Token::Equal(eq)) => Operation::Equal(eq),
            Some(Token::Plus) => Operation::Add,
            Some(Token::PlusEq) => Operation::AddEq,
            Some(Token::Tack) => Operation::Sub,
            Some(Token::TackEq) => Operation::SubEq,
            Some(Token::Star) => Operation::Mul,
            Some(Token::StarEq) => Operation::MulEq,
            Some(Token::Slash) => Operation::Div,
            Some(Token::SlashEq) => Operation::DivEq,
            _ => {
                tail = left;
                break;
            }
        };
        tokens.next();
        if let Some(&Token::Space(spc_right)) = tokens.peek() {
            spc += spc_right;
        }
        groups_buf.push((left, op, spc));
    }
    Ok(grouping::group(groups_buf, tail))
}

fn consume_whitespace<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) {
    while let Some(Token::Space(_)) = tokens.peek() {
        tokens.next();
    }
}

fn consume_bang<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) {
    while let Some(Token::Bang(_)) = tokens.peek() {
        tokens.next();
    }
}
