use rotex_math::Vec4;
use std::ops::{Deref, DerefMut};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearRgba(pub Vec4);

impl Deref for LinearRgba {
    type Target = Vec4;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LinearRgba {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for LinearRgba {
    fn default() -> Self {
        Self(Vec4::ZERO)
    }
}

impl From<Vec4> for LinearRgba {
    fn from(v: Vec4) -> Self {
        Self(v)
    }
}

impl From<LinearRgba> for Vec4 {
    fn from(c: LinearRgba) -> Self {
        c.0
    }
}

#[inline]
fn linear_to_srgb_channel(c: f32) -> f32 {
    if c <= 0.0031308 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

impl LinearRgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self(Vec4::new(r, g, b, a))
    }

    pub fn luminance(&self) -> f32 {
        self.0.x * 0.2126 + self.0.y * 0.7152 + self.0.z * 0.0722
    }

    pub fn pack_unorm4x8(&self) -> u32 {
        let c = (self.0.clamp(Vec4::ZERO, Vec4::ONE) * 255.0).as_uvec4();
        (c.w << 24) | (c.z << 16) | (c.y << 8) | c.x
    }

    pub fn unpack_unorm4x8(packed: u32) -> Self {
        let r = (packed & 0xFF) as f32 / 255.0;
        let g = ((packed >> 8) & 0xFF) as f32 / 255.0;
        let b = ((packed >> 16) & 0xFF) as f32 / 255.0;
        let a = ((packed >> 24) & 0xFF) as f32 / 255.0;
        Self(Vec4::new(r, g, b, a))
    }

    pub fn to_hex_srgb(&self) -> u32 {
        let r_srgb = linear_to_srgb_channel(self.0.x);
        let g_srgb = linear_to_srgb_channel(self.0.y);
        let b_srgb = linear_to_srgb_channel(self.0.z);

        let r = (r_srgb.clamp(0.0, 1.0) * 255.0).round() as u32;
        let g = (g_srgb.clamp(0.0, 1.0) * 255.0).round() as u32;
        let b = (b_srgb.clamp(0.0, 1.0) * 255.0).round() as u32;
        let a = (self.0.w.clamp(0.0, 1.0) * 255.0).round() as u32;

        (a << 24) | (r << 16) | (g << 8) | b
    }

    pub fn to_hex(&self, gamma: f32) -> u32 {
        let c = self.0.clamp(Vec4::ZERO, Vec4::ONE);
        let inv_gamma = 1.0 / gamma;
        let r = (c.x.powf(inv_gamma) * 255.0) as u32;
        let g = (c.y.powf(inv_gamma) * 255.0) as u32;
        let b = (c.z.powf(inv_gamma) * 255.0) as u32;
        (r << 16) | (g << 8) | b
    }
}

impl LinearRgba {
    pub const WHITE: Self = Self(Vec4::new(1.0, 1.0, 1.0, 1.0));
    pub const BLACK: Self = Self(Vec4::new(0.0, 0.0, 0.0, 1.0));
    pub const RED: Self = Self(Vec4::new(1.0, 0.0, 0.0, 1.0));
    pub const GREEN: Self = Self(Vec4::new(0.0, 1.0, 0.0, 1.0));
    pub const BLUE: Self = Self(Vec4::new(0.0, 0.0, 1.0, 1.0));
    pub const YELLOW: Self = Self(Vec4::new(1.0, 1.0, 0.0, 1.0));
    pub const CYAN: Self = Self(Vec4::new(0.0, 1.0, 1.0, 1.0));
    pub const MAGENTA: Self = Self(Vec4::new(1.0, 0.0, 1.0, 1.0));
    pub const GRAY: Self = Self(Vec4::new(0.215861, 0.215861, 0.215861, 1.0));
    pub const LIGHT_GRAY: Self = Self(Vec4::new(0.651406, 0.651406, 0.651406, 1.0));
    pub const DARK_GRAY: Self = Self(Vec4::new(0.051269, 0.051269, 0.051269, 1.0));
    pub const SILVER: Self = Self(Vec4::new(0.527115, 0.527115, 0.527115, 1.0));
    pub const ORANGE: Self = Self(Vec4::new(1.0, 0.376262, 0.0, 1.0));
    pub const GOLD: Self = Self(Vec4::new(1.0, 0.679542, 0.0, 1.0));
    pub const CORAL: Self = Self(Vec4::new(1.0, 0.212231, 0.08022, 1.0));
    pub const CRIMSON: Self = Self(Vec4::new(0.715694, 0.006995, 0.045186, 1.0));
    pub const TOMATO: Self = Self(Vec4::new(1.0, 0.124772, 0.06301, 1.0));
    pub const SALMON: Self = Self(Vec4::new(0.955973, 0.215861, 0.168269, 1.0));
    pub const HOT_PINK: Self = Self(Vec4::new(1.0, 0.141263, 0.456411, 1.0));
    pub const DARK_ORANGE: Self = Self(Vec4::new(1.0, 0.262251, 0.0, 1.0));
    pub const GOLDENROD: Self = Self(Vec4::new(0.701102, 0.376262, 0.014444, 1.0));
    pub const PINK: Self = Self(Vec4::new(1.0, 0.527115, 0.597202, 1.0));
    pub const ROYAL_BLUE: Self = Self(Vec4::new(0.052861, 0.141263, 0.752942, 1.0));
    pub const DODGER_BLUE: Self = Self(Vec4::new(0.012983, 0.278894, 1.0, 1.0));
    pub const CORNFLOWER_BLUE: Self = Self(Vec4::new(0.127438, 0.300544, 0.846873, 1.0));
    pub const STEEL_BLUE: Self = Self(Vec4::new(0.061246, 0.223228, 0.456411, 1.0));
    pub const DEEP_SKY_BLUE: Self = Self(Vec4::new(0.0, 0.520996, 1.0, 1.0));
    pub const SKY_BLUE: Self = Self(Vec4::new(0.242281, 0.617207, 0.83077, 1.0));
    pub const FOREST_GREEN: Self = Self(Vec4::new(0.015996, 0.258183, 0.015996, 1.0));
    pub const SEA_GREEN: Self = Self(Vec4::new(0.027321, 0.258183, 0.095307, 1.0));
    pub const LIME_GREEN: Self = Self(Vec4::new(0.031896, 0.610496, 0.031896, 1.0));
    pub const OLIVE: Self = Self(Vec4::new(0.215861, 0.215861, 0.0, 1.0));
    pub const BROWN: Self = Self(Vec4::new(0.376262, 0.023153, 0.023153, 1.0));
    pub const CHOCOLATE: Self = Self(Vec4::new(0.64448, 0.141263, 0.012983, 1.0));
    pub const SANDY_BROWN: Self = Self(Vec4::new(0.904661, 0.371238, 0.116971, 1.0));
    pub const PURPLE: Self = Self(Vec4::new(0.215861, 0.0, 0.215861, 1.0));
    pub const VIOLET: Self = Self(Vec4::new(0.854993, 0.223228, 0.854993, 1.0));
    pub const INDIGO: Self = Self(Vec4::new(0.07036, 0.0, 0.223228, 1.0));
    pub const MEDIUM_PURPLE: Self = Self(Vec4::new(0.291771, 0.162029, 0.708376, 1.0));
    pub const SLATE_BLUE: Self = Self(Vec4::new(0.144128, 0.102242, 0.610496, 1.0));
    pub const DARK_SLATE_BLUE: Self = Self(Vec4::new(0.064803, 0.046665, 0.258183, 1.0));
    pub const NAVY: Self = Self(Vec4::new(0.0, 0.0, 0.215861, 1.0));
    pub const TEAL: Self = Self(Vec4::new(0.0, 0.215861, 0.215861, 1.0));
    pub const TURQUOISE: Self = Self(Vec4::new(0.051269, 0.745404, 0.630757, 1.0));
    pub const DARK_TURQUOISE: Self = Self(Vec4::new(0.0, 0.617207, 0.637597, 1.0));
    pub const MAROON: Self = Self(Vec4::new(0.215861, 0.0, 0.0, 1.0));
    pub const TRANSPARENT: Self = Self(Vec4::new(0.0, 0.0, 0.0, 0.0));
}
