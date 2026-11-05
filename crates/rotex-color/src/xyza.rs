//! CIE XYZ colors under a D65 white point, with a separate alpha channel.
//!
//! Convert to [`LinearRgba`] with [`From`].

use crate::LinearRgba;
use rotex_math::Vec4;

/// CIE XYZ tristimulus values under D65, with a separate alpha channel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Xyza {
    /// CIE X tristimulus value.
    pub x: f32,
    /// CIE Y tristimulus value. This is the relative luminance.
    pub y: f32,
    /// CIE Z tristimulus value.
    pub z: f32,
    /// Alpha. `0.0` is transparent and `1.0` is opaque.
    pub a: f32,
}

impl Xyza {
    /// Creates an XYZ color from tristimulus values and alpha.
    pub fn new(x: f32, y: f32, z: f32, a: f32) -> Self {
        Self { x, y, z, a }
    }
}

impl Default for Xyza {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            a: 0.0,
        }
    }
}

// Linear sRGB → CIE XYZ D65
const RGB_TO_XYZ: [Vec4; 3] = [
    Vec4::new(0.4123907993, 0.3575843394, 0.1804807884, 0.0),
    Vec4::new(0.2126390059, 0.7151686788, 0.0721923154, 0.0),
    Vec4::new(0.0193308187, 0.1191947798, 0.9505321522, 0.0),
];

// CIE XYZ D65 → Linear sRGB
const XYZ_TO_RGB: [Vec4; 3] = [
    Vec4::new(3.2409699419, -1.5373831776, -0.4986107603, 0.0),
    Vec4::new(-0.9692436363, 1.8759675015, 0.0415550574, 0.0),
    Vec4::new(0.0556300797, -0.2039769589, 1.0569715142, 0.0),
];

#[inline]
fn apply_mat3(m: &[Vec4; 3], v: Vec4) -> Vec4 {
    Vec4::new(m[0].dot(v), m[1].dot(v), m[2].dot(v), v.w)
}

impl From<LinearRgba> for Xyza {
    fn from(linear: LinearRgba) -> Self {
        let xyz = apply_mat3(&RGB_TO_XYZ, linear.0);
        Xyza {
            x: xyz.x,
            y: xyz.y,
            z: xyz.z,
            a: linear.w,
        }
    }
}

impl From<Xyza> for LinearRgba {
    fn from(xyza: Xyza) -> Self {
        let linear = apply_mat3(&XYZ_TO_RGB, Vec4::new(xyza.x, xyza.y, xyza.z, xyza.a));
        LinearRgba(linear)
    }
}
