use crate::geometry::{Aabb, Ray3D};
use crate::{Mat4, Vec3};

/// A plane in Hessian normal form: `dot(normal, point) + distance = 0`.
///
/// Points on the side of the plane toward which the normal points yield a
/// positive distance from [`distance_to_point`](Self::distance_to_point).
///
/// # Examples
///
/// ```rust
/// use rotex_math::{Plane, Vec3};
///
/// // Plane through the origin with normal +Y
/// let plane = Plane::new(Vec3::Y, 0.0);
/// assert!(plane.distance_to_point(Vec3::new(0.0, 1.0, 0.0)) > 0.0);
/// assert!(plane.distance_to_point(Vec3::new(0.0, -1.0, 0.0)) < 0.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
}

impl Plane {
    /// Construct a plane from a normal and signed distance.
    ///
    /// The normal should ideally be unit length; if not, call
    /// [`normalize`](Self::normalize) afterwards.
    pub fn new(normal: Vec3, distance: f32) -> Self {
        Self { normal, distance }
    }

    /// Construct a plane from a point on the plane and the surface normal.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotex_math::{Plane, Vec3};
    /// let plane = Plane::from_point_normal(Vec3::new(0.0, 5.0, 0.0), Vec3::Y);
    /// assert!(plane.distance_to_point(Vec3::new(0.0, 5.0, 0.0)).abs() < 1e-6);
    /// ```
    pub fn from_point_normal(point: Vec3, normal: Vec3) -> Self {
        Self {
            normal,
            distance: -normal.dot(point),
        }
    }

    /// Normalise the plane so that `normal` becomes a unit vector.
    ///
    /// The signed `distance` is scaled by the same factor so that the plane
    /// equation remains satisfied.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotex_math::{Plane, Vec3};
    /// let plane = Plane::new(Vec3::splat(2.0), 4.0).normalize();
    /// assert!((plane.normal.length() - 1.0).abs() < 1e-6);
    /// ```
    pub fn normalize(self) -> Self {
        let length = self.normal.length();
        Self {
            normal: self.normal / length,
            distance: self.distance / length,
        }
    }

    /// Signed distance from a point to the plane.
    ///
    /// Returns a positive value for points on the side of the plane toward
    /// which the normal points, negative for the opposite side.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotex_math::{Plane, Vec3};
    /// let plane = Plane::new(Vec3::Y, 0.0);
    /// assert_eq!(plane.distance_to_point(Vec3::Y), 1.0);
    /// assert_eq!(plane.distance_to_point(-Vec3::Y), -1.0);
    /// ```
    pub fn distance_to_point(self, point: Vec3) -> f32 {
        self.normal.dot(point) + self.distance
    }

    /// Returns the distance `t` along the ray at which it intersects this
    /// plane, or `None` if the ray is parallel to the plane.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotex_math::{Plane, Ray3D, Vec3};
    /// let plane = Plane::new(Vec3::Y, 0.0);
    /// let ray = Ray3D::new(Vec3::new(0.0, -5.0, 0.0), Vec3::Y);
    /// assert!((plane.intersects_ray(&ray).unwrap() - 5.0).abs() < 1e-6);
    /// ```
    pub fn intersects_ray(self, ray: &Ray3D) -> Option<f32> {
        let denom = self.normal.dot(ray.direction);
        if denom.abs() <= f32::EPSILON {
            return None;
        }
        let t = -(self.normal.dot(ray.origin) + self.distance) / denom;
        Some(t)
    }
}

/// A view frustum defined by six planes (left, right, top, bottom, near, far).
///
/// Frustum planes are extracted from a view-projection matrix using the
/// Gribb-Hartmann method. All planes have their normals pointing inward, so
/// [`contains_aabb`](Self::contains_aabb) checks whether an AABB is on the
/// positive side of every plane.
///
/// # Examples
///
/// ```rust
/// use rotex_math::{Frustum, Aabb, Mat4, Vec3};
///
/// let vp = Mat4::perspective_rh_gl(70.0_f32.to_radians(), 1.0, 0.1, 100.0)
///     * Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
/// let frustum = Frustum::from_view_proj(vp);
///
/// let inside = Aabb::from_min_max(Vec3::splat(-0.5), Vec3::splat(0.5));
/// assert!(frustum.contains_aabb(&inside));
///
/// let outside = Aabb::from_min_max(Vec3::splat(200.0), Vec3::splat(300.0));
/// assert!(!frustum.contains_aabb(&outside));
/// ```
#[derive(Debug, Clone)]
pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    /// Extract frustum planes from a Vulkan / WGPU-style projection matrix.
    ///
    /// This uses the Gribb-Hartmann method and assumes a depth range of
    /// `[0, 1]` (the Vulkan convention).
    pub fn from_view_proj_vk(vp: Mat4) -> Self {
        Self {
            planes: [
                Plane::new(vp.row(3).truncate() + vp.row(0).truncate(), vp.row(3).w + vp.row(0).w).normalize(),
                Plane::new(vp.row(3).truncate() - vp.row(0).truncate(), vp.row(3).w - vp.row(0).w).normalize(),
                Plane::new(vp.row(3).truncate() + vp.row(1).truncate(), vp.row(3).w + vp.row(1).w).normalize(),
                Plane::new(vp.row(3).truncate() - vp.row(1).truncate(), vp.row(3).w - vp.row(1).w).normalize(),
                Plane::new(vp.row(2).truncate(), vp.row(2).w).normalize(),
                Plane::new(vp.row(3).truncate() - vp.row(2).truncate(), vp.row(3).w - vp.row(2).w).normalize(),
            ],
        }
    }

    /// Extract frustum planes from a view-projection matrix.
    ///
    /// RHI-agnostic alias for [`from_view_proj_vk`](Self::from_view_proj_vk).
    /// Currently both methods use the same Vulkan depth convention.
    pub fn from_view_proj(vp: Mat4) -> Self {
        Self::from_view_proj_vk(vp)
    }

    /// Returns `true` if the AABB is inside (or intersecting) the frustum.
    ///
    /// Uses the p-vertex test: for each frustum plane, the AABB corner that
    /// is most inside is tested. If any plane rejects that corner, the AABB
    /// is considered outside.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotex_math::{Frustum, Aabb, Mat4, Vec3};
    ///
    /// let vp = Mat4::perspective_rh_gl(70.0_f32.to_radians(), 1.0, 0.1, 100.0)
    ///     * Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    /// let frustum = Frustum::from_view_proj(vp);
    ///
    /// let aabb = Aabb::from_min_max(Vec3::splat(-0.5), Vec3::splat(0.5));
    /// assert!(frustum.contains_aabb(&aabb));
    /// ```
    pub fn contains_aabb(&self, aabb: &Aabb) -> bool {
        for plane in &self.planes {
            let mut p = aabb.min;
            if plane.normal.x >= 0.0 { p.x = aabb.max.x; }
            if plane.normal.y >= 0.0 { p.y = aabb.max.y; }
            if plane.normal.z >= 0.0 { p.z = aabb.max.z; }
            if plane.distance_to_point(p) < 0.0 {
                return false;
            }
        }
        true
    }

    /// Minimum batch size before SIMD dispatch is worthwhile.
    /// Below this, the scalar loop's absence of packing overhead wins.
    ///
    /// Measured crossover (AVX2, default build):
    ///   4096 → 8 µs scalar, 11 µs x4       — scalar wins
    ///   8192 → 51 µs scalar, 22 µs x4      — x4 wins (2.3×)
    /// The ~6× scalar jump from 4096→8192 is a cache-capacity cliff:
    /// 8192 AABBs × 32 bytes ≈ 256 KB, exceeding L2 on most client CPUs.
    /// SIMD's denser memory access pattern avoids this cliff.
    #[cfg(feature = "wide-simd")]
    const DISPATCH_THRESHOLD: usize = 8192;

    /// Batched AABB-frustum test, dispatching to the widest SIMD path
    /// supported by the current CPU. Falls back to scalar for tail elements.
    ///
    /// This method is only available with the `wide-simd` feature enabled.
    ///
    /// # Panics
    ///
    /// Panics if `aabbs.len() != out.len()` in debug builds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotex_math::{Frustum, Aabb, Mat4, Vec3};
    ///
    /// let vp = Mat4::perspective_rh_gl(70.0_f32.to_radians(), 1.0, 0.1, 100.0)
    ///     * Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    /// let frustum = Frustum::from_view_proj(vp);
    ///
    /// let aabbs = vec![Aabb::from_min_max(Vec3::splat(-0.5), Vec3::splat(0.5))];
    /// let mut out = vec![false; aabbs.len()];
    /// frustum.contains_aabb_batch(&aabbs, &mut out);
    /// assert!(out[0]);
    /// ```
    #[cfg(feature = "wide-simd")]
    pub fn contains_aabb_batch(&self, aabbs: &[Aabb], out: &mut [bool]) {
        debug_assert_eq!(aabbs.len(), out.len());

        if aabbs.len() < Self::DISPATCH_THRESHOLD {
            for (a, o) in aabbs.iter().zip(out.iter_mut()) {
                *o = self.contains_aabb(a);
            }
            return;
        }

        match crate::wide_simd::width::detected_width() {
            crate::wide_simd::width::SimdWidth::X8 => {
                crate::wide_simd::x8::contains_aabb_batch(self, aabbs, out);
            }
            crate::wide_simd::width::SimdWidth::X4 => {
                crate::wide_simd::x4::contains_aabb_batch(self, aabbs, out);
            }
            crate::wide_simd::width::SimdWidth::Scalar => {
                for (a, o) in aabbs.iter().zip(out.iter_mut()) {
                    *o = self.contains_aabb(a);
                }
            }
        }
    }
}
