pub use prelude::*;

mod token;
mod syntax;

pub mod prelude {
    pub use super::token::Token;
    pub use super::syntax::Syntax;

    pub type SResult<T> = Result<T, String>;

    #[derive(PartialEq, Eq, Debug)]
    pub enum Boolean {
        True,
        False,
        Maybe,
    }

    #[derive(PartialEq, Debug)]
    pub enum Value {
        Boolean(Boolean),
        String(String),
        Number(f64),
        Array(Vec<Value>),
        Undefined,
    }
}
