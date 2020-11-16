use num::Integer;

/// A color's hue.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Hue {
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Magenta,
}

impl Hue {
    /// Calculate the change in hue between two hues.
    ///
    /// # Examples
    ///
    /// ```
    /// use piet::Hue::{self, *};
    ///
    /// assert_eq!(Hue::shift(Red, Yellow), 1);
    /// assert_eq!(Hue::shift(Red, Magenta), 5);
    /// ```
    pub fn shift(from: Hue, to: Hue) -> u8 {
        ((to as i8) - (from as i8)).mod_floor(&6) as u8
    }
}

/// A color's lightness.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lightness {
    Light,
    Normal,
    Dark,
}

impl Lightness {
    /// Calculate the change in lightness between two lightness values.
    ///
    /// # Examples
    ///
    /// ```
    /// use piet::Lightness::{self, *};
    ///
    /// assert_eq!(Lightness::shift(Light, Normal), 1);
    /// assert_eq!(Lightness::shift(Dark, Light), 1);
    /// ```
    pub fn shift(from: Lightness, to: Lightness) -> u8 {
        ((to as i8) - (from as i8)).mod_floor(&3) as u8
    }
}

/// A color, as interpreted by Piet.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Composite(Hue, Lightness),
    Black,
    White,
}

impl Color {
    /// Create a color from an RGB triplet.
    ///
    /// # Examples
    ///
    /// ```
    /// use piet::{Color, Hue::*, Lightness::*};
    ///
    /// let color = Color::from_rgb(0xc0, 0xc0, 0xff);
    /// assert_eq!(color, Color::Composite(Blue, Light));
    /// ```
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        use crate::{Color::*, Hue::*, Lightness::*};

        let (r, g, b) = (r as u32, g as u32, b as u32);

        let color: u32 = (((r << 8) | g) << 8) | b;

        match color {
            0xffc0c0 => Composite(Red, Light),
            0xffffc0 => Composite(Yellow, Light),
            0xc0ffc0 => Composite(Green, Light),
            0xc0ffff => Composite(Cyan, Light),
            0xc0c0ff => Composite(Blue, Light),
            0xffc0ff => Composite(Magenta, Light),

            0xff0000 => Composite(Red, Normal),
            0xffff00 => Composite(Yellow, Normal),
            0x00ff00 => Composite(Green, Normal),
            0x00ffff => Composite(Cyan, Normal),
            0x0000ff => Composite(Blue, Normal),
            0xff00ff => Composite(Magenta, Normal),

            0xc00000 => Composite(Red, Dark),
            0xc0c000 => Composite(Yellow, Dark),
            0x00c000 => Composite(Green, Dark),
            0x00c0c0 => Composite(Cyan, Dark),
            0x0000c0 => Composite(Blue, Dark),
            0xc000c0 => Composite(Magenta, Dark),

            0xffffff => White,
            0x000000 => Black,

            _ => White,
            // TODO: maybe make the fallback color configurable? Or even find
            // the nearest color, and create a `Composite` based on that.
        }
    }

    /// Calculate the transition between two colors.
    ///
    /// # Returns
    ///
    /// `Some((u8, u8))` when comparing composite colors,
    /// `None` for black and white.
    ///
    /// # Examples
    ///
    /// ```
    /// use piet::{Color, Hue::*, Lightness::*};
    ///
    /// let a = Color::Composite(Blue, Light);
    /// let b = Color::Composite(Green, Dark);
    ///
    /// assert_eq!(Color::transition(a, b), Some((4, 2)))
    /// ```
    pub fn transition(from: Color, to: Color) -> Option<(u8, u8)> {
        match (from, to) {
            (Color::Composite(from_h, from_l), Color::Composite(to_h, to_l)) => {
                Some((Hue::shift(from_h, to_h), Lightness::shift(from_l, to_l)))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hue_shift() {
        use super::Hue::*;

        assert_eq!(Hue::shift(Red, Red), 0);
        assert_eq!(Hue::shift(Red, Yellow), 1);
        assert_eq!(Hue::shift(Red, Magenta), 5);
        assert_eq!(Hue::shift(Magenta, Red), 1);
        assert_eq!(Hue::shift(Magenta, Blue), 5);
    }

    #[test]
    fn test_lightness_shift() {
        use super::Lightness::*;

        assert_eq!(Lightness::shift(Light, Light), 0);
        assert_eq!(Lightness::shift(Light, Normal), 1);
        assert_eq!(Lightness::shift(Normal, Dark), 1);
        assert_eq!(Lightness::shift(Light, Dark), 2);
        assert_eq!(Lightness::shift(Dark, Light), 1);
        assert_eq!(Lightness::shift(Dark, Normal), 2);
    }

    #[test]
    fn test_color_from_rgb() {
        use super::Hue::*;
        use super::Lightness::*;

        let color = Color::from_rgb(0x00, 0x00, 0x00);
        assert_eq!(color, Color::Black);

        let color = Color::from_rgb(0xff, 0xff, 0xff);
        assert_eq!(color, Color::White);

        let color = Color::from_rgb(0x12, 0x34, 0x56);
        assert_eq!(color, Color::White);

        let color = Color::from_rgb(0x00, 0xff, 0x00);
        assert_eq!(color, Color::Composite(Green, Normal));
    }

    #[test]
    fn test_color_transition() {
        use super::Hue::*;
        use super::Lightness::*;

        let a = Color::Composite(Red, Normal);
        let b = Color::Composite(Red, Normal);
        assert_eq!(Color::transition(a, b), Some((0, 0)));

        let a = Color::Composite(Red, Dark);
        let b = Color::Composite(Yellow, Normal);
        assert_eq!(Color::transition(a, b), Some((1, 2)));

        let a = Color::Black;
        let b = Color::Composite(Red, Normal);
        assert_eq!(Color::transition(a, b), None);

        let a = Color::Composite(Red, Normal);
        let b = Color::Black;
        assert_eq!(Color::transition(a, b), None);
    }
}
