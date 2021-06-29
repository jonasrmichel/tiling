use std::ops::RangeInclusive;

use crate::{Error::*, Result};

/// The valid range of a color value (0 to 255 inclusive).
const RGB_RANGE: RangeInclusive<i32> = 0..=255;

/// A color with red, green, and blue components.
#[derive(Clone, Copy, Debug)]
pub struct Color {
    red: i32,
    green: i32,
    blue: i32,
}

impl Color {
    /// Returns a new color, validating each component is in the range [0, 255].
    pub fn new(red: i32, green: i32, blue: i32) -> Result<Color> {
        if !(RGB_RANGE.contains(&red) && RGB_RANGE.contains(&green) && RGB_RANGE.contains(&blue)) {
            return Err(InvalidColor);
        }

        Ok(Color { red, green, blue })
    }

    /// Returns the red component.
    pub fn red(&self) -> i32 {
        self.red
    }

    /// Returns the green component.
    pub fn green(&self) -> i32 {
        self.green
    }

    /// Returns the blue component.
    pub fn blue(&self) -> i32 {
        self.blue
    }

    /// Returns the red, green, and blue comonents as a tuple where each component
    /// has been translated into the unit interval (0 to 1 inclusive).
    pub fn rgb_unit_int(&self) -> (f64, f64, f64) {
        fn unit_int(i: i32) -> f64 {
            i as f64 / 255.0
        }

        (
            unit_int(self.red),
            unit_int(self.green),
            unit_int(self.blue),
        )
    }
}
