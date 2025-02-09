use crate::{ColorHSIA, ColorHSLA, ColorHSVA, ColorRGBA};

/// Clamps the value between min and max, if value < min then return min, if
/// value > max then return max, otherwise return value
///
/// # Parameters
///
/// value: The value to clamp
///
/// min: The minimum allowed value
///
/// max: The maximum allowed value
pub(crate) fn clamp<T>(value: T, min: T, max: T) -> T
where
    T: PartialOrd,
{
    if cfg!(debug_assertions) && min > max {
        panic!("min must not be larger than max");
    }

    if value < min {
        return min;
    } else if value > max {
        return max;
    } else {
        return value;
    }
}

pub fn rgb_to_hsl(color: &ColorRGBA) -> ColorHSLA {
    let v = color.get_red().max(color.get_green().max(color.get_blue()));
    let c = v - color.get_red().min(color.get_green().min(color.get_blue()));
    let l = v - 0.5 * c;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test the clamp function
    mod f_clamp {
        use super::*;

        /// Test non clamped input
        #[test]
        fn valid() {
            let result_low = clamp(1, 1, 5);
            let result_mid = clamp(2, 1, 5);
            let result_hig = clamp(5, 1, 5);

            assert_eq!(result_low, 1);
            assert_eq!(result_mid, 2);
            assert_eq!(result_hig, 5);
        }

        /// Test low input
        #[test]
        fn low() {
            let result = clamp(0, 1, 5);

            assert_eq!(result, 1);
        }

        /// Test high input
        #[test]
        fn high() {
            let result = clamp(7, 1, 5);

            assert_eq!(result, 5);
        }

        /// Test with min = max
        #[test]
        fn no_width() {
            let result_low = clamp(0, 1, 1);
            let result_mid = clamp(1, 1, 1);
            let result_hig = clamp(2, 1, 1);

            assert_eq!(result_low, 1);
            assert_eq!(result_mid, 1);
            assert_eq!(result_hig, 1);
        }

        /// Test invalid input
        #[test]
        #[should_panic]
        fn invalid() {
            let _result = clamp(7, 5, 1);
        }
    }
}
