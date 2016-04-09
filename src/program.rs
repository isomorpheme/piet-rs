use std::iter::{Enumerate, Map};

use color::Color;

pub type Coords = (usize, usize);

pub struct Program {
    size: Coords,
    image: Vec<Color>,
}

impl Program {
    pub fn new(size: Coords, image: Vec<Color>) -> Self {
        Program {
            size: size,
            image: image,
        }
    }

    pub fn get(&self, coords: Coords) -> Option<Color> {
        coords_to_index(coords, &self.size).map(|index| self.image[index])
    }

    /// Find the coordinates of a contiguous area of codels of the same color,
    /// starting from a coordinate.
    ///
    /// TODO: This implementation is super dumb, and recurses infinitely.
    /// So fix that.
    pub fn color_block(&self, coords: Coords) -> Vec<Coords> {
        let color = self.get(coords).unwrap(); // TODO: decide if this needs actual error handling
        let (x, y) = coords;


        let neighbors = [(x + 1, y), (x, y + 1), (x - 1, y), (x, y - 1)];
        let valid_neighbors = neighbors.into_iter()
                                       .map(|&item| item)
                                       .filter(|&c| self.coords_to_index(c).is_some());
        // TODO: put the above into a separate function, if needed.
        // Returning iterators is hard though :(

        let same_color = valid_neighbors.filter(|&c| self.get(c).unwrap() == color);

        let mut result = same_color.map(|c| self.color_block(c))
                                   .collect::<Vec<_>>()
                                   .concat();
        result.push(coords);

        result
    }

    fn coords_to_index(&self, coords: Coords) -> Option<usize> {
        coords_to_index(coords, &self.size)
    }

    fn index_to_coords(&self, index: usize) -> Option<Coords> {
        index_to_coords(index, &self.size)
    }

    fn check_coords(&self, coords: Coords) -> Option<Coords> {
        check_coords(coords, &self.size)
    }
}

fn coords_to_index(coords: Coords, size: &(usize, usize)) -> Option<usize> {
    let (x, y) = coords;
    let (width, height) = *size;

    check_coords(coords, size).and(Some(width * y + x % width))
}

fn index_to_coords(index: usize, size: &(usize, usize)) -> Option<Coords> {
    let (width, height) = *size;

    if index >= width * height {
        return None;
    }

    let x = index % width;
    let y = index / width;

    Some((x, y))
}

fn check_coords(coords: Coords, size: &(usize, usize)) -> Option<Coords> {
    let (x, y) = coords;
    let (width, height) = *size;

    if x >= width || y >= height {
        None
    } else {
        Some(coords)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{coords_to_index, index_to_coords, check_coords};

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

    #[test]
    fn test_check_coords() {
        let size =(5, 5);
        assert!(check_coords((0, 0), &size).is_some());
        assert!(check_coords((4, 0), &size).is_some());
        assert!(check_coords((0, 4), &size).is_some());
        assert!(check_coords((4, 4), &size).is_some());
        assert!(check_coords((5, 4), &size).is_none());
        assert!(check_coords((4, 5), &size).is_none());
        assert!(check_coords((5, 5), &size).is_none());
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_program_color_block() {
        use super::super::Color::White as W;
        use super::super::Color::Black as B;

        let program = Program::new((5, 5),
                                   vec![W, W, W, W, W,
                                        W, W, W, W, W,
                                        W, W, B, W, W,
                                        W, W, W, W, W,
                                        W, W, W, W, W]);
        assert_eq!(program.color_block((2, 2)), vec![(2, 2)]);

        let program = Program::new((5, 5),
                                   vec![W, W, W, W, W,
                                        W, B, B, B, W,
                                        W, B, B, B, W,
                                        W, B, B, B, W,
                                        W, W, W, W, W]);
        assert_eq!(program.color_block((2, 2)), vec![(2, 2)]);
    }
}
