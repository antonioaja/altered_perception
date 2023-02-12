//! Conversion between and use of various color spaces
//!
//! Currently only supports conversion between HSV & RGB; Luma & RGB

pub mod hsv;
pub mod luma;

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
    use crate::HSV;
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
}
