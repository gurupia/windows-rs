/*!
Learn more about Rust for Windows here: <https://github.com/microsoft/windows-rs>
*/

mod bindings;
use bindings::*;

mod com;
use com::*;

mod strings;
use strings::*;

mod error;
pub use error::Error;

mod hresult;
pub use hresult::HRESULT;

/// A specialized [`Result`] type that provides Windows error information.
pub type Result<T> = std::result::Result<T, Error>;