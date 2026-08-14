use crate::frustum::Frustum;
use crate::geometry::Aabb;
use wide::f32x8;

#[target_feature(enable = "avx2")]
unsafe fn contains_aabb_batch_impl(frustum: &Frustum, aabbs: &[Aabb], out: &mut [bool]) {
    debug_assert_eq!(aabbs.len(), out.len());

    let mut i = 0;
    while i + 8 <= aabbs.len() {
        let mut mx = [0.0; 8];
        let mut my = [0.0; 8];
        let mut mz = [0.0; 8];
        let mut kx = [0.0; 8];
        let mut ky = [0.0; 8];
        let mut kz = [0.0; 8];

        for j in 0..8 {
            mx[j] = aabbs[i + j].min.x;
            my[j] = aabbs[i + j].min.y;
            mz[j] = aabbs[i + j].min.z;
            kx[j] = aabbs[i + j].max.x;
            ky[j] = aabbs[i + j].max.y;
            kz[j] = aabbs[i + j].max.z;
        }

        let min_x = f32x8::new(mx);
        let min_y = f32x8::new(my);
        let min_z = f32x8::new(mz);
        let max_x = f32x8::new(kx);
        let max_y = f32x8::new(ky);
        let max_z = f32x8::new(kz);

        let zero = f32x8::splat(0.0);
        let mut inside = f32x8::splat(-1.0);

        for plane in &frustum.planes {
            let nx = f32x8::splat(plane.normal.x);
            let ny = f32x8::splat(plane.normal.y);
            let nz = f32x8::splat(plane.normal.z);
            let d = f32x8::splat(plane.distance);

            let px = nx.simd_ge(zero).blend(max_x, min_x);
            let py = ny.simd_ge(zero).blend(max_y, min_y);
            let pz = nz.simd_ge(zero).blend(max_z, min_z);

            let dist = nx * px + ny * py + nz * pz + d;
            inside = inside & dist.simd_ge(zero);
        }

        let mask = inside.to_bitmask();
        for j in 0..8 {
            out[i + j] = (mask & (1 << j)) != 0;
        }

        i += 8;
    }

    for (a, o) in aabbs[i..].iter().zip(out[i..].iter_mut()) {
        *o = frustum.contains_aabb(a);
    }
}

/// 8-wide SIMD batch AABB-frustum culling (AVX2).
///
/// Processes AABBs in groups of eight using AVX2 intrinsics. On CPUs without
/// AVX2, falls back to [`x4::contains_aabb_batch`](crate::wide_simd::x4).
///
/// The inner implementation is decorated with
/// `#[target_feature(enable = "avx2")]` and is only called after a runtime
/// `is_x86_feature_detected!("avx2")` check.
///
/// Prefer calling [`Frustum::contains_aabb_batch`] instead, which
/// automatically selects the widest SIMD path available.
///
/// # Panics
///
/// Panics if `aabbs.len() != out.len()` in debug builds.
pub fn contains_aabb_batch(frustum: &Frustum, aabbs: &[Aabb], out: &mut [bool]) {
    #[cfg(target_arch = "x86_64")]
    {
        if std::is_x86_feature_detected!("avx2") {
            return unsafe { contains_aabb_batch_impl(frustum, aabbs, out) };
        }
    }
    // Fallback: if no AVX2, f32x8 ops use SSE2 emulation (slower than native x4).
    crate::wide_simd::x4::contains_aabb_batch(frustum, aabbs, out);
}
