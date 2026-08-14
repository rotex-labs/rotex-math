#![allow(dead_code)]

use proptest::prelude::*;
use rotex_math::frustum::{Frustum, Plane};
use rotex_math::geometry::{Aabb, Ray3D};
use rotex_math::{Mat4, Vec3};

pub fn vec3_strategy() -> impl Strategy<Value = Vec3> {
    (any::<f32>(), any::<f32>(), any::<f32>()).prop_map(|(x, y, z)| Vec3::new(x, y, z))
}

pub fn aabb_strategy() -> impl Strategy<Value = Aabb> {
    (vec3_strategy(), vec3_strategy()).prop_map(|(a, b)| Aabb::from_min_max(a, b))
}

pub fn aabb_positive_strategy() -> impl Strategy<Value = Aabb> {
    (vec3_strategy(), 0.001f32..=100.0).prop_map(|(center, half)| {
        Aabb::from_min_max(center - Vec3::splat(half), center + Vec3::splat(half))
    })
}

pub fn ray_strategy() -> impl Strategy<Value = Ray3D> {
    (-1.0f32..=1.0, -1.0f32..=1.0, -1.0f32..=1.0)
        .prop_map(|(x, y, z)| {
            let d = Vec3::new(x, y, z);
            let len = d.length();
            if len < f32::EPSILON {
                Vec3::new(1.0, 1.0, 1.0).normalize()
            } else {
                d / len
            }
        })
        .prop_flat_map(|dir| vec3_strategy().prop_map(move |o| Ray3D::new(o, dir)))
}

pub fn plane_strategy() -> impl Strategy<Value = Plane> {
    let comp = -1.0f32..=1.0;
    let distance = -100.0f32..=100.0;
    (comp.clone(), comp.clone(), comp, distance)
        .prop_filter("non-zero normal", |(x, y, z, _)| {
            (*x).abs() + (*y).abs() + (*z).abs() > f32::EPSILON
        })
        .prop_map(|(x, y, z, d)| {
            let n = Vec3::new(x, y, z);
            Plane::new(n / n.length(), d)
        })
}

pub fn frustum_strategy() -> impl Strategy<Value = Frustum> {
    (30.0f32..=120.0, 0.01f32..=0.5, 10.0f32..=100.0).prop_map(|(fov_y, z_near, z_far)| {
        let proj = Mat4::perspective_rh_gl(fov_y.to_radians(), 1.0, z_near, z_far);
        let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
        Frustum::from_view_proj(proj * view)
    })
}
