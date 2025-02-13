//!
//! This module includes all default colors for quick use
//!

use crate::{Color, ColorRGBA, ColorType};

/// A color in the gray spectrum
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Grays {
    /// The value for the color, 0: black, 1: white
    v: f32,
    /// The alpha value
    a: f32,
}

impl Grays {
    /// Constructs a new gray color
    ///
    /// parameters
    ///
    /// v: The value of the gray color, 0: black, 1: white
    ///
    /// a: The alpha value
    pub fn new(v: f32, a: f32) -> Self {
        return Self {
            v: v.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        };
    }

    /// Constructs a new gray color without clamping the input values
    ///
    /// parameters
    ///
    /// v: The value of the gray color, 0: black, 1: white
    ///
    /// a: The alpha value
    pub unsafe fn new_unsafe(v: f32, a: f32) -> Self {
        return Self { v, a };
    }
}

impl Color for Grays {
    const TYPE: ColorType = ColorType::RGB;

    fn get_rgba(&self) -> crate::ColorRGBA {
        return unsafe { ColorRGBA::new_unsafe(self.v, self.v, self.v, self.a) };
    }
}
