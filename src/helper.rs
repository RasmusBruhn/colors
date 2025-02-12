use crate::{ColorHSIA, ColorHSLA, ColorHSVA, ColorRGBA};

/// A color in the hue, croma, minimum, alpha space, used to convert between HSx and RGB colors
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct ColorHCMA {
    /// The hue component of the color
    h: f32,
    /// The croma component of the color
    c: f32,
    /// The minimum value of the color
    m: f32,
    /// The alpha component of the color
    a: f32,
}

impl ColorHCMA {
    /// Converts from RGB
    fn from_rgb(color: &ColorRGBA) -> Self {
        // Get colors
        let colors = [color.get_red(), color.get_green(), color.get_blue()];

        // Get the location for the smallest and largest component
        let (i_min, i_max) = if colors[0] < colors[1] && colors[0] < colors[2] {
            let i_max = if colors[1] < colors[2] { 2 } else { 1 };

            (0, i_max)
        } else if colors[1] < colors[2] {
            let i_max = if colors[0] < colors[2] { 2 } else { 0 };

            (1, i_max)
        } else {
            let i_max = if colors[0] < colors[1] { 1 } else { 0 };

            (2, i_max)
        };

        // Get (hue' // 2)
        let hue_major = (i_min + 1) % 3;

        // Get ((hue' % 2) // 1)
        let hue_minor = ((i_max + 3) - i_min) % 3 - 1;

        // Get the minimum value, croma and x = c * (1 - (hue' % 2 - 1).abs())
        let m = colors[i_min];
        let c = colors[i_max] - m;
        let x = colors[(hue_major + (hue_minor + 1) % 2) % 3] - m;

        // Stop if it is grayscale
        if c == 0.0 {
            return Self {
                h: 0.0,
                c: 0.0,
                m: m,
                a: color.get_alpha(),
            };
        }

        // Get the hue
        let hp = 2.0 * (hue_major as f32) + if hue_minor == 0 { x / c } else { 2.0 - x / c };

        return Self {
            h: hp / 6.0,
            c: c,
            m: m,
            a: color.get_alpha(),
        };
    }

    /// Converts from HSV
    fn from_hsv(color: &ColorHSVA) -> Self {
        let c = color.get_value() * color.get_saturation();
        let m = color.get_value() - c;

        return Self {
            h: color.get_hue(),
            c,
            m,
            a: color.get_alpha(),
        };
    }

    /// Converts from HSL
    fn from_hsl(color: &ColorHSLA) -> Self {
        let c = (1.0 - (2.0 * color.get_lightness() - 1.0).abs()) * color.get_saturation();
        let m = color.get_lightness() - 0.5 * c;

        return Self {
            h: color.get_hue(),
            c,
            m,
            a: color.get_alpha(),
        };
    }

    /// Converts from HSI
    fn from_hsi(color: &ColorHSIA) -> Self {
        let z = 1.0 - ((6.0 * color.get_hue()).rem_euclid(2.0) - 1.0).abs();
        let c = 3.0 * color.get_intensity() * color.get_saturation() / (1.0 + z);
        let m = color.get_intensity() * (1.0 - color.get_saturation());

        return Self {
            h: color.get_hue(),
            c,
            m,
            a: color.get_alpha(),
        };
    }

    /// Converts to RGB
    fn to_rgb(&self) -> ColorRGBA {
        // Calculate temporary parameters for use in the calculations
        let hp = self.h * 6.0;
        let z = 1.0 - (hp.rem_euclid(2.0) - 1.0).abs();
        let x = self.c * z;

        // Finds the order of the colors
        let colors = if hp.rem_euclid(2.0) < 1.0 {
            [self.c, x, 0.0]
        } else {
            [x, self.c, 0.0]
        };

        // Finds the negative index of the red component
        let i = hp.div_euclid(2.0) as usize;

        // Calculate the final colors
        return unsafe {
            ColorRGBA::new_unsafe(
                colors[(3 - i) % 3] + self.m,
                colors[(4 - i) % 3] + self.m,
                colors[(5 - i) % 3] + self.m,
                self.a,
            )
        };
    }

    /// Converts to HSV
    fn to_hsv(&self) -> ColorHSVA {
        let v = self.m + self.c;
        let s = if v == 0.0 { 0.0 } else { self.c / v };

        return unsafe { ColorHSVA::new_unsafe(self.h, s, v, self.a) };
    }

    /// Converts to HSL
    fn to_hsl(&self) -> ColorHSLA {
        let l = self.m + 0.5 * self.c;
        let z = 1.0 - (2.0 * l - 1.0).abs();
        let s = if z == 0.0 { 0.0 } else { self.c / z };

        return unsafe { ColorHSLA::new_unsafe(self.h, s, l, self.a) };
    }

    /// Converts to HSI
    fn to_hsi(&self) -> ColorHSIA {
        let z = 1.0 - ((6.0 * self.h).rem_euclid(2.0) - 1.0).abs();
        let i = self.m + self.c * (1.0 + z) / 3.0;
        let s = if i == 0.0 { 0.0 } else { 1.0 - self.m / i };

        return unsafe { ColorHSIA::new_unsafe(self.h, s, i, self.a) };
    }
}

pub fn hsl_to_rgb(color: &ColorHSLA) -> ColorRGBA {
    let f = |n: f32| -> f32 {
        let k = (n + color.get_hue() * 12.0).rem_euclid(12.0);
        let a = color.get_saturation() * color.get_lightness().min(1.0 - color.get_lightness());
        return color.get_lightness() - a * (k - 3.0).min(9.0 - k).clamp(-1.0, 1.0);
    };

    return unsafe { ColorRGBA::new_unsafe(f(0.0), f(4.0), f(8.0), color.get_alpha()) };
}

//pub fn rgb_to_hsl(color: &ColorRGBA) -> ColorHSLA {
//    let v = color.get_red().max(color.get_green().max(color.get_blue()));
//    let c = v - color.get_red().min(color.get_green().min(color.get_blue()));
//    let l = v - 0.5 * c;
//}

#[cfg(test)]
mod tests {
    use super::*;

    /// Retrieves all test colors
    fn get_test_values() -> [(ColorHCMA, ColorRGBA, ColorHSVA, ColorHSLA, ColorHSIA); 10] {
        return [
            (
                ColorHCMA {
                    h: 0.0,
                    c: 0.0,
                    m: 1.0,
                    a: 1.0,
                },
                ColorRGBA::new_rgb(1.0, 1.0, 1.0),
                ColorHSVA::new_hsv(0.0, 0.0, 1.0),
                ColorHSLA::new_hsl(0.0, 0.0, 1.0),
                ColorHSIA::new_hsi(0.0, 0.0, 1.0),
            ),
            (
                ColorHCMA {
                    h: 0.0,
                    c: 0.0,
                    m: 0.5,
                    a: 1.0,
                },
                ColorRGBA::new_rgb(0.5, 0.5, 0.5),
                ColorHSVA::new_hsv(0.0, 0.0, 0.5),
                ColorHSLA::new_hsl(0.0, 0.0, 0.5),
                ColorHSIA::new_hsi(0.0, 0.0, 0.5),
            ),
            (
                ColorHCMA {
                    h: 0.0,
                    c: 0.0,
                    m: 0.0,
                    a: 1.0,
                },
                ColorRGBA::new_rgb(0.0, 0.0, 0.0),
                ColorHSVA::new_hsv(0.0, 0.0, 0.0),
                ColorHSLA::new_hsl(0.0, 0.0, 0.0),
                ColorHSIA::new_hsi(0.0, 0.0, 0.0),
            ),
            (
                ColorHCMA {
                    h: 0.0,
                    c: 1.0,
                    m: 0.0,
                    a: 1.0,
                },
                ColorRGBA::new_rgb(1.0, 0.0, 0.0),
                ColorHSVA::new_hsv(0.0, 1.0, 1.0),
                ColorHSLA::new_hsl(0.0, 1.0, 0.5),
                ColorHSIA::new_hsi(0.0, 1.0, 0.3333),
            ),
            (
                ColorHCMA {
                    h: 60.0 / 360.0,
                    c: 0.75,
                    m: 0.0,
                    a: 1.0,
                },
                ColorRGBA::new_rgb(0.75, 0.75, 0.0),
                ColorHSVA::new_hsv(60.0 / 360.0, 1.0, 0.75),
                ColorHSLA::new_hsl(60.0 / 360.0, 1.0, 0.375),
                ColorHSIA::new_hsi(60.0 / 360.0, 1.0, 0.5),
            ),
            (
                ColorHCMA {
                    h: 120.0 / 360.0,
                    c: 0.5,
                    m: 0.0,
                    a: 1.0,
                },
                ColorRGBA::new_rgb(0.0, 0.5, 0.0),
                ColorHSVA::new_hsv(120.0 / 360.0, 1.0, 0.5),
                ColorHSLA::new_hsl(120.0 / 360.0, 1.0, 0.25),
                ColorHSIA::new_hsi(120.0 / 360.0, 1.0, 0.1667),
            ),
            (
                ColorHCMA {
                    h: 180.0 / 360.0,
                    c: 0.5,
                    m: 0.5,
                    a: 1.0,
                },
                ColorRGBA::new_rgb(0.5, 1.0, 1.0),
                ColorHSVA::new_hsv(180.0 / 360.0, 0.5, 1.0),
                ColorHSLA::new_hsl(180.0 / 360.0, 1.0, 0.75),
                ColorHSIA::new_hsi(180.0 / 360.0, 0.4, 0.833),
            ),
            (
                ColorHCMA {
                    h: 240.0 / 360.0,
                    c: 0.5,
                    m: 0.5,
                    a: 1.0,
                },
                ColorRGBA::new_rgb(0.5, 0.5, 1.0),
                ColorHSVA::new_hsv(240.0 / 360.0, 0.5, 1.0),
                ColorHSLA::new_hsl(240.0 / 360.0, 1.0, 0.75),
                ColorHSIA::new_hsi(240.0 / 360.0, 0.25, 0.667),
            ),
            (
                ColorHCMA {
                    h: 300.0 / 360.0,
                    c: 0.5,
                    m: 0.25,
                    a: 1.0,
                },
                ColorRGBA::new_rgb(0.75, 0.25, 0.75),
                ColorHSVA::new_hsv(300.0 / 360.0, 0.667, 0.75),
                ColorHSLA::new_hsl(300.0 / 360.0, 0.5, 0.5),
                ColorHSIA::new_hsi(300.0 / 360.0, 0.571, 0.5834),
            ),
            (
                ColorHCMA {
                    h: 61.8 / 360.0,
                    c: 0.501,
                    m: 0.142,
                    a: 1.0,
                },
                ColorRGBA::new_rgb(0.628, 0.643, 0.142),
                ColorHSVA::new_hsv(61.8 / 360.0, 0.779, 0.643),
                ColorHSLA::new_hsl(61.8 / 360.0, 0.638, 0.3924),
                ColorHSIA::new_hsi(61.8 / 360.0, 0.699, 0.471),
            ),
        ];
    }

    /// Rounds the HCM color for comparisons
    fn round_hcm(color: &ColorHCMA) -> [i32; 4] {
        return [
            (color.h * 1000.0).round() as i32,
            (color.c * 1000.0).round() as i32,
            (color.m * 1000.0).round() as i32,
            (color.a * 1000.0).round() as i32,
        ];
    }

    /// Rounds the RGB color for comparisons
    fn round_rgb(color: &ColorRGBA) -> [i32; 4] {
        return [
            (color.get_red() * 1000.0).round() as i32,
            (color.get_green() * 1000.0).round() as i32,
            (color.get_blue() * 1000.0).round() as i32,
            (color.get_alpha() * 1000.0).round() as i32,
        ];
    }

    /// Rounds the HSV color for comparisons
    fn round_hsv(color: &ColorHSVA) -> [i32; 4] {
        return [
            (color.get_hue() * 1000.0).round() as i32,
            (color.get_saturation() * 1000.0).round() as i32,
            (color.get_value() * 1000.0).round() as i32,
            (color.get_alpha() * 1000.0).round() as i32,
        ];
    }

    /// Rounds the HSL color for comparisons
    fn round_hsl(color: &ColorHSLA) -> [i32; 4] {
        return [
            (color.get_hue() * 1000.0).round() as i32,
            (color.get_saturation() * 1000.0).round() as i32,
            (color.get_lightness() * 1000.0).round() as i32,
            (color.get_alpha() * 1000.0).round() as i32,
        ];
    }

    /// Rounds the HSI color for comparisons
    fn round_hsi(color: &ColorHSIA) -> [i32; 4] {
        return [
            (color.get_hue() * 1000.0).round() as i32,
            (color.get_saturation() * 1000.0).round() as i32,
            (color.get_intensity() * 1000.0).round() as i32,
            (color.get_alpha() * 1000.0).round() as i32,
        ];
    }

    mod conversion {
        use super::*;

        #[test]
        fn from_rgb() {
            let test_values = get_test_values();

            for values in test_values.iter() {
                let rgb = &values.1;
                let hcm = ColorHCMA::from_rgb(rgb);

                assert_eq!(round_hcm(&values.0), round_hcm(&hcm));
            }
        }

        #[test]
        fn from_hsv() {
            let test_values = get_test_values();

            for values in test_values.iter() {
                let hsv = &values.2;
                let hcm = ColorHCMA::from_hsv(hsv);

                assert_eq!(round_hcm(&values.0), round_hcm(&hcm));
            }
        }

        #[test]
        fn from_hsl() {
            let test_values = get_test_values();

            for values in test_values.iter() {
                let hsl = &values.3;
                let hcm = ColorHCMA::from_hsl(hsl);

                assert_eq!(round_hcm(&values.0), round_hcm(&hcm));
            }
        }

        #[test]
        fn from_hsi() {
            let test_values = get_test_values();

            for values in test_values.iter() {
                let hsi = &values.4;
                let hcm = ColorHCMA::from_hsi(hsi);

                assert_eq!(round_hcm(&values.0), round_hcm(&hcm));
            }
        }

        #[test]
        fn to_rgb() {
            let test_values = get_test_values();

            for values in test_values.iter() {
                let hcm = &values.0;
                let rgb = hcm.to_rgb();

                assert_eq!(round_rgb(&values.1), round_rgb(&rgb));
            }
        }

        #[test]
        fn to_hsv() {
            let test_values = get_test_values();

            for values in test_values.iter() {
                let hcm = &values.0;
                let hsv = hcm.to_hsv();

                assert_eq!(round_hsv(&values.2), round_hsv(&hsv));
            }
        }

        #[test]
        fn to_hsl() {
            let test_values = get_test_values();

            for values in test_values.iter() {
                let hcm = &values.0;
                let hsl = hcm.to_hsl();

                assert_eq!(round_hsl(&values.3), round_hsl(&hsl));
            }
        }

        #[test]
        fn to_hsi() {
            let test_values = get_test_values();

            for values in test_values.iter() {
                let hcm = &values.0;
                let hsi = hcm.to_hsi();

                assert_eq!(round_hsi(&values.4), round_hsi(&hsi));
            }
        }
    }
}
