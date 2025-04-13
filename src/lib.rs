// Public API surface of the kedge-wx runtime

pub mod decode;
pub mod runtime;
pub mod error;
pub mod exec;

// Optional: re-export error for easy access
pub use crate::error::KedgeError;
