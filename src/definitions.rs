use crate::utils;

/// A struct for defining a single color in RGBA space all values are between 0
/// and 1
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct ColorRGBA {
    /// The red combonent
    r: f32,
    /// The green component
    g: f32,
    /// The blue component
    b: f32,
    /// The alpha component
    a: f32,
}

impl ColorRGBA {
    /// Constructs a new rgba color, all values are clamped to between 0 and 1
    ///
    /// # Parameters
    ///
    /// r: The red component
    ///
    /// g: The green component
    ///
    /// b: The blue component
    ///
    /// a: The alpha component
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        return Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        };
    }

    /// Constructs a new rgba color with the alpha component equal to 1, all
    /// values are clamped to between 0 and 1
    ///
    /// # Parameters
    ///
    /// r: The red component
    ///
    /// g: The green component
    ///
    /// b: The blue component
    pub fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        return Self::new(r, g, b, 1.0);
    }

    /// Constructs a new rgba color without validating the input
    ///
    /// # Parameters
    ///
    /// r: The red component
    ///
    /// g: The green component
    ///
    /// b: The blue component
    ///
    /// a: The alpha component
    pub unsafe fn new_unsafe(r: f32, g: f32, b: f32, a: f32) -> Self {
        return Self {
            r: r,
            g: g,
            b: b,
            a: a,
        };
    }

    /// Retrieves the red component of the color
    pub fn get_red(&self) -> f32 {
        return self.r;
    }

    /// Retrieves the green component of the color
    pub fn get_green(&self) -> f32 {
        return self.g;
    }

    /// Retrieves the blue component of the color
    pub fn get_blue(&self) -> f32 {
        return self.b;
    }

    /// Retrieves the alpha component of the color
    pub fn get_alpha(&self) -> f32 {
        return self.a;
    }

    /// Retrieves all the color components in an array in the order: red, green,
    /// blue, alpha
    pub fn get(&self) -> [f32; 4] {
        return [self.r, self.g, self.b, self.a];
    }
}

impl Color for ColorRGBA {
    fn get_rgba(&self) -> ColorRGBA {
        return *self;
    }
}

/// A struct for defining a single color in HSLA space all values are between 0
/// and 1
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct ColorHSLA {
    /// The hue combonent
    h: f32,
    /// The saturation component
    s: f32,
    /// The lightness component
    l: f32,
    /// The alpha component
    a: f32,
}

impl ColorHSLA {
    /// Constructs a new hsla color, all values are clamped to between 0 and 1
    ///
    /// # Parameters
    ///
    /// h: The hue component
    ///
    /// s: The saturation component
    ///
    /// l: The lightness component
    ///
    /// a: The alpha component
    pub fn new(h: f32, s: f32, l: f32, a: f32) -> Self {
        return Self {
            h: h.rem_euclid(1.0),
            s: s.clamp(0.0, 1.0),
            l: l.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        };
    }

    /// Constructs a new hsla color with the alpha component equal to 1, all
    /// values are clamped to between 0 and 1
    ///
    /// # Parameters
    ///
    /// h: The hue component
    ///
    /// s: The saturation component
    ///
    /// l: The ligness component
    pub fn new_hsl(h: f32, s: f32, l: f32) -> Self {
        return Self::new(h, s, l, 1.0);
    }

    /// Constructs a new hsla color without validating the input
    ///
    /// # Parameters
    ///
    /// h: The hue component
    ///
    /// s: The saturation component
    ///
    /// l: The lightness component
    ///
    /// a: The alpha component
    pub unsafe fn new_unsafe(h: f32, s: f32, l: f32, a: f32) -> Self {
        return Self {
            h: h,
            s: s,
            l: l,
            a: a,
        };
    }

    /// Retrieves the hue component of the color
    pub fn get_hue(&self) -> f32 {
        return self.h;
    }

    /// Retrieves the saturation component of the color
    pub fn get_saturation(&self) -> f32 {
        return self.s;
    }

    /// Retrieves the lightness component of the color
    pub fn get_lightness(&self) -> f32 {
        return self.l;
    }

    /// Retrieves the alpha component of the color
    pub fn get_alpha(&self) -> f32 {
        return self.a;
    }

    /// Retrieves all the color components in an array in the order: hue,
    /// saturation, lightness, alpha
    pub fn get(&self) -> [f32; 4] {
        return [self.h, self.s, self.l, self.a];
    }
}

/// A struct for defining a single color in HSLV space all values are between 0
/// and 1
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct ColorHSVA {
    /// The hue combonent
    h: f32,
    /// The saturation component
    s: f32,
    /// The value component
    v: f32,
    /// The alpha component
    a: f32,
}

impl ColorHSVA {
    /// Constructs a new hsva color, all values are clamped to between 0 and 1
    ///
    /// # Parameters
    ///
    /// h: The hue component
    ///
    /// s: The saturation component
    ///
    /// v: The value component
    ///
    /// a: The alpha component
    pub fn new(h: f32, s: f32, v: f32, a: f32) -> Self {
        return Self {
            h: h.rem_euclid(1.0),
            s: s.clamp(0.0, 1.0),
            v: v.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        };
    }

    /// Constructs a new hsva color with the alpha component equal to 1, all
    /// values are clamped to between 0 and 1
    ///
    /// # Parameters
    ///
    /// h: The hue component
    ///
    /// s: The saturation component
    ///
    /// v: The value component
    pub fn new_hsv(h: f32, s: f32, v: f32) -> Self {
        return Self::new(h, s, v, 1.0);
    }

    /// Constructs a new hsva color without validating the input
    ///
    /// # Parameters
    ///
    /// h: The hue component
    ///
    /// s: The saturation component
    ///
    /// v: The value component
    ///
    /// a: The alpha component
    pub unsafe fn new_unsafe(h: f32, s: f32, v: f32, a: f32) -> Self {
        return Self {
            h: h,
            s: s,
            v: v,
            a: a,
        };
    }

    /// Retrieves the hue component of the color
    pub fn get_hue(&self) -> f32 {
        return self.h;
    }

    /// Retrieves the saturation component of the color
    pub fn get_saturation(&self) -> f32 {
        return self.s;
    }

    /// Retrieves the value component of the color
    pub fn get_value(&self) -> f32 {
        return self.v;
    }

    /// Retrieves the alpha component of the color
    pub fn get_alpha(&self) -> f32 {
        return self.a;
    }

    /// Retrieves all the color components in an array in the order: hue,
    /// saturation, value, alpha
    pub fn get(&self) -> [f32; 4] {
        return [self.h, self.s, self.v, self.a];
    }
}

/// A struct for defining a single color in HSIA space all values are between 0
/// and 1
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct ColorHSIA {
    /// The hue combonent
    h: f32,
    /// The saturation component
    s: f32,
    /// The lightness component
    i: f32,
    /// The alpha component
    a: f32,
}

impl ColorHSIA {
    /// Constructs a new hsia color, all values are clamped to between 0 and 1
    ///
    /// # Parameters
    ///
    /// h: The hue component
    ///
    /// s: The saturation component
    ///
    /// i: The intensity component
    ///
    /// a: The alpha component
    pub fn new(h: f32, s: f32, i: f32, a: f32) -> Self {
        return Self {
            h: h.rem_euclid(1.0),
            s: s.clamp(0.0, 1.0),
            i: i.clamp(0.0, 1.0),
            a: a.clamp(0.0, 1.0),
        };
    }

    /// Constructs a new hsli color with the alpha component equal to 1, all
    /// values are clamped to between 0 and 1
    ///
    /// # Parameters
    ///
    /// h: The hue component
    ///
    /// s: The saturation component
    ///
    /// i: The intensity component
    pub fn new_hsi(h: f32, s: f32, i: f32) -> Self {
        return Self::new(h, s, i, 1.0);
    }

    /// Constructs a new hsia color without validating the input
    ///
    /// # Parameters
    ///
    /// h: The hue component
    ///
    /// s: The saturation component
    ///
    /// i: The intensity component
    ///
    /// a: The alpha component
    pub unsafe fn new_unsafe(h: f32, s: f32, i: f32, a: f32) -> Self {
        return Self {
            h: h,
            s: s,
            i: i,
            a: a,
        };
    }

    /// Retrieves the hue component of the color
    pub fn get_hue(&self) -> f32 {
        return self.h;
    }

    /// Retrieves the saturation component of the color
    pub fn get_saturation(&self) -> f32 {
        return self.s;
    }

    /// Retrieves the intensity component of the color
    pub fn get_intensity(&self) -> f32 {
        return self.i;
    }

    /// Retrieves the alpha component of the color
    pub fn get_alpha(&self) -> f32 {
        return self.a;
    }

    /// Retrieves all the color components in an array in the order: hue,
    /// saturation, intensity, alpha
    pub fn get(&self) -> [f32; 4] {
        return [self.h, self.s, self.i, self.a];
    }
}

/// A generic N-dimensional color, all components are clamped between 0 and 1
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct ColorND<const N: usize> {
    values: [f32; N],
}

impl<const N: usize> ColorND<N> {
    /// Constructs a new N-dimensional color
    ///
    /// # Parameters
    ///
    /// values: All color components
    pub fn new(values: &[f32; N]) -> Self {
        let values: [f32; N] = values
            .iter()
            .map(|value| {
                return value.clamp(0.0, 1.0);
            })
            .collect::<Vec<f32>>()
            .try_into()
            .expect("Will never fail");

        return Self { values };
    }

    /// Retrieves all the color components
    pub fn get(&self) -> &[f32; N] {
        return &self.values;
    }
}

/// Defines a single color which can be expressed in RGBA
pub trait Color {
    /// Retrieves the rgba color for this color
    fn get_rgba(&self) -> ColorRGBA;
}

/// Defines a color map which can convert a N-dimensional color into a normal
/// color
pub trait ColorMap<const N: usize> {
    /// Retrieves the normal color from the N-dimensional color
    ///
    /// # Parameters
    ///
    /// color: The N-dimensional color to convert
    fn get_color(&self, color: ColorND<N>) -> impl Color;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the ColorRGBA struct
    mod color_rgba {
        use super::*;

        /// Test new method
        #[test]
        fn new() {
            let result_valid = ColorRGBA::new(0.1, 0.2, 0.3, 0.4);
            let result_low_r = ColorRGBA::new(-0.1, 0.2, 0.3, 0.4);
            let result_low_g = ColorRGBA::new(0.1, -0.2, 0.3, 0.4);
            let result_low_b = ColorRGBA::new(0.1, 0.2, -0.3, 0.4);
            let result_low_a = ColorRGBA::new(0.1, 0.2, 0.3, -0.4);
            let result_hig_r = ColorRGBA::new(1.1, 0.2, 0.3, 0.4);
            let result_hig_g = ColorRGBA::new(0.1, 1.2, 0.3, 0.4);
            let result_hig_b = ColorRGBA::new(0.1, 0.2, 1.3, 0.4);
            let result_hig_a = ColorRGBA::new(0.1, 0.2, 0.3, 1.4);

            assert_eq!(
                result_valid,
                ColorRGBA {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 0.4,
                }
            );
            assert_eq!(
                result_low_r,
                ColorRGBA {
                    r: 0.0,
                    g: 0.2,
                    b: 0.3,
                    a: 0.4,
                }
            );
            assert_eq!(
                result_low_g,
                ColorRGBA {
                    r: 0.1,
                    g: 0.0,
                    b: 0.3,
                    a: 0.4,
                }
            );
            assert_eq!(
                result_low_b,
                ColorRGBA {
                    r: 0.1,
                    g: 0.2,
                    b: 0.0,
                    a: 0.4,
                }
            );
            assert_eq!(
                result_low_a,
                ColorRGBA {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 0.0,
                }
            );
            assert_eq!(
                result_hig_r,
                ColorRGBA {
                    r: 1.0,
                    g: 0.2,
                    b: 0.3,
                    a: 0.4,
                }
            );
            assert_eq!(
                result_hig_g,
                ColorRGBA {
                    r: 0.1,
                    g: 1.0,
                    b: 0.3,
                    a: 0.4,
                }
            );
            assert_eq!(
                result_hig_b,
                ColorRGBA {
                    r: 0.1,
                    g: 0.2,
                    b: 1.0,
                    a: 0.4,
                }
            );
            assert_eq!(
                result_hig_a,
                ColorRGBA {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }
            );
        }

        /// Test new_rgb method
        #[test]
        fn new_rgb() {
            let result_valid = ColorRGBA::new_rgb(0.1, 0.2, 0.3);
            let result_low_r = ColorRGBA::new_rgb(-0.1, 0.2, 0.3);
            let result_low_g = ColorRGBA::new_rgb(0.1, -0.2, 0.3);
            let result_low_b = ColorRGBA::new_rgb(0.1, 0.2, -0.3);
            let result_hig_r = ColorRGBA::new_rgb(1.1, 0.2, 0.3);
            let result_hig_g = ColorRGBA::new_rgb(0.1, 1.2, 0.3);
            let result_hig_b = ColorRGBA::new_rgb(0.1, 0.2, 1.3);

            assert_eq!(
                result_valid,
                ColorRGBA {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }
            );
            assert_eq!(
                result_low_r,
                ColorRGBA {
                    r: 0.0,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }
            );
            assert_eq!(
                result_low_g,
                ColorRGBA {
                    r: 0.1,
                    g: 0.0,
                    b: 0.3,
                    a: 1.0,
                }
            );
            assert_eq!(
                result_low_b,
                ColorRGBA {
                    r: 0.1,
                    g: 0.2,
                    b: 0.0,
                    a: 1.0,
                }
            );
            assert_eq!(
                result_hig_r,
                ColorRGBA {
                    r: 1.0,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }
            );
            assert_eq!(
                result_hig_g,
                ColorRGBA {
                    r: 0.1,
                    g: 1.0,
                    b: 0.3,
                    a: 1.0,
                }
            );
            assert_eq!(
                result_hig_b,
                ColorRGBA {
                    r: 0.1,
                    g: 0.2,
                    b: 1.0,
                    a: 1.0,
                }
            );
        }

        /// Test get_red method
        #[test]
        fn get_red() {
            let value = ColorRGBA {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 0.4,
            };

            assert_eq!(value.get_red(), 0.1);
        }

        /// Test get_green method
        #[test]
        fn get_green() {
            let value = ColorRGBA {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 0.4,
            };

            assert_eq!(value.get_green(), 0.2);
        }

        /// Test get_blue method
        #[test]
        fn get_blue() {
            let value = ColorRGBA {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 0.4,
            };

            assert_eq!(value.get_blue(), 0.3);
        }

        /// Test get_alpha method
        #[test]
        fn get_alpha() {
            let value = ColorRGBA {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 0.4,
            };

            assert_eq!(value.get_alpha(), 0.4);
        }

        /// Test get method
        #[test]
        fn get() {
            let value = ColorRGBA {
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 0.4,
            };

            assert_eq!(value.get(), [0.1, 0.2, 0.3, 0.4]);
        }
    }

    /// Test the ColorND struct
    mod color_nd {
        use super::*;

        /// Test new method
        #[test]
        fn new() {
            let result_1_low = ColorND::new(&[-0.2]);
            let result_1_mid = ColorND::new(&[0.2]);
            let result_1_hig = ColorND::new(&[1.2]);
            let result_3_low = ColorND::new(&[-0.2, 0.3, 0.4]);
            let result_3_mid = ColorND::new(&[0.2, 0.3, 0.4]);
            let result_3_hig = ColorND::new(&[0.2, 0.3, 1.4]);

            assert_eq!(result_1_low, ColorND { values: [0.0] });
            assert_eq!(result_1_mid, ColorND { values: [0.2] });
            assert_eq!(result_1_hig, ColorND { values: [1.0] });
            assert_eq!(
                result_3_low,
                ColorND {
                    values: [0.0, 0.3, 0.4]
                }
            );
            assert_eq!(
                result_3_mid,
                ColorND {
                    values: [0.2, 0.3, 0.4]
                }
            );
            assert_eq!(
                result_3_hig,
                ColorND {
                    values: [0.2, 0.3, 1.0]
                }
            );
        }
    }
}
