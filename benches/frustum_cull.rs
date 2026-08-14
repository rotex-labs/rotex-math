use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::RngExt;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rotex_math::frustum::Frustum;
use rotex_math::geometry::Aabb;
use rotex_math::{Mat4, Vec3};
use std::hint::black_box;

const COUNTS: &[usize] = &[8, 16, 64, 256, 512, 1024, 2048, 4096, 8192, 65536];

fn generate_stub_aabbs(count: usize, seed: u64) -> Vec<Aabb> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..count)
        .map(|_| {
            let center = Vec3::new(
                rng.random_range(-1000.0..1000.0),
                rng.random_range(-1000.0..1000.0),
                rng.random_range(-1000.0..1000.0),
            );
            let half = rng.random_range(0.1..50.0);
            Aabb::from_min_max(center - Vec3::splat(half), center + Vec3::splat(half))
        })
        .collect()
}

fn test_frustum() -> Frustum {
    let proj = Mat4::perspective_rh_gl(70.0_f32.to_radians(), 1.0, 0.1, 100.0);
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    Frustum::from_view_proj(proj * view)
}

fn bench_frustum_culling(c: &mut Criterion) {
    let frustum = test_frustum();
    let mut group = c.benchmark_group("frustum_cull/scalar");

    for &count in COUNTS {
        let aabbs = generate_stub_aabbs(count, 42);
        group.bench_with_input(BenchmarkId::from_parameter(count), &aabbs, |b, aabbs| {
            b.iter(|| {
                aabbs
                    .iter()
                    .map(|a| frustum.contains_aabb(black_box(a)))
                    .collect::<Vec<_>>()
            });
        });
    }
    group.finish();
}

#[cfg(feature = "wide-simd")]
fn bench_wide_simd(c: &mut Criterion) {
    let frustum = test_frustum();

    {
        let mut group = c.benchmark_group("frustum_cull/x4");
        for &count in COUNTS {
            let aabbs = generate_stub_aabbs(count, 42);
            group.bench_with_input(BenchmarkId::from_parameter(count), &aabbs, |b, aabbs| {
                let mut out = vec![false; aabbs.len()];
                b.iter(|| {
                    rotex_math::wide_simd::x4::contains_aabb_batch(
                        black_box(&frustum),
                        aabbs,
                        &mut out,
                    );
                });
            });
        }
        group.finish();
    }

    {
        let mut group = c.benchmark_group("frustum_cull/x8");
        for &count in COUNTS {
            let aabbs = generate_stub_aabbs(count, 42);
            group.bench_with_input(BenchmarkId::from_parameter(count), &aabbs, |b, aabbs| {
                let mut out = vec![false; aabbs.len()];
                b.iter(|| {
                    rotex_math::wide_simd::x8::contains_aabb_batch(
                        black_box(&frustum),
                        aabbs,
                        &mut out,
                    );
                });
            });
        }
        group.finish();
    }

    {
        let mut group = c.benchmark_group("frustum_cull/dispatch");
        for &count in COUNTS {
            let aabbs = generate_stub_aabbs(count, 42);
            group.bench_with_input(BenchmarkId::from_parameter(count), &aabbs, |b, aabbs| {
                let mut out = vec![false; aabbs.len()];
                b.iter(|| frustum.contains_aabb_batch(black_box(aabbs), &mut out));
            });
        }
        group.finish();
    }
}

criterion_group!(scalar, bench_frustum_culling);

#[cfg(feature = "wide-simd")]
criterion_group!(wide, bench_wide_simd);

#[cfg(feature = "wide-simd")]
criterion_main!(scalar, wide);
#[cfg(not(feature = "wide-simd"))]
criterion_main!(scalar);
