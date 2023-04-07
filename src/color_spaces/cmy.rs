use crate::CMY;
use num_traits::cast::AsPrimitive;
use num_traits::Bounded;
use rgb::RGB;

impl<N: std::convert::Into<f64> + Copy + 'static> CMY<N> {
    /// Converts an rgb pixel into an cmy pixel
    ///
    /// ```
    /// use altered_perception::CMY;
    /// use rgb::RGB;
    ///
    /// let rgb_pixel = RGB::new(1,2,3);
    /// let cmy_pixel: CMY<f64> = CMY::from_rgb::<u8>(rgb_pixel);
    /// ```
    pub fn from_rgb<T: std::convert::Into<f64> + Bounded>(rgb_pixel: RGB<T>) -> CMY<N>
    where
        f64: AsPrimitive<N>,
    {
        CMY {
            c: (1.0 - (rgb_pixel.r.into() / T::max_value().into())).as_(),
            m: (1.0 - (rgb_pixel.g.into() / T::max_value().into())).as_(),
            y: (1.0 - (rgb_pixel.b.into() / T::max_value().into())).as_(),
        }
    }

    /// Creates a single pixel of type N
    pub fn new(c: N, m: N, y: N) -> CMY<N> {
        CMY { c, m, y }
    }

    /// Converts an cmy pixel to an rbg pixel
    ///
    /// ```
    /// use altered_perception::CMY;
    /// use rgb::RGB;
    ///
    /// let cmy_pixel = CMY::new(0.5, 0.5, 0.3);
    /// let rgb_pixel = CMY::to_rgb::<u8>(cmy_pixel);
    /// ```
    pub fn to_rgb<T: std::marker::Copy + 'static + Bounded + std::convert::Into<f64>>(
        cmy_pixel: CMY<N>,
    ) -> RGB<T>
    where
        f64: AsPrimitive<T>,
    {
        RGB {
            r: (T::max_value().into() * (1.0 - cmy_pixel.c.into())).as_(),
            g: (T::max_value().into() * (1.0 - cmy_pixel.m.into())).as_(),
            b: (T::max_value().into() * (1.0 - cmy_pixel.y.into())).as_(),
        }
    }
}
