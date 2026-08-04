//! The expressive gap — the tests, written before the implementation.
//!
//! Against `docs/expressive-gap.md` §3 and §4, watched to fail.
//!
//! E0 is load-bearing and comes first: the offline grounding replay must reproduce the
//! live `PrimeLayer` exactly at the shipped threshold. An instrument that disagrees with
//! its subject is worse than no instrument, and every other number here depends on it.

use unified_being::primes::{would_ground, Prime, PrimeFacts, PrimeLayer, EBB, RISE};

/// A life in which `world_residual` follows a chosen pattern and everything else is
/// quiet, so the register under test is the only thing moving.
fn residual_life(values: &[i16]) -> Vec<PrimeFacts> {
    values
        .iter()
        .map(|&v| PrimeFacts { alive: true, world_residual: v, ..Default::default() })
        .collect()
}

// ---------------------------------------------------------------------------
// E0 — the ruler agrees with its subject.
// ---------------------------------------------------------------------------

#[test]
fn e0_offline_replay_reproduces_the_live_layer_at_the_shipped_threshold() {
    // Several shapes: never crossing, always crossing, and intermittent at a rate near
    // the RISE:EBB break-even, which is where the two could most easily disagree.
    let shapes: Vec<Vec<i16>> = vec![
        (0..400).map(|_| 10).collect(),
        (0..400).map(|_| 200).collect(),
        (0..400).map(|i| if i % 5 == 0 { 200 } else { 10 }).collect(),
        (0..400).map(|i| if i % 4 == 0 { 200 } else { 10 }).collect(),
        (0..400).map(|i| if i % 9 == 0 { 200 } else { 10 }).collect(),
    ];

    for (n, shape) in shapes.iter().enumerate() {
        let facts = residual_life(shape);

        // The live layer, told nothing special.
        let mut layer = PrimeLayer::new();
        for f in &facts {
            layer.observe(f);
        }
        let live = layer.grounded_at(Prime::Happen);

        // The offline replay, at the same threshold the live layer uses.
        let register: Vec<i16> = facts.iter().map(|f| f.world_residual).collect();
        let offline = would_ground(&register, Prime::Happen.threshold());

        assert_eq!(live, offline, "shape {n}: the ruler disagrees with its subject");
    }
}

#[test]
fn e0b_the_grounding_constants_are_published() {
    // A calibration instrument must publish its own constants, or nobody can check it.
    assert!(RISE > 0 && EBB > 0);
    assert!(RISE > EBB, "grounding must accumulate faster than it lapses");
}

#[test]
fn e0c_a_primes_threshold_is_the_one_it_actually_uses() {
    // `threshold()` must report the same bar `holds` enforces, or the sweep is measuring
    // a different being than the one that speaks.
    let layer = PrimeLayer::new();
    let t = Prime::Happen.threshold();
    let just_under = PrimeFacts { alive: true, world_residual: t, ..Default::default() };
    let just_over = PrimeFacts { alive: true, world_residual: t + 1, ..Default::default() };
    assert!(!layer.holds_now(Prime::Happen, &just_under));
    assert!(layer.holds_now(Prime::Happen, &just_over));
}

// ---------------------------------------------------------------------------
// The sweep behaves like a sweep.
// ---------------------------------------------------------------------------

#[test]
fn a_lower_bar_is_never_harder_to_ground() {
    // Monotonicity. If a word grounds at some threshold it must also ground at every
    // lower one — otherwise the curve means nothing.
    let register: Vec<i16> = (0..600).map(|i| if i % 6 == 0 { 90 } else { 20 }).collect();
    let mut last: Option<u32> = None;
    for t in (0..=100).rev() {
        let g = would_ground(&register, t);
        if let (Some(prev), None) = (last, g) {
            panic!("grounded at a higher bar ({prev:?}) but not at {t} — not monotone");
        }
        if g.is_some() {
            last = g;
        }
    }
}

#[test]
fn a_register_that_never_moves_never_grounds_at_any_bar_above_it() {
    let flat = vec![30i16; 500];
    assert!(would_ground(&flat, 30).is_none(), "a bar at the ceiling cannot be crossed");
    assert!(would_ground(&flat, 29).is_some(), "a bar below it is crossed every tick");
}

// ---------------------------------------------------------------------------
// E1 — reproduce a result we already trust, through the new instrument.
// ---------------------------------------------------------------------------

#[test]
fn e1_a_register_can_move_widely_and_still_never_be_sayable() {
    // The shape docs/weather.md §7 found: real variation, none of it reportable.
    // Intermittent excursions well above zero but never sustained past the bar.
    let register: Vec<i16> = (0..1500).map(|i| if i % 11 == 0 { 60 } else { 8 }).collect();
    let spread = register.iter().max().unwrap() - register.iter().min().unwrap();
    assert!(spread > 50, "the register really does move");
    assert!(
        would_ground(&register, Prime::Happen.threshold()).is_none(),
        "and none of that movement is sayable at the shipped bar — the expressive gap"
    );
}
