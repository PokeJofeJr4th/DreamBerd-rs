pub use prelude::*;

mod syntax;
mod token;
mod value;

pub mod prelude {
    pub use super::syntax::{Syntax, VarType};
    pub use super::token::Token;
    pub use super::value::{Boolean, Keyword, Pointer, Value};

    pub type SResult<T> = Result<T, String>;
}
