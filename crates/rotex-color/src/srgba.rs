use crate::LinearRgba;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Srgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Srgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        Self { r, g, b, a: 1.0 }
    }

    pub fn to_hex(&self) -> u32 {
        let r = (self.r.clamp(0.0, 1.0) * 255.0).round() as u32;
        let g = (self.g.clamp(0.0, 1.0) * 255.0).round() as u32;
        let b = (self.b.clamp(0.0, 1.0) * 255.0).round() as u32;
        (r << 16) | (g << 8) | b
    }

    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Self = Self { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const CYAN: Self = Self { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const MAGENTA: Self = Self { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const GRAY: Self = Self { r: 0.501961, g: 0.501961, b: 0.501961, a: 1.0 };
    pub const LIGHT_GRAY: Self = Self { r: 0.827451, g: 0.827451, b: 0.827451, a: 1.0 };
    pub const DARK_GRAY: Self = Self { r: 0.25098, g: 0.25098, b: 0.25098, a: 1.0 };
    pub const SILVER: Self = Self { r: 0.752941, g: 0.752941, b: 0.752941, a: 1.0 };
    pub const ORANGE: Self = Self { r: 1.0, g: 0.647059, b: 0.0, a: 1.0 };
    pub const GOLD: Self = Self { r: 1.0, g: 0.843137, b: 0.0, a: 1.0 };
    pub const CORAL: Self = Self { r: 1.0, g: 0.498039, b: 0.313725, a: 1.0 };
    pub const CRIMSON: Self = Self { r: 0.862745, g: 0.078431, b: 0.235294, a: 1.0 };
    pub const TOMATO: Self = Self { r: 1.0, g: 0.388235, b: 0.278431, a: 1.0 };
    pub const SALMON: Self = Self { r: 0.980392, g: 0.501961, b: 0.447059, a: 1.0 };
    pub const HOT_PINK: Self = Self { r: 1.0, g: 0.411765, b: 0.705882, a: 1.0 };
    pub const DARK_ORANGE: Self = Self { r: 1.0, g: 0.54902, b: 0.0, a: 1.0 };
    pub const GOLDENROD: Self = Self { r: 0.854902, g: 0.647059, b: 0.12549, a: 1.0 };
    pub const PINK: Self = Self { r: 1.0, g: 0.752941, b: 0.796078, a: 1.0 };
    pub const ROYAL_BLUE: Self = Self { r: 0.254902, g: 0.411765, b: 0.882353, a: 1.0 };
    pub const DODGER_BLUE: Self = Self { r: 0.117647, g: 0.564706, b: 1.0, a: 1.0 };
    pub const CORNFLOWER_BLUE: Self = Self { r: 0.392157, g: 0.584314, b: 0.929412, a: 1.0 };
    pub const STEEL_BLUE: Self = Self { r: 0.27451, g: 0.509804, b: 0.705882, a: 1.0 };
    pub const DEEP_SKY_BLUE: Self = Self { r: 0.0, g: 0.74902, b: 1.0, a: 1.0 };
    pub const SKY_BLUE: Self = Self { r: 0.529412, g: 0.807843, b: 0.921569, a: 1.0 };
    pub const FOREST_GREEN: Self = Self { r: 0.133333, g: 0.545098, b: 0.133333, a: 1.0 };
    pub const SEA_GREEN: Self = Self { r: 0.180392, g: 0.545098, b: 0.341176, a: 1.0 };
    pub const LIME_GREEN: Self = Self { r: 0.196078, g: 0.803922, b: 0.196078, a: 1.0 };
    pub const OLIVE: Self = Self { r: 0.501961, g: 0.501961, b: 0.0, a: 1.0 };
    pub const BROWN: Self = Self { r: 0.647059, g: 0.164706, b: 0.164706, a: 1.0 };
    pub const CHOCOLATE: Self = Self { r: 0.823529, g: 0.411765, b: 0.117647, a: 1.0 };
    pub const SANDY_BROWN: Self = Self { r: 0.956863, g: 0.643137, b: 0.376471, a: 1.0 };
    pub const PURPLE: Self = Self { r: 0.501961, g: 0.0, b: 0.501961, a: 1.0 };
    pub const VIOLET: Self = Self { r: 0.933333, g: 0.509804, b: 0.933333, a: 1.0 };
    pub const INDIGO: Self = Self { r: 0.294118, g: 0.0, b: 0.509804, a: 1.0 };
    pub const MEDIUM_PURPLE: Self = Self { r: 0.576471, g: 0.439216, b: 0.858824, a: 1.0 };
    pub const SLATE_BLUE: Self = Self { r: 0.415686, g: 0.352941, b: 0.803922, a: 1.0 };
    pub const DARK_SLATE_BLUE: Self = Self { r: 0.282353, g: 0.239216, b: 0.545098, a: 1.0 };
    pub const NAVY: Self = Self { r: 0.0, g: 0.0, b: 0.501961, a: 1.0 };
    pub const TEAL: Self = Self { r: 0.0, g: 0.501961, b: 0.501961, a: 1.0 };
    pub const TURQUOISE: Self = Self { r: 0.25098, g: 0.878431, b: 0.815686, a: 1.0 };
    pub const DARK_TURQUOISE: Self = Self { r: 0.0, g: 0.807843, b: 0.819608, a: 1.0 };
    pub const MAROON: Self = Self { r: 0.501961, g: 0.0, b: 0.0, a: 1.0 };
}

impl Default for Srgba {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }
}

impl From<Srgba> for LinearRgba {
    fn from(srgb: Srgba) -> Self {
        fn srgb_to_linear_channel(c: f32) -> f32 {
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }
        LinearRgba::new(
            srgb_to_linear_channel(srgb.r),
            srgb_to_linear_channel(srgb.g),
            srgb_to_linear_channel(srgb.b),
            srgb.a,
        )
    }
}

impl From<LinearRgba> for Srgba {
    fn from(linear: LinearRgba) -> Self {
        fn linear_to_srgb_channel(c: f32) -> f32 {
            if c <= 0.0031308 {
                c * 12.92
            } else {
                1.055 * c.powf(1.0 / 2.4) - 0.055
            }
        }
        Srgba {
            r: linear_to_srgb_channel(linear.x),
            g: linear_to_srgb_channel(linear.y),
            b: linear_to_srgb_channel(linear.z),
            a: linear.w,
        }
    }
}
