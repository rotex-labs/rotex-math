#![allow(clippy::excessive_precision)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

//! Color types and conversions between sRGB, linear light, HSV, Oklch, CMYK, and CIE XYZ.
//!
//! [`LinearRgba`] is the working space. The other types convert to and from it
//! through [`From`]. Named colors are the CSS basic and extended names, stored
//! in each type's own encoding.
//!
//! # Examples
//!
//! ```
//! use rotex_color::Srgba;
//!
//! let red = Srgba::from_hex(0xFF0000);
//! assert_eq!(red.to_hex(), 0xFF0000);
//! ```

/// CMYK ink amounts.
pub mod cmyka;
/// Hue, saturation, and value.
pub mod hsva;
/// Linear-light RGBA.
pub mod linear_rgba;
/// Oklch (Oklab lightness, chroma, and hue).
pub mod oklcha;
/// sRGB with a separate alpha channel.
pub mod srgba;
/// CIE XYZ under D65.
pub mod xyza;

/// [`Cmyka`] at the crate root.
pub use cmyka::Cmyka;
/// [`Hsva`] at the crate root.
pub use hsva::Hsva;
/// [`LinearRgba`] at the crate root.
pub use linear_rgba::LinearRgba;
/// [`Oklcha`] at the crate root.
pub use oklcha::Oklcha;
/// [`Srgba`] at the crate root.
pub use srgba::Srgba;
/// [`Xyza`] at the crate root.
pub use xyza::Xyza;
