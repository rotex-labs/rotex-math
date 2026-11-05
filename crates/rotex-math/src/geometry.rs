use crate::{Vec2, Vec3};

/// A 3D axis-aligned bounding box defined by its minimum and maximum corners.
///
/// The box includes all points `p` where `min ≤ p ≤ max` (component-wise).
///
/// # Examples
///
/// ```rust
/// use rotex_math::{Aabb, Vec3};
///
/// let aabb = Aabb::from_min_max(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
/// assert!(aabb.contains(Vec3::ZERO));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    /// Corner with the smallest x, y, and z.
    pub min: Vec3,
    /// Corner with the largest x, y, and z.
    pub max: Vec3,
}

impl Aabb {
    /// Construct an AABB from two arbitrary corner points.
    ///
    /// The input order does not matter — `min` and `max` are computed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotex_math::{Aabb, Vec3};
    /// let aabb = Aabb::from_min_max(Vec3::new(1.0, 1.0, 1.0), Vec3::new(-1.0, -1.0, -1.0));
    /// assert_eq!(aabb.min, Vec3::splat(-1.0));
    /// assert_eq!(aabb.max, Vec3::splat(1.0));
    /// ```
    pub fn from_min_max(p1: Vec3, p2: Vec3) -> Self {
        Self {
            min: p1.min(p2),
            max: p1.max(p2),
        }
    }

    /// Create an infinite empty AABB, useful as an accumulator for [`merge`].
    ///
    /// The constructed AABB has `min = MAX` and `max = MIN` so that any
    /// finite AABB merged with it produces that finite AABB.
    ///
    /// [`merge`]: Self::merge
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotex_math::{Aabb, Vec3};
    /// let empty = Aabb::empty();
    /// let a = Aabb::from_min_max(Vec3::splat(-1.0), Vec3::splat(1.0));
    /// assert_eq!(empty.merge(a).min, a.min);
    /// assert_eq!(empty.merge(a).max, a.max);
    /// ```
    pub fn empty() -> Self {
        Self {
            min: Vec3::splat(f32::MAX),
            max: Vec3::splat(f32::MIN),
        }
    }

    /// Compute the union of two AABBs — the smallest AABB containing both.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotex_math::{Aabb, Vec3};
    /// let a = Aabb::from_min_max(Vec3::new(-2.0, 0.0, 0.0), Vec3::new(-1.0, 1.0, 1.0));
    /// let b = Aabb::from_min_max(Vec3::new(1.0, -1.0, 0.0), Vec3::new(2.0, 0.0, 1.0));
    /// let u = a.merge(b);
    /// assert_eq!(u.min, Vec3::new(-2.0, -1.0, 0.0));
    /// assert_eq!(u.max, Vec3::new(2.0, 1.0, 1.0));
    /// ```
    pub fn merge(self, other: Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Expand this AABB to include a point.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotex_math::{Aabb, Vec3};
    /// let mut aabb = Aabb::from_min_max(Vec3::splat(-1.0), Vec3::splat(1.0));
    /// aabb.expand_to_include(Vec3::new(5.0, 0.0, 0.0));
    /// assert_eq!(aabb.max.x, 5.0);
    /// ```
    pub fn expand_to_include(&mut self, point: Vec3) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
    }

    /// Returns `true` if the point is inside the AABB (or on its boundary).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotex_math::{Aabb, Vec3};
    /// let aabb = Aabb::from_min_max(Vec3::splat(-1.0), Vec3::splat(1.0));
    /// assert!(aabb.contains(Vec3::ZERO));
    /// assert!(!aabb.contains(Vec3::new(2.0, 0.0, 0.0)));
    /// ```
    pub fn contains(self, point: Vec3) -> bool {
        point.cmpge(self.min).all() && point.cmple(self.max).all()
    }

    /// Returns the geometric center of the AABB.
    ///
    /// Uses `min + (max - min) * 0.5` to avoid overflow for extreme values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotex_math::{Aabb, Vec3};
    /// let aabb = Aabb::from_min_max(Vec3::splat(-2.0), Vec3::splat(2.0));
    /// assert_eq!(aabb.center(), Vec3::ZERO);
    /// ```
    pub fn center(&self) -> Vec3 {
        self.min + (self.max - self.min) * 0.5
    }

    /// Returns `true` if the other AABB is fully contained within this one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotex_math::{Aabb, Vec3};
    /// let outer = Aabb::from_min_max(Vec3::splat(-2.0), Vec3::splat(2.0));
    /// let inner = Aabb::from_min_max(Vec3::splat(-1.0), Vec3::splat(1.0));
    /// assert!(outer.contains_aabb(&inner));
    /// assert!(!inner.contains_aabb(&outer));
    /// ```
    pub fn contains_aabb(&self, other: &Aabb) -> bool {
        self.min.cmple(other.min).all() && self.max.cmpge(other.max).all()
    }

    /// Returns the distance `t` along the ray at which it first
    /// intersects this AABB, or `None` if there is no intersection.
    ///
    /// Uses the slab method. Returns `None` when the AABB lies entirely behind
    /// the ray origin, or when the ray is parallel and misses.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotex_math::{Aabb, Ray3D, Vec3};
    /// let aabb = Aabb::from_min_max(Vec3::splat(-1.0), Vec3::splat(1.0));
    /// let ray = Ray3D::new(Vec3::new(0.0, 0.0, -5.0), Vec3::Z);
    /// let t = ray.intersects_aabb(&aabb);
    /// assert!(t.is_some());
    /// assert!(t.unwrap() > 0.0);
    /// ```
    pub fn intersects_ray(self, ray: &Ray3D) -> Option<f32> {
        let inv_dir = Vec3::ONE / ray.direction;
        let t1 = (self.min - ray.origin) * inv_dir;
        let t2 = (self.max - ray.origin) * inv_dir;
        let t_min = t1.min(t2).max_element();
        let t_max = t1.max(t2).min_element();
        if t_max < 0.0 || t_min > t_max {
            None
        } else {
            Some(t_min)
        }
    }
}

/// A 2D axis-aligned bounding box.
///
/// The 2D counterpart of [`Aabb`]. All methods behave identically but operate
/// on [`Vec2`] instead of [`Vec3`].
///
/// # Examples
///
/// ```rust
/// use rotex_math::{Aabb2, Vec2};
/// let aabb = Aabb2::from_min_max(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0));
/// assert!(aabb.contains(Vec2::ZERO));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Aabb2 {
    /// Corner with the smallest x and y.
    pub min: Vec2,
    /// Corner with the largest x and y.
    pub max: Vec2,
}

impl Aabb2 {
    /// Construct an AABB2 from two arbitrary corner points. Input order does not matter.
    pub fn from_min_max(p1: Vec2, p2: Vec2) -> Self {
        Self {
            min: p1.min(p2),
            max: p1.max(p2),
        }
    }

    /// Create an infinitely small empty AABB2, useful as an accumulator.
    pub fn empty() -> Self {
        Self {
            min: Vec2::splat(f32::MAX),
            max: Vec2::splat(f32::MIN),
        }
    }

    /// Compute the union of two AABB2s.
    pub fn merge(self, other: Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Expand this AABB2 to include a point.
    pub fn expand_to_include(&mut self, point: Vec2) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
    }

    /// Returns `true` if the point is inside this AABB2 (or on its boundary).
    pub fn contains(self, point: Vec2) -> bool {
        point.cmpge(self.min).all() && point.cmple(self.max).all()
    }

    /// Returns the distance `t` along the ray at which it first
    /// intersects this AABB2, or `None` if there is no intersection.
    ///
    /// Uses the slab method in 2D. See [`Aabb::intersects_ray`] for details.
    pub fn intersects_ray(self, ray: &Ray2D) -> Option<f32> {
        let inv_dir = Vec2::ONE / ray.direction;
        let t1 = (self.min - ray.origin) * inv_dir;
        let t2 = (self.max - ray.origin) * inv_dir;
        let t_min = t1.min(t2).max_element();
        let t_max = t1.max(t2).min_element();
        if t_max < 0.0 || t_min > t_max {
            None
        } else {
            Some(t_min)
        }
    }
}

/// A 3D ray defined by an origin and a direction vector.
///
/// The direction is **not** required to be normalized, but intersection
/// distances scale with the reciprocal of the direction length.
///
/// # Examples
///
/// ```rust
/// use rotex_math::{Ray3D, Vec3};
/// let ray = Ray3D::new(Vec3::new(0.0, 0.0, -5.0), Vec3::Z);
/// assert_eq!(ray.at(5.0), Vec3::ZERO);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Ray3D {
    /// Point where the parameter `t` is `0`.
    pub origin: Vec3,
    /// Direction multiplied by `t`. Not required to be a unit vector.
    pub direction: Vec3,
}

impl Ray3D {
    /// Construct a ray from an origin and direction.
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Evaluate the ray at parameter `t`: `origin + direction * t`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotex_math::{Ray3D, Vec3};
    /// let ray = Ray3D::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0));
    /// assert_eq!(ray.at(0.5), Vec3::new(1.0, 1.0, 0.0));
    /// ```
    pub fn at(self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    /// Returns the distance `t` along the ray at which it intersects the
    /// AABB, or `None` if there is no intersection.
    ///
    /// Delegates to [`Aabb::intersects_ray`].
    pub fn intersects_aabb(self, aabb: &Aabb) -> Option<f32> {
        aabb.intersects_ray(&self)
    }

    /// Translate the ray by `offset`, preserving direction.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rotex_math::{Ray3D, Vec3};
    /// let ray = Ray3D::new(Vec3::ZERO, Vec3::X);
    /// let shifted = ray.translated(Vec3::new(0.0, 1.0, 0.0));
    /// assert_eq!(shifted.origin, Vec3::new(0.0, 1.0, 0.0));
    /// assert_eq!(shifted.direction, Vec3::X);
    /// ```
    pub fn translated(self, offset: Vec3) -> Self {
        Self {
            origin: self.origin + offset,
            direction: self.direction,
        }
    }
}

/// A 2D ray. The 2D counterpart of [`Ray3D`].
#[derive(Debug, Clone, Copy)]
pub struct Ray2D {
    /// Point where the parameter `t` is `0`.
    pub origin: Vec2,
    /// Direction multiplied by `t`. Not required to be a unit vector.
    pub direction: Vec2,
}

impl Ray2D {
    /// Construct a 2D ray from an origin and direction.
    pub fn new(origin: Vec2, direction: Vec2) -> Self {
        Self { origin, direction }
    }

    /// Evaluate the ray at parameter `t`: `origin + direction * t`.
    pub fn at(self, t: f32) -> Vec2 {
        self.origin + self.direction * t
    }

    /// Returns the distance `t` along the ray at which it intersects the
    /// AABB2, or `None` if there is no intersection.
    pub fn intersects_aabb(self, aabb: &Aabb2) -> Option<f32> {
        aabb.intersects_ray(&self)
    }
}

/// A 3D line segment defined by two endpoints.
#[derive(Debug, Clone, Copy)]
pub struct LineSegment3D {
    /// First endpoint.
    pub a: Vec3,
    /// Second endpoint.
    pub b: Vec3,
}

impl LineSegment3D {
    /// Construct a line segment between two points.
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Self { a, b }
    }
}

/// A 2D line segment. The 2D counterpart of [`LineSegment3D`].
#[derive(Debug, Clone, Copy)]
pub struct LineSegment2D {
    /// First endpoint.
    pub a: Vec2,
    /// Second endpoint.
    pub b: Vec2,
}

impl LineSegment2D {
    /// Construct a 2D line segment between two points.
    pub fn new(a: Vec2, b: Vec2) -> Self {
        Self { a, b }
    }
}
