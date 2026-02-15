pub mod ast;
pub mod backend;
pub mod frontend;
pub mod lox;
pub mod lox_error;

// re-export as a flat package
pub use lox::*;
pub use lox_error::*;
