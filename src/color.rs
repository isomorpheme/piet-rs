use num::Integer;

/// A color's hue.
#[derive(Debug, Clone, Copy)]
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
    /// use piet::Hue;
    /// use piet::Hue::*;
    ///
    /// assert_eq!(Hue::shift(Red, Yellow), 1);
    /// assert_eq!(Hue::shift(Red, Magenta), 5);
    /// ```
    pub fn shift(from: Hue, to: Hue) -> u8 {
        ((to as i8) - (from as i8)).mod_floor(&6) as u8
    }
}

/// A color's lightness.
#[derive(Debug, Clone, Copy)]
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
    /// use piet::Lightness;
    /// use piet::Lightness::*;
    ///
    /// assert_eq!(Lightness::shift(Light, Normal), 1);
    /// assert_eq!(Lightness::shift(Dark, Light), 1);
    /// ```
    pub fn shift(from: Lightness, to: Lightness) -> u8 {
        ((to as i8) - (from as i8)).mod_floor(&3) as u8
    }
}

/// A color, as interpreted by Piet.
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Composite(Hue, Lightness),
    Black,
    White,
}

impl Color {
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
    /// use piet::Color;
    /// use piet::Hue::*;
    /// use piet::Lightness::*;
    ///
    /// let a = Color::Composite(Blue, Light);
    /// let b = Color::Composite(Green, Dark);
    ///
    /// assert_eq!(Color::transition(a, b), Some((4, 2)))
    /// ```
    pub fn transition(from: Color, to: Color) -> Option<(u8, u8)> {
        let (from_h, from_l) = match from {
            Color::Composite(h, l) => (h, l),
            _ => return None,
        };

        let (to_h, to_l) = match to {
            Color::Composite(h, l) => (h, l),
            _ => return None,
        };

        Some((Hue::shift(from_h, to_h), Lightness::shift(from_l, to_l)))
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
