//! Conversion between and use of various color spaces
//!
//! Currently only supports conversion between HSV & RGB; Luma & RGB; HSL & RGB

pub mod hsl;
pub mod hsv;
pub mod luma;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Hash, Ord)]
/// The HSL pixel
///
/// A container for the hue, saturation, and lightness of a certain pixel
///
/// # Examples
///
/// Here is how to create a single pixel
///
/// ```
/// use altered_perception::HSL;
///
/// let pixel = HSL::new(90.0, 0.5, 0.3);
/// ```
pub struct HSL<T> {
    /// Hue (in degrees)
    pub h: T,
    /// Saturation (between 0 and 1)
    pub s: T,
    /// Lightness (between 0 and 1)
    pub l: T,
}

#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Hash, Ord)]
/// The HSV pixel
///
/// A container for the hue, saturation, and value of a certain pixel
///
/// # Examples
///
/// Here is how to create a single pixel
///
/// ```
/// use altered_perception::HSV;
///
/// let pixel = HSV::new(90.0, 0.5, 0.3);
/// ```
pub struct HSV<T> {
    /// Hue (in degrees)
    pub h: T,
    /// Saturation (between 0 and 1)
    pub s: T,
    /// Value (between 0 and 1)
    pub v: T,
}

#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Hash, Ord)]
/// The Luma pixel
///
/// # Example
///
/// Here is how to create a single pixel
///
/// ```
/// use altered_perception::Luma;
///
/// let pixel = Luma::new(167);
/// ```
pub struct Luma<T> {
    pub luminance: T,
}

#[cfg(test)]
mod tests {
    use crate::{Luma, HSL, HSV};
    use rayon::prelude::*;
    use rgb::RGB;

    #[test]
    fn rgb8_to_hsv_f64() {
        (0..=u8::MAX).into_par_iter().for_each(|r| {
            (0..=u8::MAX).for_each(|g| {
                (0..=u8::MAX).for_each(|b| {
                    let original = RGB::new(r, g, b);
                    let intermediate: HSV<f64> = HSV::from_rgb::<u8>(original);
                    let final_out = HSV::<f64>::to_rgb(intermediate);

                    assert_eq!(original, final_out);
                });
            });
        });
    }

    #[test]
    fn rgb8_to_hsv_f32() {
        (0..=u8::MAX).into_par_iter().for_each(|r| {
            (0..=u8::MAX).for_each(|g| {
                (0..=u8::MAX).for_each(|b| {
                    let original = RGB::new(r, g, b);
                    let intermediate: HSV<f32> = HSV::from_rgb::<u8>(original);
                    let final_out = HSV::<f32>::to_rgb(intermediate);

                    assert_eq!(original, final_out);
                });
            });
        });
    }

    #[test]
    fn rgb8_to_hsl_f64() {
        (0..=u8::MAX).into_par_iter().for_each(|r| {
            (0..=u8::MAX).for_each(|g| {
                (0..=u8::MAX).for_each(|b| {
                    let original = RGB::new(r, g, b);
                    let intermediate: HSL<f64> = HSL::from_rgb::<u8>(original);
                    let final_out = HSL::<f64>::to_rgb(intermediate);

                    assert_eq!(original, final_out);
                });
            });
        });
    }

    #[test]
    fn rgb8_to_hsl_f32() {
        (0..=u8::MAX).into_par_iter().for_each(|r| {
            (0..=u8::MAX).for_each(|g| {
                (0..=u8::MAX).for_each(|b| {
                    let original = RGB::new(r, g, b);
                    let intermediate: HSL<f32> = HSL::from_rgb::<u8>(original);
                    let final_out = HSL::<f32>::to_rgb(intermediate);

                    assert_eq!(original, final_out);
                });
            });
        });
    }

    #[test]
    fn luma8_to_rgb8_to_luma8() {
        (0..=u8::MAX).into_par_iter().for_each(|r| {
            let original = Luma::new(r);
            let intermediate: RGB<u8> = Luma::to_rgb::<u8>(original);
            let final_out = Luma::from_rgb(intermediate);

            assert_eq!(original, final_out);
        });
    }
}
