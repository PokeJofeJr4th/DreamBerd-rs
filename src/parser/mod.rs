use std::iter::Peekable;

use crate::types::prelude::*;

mod grouping;

pub fn parse(tokens: Vec<Token>) -> SResult<Syntax> {
    let mut tokens = tokens.into_iter().peekable();
    let mut syntax = Vec::new();
    while tokens.peek().is_some() {
        syntax.push(grouping::parse_group(&mut tokens)?);
    }
    Ok(optimize(Syntax::Block(syntax)))
}

fn inner_parse<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> SResult<Syntax> {
    // println!("{:?}", tokens.peek());
    match tokens.next() {
        Some(Token::String(str)) => Ok(Syntax::String(str)),
        Some(Token::Tack | Token::Semicolon) => Ok(Syntax::Negate(Box::new(inner_parse(tokens)?))),
        Some(Token::Ident(id)) => {
            consume_whitespace(tokens);
            if id.as_ref() == "const" || id.as_ref() == "var" {
                declare(tokens, &id)
            } else {
                match tokens.peek() {
                    // call as a function
                    Some(Token::LParen) => {
                        tokens.next();
                        consume_whitespace(tokens);
                        let input = get_tuple(tokens)?;
                        Ok(Syntax::Call(id, input))
                    }
                    Some(Token::Colon) => {
                        tokens.next();
                        consume_whitespace(tokens);
                        get_type(tokens)?;
                        Ok(Syntax::Ident(id))
                    }
                    // get the value of the variable
                    _ => Ok(Syntax::Ident(id)),
                }
            }
        }
        Some(Token::LSquirrely) => {
            let mut statements_buf = Vec::new();
            while let Some(tok) = tokens.peek() {
                match tok {
                    Token::RSquirrely => break,
                    Token::Space(_) => {
                        tokens.next();
                        continue;
                    }
                    _ => {}
                }
                let inner = grouping::parse_group::<T>(tokens)?;
                statements_buf.push(consume_bang(inner, tokens));
            }
            if tokens.next() == Some(Token::RSquirrely) {
                Ok(Syntax::Block(statements_buf))
            } else {
                Err(String::from("Expected `}`"))
            }
        }
        Some(Token::Space(_)) => inner_parse(tokens),
        Some(Token::LParen) => {
            let val = get_tuple(tokens)?;
            if let [x] = &val[..] {
                Ok(x.clone())
            } else {
                Ok(Syntax::Block(val))
            }
        }
        Some(other) => Err(format!("Unexpected token `{other:?}`")),
        None => Err(String::from("Unexpected End of File")),
    }
}

fn consume_whitespace<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> u8 {
    let mut sp = 0;
    while let Some(&Token::Space(s)) = tokens.peek() {
        tokens.next();
        sp += s;
    }
    sp
}

fn consume_bang<T: Iterator<Item = Token>>(syn: Syntax, tokens: &mut Peekable<T>) -> Syntax {
    match tokens.peek() {
        Some(&Token::Bang(q)) => {
            tokens.next();
            Syntax::Statement(false, Box::new(syn), q)
        }
        Some(&Token::Question(q)) => {
            tokens.next();
            Syntax::Statement(true, Box::new(syn), q)
        }
        _ => syn,
    }
}

fn declare<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>, id: &str) -> SResult<Syntax> {
    let Some(Token::Ident(second)) = tokens.next() else {
                    return Err(String::from("Expected `const` or `var` after `{id}`"))
                };
    let var_type = match (id, second.as_ref()) {
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
    // consume a type definition
    if tokens.peek() == Some(&Token::Colon) {
        tokens.next();
        consume_whitespace(tokens);
        get_type(tokens)?;
        consume_whitespace(tokens);
    }
    let value = match tokens.next() {
        Some(Token::Bang(_)) => Syntax::Ident(String::new().into()),
        Some(Token::Equal(1)) => {
            consume_whitespace(tokens);
            grouping::parse_group::<T>(tokens)?
        }
        other => {
            return Err(format!(
                "Expected `!`, `:`, or `=` after variable name, got `{other:?}`"
            ))
        }
    };
    Ok(consume_bang(
        Syntax::Declare(var_type, varname, Box::new(value)),
        tokens,
    ))
}

fn get_tuple<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> SResult<Vec<Syntax>> {
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
            _ => args_buf.push(grouping::parse_group::<T>(tokens)?),
        }
    }
    Ok(args_buf)
}

fn get_type<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> SResult<()> {
    match tokens.next() {
        Some(Token::Ident(_)) => {}
        other => return Err(format!("Expected a type after `:`; got `{other:?}`")),
    }
    consume_whitespace(tokens);
    match tokens.peek() {
        Some(Token::LSquare) => {
            tokens.next();
            let Some(Token::RSquare) = tokens.next() else {
                return Err(String::from("Expected `]` after `[` in type definition"))
            };
        }
        Some(Token::LCaret) => {
            tokens.next();
            get_type(tokens)?;
            while tokens.peek() == Some(&Token::Comma) {
                tokens.next();
                get_type(tokens)?;
                consume_whitespace(tokens);
            }
            let Some(Token::RCaret) = tokens.next() else {
                return Err(String::from("Missing `>` in type definition"))
            };
        }
        _ => {}
    }
    Ok(())
}

fn optimize(syn: Syntax) -> Syntax {
    match syn {
        Syntax::Declare(typ, ident, inner) => {
            Syntax::Declare(typ, ident, Box::new(optimize(*inner)))
        }
        Syntax::Function(args, inner) => Syntax::Function(args, Box::new(optimize(*inner))),
        Syntax::Call(ident, args) => Syntax::Call(ident, args.into_iter().map(optimize).collect()),
        Syntax::Operation(lhs, op, rhs) => {
            Syntax::Operation(Box::new(optimize(*lhs)), op, Box::new(optimize(*rhs)))
        }
        Syntax::UnaryOperation(unary, operand) => {
            Syntax::UnaryOperation(unary, Box::new(optimize(*operand)))
        }
        Syntax::Block(inner) => {
            let mut new_inner: Vec<_> = Vec::with_capacity(inner.len());
            // flatten nested blocks
            for item in inner {
                match item {
                    Syntax::Block(block) => new_inner.extend(block),
                    other => new_inner.push(other),
                }
            }
            // replace a block containing a single element with that element
            if new_inner.len() == 1 {
                new_inner.pop().unwrap()
            } else {
                Syntax::Block(new_inner)
            }
        }
        Syntax::Statement(is_debug, inner, lvl) => {
            Syntax::Statement(is_debug, Box::new(optimize(*inner)), lvl)
        }
        Syntax::Negate(inner) => Syntax::Negate(Box::new(optimize(*inner))),
        basic @ (Syntax::Ident(_) | Syntax::String(_)) => basic,
    }
}
