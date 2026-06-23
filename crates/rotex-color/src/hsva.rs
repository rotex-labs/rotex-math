use crate::LinearRgba;
use rotex_math::Vec4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsva {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    pub a: f32,
}

impl Hsva {
    pub fn new(h: f32, s: f32, v: f32, a: f32) -> Self {
        Self { h, s, v, a }
    }

    pub const WHITE: Self = Self { h: 0.0, s: 0.0, v: 1.0, a: 1.0 };
    pub const BLACK: Self = Self { h: 0.0, s: 0.0, v: 0.0, a: 1.0 };
    pub const RED: Self = Self { h: 0.0, s: 1.0, v: 1.0, a: 1.0 };
    pub const GREEN: Self = Self { h: 120.0, s: 1.0, v: 1.0, a: 1.0 };
    pub const BLUE: Self = Self { h: 240.0, s: 1.0, v: 1.0, a: 1.0 };
    pub const YELLOW: Self = Self { h: 60.0, s: 1.0, v: 1.0, a: 1.0 };
    pub const CYAN: Self = Self { h: 180.0, s: 1.0, v: 1.0, a: 1.0 };
    pub const MAGENTA: Self = Self { h: 300.0, s: 1.0, v: 1.0, a: 1.0 };
    pub const GRAY: Self = Self { h: 0.0, s: 0.0, v: 0.215861, a: 1.0 };
    pub const LIGHT_GRAY: Self = Self { h: 0.0, s: 0.0, v: 0.651406, a: 1.0 };
    pub const DARK_GRAY: Self = Self { h: 0.0, s: 0.0, v: 0.051269, a: 1.0 };
    pub const SILVER: Self = Self { h: 0.0, s: 0.0, v: 0.527115, a: 1.0 };
    pub const ORANGE: Self = Self { h: 22.575727, s: 1.0, v: 1.0, a: 1.0 };
    pub const GOLD: Self = Self { h: 40.772548, s: 1.0, v: 1.0, a: 1.0 };
    pub const CORAL: Self = Self { h: 8.611467, s: 0.91978, v: 1.0, a: 1.0 };
    pub const CRIMSON: Self = Self { h: 356.76668, s: 0.990226, v: 0.715694, a: 1.0 };
    pub const TOMATO: Self = Self { h: 3.954907, s: 0.93699, v: 1.0, a: 1.0 };
    pub const SALMON: Self = Self { h: 3.62505, s: 0.823981, v: 0.955973, a: 1.0 };
    pub const HOT_PINK: Self = Self { h: 337.980604, s: 0.858737, v: 1.0, a: 1.0 };
    pub const DARK_ORANGE: Self = Self { h: 15.735039, s: 1.0, v: 1.0, a: 1.0 };
    pub const GOLDENROD: Self = Self { h: 31.615586, s: 0.979398, v: 0.701102, a: 1.0 };
    pub const PINK: Self = Self { h: 351.10735, s: 0.472885, v: 1.0, a: 1.0 };
    pub const ROYAL_BLUE: Self = Self { h: 232.423513, s: 0.929795, v: 0.752942, a: 1.0 };
    pub const DODGER_BLUE: Self = Self { h: 223.835461, s: 0.987017, v: 1.0, a: 1.0 };
    pub const CORNFLOWER_BLUE: Self = Self { h: 225.563173, s: 0.84952, v: 0.846873, a: 1.0 };
    pub const STEEL_BLUE: Self = Self { h: 215.405426, s: 0.865809, v: 0.456411, a: 1.0 };
    pub const DEEP_SKY_BLUE: Self = Self { h: 208.740266, s: 1.0, v: 1.0, a: 1.0 };
    pub const SKY_BLUE: Self = Self { h: 201.774076, s: 0.708366, v: 0.83077, a: 1.0 };
    pub const FOREST_GREEN: Self = Self { h: 120.0, s: 0.938043, v: 0.258183, a: 1.0 };
    pub const SEA_GREEN: Self = Self { h: 137.669409, s: 0.89418, v: 0.258183, a: 1.0 };
    pub const LIME_GREEN: Self = Self { h: 120.0, s: 0.947754, v: 0.610496, a: 1.0 };
    pub const OLIVE: Self = Self { h: 60.0, s: 1.0, v: 0.215861, a: 1.0 };
    pub const BROWN: Self = Self { h: 0.0, s: 0.938465, v: 0.376262, a: 1.0 };
    pub const CHOCOLATE: Self = Self { h: 12.188213, s: 0.979855, v: 0.64448, a: 1.0 };
    pub const SANDY_BROWN: Self = Self { h: 19.368039, s: 0.870702, v: 0.904661, a: 1.0 };
    pub const PURPLE: Self = Self { h: 300.0, s: 1.0, v: 0.215861, a: 1.0 };
    pub const VIOLET: Self = Self { h: 300.0, s: 0.738912, v: 0.854993, a: 1.0 };
    pub const INDIGO: Self = Self { h: 258.911635, s: 1.0, v: 0.223228, a: 1.0 };
    pub const MEDIUM_PURPLE: Self = Self { h: 254.248243, s: 0.771266, v: 0.708376, a: 1.0 };
    pub const SLATE_BLUE: Self = Self { h: 244.944782, s: 0.832527, v: 0.610496, a: 1.0 };
    pub const DARK_SLATE_BLUE: Self = Self { h: 245.145151, s: 0.819256, v: 0.258183, a: 1.0 };
    pub const NAVY: Self = Self { h: 240.0, s: 1.0, v: 0.215861, a: 1.0 };
    pub const TEAL: Self = Self { h: 180.0, s: 1.0, v: 0.215861, a: 1.0 };
    pub const TURQUOISE: Self = Self { h: 170.090073, s: 0.931219, v: 0.745404, a: 1.0 };
    pub const DARK_TURQUOISE: Self = Self { h: 181.918797, s: 1.0, v: 0.637597, a: 1.0 };
    pub const MAROON: Self = Self { h: 0.0, s: 1.0, v: 0.215861, a: 1.0 };
}

impl Default for Hsva {
    fn default() -> Self {
        Self {
            h: 0.0,
            s: 0.0,
            v: 0.0,
            a: 0.0,
        }
    }
}

impl From<LinearRgba> for Hsva {
    fn from(color: LinearRgba) -> Self {
        let c_max = color.x.max(color.y).max(color.z);
        let c_min = color.x.min(color.y).min(color.z);
        let delta = c_max - c_min;

        let mut h = if delta == 0.0 {
            0.0
        } else if c_max == color.x {
            60.0 * (((color.y - color.z) / delta) % 6.0)
        } else if c_max == color.y {
            60.0 * (((color.z - color.x) / delta) + 2.0)
        } else {
            60.0 * (((color.x - color.y) / delta) + 4.0)
        };

        if h < 0.0 {
            h += 360.0;
        }

        let s = if c_max == 0.0 { 0.0 } else { delta / c_max };
        let v = c_max;

        Hsva {
            h,
            s,
            v,
            a: color.w,
        }
    }
}

impl From<Hsva> for LinearRgba {
    fn from(hsva: Hsva) -> Self {
        let c = hsva.v * hsva.s;
        let h_prime = (hsva.h % 360.0) / 60.0;
        let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
        let m = hsva.v - c;

        let (r, g, b) = if h_prime < 1.0 {
            (c, x, 0.0)
        } else if h_prime < 2.0 {
            (x, c, 0.0)
        } else if h_prime < 3.0 {
            (0.0, c, x)
        } else if h_prime < 4.0 {
            (0.0, x, c)
        } else if h_prime < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        LinearRgba(Vec4::new(r + m, g + m, b + m, hsva.a))
    }
}
