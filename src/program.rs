use color::Color;

pub struct Program {
    size: (usize, usize),
    image: Vec<Color>,
}

impl Program {
    pub fn new(size: (usize, usize), image: Vec<Color>) -> Self {
        Program {
            size: (0, 0),
            image: Vec::new(),
        }
    }

    pub fn get(&self, coords: (usize, usize)) -> Option<Color> {
        coords_to_index(coords, &self.size).map(|index| self.image[index])
    }

    pub fn color_block(&self, coords: (usize, usize)) -> &[Color] {
        let color = self.get(coords).unwrap(); // TODO: decide if this needs actual error handling
        self.neighbors_with_coords(coords)
            .into_iter()
            .filter(|&&(_, other)| other == color)
            .map(|&(c, _)| self.color_block(c));

        unimplemented!()
    }

    pub fn neighbors(&self, coords: (usize, usize)) -> &[Color] {
        unimplemented!();
    }

    pub fn neighbors_with_coords(&self, coords: (usize, usize)) -> &[((usize, usize), Color)] {
        unimplemented!();
    }
}

fn coords_to_index(coords: (usize, usize), size: &(usize, usize)) -> Option<usize> {
    let (x, y) = coords;
    let (width, height) = *size;

    if x >= width || y >= height {
        return None;
    }

    Some(width * y + x % width)
}

fn index_to_coords(index: usize, size: &(usize, usize)) -> Option<(usize, usize)> {
    let (width, height) = *size;

    if index >= width * height {
        return None;
    }

    let x = index % width;
    let y = index / width;

    Some((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{coords_to_index, index_to_coords};

    #[test]
    fn test_coords_to_index() {
        let size = (5, 5);
        assert_eq!(coords_to_index((0, 0), &size), Some(0));
        assert_eq!(coords_to_index((2, 0), &size), Some(2));
        assert_eq!(coords_to_index((0, 2), &size), Some(10));
        assert_eq!(coords_to_index((2, 2), &size), Some(12));
        assert_eq!(coords_to_index((4, 4), &size), Some(24));
        assert_eq!(coords_to_index((5, 5), &size), None);
    }

    #[test]
    fn test_index_to_coords() {
        let size = (5, 5);
        assert_eq!(index_to_coords(0, &size), Some((0, 0)));
        assert_eq!(index_to_coords(2, &size), Some((2, 0)));
        assert_eq!(index_to_coords(10, &size), Some((0, 2)));
        assert_eq!(index_to_coords(12, &size), Some((2, 2)));
        assert_eq!(index_to_coords(24, &size), Some((4, 4)));
        assert_eq!(index_to_coords(25, &size), None);
    }
}
