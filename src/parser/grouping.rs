use crate::types::prelude::*;

pub(super) fn group(mut src: Vec<(Syntax, Operation, u8)>, mut tail: Syntax) -> Syntax {
    while let Some(&(_, _, val)) = src.iter().min_by_key(|(_, _, u)| u) {
        (src, tail) = inner_group(src, tail, val);
    }
    tail
}

fn inner_group(
    src: Vec<(Syntax, Operation, u8)>,
    mut tail: Syntax,
    val: u8,
) -> (Vec<(Syntax, Operation, u8)>, Syntax) {
    let mut grouping_buf = Vec::new();
    let mut src_iter = src.into_iter();
    while let Some((left, op, spc)) = src_iter.next() {
        if spc != val {
            grouping_buf.push((left, op, spc));
            continue;
        }
        if let Some((right, op_2, spc_2)) = src_iter.next() {
            grouping_buf.push((
                Syntax::Operation(Box::new(left), op, Box::new(right)),
                op_2,
                spc_2,
            ));
        } else {
            tail = Syntax::Operation(Box::new(left), op, Box::new(tail));
        }
    }
    (grouping_buf, tail)
}
