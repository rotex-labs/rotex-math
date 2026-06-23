mod common;

use common::*;
use rotex_math::Vec3;
use proptest::prelude::*;
use rotex_math::geometry::*;
use rotex_math::frustum::Plane;

proptest! {
    #[test]
    fn aabb_contains_its_own_center(a in aabb_positive_strategy()) {
        prop_assert!(a.contains(a.center()));
    }

    #[test]
    fn aabb_union_contains_both_inputs(a in aabb_strategy(), b in aabb_strategy()) {
        let u = a.merge(b);
        prop_assert!(u.contains_aabb(&a));
        prop_assert!(u.contains_aabb(&b));
    }

    #[test]
    fn aabb_empty_contains_nothing(p in vec3_strategy()) {
        let empty = Aabb::empty();
        prop_assert!(!empty.contains(p));
    }

    #[test]
    fn aabb_contains_its_corners(a in aabb_positive_strategy()) {
        prop_assert!(a.contains(a.min));
        prop_assert!(a.contains(a.max));
        prop_assert!(a.contains(Vec3::new(a.min.x, a.min.y, a.max.z)));
        prop_assert!(a.contains(Vec3::new(a.max.x, a.min.y, a.min.z)));
    }

    #[test]
    fn plane_distance_sign(p in plane_strategy(), _offset in vec3_strategy()) {
        let origin_plane = Plane::from_point_normal(Vec3::ZERO, p.normal);
        let on_plane = origin_plane.normal * (-origin_plane.distance);
        let d = origin_plane.distance_to_point(on_plane);
        prop_assert!(d.abs() < 1e-5);

        let front = on_plane + p.normal;
        prop_assert!(origin_plane.distance_to_point(front) > 0.0);

        let back = on_plane - p.normal;
        prop_assert!(origin_plane.distance_to_point(back) < 0.0);
    }

    #[test]
    fn plane_normalize_preserves_side(p in plane_strategy(), point in vec3_strategy()) {
        let n = p.normalize();
        let sign_before = p.distance_to_point(point).signum();
        let sign_after = n.distance_to_point(point).signum();
        prop_assert_eq!(sign_before, sign_after);
    }

    #[test]
    fn plane_intersects_ray_parallel(p in plane_strategy(), origin in vec3_strategy()) {
        let dir = p.normal.cross(Vec3::new(1.0, 0.37, 0.0));
        let len = dir.length();
        if len > f32::EPSILON {
            let ray = Ray3D::new(origin, dir / len);
            let denom = p.normal.dot(ray.direction).abs();
            if denom < 1e-4 {
                prop_assert!(p.intersects_ray(&ray).is_none());
            }
        }
    }

    #[test]
    fn ray_aabb_intersection_translation_invariant(
        ray in ray_strategy(),
        aabb in aabb_positive_strategy(),
        offset in vec3_strategy(),
    ) {
        let result_a = ray.intersects_aabb(&aabb);
        let shifted_ray = ray.translated(offset);
        let shifted_aabb = Aabb::from_min_max(aabb.min + offset, aabb.max + offset);
        let result_b = shifted_ray.intersects_aabb(&shifted_aabb);
        if let Some(a) = result_a { prop_assert!(a.is_finite()); }
        if let Some(b) = result_b { prop_assert!(b.is_finite()); }
    }

    #[test]
    fn frustum_contains_known_inside_aabb(frustum in frustum_strategy()) {
        let inside = Aabb::from_min_max(Vec3::new(-0.01, -0.01, 0.5), Vec3::new(0.01, 0.01, 0.8));
        prop_assert!(frustum.contains_aabb(&inside));
    }

    #[test]
    fn frustum_large_straddling_aabb_is_visible(frustum in frustum_strategy()) {
        let straddle = Aabb::from_min_max(Vec3::splat(-10.0), Vec3::splat(10.0));
        prop_assert!(frustum.contains_aabb(&straddle));
    }

    #[test]
    fn frustum_excludes_known_outside_aabb(frustum in frustum_strategy()) {
        let outside = Aabb::from_min_max(Vec3::splat(100.0), Vec3::splat(200.0));
        prop_assert!(!frustum.contains_aabb(&outside));
    }
}
