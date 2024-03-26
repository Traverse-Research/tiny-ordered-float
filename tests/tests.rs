#![allow(clippy::float_cmp, clippy::eq_op, clippy::op_ref)]
use tiny_ordered_float::{OrderedF32, OrderedF64};

pub use std::cmp::Ordering::*;
pub use std::convert::TryFrom;
pub use std::{f32, panic};

pub use std::collections::hash_map::RandomState;
pub use std::collections::HashSet;
pub use std::hash::*;

#[test]
fn test_total_order() {
    let numberline = [
        (-f32::INFINITY, 0),
        (-1.0, 1),
        (-0.0, 2),
        (0.0, 2),
        (1.0, 3),
        (f32::INFINITY, 4),
        (f32::NAN, 5),
        (-f32::NAN, 5),
    ];

    for &(fi, i) in &numberline {
        for &(fj, j) in &numberline {
            assert_eq!(OrderedF32(fi) < OrderedF32(fj), i < j);
            assert_eq!(OrderedF32(fi) > OrderedF32(fj), i > j);
            assert_eq!(OrderedF32(fi) <= OrderedF32(fj), i <= j);
            assert_eq!(OrderedF32(fi) >= OrderedF32(fj), i >= j);
            assert_eq!(OrderedF32(fi) == OrderedF32(fj), i == j);
            assert_eq!(OrderedF32(fi) != OrderedF32(fj), i != j);
            assert_eq!(OrderedF32(fi).cmp(&OrderedF32(fj)), i.cmp(&j));
        }
    }
}

#[test]
fn ordered_f64_compare_regular_floats() {
    assert_eq!(OrderedF64(7.0f64).cmp(&OrderedF64(7.0)), Equal);
    assert_eq!(OrderedF64(8.0f64).cmp(&OrderedF64(7.0)), Greater);
    assert_eq!(OrderedF64(4.0f64).cmp(&OrderedF64(7.0)), Less);
}

#[test]
fn ordered_f64_compare_regular_floats_op() {
    assert!(OrderedF64(7.0f64) == OrderedF64(7.0));
    assert!(OrderedF64(7.0f64) <= OrderedF64(7.0));
    assert!(OrderedF64(7.0f64) >= OrderedF64(7.0));
    assert!(OrderedF64(8.0f64) > OrderedF64(7.0));
    assert!(OrderedF64(8.0f64) >= OrderedF64(7.0));
    assert!(OrderedF64(4.0f64) < OrderedF64(7.0));
    assert!(OrderedF64(4.0f64) <= OrderedF64(7.0));
}

#[test]
fn ordered_f64_compare_nan() {
    let f64_nan: f64 = f64::NAN;
    assert_eq!(OrderedF64(f64_nan).cmp(&OrderedF64(f64::NAN)), Equal);
    assert_eq!(OrderedF64(f64_nan).cmp(&OrderedF64(-100000.0f64)), Greater);
    assert_eq!(OrderedF64(-100.0f64).cmp(&OrderedF64(f64::NAN)), Less);
}

#[test]
fn ordered_f64_compare_nan_op() {
    let f64_nan: OrderedF64 = OrderedF64(f64::NAN);
    assert!(f64_nan == f64_nan);
    assert!(f64_nan <= f64_nan);
    assert!(f64_nan >= f64_nan);
    assert!(f64_nan > OrderedF64(-100000.0f64));
    assert!(f64_nan >= OrderedF64(-100000.0f64));
    assert!(OrderedF64(-100.0f64) < f64_nan);
    assert!(OrderedF64(-100.0f64) <= f64_nan);
    assert!(f64_nan > OrderedF64(f64::INFINITY));
    assert!(f64_nan >= OrderedF64(f64::INFINITY));
    assert!(f64_nan > OrderedF64(f64::NEG_INFINITY));
    assert!(f64_nan >= OrderedF64(f64::NEG_INFINITY));
}

#[test]
fn ordered_f32_compare_regular_floats() {
    assert_eq!(OrderedF32(7.0f32).cmp(&OrderedF32(7.0)), Equal);
    assert_eq!(OrderedF32(8.0f32).cmp(&OrderedF32(7.0)), Greater);
    assert_eq!(OrderedF32(4.0f32).cmp(&OrderedF32(7.0)), Less);
}

#[test]
fn ordered_f32_compare_nan() {
    let f32_nan: f32 = f32::NAN;
    assert_eq!(OrderedF32(f32_nan).cmp(&OrderedF32(f32::NAN)), Equal);
    assert_eq!(OrderedF32(f32_nan).cmp(&OrderedF32(-100000.0f32)), Greater);
    assert_eq!(OrderedF32(-100.0f32).cmp(&OrderedF32(f32::NAN)), Less);
}

#[test]
fn ordered_f32_compare_regular_floats_op() {
    assert!(OrderedF32(7.0) == OrderedF32(7.0));
    assert!(OrderedF32(7.0) <= OrderedF32(7.0));
    assert!(OrderedF32(7.0) >= OrderedF32(7.0));
    assert!(OrderedF32(8.0) > OrderedF32(7.0));
    assert!(OrderedF32(8.0) >= OrderedF32(7.0));
    assert!(OrderedF32(4.0) < OrderedF32(7.0));
    assert!(OrderedF32(4.0) <= OrderedF32(7.0));
}

#[test]
fn ordered_f32_compare_nan_op() {
    let f32_nan: OrderedF32 = OrderedF32(f32::NAN);
    assert!(f32_nan == f32_nan);
    assert!(f32_nan <= f32_nan);
    assert!(f32_nan >= f32_nan);
    assert!(f32_nan > OrderedF32(-100000.0));
    assert!(f32_nan >= OrderedF32(-100000.0));
    assert!(OrderedF32(-100.0) < f32_nan);
    assert!(OrderedF32(-100.0) <= f32_nan);
    assert!(f32_nan > OrderedF32(f32::INFINITY));
    assert!(f32_nan >= OrderedF32(f32::INFINITY));
    assert!(f32_nan > OrderedF32(f32::NEG_INFINITY));
    assert!(f32_nan >= OrderedF32(f32::NEG_INFINITY));
}

#[test]
fn hash_zero_and_neg_zero_to_the_same_hc_ordered_float64() {
    let state = RandomState::new();
    let mut h1 = state.build_hasher();
    let mut h2 = state.build_hasher();
    OrderedF32(0f32).hash(&mut h1);
    OrderedF32(-0f32).hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn hash_different_nans_to_the_same_hc() {
    let state = RandomState::new();
    let mut h1 = state.build_hasher();
    let mut h2 = state.build_hasher();
    OrderedF32(f32::NAN).hash(&mut h1);
    OrderedF32(-f32::NAN).hash(&mut h2);
    assert_eq!(h1.finish(), h2.finish());
}

#[test]
fn hash_inf_and_neg_inf_to_different_hcs() {
    let state = RandomState::new();
    let mut h1 = state.build_hasher();
    let mut h2 = state.build_hasher();
    OrderedF32(f32::INFINITY).hash(&mut h1);
    OrderedF32(f32::NEG_INFINITY).hash(&mut h2);
    assert!(h1.finish() != h2.finish());
}

#[test]
fn hash_is_good_for_whole_numbers() {
    let state = RandomState::new();
    let limit = 10000;

    let mut set = ::std::collections::HashSet::with_capacity(limit);
    for i in 0..limit {
        let mut h = state.build_hasher();
        OrderedF32(i as f32).hash(&mut h);
        set.insert(h.finish());
    }

    // This allows 100 collisions, which is far too
    // many, but should guard against transient issues
    // that will result from using RandomState
    let pct_unique = set.len() as f32 / limit as f32;
    assert!(0.99f32 < pct_unique, "percent-unique={}", pct_unique);
}

#[test]
fn hash_is_good_for_fractional_numbers() {
    let state = RandomState::new();
    let limit = 10000;

    let mut set = ::std::collections::HashSet::with_capacity(limit);
    for i in 0..limit {
        let mut h = state.build_hasher();
        OrderedF32(i as f32 * (1f32 / limit as f32)).hash(&mut h);
        set.insert(h.finish());
    }

    // This allows 100 collisions, which is far too
    // many, but should guard against transient issues
    // that will result from using RandomState
    let pct_unique = set.len() as f32 / limit as f32;
    assert!(0.99f32 < pct_unique, "percent-unique={}", pct_unique);
}
