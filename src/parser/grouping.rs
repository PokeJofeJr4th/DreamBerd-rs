use std::iter::Peekable;

use crate::types::prelude::*;

use super::{consume_whitespace, inner_parse};

#[derive(Debug, Clone)]
enum GroupThingieEnum {
    Syntax(Syntax, u8),
    Operation(Operation, u8),
    Unary(UnaryOperation, u8),
}

pub(super) fn parse_group<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> SResult<Syntax> {
    let new_toks = fancify_toks(tokens)?;
    let max_spc = new_toks
        .iter()
        .map(|group| match group {
            GroupThingieEnum::Unary(_, spc)
            | GroupThingieEnum::Operation(_, spc)
            | GroupThingieEnum::Syntax(_, spc) => spc,
        })
        .max()
        .copied()
        .unwrap_or(0);

    // println!("{new_toks:?}");
    inner_parse_group_better(&mut new_toks.into_iter().rev().peekable(), max_spc + 1)
}

fn fancify_toks<T: Iterator<Item = Token>>(
    tokens: &mut Peekable<T>,
) -> SResult<Vec<GroupThingieEnum>> {
    let mut toks = Vec::new();
    loop {
        let mut whitespace = consume_whitespace(tokens);
        match tokens.peek() {
            Some(Token::TackTack) => {
                tokens.next();
                toks.push(GroupThingieEnum::Unary(
                    UnaryOperation::Decrement,
                    whitespace,
                ));
            }
            Some(Token::PlusPlus) => {
                tokens.next();
                toks.push(GroupThingieEnum::Unary(
                    UnaryOperation::Increment,
                    whitespace,
                ));
            }
            Some(
                Token::RParen
                | Token::Bang(_)
                | Token::Question(_)
                | Token::RSquare
                | Token::Comma
                | Token::RSquirrely,
            )
            | None => break,
            Some(tok) => {
                if let Ok(op) = Operation::try_from(tok.clone()) {
                    tokens.next();
                    whitespace += consume_whitespace(tokens);
                    toks.push(GroupThingieEnum::Operation(op, whitespace));
                } else {
                    let inner = inner_parse(tokens)?;
                    if matches!(inner, Syntax::Statement(..)) {
                        toks.push(GroupThingieEnum::Syntax(inner, whitespace));
                        break;
                    }
                    toks.push(GroupThingieEnum::Syntax(inner, whitespace));
                }
            }
        }
    }
    Ok(toks)
}

fn inner_parse_group_better<T: Iterator<Item = GroupThingieEnum>>(
    tokens: &mut Peekable<T>,
    spacing: u8,
) -> SResult<Syntax> {
    if spacing == 0 {
        return match tokens.next() {
            Some(GroupThingieEnum::Syntax(lhs, _)) => Ok(lhs),
            Some(GroupThingieEnum::Unary(unary, spc)) => Ok(Syntax::UnaryOperation(
                unary,
                Box::new(inner_parse_group_better(tokens, spc + 1)?),
            )),
            Some(other) => Err(format!("Expected expression; got `{other:?}`")),
            None => Err(String::from("Unexpected EOF")),
        };
    }
    if let Some(GroupThingieEnum::Unary(unary, spc)) = tokens.peek() {
        let unary = unary.clone();
        let spc = *spc;
        tokens.next();
        return Ok(Syntax::UnaryOperation(
            unary,
            Box::new(inner_parse_group_better(tokens, spc + 1)?),
        ));
    }
    let rhs = inner_parse_group_better(tokens, spacing - 1)?;
    // println!("{rhs}");
    let starter_val = match tokens.peek() {
        Some(GroupThingieEnum::Operation(op, spc)) if *spc < spacing => {
            let op = *op;
            tokens.next();
            let lhs = inner_parse_group_better(tokens, spacing)?;
            make_operation(lhs, op, rhs)
        }
        _ => Ok(rhs),
    }?;
    match tokens.peek() {
        Some(GroupThingieEnum::Syntax(_, spc)) if *spc <= spacing => {
            let Some(GroupThingieEnum::Syntax(func, _)) = tokens.next() else {unreachable!()};
            let args = match starter_val {
                Syntax::Block(args) => args,
                other => vec![other],
            };
            Ok(Syntax::UnaryOperation(
                UnaryOperation::Call(args),
                Box::new(func),
            ))
        }
        _ => Ok(starter_val),
    }
}

/// if `op` is `->`, try to make it into a function
fn make_operation(left: Syntax, op: Operation, right: Syntax) -> SResult<Syntax> {
    if op == Operation::Arrow {
        // println!("{left:?} -> {right:?}");
        let input = match left {
            Syntax::Block(vals) => vals
                .into_iter()
                .map(|syn| match syn {
                    Syntax::Ident(ident) => Ok(ident),
                    other => Err(format!(
                        "Function input can only have identifiers, not {other:?}"
                    )),
                })
                .collect::<Result<Vec<_>, _>>()?,
            Syntax::Ident(ident) => vec![ident],
            other => return Err(format!("Function input can only have identifier or parenthesized list of values; got {other}")),
        };
        Ok(Syntax::Function(input, Box::new(right)))
    } else {
        Ok(Syntax::Operation(Box::new(left), op, Box::new(right)))
    }
}
