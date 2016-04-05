use color::Color;

pub struct Program {
    image: Vec<Vec<Color>>,
}

impl Program {
    pub fn get(coords: (usize, usize)) -> Option<Color> {
        unimplemented!();
    }

    pub fn neighbors(coords: (usize, usize)) -> &[Color] {
        unimplemented!();
    }

    pub fn neighbors_with_coords(coords: (usize, usize)) -> &[(usize, usize, Color)] {
        unimplemented!();
    }
}
