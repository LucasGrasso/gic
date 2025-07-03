pub mod ast;
pub mod errors;

pub use ast::Expression; // Re-exporting `Expression`
pub use errors::GicError; // Re-exporting `GicError`

pub type Result<T> = std::result::Result<T, GicError>;
pub type GicResult = Result<Expression>;
