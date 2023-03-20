use crate::CMYK;
use num_traits::cast::AsPrimitive;
use num_traits::Bounded;
use rgb::RGB;

impl<N: std::convert::Into<f64> + Copy + 'static> CMYK<N> {
    /// Converts an rgb pixel into an hsv pixel
    ///
    /// ```
    /// use altered_perception::CMYK;
    /// use rgb::RGB;
    ///
    /// let rgb_pixel = RGB::new(1,2,3);
    /// let hsv_pixel: CMYK<f64> = CMYK::from_rgb::<u8>(rgb_pixel);
    /// ```
    pub fn from_rgb<T: std::convert::Into<f64> + Bounded>(rgb_pixel: RGB<T>) -> CMYK<N>
    where
        f64: AsPrimitive<N>,
    {
        let r_prime: f64 = rgb_pixel.r.into() / T::max_value().into();
        let g_prime: f64 = rgb_pixel.g.into() / T::max_value().into();
        let b_prime: f64 = rgb_pixel.b.into() / T::max_value().into();

        let k: f64 = 1.0 - r_prime.max(g_prime.max(b_prime));

        CMYK {
            c: ((1.0 - r_prime - k) / (1.0 - k)).as_(),
            m: ((1.0 - g_prime - k) / (1.0 - k)).as_(),
            y: ((1.0 - b_prime - k) / (1.0 - k)).as_(),
            k: k.as_(),
        }
    }

    /// Creates a single pixel of type N
    pub fn new(c: N, m: N, y: N, k: N) -> CMYK<N> {
        CMYK { c, m, y, k }
    }

    /// Converts an hsv pixel to an rbg pixel
    ///
    /// ```
    /// use altered_perception::CMYK;
    /// use rgb::RGB;
    ///
    /// let cmyk_pixel = CMYK::new(0.5, 0.5, 0.3, 0.7);
    /// let rgb_pixel = CMYK::to_rgb::<u8>(cmyk_pixel);
    /// ```
    pub fn to_rgb<T: std::marker::Copy + 'static + Bounded + std::convert::Into<f64>>(
        cmyk_pixel: CMYK<N>,
    ) -> RGB<T>
    where
        f64: AsPrimitive<T>,
    {
        RGB {
            r: (T::max_value().into() * (1.0 - cmyk_pixel.c.into()) * (1.0 - cmyk_pixel.k.into()))
                .as_(),
            g: (T::max_value().into() * (1.0 - cmyk_pixel.m.into()) * (1.0 - cmyk_pixel.k.into()))
                .as_(),
            b: (T::max_value().into() * (1.0 - cmyk_pixel.y.into()) * (1.0 - cmyk_pixel.k.into()))
                .as_(),
        }
    }
}
