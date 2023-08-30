pub mod linker;
pub mod macros;
pub mod parser;
pub mod utils;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PACKAGE: &str = env!("CARGO_PKG_NAME");
