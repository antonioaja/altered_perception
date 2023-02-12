use num_traits::cast::AsPrimitive;
use rgb::RGB;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub struct Luma<T> {
    pub luminance: T,
}
impl<T> Luma<T> {
    /// Converts an RGB pixel into a Luma pixel
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
