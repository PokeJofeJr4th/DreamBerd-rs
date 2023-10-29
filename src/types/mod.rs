pub use prelude::*;

mod pointer;
mod state;
mod syntax;
mod token;
mod value;

pub mod prelude {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub use super::pointer::{MutValue, Pointer};
    pub use super::state::State;
    pub use super::syntax::{Lifetime, Operation, Syntax, UnaryOperation, VarType};
    pub use super::token::{StringSegment, Token};
    pub use super::value::{Boolean, Keyword, Value};

    pub type SResult<T> = Result<T, String>;
    pub type RcMut<T> = Rc<RefCell<T>>;

    pub fn rc_mut_new<T>(content: T) -> RcMut<T> {
        Rc::new(RefCell::new(content))
    }
}
