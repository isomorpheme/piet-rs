#[macro_use]
extern crate clap;
extern crate image;
extern crate piet;

use image::GenericImage;

use piet::{Color, Interpreter, Program};
use piet::util;

fn main() {
    let matches = clap_app!(pieti =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg SOURCE: +required "Piet source file")
        (@arg codel_size: --codel_size -c +takes_value "Width & height to read codels at")
    ).get_matches();

    println!("{:?}", matches);

    let source_path = matches.value_of("SOURCE").expect("no source supplied");
    let _codel_size = matches.value_of("codel_size").map(|_| unimplemented!());

    let source = image::open(source_path).expect("could not open image");

    let colors = source.pixels()
        .map(|(_, _, p)| Color::from_rgb(p[0], p[1], p[3]))
        .collect();
    let dimensions = util::map_pair(source.dimensions(), |x| x as usize);

    let program = Program::new(dimensions, colors);
    let mut interpreter = Interpreter::new(program);
    interpreter.run();
    println!("{:?}", interpreter);
}
