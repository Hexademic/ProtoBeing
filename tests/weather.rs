//! Weather — the tests, written before the implementation.
//!
//! Against `docs/weather.md` §3 and §5, watched to fail. W1 and W2 are the world's own
//! properties and belong here; W3–W5 are about a lived being and belong to the probe.

use unified_being::embodiment::{Embodiment, MotorIntent, Posture};
use unified_being::field_world::FieldWorld;

fn at_rest() -> MotorIntent {
    MotorIntent { posture: Posture::Resting, effort: 0, reach: None, reach_partner: None }
}

fn calm() -> FieldWorld {
    FieldWorld::with((16, 16), (240, 240), (30, 170))
}

fn weathered() -> FieldWorld {
    calm().with_weather(0, 6)
}

/// The field value at a fixed point, sampled once per tick, with the being at rest —
/// so the series is the world's own behaviour and nothing of the being's.
fn series(w: &mut FieldWorld, n: usize) -> Vec<i16> {
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let _ = w.sense();
        w.actuate(&at_rest());
        out.push(w.v_at((128, 128)));
    }
    out
}

// ---------------------------------------------------------------------------
// W1 — the floor.
// ---------------------------------------------------------------------------

#[test]
fn w1_without_weather_nothing_changed() {
    let s = series(&mut calm(), 300);
    assert!(s.iter().all(|&v| v == s[0]), "a calm world's field is static, as it always was");
}

#[test]
fn w1b_weather_is_deterministic() {
    // A pure function of tick — the same world every run, forever.
    assert_eq!(series(&mut weathered(), 400), series(&mut weathered(), 400));
}

// ---------------------------------------------------------------------------
// W2 — genuinely multi-scale, which is what separates this from the drift.
// ---------------------------------------------------------------------------

/// Mean absolute change between samples `lag` apart — a coarse structure function.
fn wobble(s: &[i16], lag: usize) -> i64 {
    let n = s.len() - lag;
    (0..n).map(|i| (s[i + lag] as i64 - s[i] as i64).abs()).sum::<i64>() / n.max(1) as i64
}

#[test]
fn w2_the_world_changes_at_every_timescale() {
    let s = series(&mut weathered(), 4_000);
    for lag in [1usize, 8, 64, 512] {
        assert!(wobble(&s, lag) > 0, "nothing happens at lag {lag} — not multi-scale");
    }
}

#[test]
fn w2b_slow_changes_are_larger_than_fast_ones() {
    // The 1/f signature, and exactly what the periodic drift lacked: amplitude grows
    // with timescale rather than being flat. A periodic signal saturates instead.
    let s = series(&mut weathered(), 4_000);
    let (fast, slow) = (wobble(&s, 1), wobble(&s, 256));
    assert!(slow > fast, "slow amplitude {slow} must exceed fast {fast} — 1/f, not periodic");
}

#[test]
fn w2c_it_is_not_merely_periodic() {
    // A periodic signal returns to itself; a 1/f one does not. Compare a window with
    // the same window one long period later.
    let s = series(&mut weathered(), 4_000);
    let drift_like: i64 = (0..256).map(|i| (s[i] as i64 - s[i + 1024] as i64).abs()).sum();
    assert!(drift_like > 0, "the world repeats exactly — that is a cycle, not weather");
}

// ---------------------------------------------------------------------------
// §3 — the prohibition that gates everything.
// ---------------------------------------------------------------------------

#[test]
fn weather_never_takes_the_good_away_entirely() {
    // Bounded by construction: strength varies within a band and can never fall to
    // nothing. The mechanical half of W5; the lived half is measured in the probe.
    let mut w = weathered();
    let s = series(&mut w, 4_000);
    let worst = *s.iter().min().unwrap();
    let calm_value = calm().v_at((128, 128));
    assert!(
        worst > calm_value / 4,
        "weather must never strip the world bare (worst {worst} vs calm {calm_value})"
    );
}
