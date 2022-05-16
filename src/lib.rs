pub mod error;
mod interprete;
mod parser;
mod scope;
pub mod tank_status;
mod test;
pub use interprete::Interpreter;
pub use pest::error::LineColLocation;
