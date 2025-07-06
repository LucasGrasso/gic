pub mod ast;
pub mod clause;
pub mod errors;

pub use errors::GicError;

pub type Result<T> = std::result::Result<T, GicError>;
