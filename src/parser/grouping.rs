use std::iter::Peekable;

use crate::types::prelude::*;

use super::inner_parse;

pub(super) fn parse_group<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> SResult<Syntax> {
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
            Some(Token::Percent) => Operation::Mod,
            Some(Token::PercentEq) => Operation::ModEq,
            Some(Token::LCaret) => Operation::Ls,
            Some(Token::LCaretEq) => Operation::LsEq,
            Some(Token::RCaret) => Operation::Gr,
            Some(Token::RCaretEq) => Operation::GrEq,
            Some(Token::Dot) => Operation::Dot,
            Some(Token::And) => Operation::And,
            Some(Token::Or) => Operation::Or,
            Some(Token::Arrow) => Operation::Arrow,
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
    group(groups_buf, tail)
}

/// merge operators until only the tail remains
fn group(mut src: Vec<OpGroup>, mut tail: Syntax) -> SResult<Syntax> {
    while let Some(&(_, _, val)) = src.iter().min_by_key(|(_, _, u)| u) {
        (src, tail) = inner_group(src, tail, val)?;
    }
    Ok(tail)
}

/// merge operators separated by `val` space
fn inner_group(src: Vec<OpGroup>, mut tail: Syntax, val: u8) -> SResult<(Vec<OpGroup>, Syntax)> {
    let mut grouping_buf: Vec<OpGroup> = Vec::new();
    let mut src_iter = src.into_iter();
    while let Some((left, op, spc)) = src_iter.next() {
        // if the operators are further apart, push it to a later iteration
        if spc != val {
            grouping_buf.push((left, op, spc));
        // if there's a next item, turn `[l op sp, r op sp]` into `[(l op r) op sp]`
        } else if let Some((right, op_2, spc_2)) = src_iter.next() {
            grouping_buf.push((make_operation(left, op, right)?, op_2, spc_2));
        // if there's no next item, turn `[l op sp] r` into `l op r`
        } else {
            tail = make_operation(left, op, tail)?;
        }
    }
    Ok((grouping_buf, tail))
}

/// if `op` is `->`, try to make it into a function
fn make_operation(left: Syntax, op: Operation, right: Syntax) -> SResult<Syntax> {
    if op == Operation::Arrow {
        println!("{left:?} -> {right:?}");
        let input = match left {
            Syntax::Block(vals) => vals
                .into_iter()
                .map(|syn| match syn {
                    Syntax::Ident(ident) => Ok(ident),
                    other => Err(format!(
                        "Function input can only be identifiers, not {other:?}"
                    )),
                })
                .collect::<Result<Vec<_>, _>>()?,
            Syntax::Ident(ident) => vec![ident],
            _ => todo!(),
        };
        Ok(Syntax::Function(input, Box::new(right)))
    } else {
        Ok(Syntax::Operation(Box::new(left), op, Box::new(right)))
    }
}
