# altered_perception

A rust library for converting and using various color spaces. It currently supports HSV, HSL, CMYK, CMY, and Luma pixels with conversion to and from RGB pixels.

This library is inspired by the [rgb crate](https://github.com/kornelski/rust-rgb).

## Changes

Various changes will be made since this is my first library. All methods/structs will stabilize by the v1.0.0 release, and the crate's structure is still being decided. 

This should stabilize by May 2023.

## Usage

Conversion between and use of various color spaces can be done using the following modules:

* `cmyk`: Cyan-Magenta-Yellow-Black color space
* `cmy`: Cyan-Magenta-Yellow color space
* `hsl`: Hue-Saturation-Lightness color space
* `hsv`: Hue-Saturation-Value color space
* `luma`: Grayscale color space

Currently, only conversion between HSV & RGB, Luma & RGB, CMYK & RGB, CMY & RGB, and HSL & RGB is supported.

## Additional Color Spaces

Support for more color spaces will be added as time goes on. Contributions are welcome.

## MIT License

This create is licensed under an MIT license.