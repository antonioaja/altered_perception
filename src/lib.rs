//! Conversion between and use of various color spaces
//! 
//! Currently only supports conversion between HSV & RGB, and Luma & RGB

pub mod hsv;
pub mod luma;

#[cfg(test)]
mod tests {
    use crate::hsv::HSV;
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
