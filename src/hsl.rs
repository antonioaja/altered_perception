use crate::HSL;
use num_traits::cast::AsPrimitive;
use num_traits::Bounded;
use rgb::RGB;

impl<N: std::convert::Into<f64> + Copy + 'static> HSL<N> {
    /// Converts an rgb pixel into an hsl pixel
    ///
    /// ```
    /// use altered_perception::HSL;
    /// use rgb::RGB;
    ///
    /// let rgb_pixel = RGB::new(1,2,3);
    /// let hsl_pixel: HSL<f64> = HSL::from_rgb::<u8>(rgb_pixel);
    /// ```
    pub fn from_rgb<T: std::convert::Into<f64> + Bounded>(rgb_pixel: RGB<T>) -> HSL<N>
    where
        f64: AsPrimitive<N>,
    {
        let r_prime: f64 = rgb_pixel.r.into() / T::max_value().into();
        let g_prime: f64 = rgb_pixel.g.into() / T::max_value().into();
        let b_prime: f64 = rgb_pixel.b.into() / T::max_value().into();

        let c_max: f64 = r_prime.max(g_prime.max(b_prime));
        let c_min: f64 = r_prime.min(g_prime.min(b_prime));

        let delta: f64 = c_max - c_min;

        let mut hue: f64 = if delta == 0.0 {
            0.0
        } else if c_max == r_prime {
            60.0 * (((g_prime - b_prime) / delta) % 6.0)
        } else if c_max == g_prime {
            60.0 * (((b_prime - r_prime) / delta) + 2.0)
        } else if c_max == b_prime {
            60.0 * (((r_prime - g_prime) / delta) + 4.0)
        } else {
            0.0
        };

        // Accounts for negative degree wrap around
        if hue < 0.0 {
            hue += 360.0;
        }

        let lightness: f64 = (c_max + c_min) / 2.0;

        let saturation: f64 = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * lightness - 1.0).abs())
        };

        HSL {
            h: hue.as_(),
            s: saturation.as_(),
            l: lightness.as_(),
        }
    }

    /// Creates a single pixel of type N
    pub fn new(h: N, s: N, l: N) -> HSL<N> {
        HSL { h, s, l }
    }

    /// Converts an hsv pixel to an rbg pixel
    ///
    /// ```
    /// use altered_perception::HSL;
    /// use rgb::RGB;
    ///
    /// let hsl_pixel = HSL::new(90.0, 0.5, 0.3);
    /// let rgb_pixel = HSL::to_rgb::<u8>(hsl_pixel);
    /// ```
    pub fn to_rgb<T: std::marker::Copy + 'static + Bounded + std::convert::Into<f64>>(
        hsl_pixel: HSL<N>,
    ) -> RGB<T>
    where
        f64: AsPrimitive<T>,
    {
        let c: f64 = (1.0 - (2.0 * hsl_pixel.l.into() - 1.0).abs()) * hsl_pixel.s.into();
        let x: f64 = c * (1.0 - ((hsl_pixel.h.into() / 60.0) % 2.0 - 1.0).abs());
        let m: f64 = hsl_pixel.l.into() - (c / 2.0);

        let (r_prime, g_prime, b_prime) = match hsl_pixel.h.into() as u32 % 360 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            300..=359 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        // Account for overflow
        let r: f64 = if (r_prime + m) * T::max_value().into() > T::max_value().into() {
            0.0
        } else {
            (r_prime + m) * T::max_value().into()
        };
        let g: f64 = if (g_prime + m) * T::max_value().into() > T::max_value().into() {
            0.0
        } else {
            (g_prime + m) * T::max_value().into()
        };
        let b: f64 = if (b_prime + m) * T::max_value().into() > T::max_value().into() {
            0.0
        } else {
            (b_prime + m) * T::max_value().into()
        };

        RGB {
            r: r.round().as_(),
            g: g.round().as_(),
            b: b.round().as_(),
        }
    }
}
