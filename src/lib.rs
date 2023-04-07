//! Conversion between and use of various color spaces
//!
//! Currently only supports conversion between HSV & RGB; Luma & RGB; HSL & RGB

pub mod color_spaces;
mod tests;

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

#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Hash, Ord)]
/// The CMYK pixel
///
/// A container for the cyan, magenta, yellow, and black levels of a single pixel
///
/// # Examples
///
/// Here is how to create a single pixel
///
/// ```
/// use altered_perception::CMYK;
///
/// let pixel = CMYK::new(0.5, 0.5, 0.3, 0.7);
/// ```
pub struct CMYK<T> {
    /// Cyan (between 0 and 1)
    pub c: T,
    /// Magenta (between 0 and 1)
    pub m: T,
    /// Yellow (between 0 and 1)
    pub y: T,
    /// Black (between 0 and 1)
    pub k: T,
}

#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Eq, Hash, Ord)]
/// The CMY pixel
///
/// A container for the cyan, magenta, and yellow levels of a single pixel
///
/// # Examples
///
/// Here is how to create a single pixel
///
/// ```
/// use altered_perception::CMY;
///
/// let pixel = CMY::new(0.5, 0.5, 0.3);
/// ```
pub struct CMY<T> {
    /// Cyan (between 0 and 1)
    pub c: T,
    /// Magenta (between 0 and 1)
    pub m: T,
    /// Yellow (between 0 and 1)
    pub y: T,
}