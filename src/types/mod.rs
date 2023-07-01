pub use prelude::*;

mod pointer;
mod syntax;
mod token;
mod value;
mod state;

pub mod prelude {
    pub use super::pointer::Pointer;
    pub use super::syntax::{Operation, Syntax, VarType};
    pub use super::token::Token;
    pub use super::value::{Boolean, Keyword, Value};
    pub use super::state::State;

    pub type SResult<T> = Result<T, String>;
    pub type OpGroup = (Syntax, Operation, u8);
}
