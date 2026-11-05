//! CMYK ink amounts with a separate alpha channel.
//!
//! Conversion to [`LinearRgba`] goes through sRGB.

use crate::{LinearRgba, Srgba};

/// CMYK ink amounts with a separate alpha channel.
///
/// Ink channels are typically in `0.0..=1.0`, where `1.0` is full ink.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cmyka {
    /// Cyan ink amount. `0.0` is none and `1.0` is full cyan.
    pub c: f32,
    /// Magenta ink amount. `0.0` is none and `1.0` is full magenta.
    pub m: f32,
    /// Yellow ink amount. `0.0` is none and `1.0` is full yellow.
    pub y: f32,
    /// Black ink amount. `0.0` is none and `1.0` is full black.
    pub k: f32,
    /// Alpha. `0.0` is transparent and `1.0` is opaque.
    pub a: f32,
}

impl Cmyka {
    /// Creates a CMYK color from ink amounts and alpha.
    pub fn new(c: f32, m: f32, y: f32, k: f32, a: f32) -> Self {
        Self { c, m, y, k, a }
    }

    /// The CMYK color `cyan`.
    pub const CYAN: Self = Self {
        c: 1.0,
        m: 0.0,
        y: 0.0,
        k: 0.0,
        a: 1.0,
    };
    /// The CMYK color `magenta`.
    pub const MAGENTA: Self = Self {
        c: 0.0,
        m: 1.0,
        y: 0.0,
        k: 0.0,
        a: 1.0,
    };
    /// The CMYK color `yellow`.
    pub const YELLOW: Self = Self {
        c: 0.0,
        m: 0.0,
        y: 1.0,
        k: 0.0,
        a: 1.0,
    };
    /// The CMYK color `black`.
    pub const BLACK: Self = Self {
        c: 0.0,
        m: 0.0,
        y: 0.0,
        k: 1.0,
        a: 1.0,
    };
    /// The CMYK color `white`.
    pub const WHITE: Self = Self {
        c: 0.0,
        m: 0.0,
        y: 0.0,
        k: 0.0,
        a: 1.0,
    };
}

impl Default for Cmyka {
    fn default() -> Self {
        Self {
            c: 0.0,
            m: 0.0,
            y: 0.0,
            k: 1.0,
            a: 0.0,
        }
    }
}

impl From<Cmyka> for LinearRgba {
    fn from(cmyk: Cmyka) -> Self {
        let r = (1.0 - cmyk.c) * (1.0 - cmyk.k);
        let g = (1.0 - cmyk.m) * (1.0 - cmyk.k);
        let b = (1.0 - cmyk.y) * (1.0 - cmyk.k);
        Srgba { r, g, b, a: cmyk.a }.into()
    }
}

impl From<LinearRgba> for Cmyka {
    fn from(linear: LinearRgba) -> Self {
        let srgb: Srgba = linear.into();
        let k = 1.0 - srgb.r.max(srgb.g).max(srgb.b);
        if k >= 1.0 {
            return Self {
                c: 0.0,
                m: 0.0,
                y: 0.0,
                k: 1.0,
                a: srgb.a,
            };
        }
        Self {
            c: (1.0 - srgb.r - k) / (1.0 - k),
            m: (1.0 - srgb.g - k) / (1.0 - k),
            y: (1.0 - srgb.b - k) / (1.0 - k),
            k,
            a: srgb.a,
        }
    }
}
