use crate::geometry::Aabb;
use crate::frustum::Frustum;
use wide::f32x4;

/// 4-wide SIMD batch AABB-frustum culling.
///
/// Processes AABBs in groups of four using SSE2 (x86_64) or NEON (aarch64).
/// Elements beyond the last full group of four fall back to the scalar
/// [`Frustum::contains_aabb`].
///
/// Prefer calling [`Frustum::contains_aabb_batch`] instead, which
/// automatically selects the widest SIMD path available.
///
/// # Panics
///
/// Panics if `aabbs.len() != out.len()` in debug builds.
pub fn contains_aabb_batch(frustum: &Frustum, aabbs: &[Aabb], out: &mut [bool]) {
    debug_assert_eq!(aabbs.len(), out.len());

    let mut i = 0;
    while i + 4 <= aabbs.len() {
        let mx = [
            aabbs[i].min.x, aabbs[i + 1].min.x, aabbs[i + 2].min.x, aabbs[i + 3].min.x,
        ];
        let my = [
            aabbs[i].min.y, aabbs[i + 1].min.y, aabbs[i + 2].min.y, aabbs[i + 3].min.y,
        ];
        let mz = [
            aabbs[i].min.z, aabbs[i + 1].min.z, aabbs[i + 2].min.z, aabbs[i + 3].min.z,
        ];
        let kx = [
            aabbs[i].max.x, aabbs[i + 1].max.x, aabbs[i + 2].max.x, aabbs[i + 3].max.x,
        ];
        let ky = [
            aabbs[i].max.y, aabbs[i + 1].max.y, aabbs[i + 2].max.y, aabbs[i + 3].max.y,
        ];
        let kz = [
            aabbs[i].max.z, aabbs[i + 1].max.z, aabbs[i + 2].max.z, aabbs[i + 3].max.z,
        ];

        let min_x = f32x4::new(mx);
        let min_y = f32x4::new(my);
        let min_z = f32x4::new(mz);
        let max_x = f32x4::new(kx);
        let max_y = f32x4::new(ky);
        let max_z = f32x4::new(kz);

        let zero = f32x4::splat(0.0);
        let mut inside = f32x4::splat(-1.0);

        for plane in &frustum.planes {
            let nx = f32x4::splat(plane.normal.x);
            let ny = f32x4::splat(plane.normal.y);
            let nz = f32x4::splat(plane.normal.z);
            let d = f32x4::splat(plane.distance);

            let px = nx.simd_ge(zero).blend(max_x, min_x);
            let py = ny.simd_ge(zero).blend(max_y, min_y);
            let pz = nz.simd_ge(zero).blend(max_z, min_z);

            let dist = nx * px + ny * py + nz * pz + d;
            inside = inside & dist.simd_ge(zero);
        }

        let mask = inside.to_bitmask();
        out[i] = (mask & 1) != 0;
        out[i + 1] = (mask & 2) != 0;
        out[i + 2] = (mask & 4) != 0;
        out[i + 3] = (mask & 8) != 0;

        i += 4;
    }

    for (a, o) in aabbs[i..].iter().zip(out[i..].iter_mut()) {
        *o = frustum.contains_aabb(a);
    }
}
