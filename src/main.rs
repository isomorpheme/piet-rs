use std::path::PathBuf;

use image::{self, GenericImageView};
use structopt::StructOpt;

use piet::util;
use piet::{Color, Interpreter, Program};

#[derive(Debug, StructOpt)]
struct Opt {
    /// Piet source file.
    ///
    /// Supported file types: PNG, GIF, BMP.
    #[structopt(name = "SOURCE", parse(from_os_str))]
    source_path: PathBuf,

    /// Width & height to read codels at.
    #[structopt(short, long)]
    codel_size: usize,
}

fn main() {
    let opt = Opt::from_args();
    let source = image::open(opt.source_path).expect("could not open image");

    let colors = source
        .pixels()
        .map(|(_, _, p)| Color::from_rgb(p[0], p[1], p[2]))
        .collect();
    let dimensions = util::map_pair(source.dimensions(), |x| x as usize);

    let program = Program::new(dimensions, colors);
    let mut interpreter = Interpreter::new(program);
    interpreter.run();
    println!("{:?}", interpreter);
}
