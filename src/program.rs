use std::collections::HashSet;

use color::Color;
use util::lift_tuple;

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
    /// TODO: This implementation is recursive, probably not the most optimal.
    /// Fix it if necessary. (Or if you've had enough of the terrible code.)
    /// See also: https://en.wikipedia.org/wiki/Flood_fill
    pub fn color_block(&self, start_coords: Coords) -> HashSet<Coords> {
        let mut result = HashSet::new();

        let start_color = self.get(start_coords).unwrap();
        // TODO: decide if this needs actual error handling

        self.mark_blocks(start_coords, start_color, &mut result);

        result
    }

    /// The recursive part of `color_block`.
    fn mark_blocks(&self, coords: Coords, start_color: Color, marked: &mut HashSet<Coords>) {
        if marked.contains(&coords) {
            return;
        }

        if let Some(color) = self.get(coords) {
            if color != start_color {
                return;
            }

            marked.insert(coords);

            let (x, y) = coords; for neighbor in self.neighbors(coords).iter().filter_map(|&x| x) {
                self.mark_blocks(neighbor, start_color, marked);
            }
        }
    }

    fn neighbors(&self, coords: Coords) -> [Option<Coords>; 4] {
        let (x, y) = coords;

        let right = lift_tuple((Some(x + 1), Some(y)));
        let left = lift_tuple((x.checked_sub(1), Some(y)));
        let above = lift_tuple((Some(x), Some(y + 1)));
        let below = lift_tuple((Some(x), y.checked_sub(1)));

        // I would have written this with iterators, but AFAIK it's not possible
        // to `.collect()` into a fixed size array.
        [right.and_then(|c| self.check_coords(c)),
         left.and_then(|c| self.check_coords(c)),
         above.and_then(|c| self.check_coords(c)),
         below.and_then(|c| self.check_coords(c))]
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
        assert_eq!(program.color_block((2, 2)), vec![(2, 2)].into_iter().collect());

        let program = Program::new((5, 5),
                                   vec![B, W, W, W, W,
                                        W, W, W, W, W,
                                        W, W, W, W, W,
                                        W, W, W, W, W,
                                        W, W, W, W, W]);
        assert_eq!(program.color_block((0, 0)), vec![(0, 0)] .into_iter().collect());

        let program = Program::new((5, 5),
                                   vec![W, W, W, W, W,
                                        W, B, B, B, W,
                                        W, B, B, B, W,
                                        W, B, B, B, W,
                                        W, W, W, W, W]);
        assert_eq!(program.color_block((2, 2)), vec![(1, 1), (2, 1), (3, 1),
                                                     (1, 2), (2, 2), (3, 2),
                                                     (1, 3), (2, 3), (3, 3)]
                                                    .into_iter().collect());

        let program = Program::new((5, 5),
                                   vec![W, W, W, W, W,
                                        W, B, B, B, W,
                                        W, B, W, B, W,
                                        W, B, W, B, W,
                                        W, W, W, W, W]);
        assert_eq!(program.color_block((1, 1)), vec![(1, 1), (2, 1), (3, 1),
                                                     (1, 2),         (3, 2),
                                                     (1, 3),         (3, 3)]
                                                    .into_iter().collect());

        let program = Program::new((5, 5),
                                   vec![W, W, W, W, W,
                                        W, B, W, B, W,
                                        W, B, W, B, W,
                                        W, B, W, B, W,
                                        W, W, W, W, W]);
        assert_eq!(program.color_block((1, 1)), vec![(1, 1),
                                                     (1, 2),
                                                     (1, 3)]
                                                    .into_iter().collect());
    }
}
