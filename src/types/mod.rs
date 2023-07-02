pub use prelude::*;

mod pointer;
mod state;
mod syntax;
mod token;
mod value;

pub mod prelude {
    pub use super::pointer::Pointer;
    pub use super::state::State;
    pub use super::syntax::{Operation, Syntax, VarType};
    pub use super::token::{StringSegment, Token};
    pub use super::value::{Boolean, Keyword, Value};

    pub type SResult<T> = Result<T, String>;
    pub type OpGroup = (Syntax, Operation, u8);
}
