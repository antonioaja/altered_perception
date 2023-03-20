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
    fn luma8_to_rgb8() {
        (0..=u8::MAX).into_par_iter().for_each(|r| {
            let original = Luma::new(r);
            let intermediate: RGB<u8> = Luma::to_rgb::<u8>(original);
            let final_out = Luma::from_rgb(intermediate);

            assert_eq!(original, final_out);
        });
    }
}