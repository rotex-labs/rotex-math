#![cfg(feature = "wide-simd")]

mod common;

use common::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn batch_matches_scalar(
        frustum in frustum_strategy(),
        aabbs in prop::collection::vec(aabb_strategy(), 0..50),
    ) {
        let mut batch_out = vec![false; aabbs.len()];
        frustum.contains_aabb_batch(&aabbs, &mut batch_out);
        let scalar_out: Vec<bool> = aabbs.iter().map(|a| frustum.contains_aabb(a)).collect();
        prop_assert_eq!(batch_out, scalar_out);
    }
}
