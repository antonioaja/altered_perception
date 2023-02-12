use crate::Luma;
use num_traits::cast::AsPrimitive;
use rgb::RGB;

impl<T> Luma<T> {
    /// Converts an RGB pixel into a Luma pixel
    ///
    /// ```
    /// use altered_perception::Luma;
    /// use rgb::RGB;
    ///
    /// let rgb_pixel = RGB::new(1,2,3);
    /// let luma_pixel: Luma<u8> = Luma::from_rgb::<u8>(rgb_pixel);
    /// ```
    pub fn from_rgb<N: std::convert::Into<f64>>(rgb_pixel: RGB<N>) -> Luma<T>
    where
        T: std::convert::Into<f64> + std::marker::Copy + 'static,
        f64: AsPrimitive<T>,
    {
        Luma {
            luminance: ((0.299 * (rgb_pixel.r.into()).powi(2)
                + 0.587 * (rgb_pixel.g.into()).powi(2)
                + 0.114 * (rgb_pixel.b.into()).powi(2))
            .sqrt()
            .round())
            .as_(),
        }
    }

    /// Creates a Luma pixel
    pub fn new(luminance: T) -> Luma<T> {
        Luma { luminance }
    }

    /// Converts a luma pixel into an RGB pixel
    ///
    /// ```
    /// use altered_perception::Luma;
    /// use rgb::RGB;
    ///
    /// let luma_pixel = Luma::new(155);
    /// let rgb_pixel = Luma::to_rgb::<u8>(luma_pixel);
    /// ```
    pub fn to_rgb<N: std::convert::From<T>>(luma_pixel: Luma<T>) -> RGB<N>
    where
        T: Copy,
    {
        RGB {
            r: luma_pixel.luminance.into(),
            g: luma_pixel.luminance.into(),
            b: luma_pixel.luminance.into(),
        }
    }
}
