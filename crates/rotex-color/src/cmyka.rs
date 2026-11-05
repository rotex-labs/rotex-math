use crate::{LinearRgba, Srgba};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cmyka {
    pub c: f32,
    pub m: f32,
    pub y: f32,
    pub k: f32,
    pub a: f32,
}

impl Cmyka {
    pub fn new(c: f32, m: f32, y: f32, k: f32, a: f32) -> Self {
        Self { c, m, y, k, a }
    }

    pub const CYAN: Self = Self {
        c: 1.0,
        m: 0.0,
        y: 0.0,
        k: 0.0,
        a: 1.0,
    };
    pub const MAGENTA: Self = Self {
        c: 0.0,
        m: 1.0,
        y: 0.0,
        k: 0.0,
        a: 1.0,
    };
    pub const YELLOW: Self = Self {
        c: 0.0,
        m: 0.0,
        y: 1.0,
        k: 0.0,
        a: 1.0,
    };
    pub const BLACK: Self = Self {
        c: 0.0,
        m: 0.0,
        y: 0.0,
        k: 1.0,
        a: 1.0,
    };
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
