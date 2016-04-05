use color::Color;

pub struct Program {
    image: Vec<Vec<Color>>,
}

impl Program {
    pub fn get(&self, coords: (usize, usize)) -> Option<Color> {
        unimplemented!();
    }

    pub fn neighbors(&self, coords: (usize, usize)) -> &[Color] {
        unimplemented!();
    }

    pub fn neighbors_with_coords(&self, coords: (usize, usize)) -> &[(usize, usize, Color)] {
        unimplemented!();
    }
}
