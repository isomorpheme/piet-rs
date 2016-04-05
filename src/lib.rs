#![feature(associated_consts)]

extern crate num;

pub use color::{Color, Hue, Lightness};

mod color;
mod command;
mod interpreter;
mod program;
