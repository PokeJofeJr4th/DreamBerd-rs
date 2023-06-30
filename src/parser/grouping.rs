use crate::types::prelude::*;

pub(super) fn group(mut src: Vec<OpGroup>, mut tail: Syntax) -> SResult<Syntax> {
    while let Some(&(_, _, val)) = src.iter().min_by_key(|(_, _, u)| u) {
        (src, tail) = inner_group(src, tail, val)?;
    }
    Ok(tail)
}

fn inner_group(src: Vec<OpGroup>, mut tail: Syntax, val: u8) -> SResult<(Vec<OpGroup>, Syntax)> {
    let mut grouping_buf: Vec<OpGroup> = Vec::new();
    let mut src_iter = src.into_iter();
    while let Some((left, op, spc)) = src_iter.next() {
        if spc != val {
            grouping_buf.push((left, op, spc));
            continue;
        }
        if let Some((right, op_2, spc_2)) = src_iter.next() {
            grouping_buf.push((make_operation(left, op, right)?, op_2, spc_2));
        } else {
            tail = make_operation(left, op, tail)?;
        }
    }
    Ok((grouping_buf, tail))
}

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
