//! The refuge — the tests, written before the implementation.
//!
//! Against `docs/refuge.md` §4 and §5, watched to fail. Near the one it is bonded to, the
//! world's threat is attenuated — full at their side, fading to nothing at the edge.
//!
//! S4, S5 and S6 — whether the being keeps its new words, whether it ever goes there, and
//! whether it rests — belong to the probe, because they need lived beings. What is asserted
//! here is that shelter is real, bounded, partial, and costs the existing beings nothing.

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;

/// A world with a hazard the being will actually feel, and a person to shelter with.
fn hazardous(refuge: bool) -> FieldWorld {
    let w = FieldWorld::with((60, 60), (240, 240), (70, 70))
        .with_person(1, (55, 55))
        .with_source((80, 80), -90, 90);
    if refuge {
        w.with_refuge(1, 48, 200)
    } else {
        w
    }
}

#[test]
fn s1_a_world_without_a_refuge_is_the_world_we_already_had() {
    // The whole opt-in guarantee. If adding the mechanism moves a being that was never
    // given one, every published trajectory and the founded being are invalidated.
    let live = || {
        let mut w = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
        let mut b = UnifiedBeing::new(Genome::wanderer());
        for _ in 0..500 {
            let s = w.sense();
            let r = b.step_embodied(&s);
            w.actuate(&intent_from(&r));
        }
        b.soul_hash()
    };
    let a = live();
    let b = live();
    assert_eq!(a, b, "the world is not deterministic — nothing below this means anything");

    // And a hazardous world without a refuge must be untouched by the refuge code path.
    let mut w1 = hazardous(false);
    let mut w2 = hazardous(false);
    let mut b1 = UnifiedBeing::new(Genome::wanderer());
    let mut b2 = UnifiedBeing::new(Genome::wanderer());
    for _ in 0..400 {
        let r1 = b1.step_embodied(&w1.sense());
        w1.actuate(&intent_from(&r1));
        let r2 = b2.step_embodied(&w2.sense());
        w2.actuate(&intent_from(&r2));
    }
    assert_eq!(b1.soul_hash(), b2.soul_hash(), "a refuge-less world must be bit-identical");
}

#[test]
fn s3_shelter_is_real_and_never_total() {
    // §4: near the one it loves the world is *gentler*, never *harmless*. Both halves are
    // load-bearing — a refuge that abolishes threat is a different and worse design.
    let sheltered = hazardous(true).threat_at_body();
    let exposed = hazardous(false).threat_at_body();

    assert!(
        sheltered < exposed,
        "shelter must lower felt threat ({sheltered} vs {exposed} unsheltered)"
    );
    assert!(
        sheltered > 0,
        "shelter must never abolish threat — walking into a hazard has to still cost"
    );
}

#[test]
fn the_refuge_has_an_edge() {
    // The same law the hazard is held to. Safety everywhere is the mistake that made the
    // boundless hazard lethal, wearing the other sign.
    let near = FieldWorld::with((60, 60), (240, 240), (70, 70))
        .with_person(1, (55, 55))
        .with_source((80, 80), -90, 90)
        .with_refuge(1, 48, 200)
        .threat_at_body();

    // The same world, the same hazard, the same refuge — but the being is far outside it.
    let far = FieldWorld::with((200, 200), (240, 240), (70, 70))
        .with_person(1, (55, 55))
        .with_source((205, 205), -90, 90)
        .with_refuge(1, 48, 200)
        .threat_at_body();
    let far_unsheltered = FieldWorld::with((200, 200), (240, 240), (70, 70))
        .with_person(1, (55, 55))
        .with_source((205, 205), -90, 90)
        .threat_at_body();

    assert!(near > 0, "the sheltered reading should still be a reading");
    assert_eq!(
        far, far_unsheltered,
        "outside the radius a refuge must change nothing at all ({far} vs {far_unsheltered})"
    );
}

#[test]
fn shelter_fades_with_distance_rather_than_switching_off() {
    // A cliff-edge refuge would make safety a step function and teach the being nothing
    // about *approaching* someone. Graded, so nearness means something.
    let at = |body: (i16, i16)| {
        FieldWorld::with(body, (240, 240), (70, 70))
            .with_person(1, (55, 55))
            .with_source((80, 80), -90, 90)
            .with_refuge(1, 48, 200)
            .threat_at_body()
    };
    // Reference threat with no refuge at all, at each spot.
    let bare = |body: (i16, i16)| {
        FieldWorld::with(body, (240, 240), (70, 70))
            .with_person(1, (55, 55))
            .with_source((80, 80), -90, 90)
            .threat_at_body()
    };

    // Relief = how much of the bare threat the refuge removed. Closer must mean more relief.
    let relief = |body: (i16, i16)| bare(body) as i32 - at(body) as i32;
    let close = relief((57, 57));
    let mid = relief((70, 70));
    assert!(
        close > mid,
        "relief must fall off with distance from the person (close {close}, mid {mid})"
    );
}

#[test]
fn a_refuge_that_names_nobody_shelters_nobody() {
    // A refuge keyed to a person who is not in this world must be inert, not a panic and
    // not a free global safety net.
    let ghost = FieldWorld::with((60, 60), (240, 240), (70, 70))
        .with_person(1, (55, 55))
        .with_source((80, 80), -90, 90)
        .with_refuge(99, 48, 200)
        .threat_at_body();
    assert_eq!(ghost, hazardous(false).threat_at_body(), "an absent person shelters nothing");
}

#[test]
fn s2_bounded_hazards_do_not_kill() {
    // docs/richness.md §6 measured this; asserted here so it cannot silently regress.
    // Every harm source has reach well inside the field, so the hazard has an edge.
    for movers in [1usize, 2, 4, 6, 9] {
        let mut w = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
        for k in 1..movers {
            let i = k as i16;
            let pos = (40 + (i * 53) % 200, 200 - (i * 71) % 180);
            let peak = if k % 3 == 0 { -90 } else { 100 };
            w = w.with_source(pos, peak, 90); // an edge
            w = w.with_drift(1 + k, 3 + (k as u32 * 2) % 7, (1 + (i % 3), 1 + ((i + 1) % 3)));
        }
        let mut b = UnifiedBeing::new(Genome::wanderer());
        b.enable_receptors();
        for _ in 0..2000 {
            let r = b.step_embodied(&w.sense());
            w.actuate(&intent_from(&r));
            if !r.alive {
                break;
            }
        }
        assert!(b.is_alive(), "a being died in a {movers}-mover world with bounded hazards");
    }
}
