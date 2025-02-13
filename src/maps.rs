//!
//! This module includes all default color maps for quick use
//!

use crate::{ColorMap, ColorND};

/// A color map in the gray spectrum
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Grays {
    /// The alpha value
    a: f32,
}

impl Grays {
    /// Constructs a new gray color map
    ///
    /// parameters
    ///
    /// a: The alpha value
    pub fn new(a: f32) -> Self {
        return Self {
            a: a.clamp(0.0, 1.0),
        };
    }

    /// Constructs a new gray color map without clamping the input values
    ///
    /// parameters
    ///
    /// a: The alpha value
    pub unsafe fn new_unsafe(a: f32) -> Self {
        return Self { a };
    }
}

impl ColorMap<1> for Grays {
    fn get_color(&self, color: ColorND<1>) -> impl crate::Color {
        return unsafe { crate::colors::Grays::new_unsafe(color.get()[0], self.a) };
    }
}
