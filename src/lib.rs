#[macro_use]
extern crate error_chain;

pub use crate::color::{Color, Hue, Lightness};
pub use crate::interpreter::Interpreter;
pub use crate::program::Program;
pub use crate::stack::Stack;

mod color;
mod command;
mod errors;
mod interpreter;
mod program;
mod stack;
pub mod util;
