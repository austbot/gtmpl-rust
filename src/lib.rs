//! The Golang Templating Language for Rust
//!
//! ## Example
//! ```rust
//! use gtmpl;
//!
//! let output = gtmpl::template("Finally! Some {{ . }} for Rust", "gtmpl");
//! assert_eq!(&output.unwrap(), "Finally! Some gtmpl for Rust");
//! ```
#[macro_use]
extern crate lazy_static;
extern crate itertools;
extern crate serde;
extern crate serde_json;
#[cfg_attr(feature = "cargo-clippy", allow(useless_attribute))]
#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;
mod lexer;
mod node;
mod parse;
pub mod funcs;
pub mod template;
pub mod exec;
mod utils;

pub use template::Template;

pub use exec::Context;

pub use funcs::Func;

pub use serde_json::Value;

use serde::Serialize;
/// Provides simple basic templating given just a template sting and context.
///
/// ## Example
/// ```rust
/// let output = gtmpl::template("Finally! Some {{ . }} for Rust", "gtmpl");
/// assert_eq!(&output.unwrap(), "Finally! Some gtmpl for Rust");
/// ```
pub fn template<T: Serialize>(template_str: &str, context: T) -> Result<String, String> {
    let mut tmpl = Template::default();
    tmpl.parse(template_str)?;
    tmpl.render(Context::from(context)?)
}
