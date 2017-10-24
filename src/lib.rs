#[macro_use]
extern crate error_chain;
extern crate num;

pub use color::{Color, Hue, Lightness};
pub use interpreter::Interpreter;
pub use program::Program;
pub use stack::Stack;

mod color;
mod command;
mod errors;
mod interpreter;
mod program;
mod stack;
pub mod util;
