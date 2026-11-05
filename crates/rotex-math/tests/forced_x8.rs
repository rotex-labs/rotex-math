#![cfg(feature = "wide-simd")]

mod common;

use common::*;
use proptest::prelude::*;
use rotex_math::wide_simd::x8;

proptest! {
    #[test]
    fn matches_scalar(
        frustum in frustum_strategy(),
        aabbs in prop::collection::vec(aabb_strategy(), 0..20),
    ) {
        let mut out = vec![false; aabbs.len()];
        x8::contains_aabb_batch(&frustum, &aabbs, &mut out);
        let scalar: Vec<bool> = aabbs.iter().map(|a| frustum.contains_aabb(a)).collect();
        prop_assert_eq!(out, scalar);
    }
}
