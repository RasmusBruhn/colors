#![doc = include_str!("../README.md")]

mod definitions;

pub mod colors;
pub mod maps;
pub mod utils;

pub use definitions::{
    Color, ColorHSIA, ColorHSLA, ColorHSVA, ColorMap, ColorND, ColorRGBA, ColorType,
};
