//! Oklch colors (Oklab lightness, chroma, and hue).
//!
//! [`Oklcha::lerp`] interpolates in this space. Convert to [`LinearRgba`] with [`From`].

use crate::LinearRgba;
use rotex_math::Vec4;

/// Oklch color: Oklab lightness, chroma, and hue, plus alpha.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Oklcha {
    /// Oklab lightness.
    pub l: f32,
    /// Oklab chroma. `0.0` is a neutral gray at this lightness.
    pub c: f32,
    /// Hue in degrees.
    pub h: f32,
    /// Alpha. `0.0` is transparent and `1.0` is opaque.
    pub a: f32,
}

impl Oklcha {
    /// Creates an Oklch color. `h` is degrees.
    pub fn new(l: f32, c: f32, h: f32, a: f32) -> Self {
        Self { l, c, h, a }
    }

    /// Linearly interpolates each channel toward `other` by `t`.
    ///
    /// Hue is not wrapped, so a short hue arc has to be arranged by the caller.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            l: self.l + (other.l - self.l) * t,
            c: self.c + (other.c - self.c) * t,
            h: self.h + (other.h - self.h) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    /// The Oklch color `white`.
    pub const WHITE: Self = Self {
        l: 1.0,
        c: 0.0,
        h: 89.875563,
        a: 1.0,
    };
    /// The Oklch color `black`.
    pub const BLACK: Self = Self {
        l: 0.0,
        c: 0.0,
        h: 0.0,
        a: 1.0,
    };
    /// The Oklch color `red`.
    pub const RED: Self = Self {
        l: 0.627955,
        c: 0.257683,
        h: 29.233885,
        a: 1.0,
    };
    /// The Oklch color `green`.
    pub const GREEN: Self = Self {
        l: 0.86644,
        c: 0.294827,
        h: 142.495339,
        a: 1.0,
    };
    /// The Oklch color `blue`.
    pub const BLUE: Self = Self {
        l: 0.452014,
        c: 0.313214,
        h: 264.052021,
        a: 1.0,
    };
    /// The Oklch color `yellow`.
    pub const YELLOW: Self = Self {
        l: 0.967983,
        c: 0.211006,
        h: 109.769232,
        a: 1.0,
    };
    /// The Oklch color `cyan`.
    pub const CYAN: Self = Self {
        l: 0.905399,
        c: 0.15455,
        h: 194.768948,
        a: 1.0,
    };
    /// The Oklch color `magenta`.
    pub const MAGENTA: Self = Self {
        l: 0.701674,
        c: 0.322491,
        h: 328.363418,
        a: 1.0,
    };
    /// The Oklch color `gray`.
    pub const GRAY: Self = Self {
        l: 0.599871,
        c: 0.0,
        h: 89.875562,
        a: 1.0,
    };
    /// The Oklch color `light gray`.
    pub const LIGHT_GRAY: Self = Self {
        l: 0.866863,
        c: 0.0,
        h: 89.875563,
        a: 1.0,
    };
    /// The Oklch color `dark gray`.
    pub const DARK_GRAY: Self = Self {
        l: 0.371495,
        c: 0.0,
        h: 89.875563,
        a: 1.0,
    };
    /// The Oklch color `silver`.
    pub const SILVER: Self = Self {
        l: 0.807796,
        c: 0.0,
        h: 89.875563,
        a: 1.0,
    };
    /// The Oklch color `orange`.
    pub const ORANGE: Self = Self {
        l: 0.792688,
        c: 0.171026,
        h: 70.66992,
        a: 1.0,
    };
    /// The Oklch color `gold`.
    pub const GOLD: Self = Self {
        l: 0.886771,
        c: 0.182186,
        h: 95.330493,
        a: 1.0,
    };
    /// The Oklch color `coral`.
    pub const CORAL: Self = Self {
        l: 0.735113,
        c: 0.167994,
        h: 40.24667,
        a: 1.0,
    };
    /// The Oklch color `crimson`.
    pub const CRIMSON: Self = Self {
        l: 0.571189,
        c: 0.221938,
        h: 20.087367,
        a: 1.0,
    };
    /// The Oklch color `tomato`.
    pub const TOMATO: Self = Self {
        l: 0.696219,
        c: 0.195525,
        h: 32.321441,
        a: 1.0,
    };
    /// The Oklch color `salmon`.
    pub const SALMON: Self = Self {
        l: 0.735002,
        c: 0.151508,
        h: 28.060457,
        a: 1.0,
    };
    /// The Oklch color `hot pink`.
    pub const HOT_PINK: Self = Self {
        l: 0.728297,
        c: 0.197075,
        h: 351.994708,
        a: 1.0,
    };
    /// The Oklch color `dark orange`.
    pub const DARK_ORANGE: Self = Self {
        l: 0.750544,
        c: 0.179115,
        h: 58.282686,
        a: 1.0,
    };
    /// The Oklch color `goldenrod`.
    pub const GOLDENROD: Self = Self {
        l: 0.751572,
        c: 0.146934,
        h: 83.988117,
        a: 1.0,
    };
    /// The Oklch color `pink`.
    pub const PINK: Self = Self {
        l: 0.867738,
        c: 0.073542,
        h: 7.085514,
        a: 1.0,
    };
    /// The Oklch color `royal blue`.
    pub const ROYAL_BLUE: Self = Self {
        l: 0.559848,
        c: 0.188234,
        h: 266.399595,
        a: 1.0,
    };
    /// The Oklch color `dodger blue`.
    pub const DODGER_BLUE: Self = Self {
        l: 0.652006,
        c: 0.190119,
        h: 253.20541,
        a: 1.0,
    };
    /// The Oklch color `cornflower blue`.
    pub const CORNFLOWER_BLUE: Self = Self {
        l: 0.674622,
        c: 0.141357,
        h: 261.338021,
        a: 1.0,
    };
    /// The Oklch color `steel blue`.
    pub const STEEL_BLUE: Self = Self {
        l: 0.588001,
        c: 0.099339,
        h: 245.739412,
        a: 1.0,
    };
    /// The Oklch color `deep sky blue`.
    pub const DEEP_SKY_BLUE: Self = Self {
        l: 0.75535,
        c: 0.153422,
        h: 231.63915,
        a: 1.0,
    };
    /// The Oklch color `sky blue`.
    pub const SKY_BLUE: Self = Self {
        l: 0.814817,
        c: 0.081916,
        h: 225.753653,
        a: 1.0,
    };
    /// The Oklch color `forest green`.
    pub const FOREST_GREEN: Self = Self {
        l: 0.557805,
        c: 0.168781,
        h: 142.893115,
        a: 1.0,
    };
    /// The Oklch color `sea green`.
    pub const SEA_GREEN: Self = Self {
        l: 0.568526,
        c: 0.118714,
        h: 154.949141,
        a: 1.0,
    };
    /// The Oklch color `lime green`.
    pub const LIME_GREEN: Self = Self {
        l: 0.741874,
        c: 0.228646,
        h: 142.834802,
        a: 1.0,
    };
    /// The Oklch color `olive`.
    pub const OLIVE: Self = Self {
        l: 0.580665,
        c: 0.126576,
        h: 109.769232,
        a: 1.0,
    };
    /// The Oklch color `brown`.
    pub const BROWN: Self = Self {
        l: 0.480613,
        c: 0.159657,
        h: 25.562112,
        a: 1.0,
    };
    /// The Oklch color `chocolate`.
    pub const CHOCOLATE: Self = Self {
        l: 0.634398,
        c: 0.154992,
        h: 50.266483,
        a: 1.0,
    };
    /// The Oklch color `sandy brown`.
    pub const SANDY_BROWN: Self = Self {
        l: 0.783997,
        c: 0.126913,
        h: 59.709772,
        a: 1.0,
    };
    /// The Oklch color `purple`.
    pub const PURPLE: Self = Self {
        l: 0.420914,
        c: 0.193453,
        h: 328.363418,
        a: 1.0,
    };
    /// The Oklch color `violet`.
    pub const VIOLET: Self = Self {
        l: 0.761899,
        c: 0.186122,
        h: 327.209627,
        a: 1.0,
    };
    /// The Oklch color `indigo`.
    pub const INDIGO: Self = Self {
        l: 0.338982,
        c: 0.179272,
        h: 301.684759,
        a: 1.0,
    };
    /// The Oklch color `medium purple`.
    pub const MEDIUM_PURPLE: Self = Self {
        l: 0.626914,
        c: 0.158031,
        h: 296.671818,
        a: 1.0,
    };
    /// The Oklch color `slate blue`.
    pub const SLATE_BLUE: Self = Self {
        l: 0.543567,
        c: 0.171226,
        h: 285.541337,
        a: 1.0,
    };
    /// The Oklch color `dark slate blue`.
    pub const DARK_SLATE_BLUE: Self = Self {
        l: 0.414343,
        c: 0.124836,
        h: 286.039109,
        a: 1.0,
    };
    /// The Oklch color `navy`.
    pub const NAVY: Self = Self {
        l: 0.27115,
        c: 0.187888,
        h: 264.052021,
        a: 1.0,
    };
    /// The Oklch color `teal`.
    pub const TEAL: Self = Self {
        l: 0.543123,
        c: 0.09271,
        h: 194.768948,
        a: 1.0,
    };
    /// The Oklch color `turquoise`.
    pub const TURQUOISE: Self = Self {
        l: 0.822334,
        c: 0.130744,
        h: 185.088963,
        a: 1.0,
    };
    /// The Oklch color `dark turquoise`.
    pub const DARK_TURQUOISE: Self = Self {
        l: 0.771929,
        c: 0.13144,
        h: 196.643336,
        a: 1.0,
    };
    /// The Oklch color `maroon`.
    pub const MAROON: Self = Self {
        l: 0.376692,
        c: 0.154577,
        h: 29.233885,
        a: 1.0,
    };
}

impl Default for Oklcha {
    fn default() -> Self {
        Self {
            l: 0.0,
            c: 0.0,
            h: 0.0,
            a: 0.0,
        }
    }
}

// Linear sRGB → LMS (M1)
const M1: [Vec4; 3] = [
    Vec4::new(0.4122214708, 0.5363325363, 0.0514459929, 0.0),
    Vec4::new(0.2119034982, 0.6806995451, 0.1073969566, 0.0),
    Vec4::new(0.0883024619, 0.2817188376, 0.6299787005, 0.0),
];

// LMS' → OKLab (M2)
const M2: [Vec4; 3] = [
    Vec4::new(0.2104542553, 0.7936177850, -0.0040720468, 0.0),
    Vec4::new(1.9779984951, -2.4285922050, 0.4505937099, 0.0),
    Vec4::new(0.0259040371, 0.7827717662, -0.8086757660, 0.0),
];

// OKLab → LMS' (M2⁻¹)
const M2_INV: [Vec4; 3] = [
    Vec4::new(1.0000000000, 0.3963377774, 0.2158037573, 0.0),
    Vec4::new(1.0000000000, -0.1055613458, -0.0638541728, 0.0),
    Vec4::new(1.0000000000, -0.0894841775, -1.2914855480, 0.0),
];

// LMS → Linear sRGB (M1⁻¹)
const M1_INV: [Vec4; 3] = [
    Vec4::new(4.0767416621, -3.3077115913, 0.2309699292, 0.0),
    Vec4::new(-1.2684380046, 2.6097574011, -0.3413193965, 0.0),
    Vec4::new(-0.0041960863, -0.7034186147, 1.7076147010, 0.0),
];

#[inline]
fn apply_mat3(m: &[Vec4; 3], v: Vec4) -> Vec4 {
    Vec4::new(m[0].dot(v), m[1].dot(v), m[2].dot(v), v.w)
}

#[inline]
fn cbrt_vec4(v: Vec4) -> Vec4 {
    Vec4::new(v.x.cbrt(), v.y.cbrt(), v.z.cbrt(), v.w)
}

#[inline]
fn cube_vec4(v: Vec4) -> Vec4 {
    Vec4::new(v.x * v.x * v.x, v.y * v.y * v.y, v.z * v.z * v.z, v.w)
}

impl From<LinearRgba> for Oklcha {
    fn from(linear: LinearRgba) -> Self {
        let lms = apply_mat3(&M1, linear.0);
        let lms_prime = cbrt_vec4(lms);
        let oklab = apply_mat3(&M2, lms_prime);
        let l = oklab.x;
        let a = oklab.y;
        let b = oklab.z;
        let c = (a * a + b * b).sqrt();
        let h = b.atan2(a).to_degrees();
        let h = if h < 0.0 { h + 360.0 } else { h };
        Oklcha {
            l,
            c,
            h,
            a: linear.w,
        }
    }
}

impl From<Oklcha> for LinearRgba {
    fn from(oklcha: Oklcha) -> Self {
        let h_rad = oklcha.h.to_radians();
        let oklab = Vec4::new(
            oklcha.l,
            oklcha.c * h_rad.cos(),
            oklcha.c * h_rad.sin(),
            oklcha.a,
        );
        let lms_prime = apply_mat3(&M2_INV, oklab);
        let lms = cube_vec4(lms_prime);
        let linear = apply_mat3(&M1_INV, lms);
        LinearRgba(linear)
    }
}
