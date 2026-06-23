//! Linear algebra, spatial geometry, and frustum culling for the Rotex engine.
//!
//! This crate re-exports all [`glam`] types at the crate root so you can write
//! `rotex_math::Vec3` instead of `rotex_math::glam::Vec3`. All `glam` types
//! remain available under `rotex_math::glam::*` for compatibility.
//!
//! # Feature flags
//!
//! * `wide-simd` — Enables SIMD-batched frustum culling via the
//!   [`ultraviolet`] and [`wide`] crates. Without this feature the crate has
//!   zero additional dependencies beyond `glam`.
//!
//! # Examples
//!
//! ```rust
//! use rotex_math::{Vec3, Mat4, Frustum, Aabb};
//!
//! let vp = Mat4::perspective_rh_gl(70.0_f32.to_radians(), 1.0, 0.1, 100.0)
//!     * Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
//! let frustum = Frustum::from_view_proj(vp);
//!
//! let aabb = Aabb::from_min_max(Vec3::splat(-0.5), Vec3::splat(0.5));
//! assert!(frustum.contains_aabb(&aabb));
//! ```

pub use glam;
/// Re-exported [`glam`] types for direct access at the crate root.
///
/// | Category | 2D | 3D | 4D |
/// |---|---|---|---|
/// | **f32** (default) | [`Vec2`] | [`Vec3`], [`Vec3A`] | [`Vec4`] |
/// | **f32** matrices | [`Mat2`] | [`Mat3`], [`Mat3A`] | [`Mat4`] |
/// | **f32** rotation | — | [`Quat`] | — |
/// | **f32** affine | [`Affine2`] | [`Affine3`], [`Affine3A`] | — |
/// | **f64** (double) | [`DVec2`] | [`DVec3`] | [`DVec4`] |
/// | **f64** matrices | [`DMat2`] | [`DMat3`] | [`DMat4`] |
/// | **f64** rotation | — | [`DQuat`] | — |
/// | **f64** affine | [`DAffine2`] | [`DAffine3`] | — |
/// | **i8** | [`I8Vec2`] | [`I8Vec3`] | [`I8Vec4`] |
/// | **i16** | [`I16Vec2`] | [`I16Vec3`] | [`I16Vec4`] |
/// | **i32** | [`IVec2`] | [`IVec3`] | [`IVec4`] |
/// | **i64** | [`I64Vec2`] | [`I64Vec3`] | [`I64Vec4`] |
/// | **isize** | [`ISizeVec2`] | [`ISizeVec3`] | [`ISizeVec4`] |
/// | **u8** | [`U8Vec2`] | [`U8Vec3`] | [`U8Vec4`] |
/// | **u16** | [`U16Vec2`] | [`U16Vec3`] | [`U16Vec4`] |
/// | **u32** | [`UVec2`] | [`UVec3`] | [`UVec4`] |
/// | **u64** | [`U64Vec2`] | [`U64Vec3`] | [`U64Vec4`] |
/// | **usize** | [`USizeVec2`] | [`USizeVec3`] | [`USizeVec4`] |
/// | **bool** | [`BVec2`] | [`BVec3`] | [`BVec4`] |
/// | **enum** | [`EulerRot`] — Euler rotation order | | |
/// | **traits** | [`FloatExt`], [`Vec2Swizzles`], [`Vec3Swizzles`], [`Vec4Swizzles`] | | |
///
/// All types link to their upstream [`glam`] documentation for full method lists.
pub use glam::{
    Vec2, Vec3, Vec3A, Vec4,
    Mat2, Mat3, Mat3A, Mat4,
    Quat,
    Affine2, Affine3, Affine3A,
    DVec2, DVec3, DVec4,
    DMat2, DMat3, DMat4,
    DQuat,
    DAffine2, DAffine3,
    I8Vec2, I8Vec3, I8Vec4,
    I16Vec2, I16Vec3, I16Vec4,
    IVec2, IVec3, IVec4,
    I64Vec2, I64Vec3, I64Vec4,
    ISizeVec2, ISizeVec3, ISizeVec4,
    U8Vec2, U8Vec3, U8Vec4,
    U16Vec2, U16Vec3, U16Vec4,
    UVec2, UVec3, UVec4,
    U64Vec2, U64Vec3, U64Vec4,
    USizeVec2, USizeVec3, USizeVec4,
    BVec2, BVec3, BVec4,
    EulerRot,
    FloatExt,
    Vec2Swizzles, Vec3Swizzles, Vec4Swizzles,
};

/// Spatial primitives: bounding boxes, rays, and line segments.
pub mod geometry;

/// Plane and frustum types for view-volume culling.
pub mod frustum;

/// SIMD-batched frustum culling (requires `wide-simd` feature).
pub mod wide_simd;

pub use geometry::{Aabb, Aabb2, LineSegment2D, LineSegment3D, Ray2D, Ray3D};
pub use frustum::{Frustum, Plane};
