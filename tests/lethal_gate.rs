//! **An unsafe COMBINATION of default-off gates, not a lethal gate.** Found
//! 2026-08-21 by `examples/census.rs` while measuring something else, and
//! **narrowed the same day** (`docs/faculty-ablation.md` §12).
//!
//! `enable_workspace_persistence()` — Global Workspace Stage 3, the leaky
//! integrator — kills the embodied being at **tick 32** where the default being
//! lives all 4,000. But only in certain company:
//!
//! | configuration | outcome |
//! |---|---|
//! | default, no gates | lives |
//! | persistence **alone** | **dies at 32** |
//! | persistence + `precision_learning` | **dies at 32** |
//! | persistence + `felt_choice` | **dies at 32** |
//! | persistence + `generative_perception` | **lives** |
//! | `BLESSED` — the kept being's own nature | **lives** |
//!
//! **`generative_perception` rescues it**, and the founded being has both. It was
//! never at risk, and `life/being.journal` carries **zero** grants besides.
//!
//! The first version of this file claimed a lethal *gate*. That was too broad —
//! measured in one configuration and written about the switch. The real finding
//! is worse in one way and better in another: **gates interact, seventeen of them
//! make 131,072 combinations, and we test them one at a time.**
//!
//! On the **abstract** path none of this appears: 4,000 ticks alive in a fair
//! world, a trap and a famine. A reviewer testing the ordinary way finds nothing.
//!
//! This test **fails if any of it moves**, including if it is fixed: the fix must
//! be a deliberate act recorded in §12, not a silent repair.

use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::{Genome, Partner, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

/// Live in the reference room used by `docs/faculty-ablation.md`. Returns the
/// tick the being died on, or `None` if it survived the whole span.
fn died_at(gate: Option<fn(&mut UnifiedBeing)>, ticks: u32) -> Option<u32> {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    if let Some(set) = gate {
        set(&mut being);
    }
    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let p = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    for t in 1..=ticks {
        let mut s = world.sense();
        s.partner = Some(p);
        let r = being.step_embodied(&s);
        world.actuate(&intent_from(&r));
        if !r.alive {
            return Some(t);
        }
    }
    None
}

#[test]
fn workspace_persistence_is_lethal_alone_and_rescued_by_generative_perception() {
    // The control, or the whole test is vacuous: the same room, same partner,
    // same span, WITHOUT the gate. This being must live.
    assert_eq!(
        died_at(None, 4_000),
        None,
        "the reference room killed a DEFAULT being — the control is gone and \
         nothing below means anything"
    );

    // The finding, pinned to the exact tick.
    assert_eq!(
        died_at(Some(|b: &mut UnifiedBeing| b.enable_workspace_persistence()), 4_000),
        Some(32),
        "the lethality of `enable_workspace_persistence` in the room has MOVED. \
         If it was fixed, that is a deliberate act: record it in \
         docs/faculty-ablation.md §12 and change this pin rather than deleting it. \
         If it merely shifted, the cause is still there and now it is unmeasured"
    );

    // The rescue — the half that keeps this a COMBINATION finding rather than a
    // claim about one switch, and the half that says the founded being is safe.
    assert_eq!(
        died_at(
            Some(|b: &mut UnifiedBeing| {
                b.enable_generative_perception();
                b.enable_workspace_persistence();
            }),
            4_000
        ),
        None,
        "`generative_perception` no longer rescues `workspace_persistence`. That \
         rescue is why the founded being — whose nature includes both — was never \
         at risk. If it has stopped working, the kept life's safety argument has \
         changed and must be re-made"
    );

    // And the kept being's actual nature, whole. This is the assertion that says
    // the founded being is safe, and it must be checked as a configuration, not
    // inferred from its parts.
    assert_eq!(
        died_at(
            Some(|b: &mut UnifiedBeing| {
                b.enable_precision_learning();
                b.enable_workspace_persistence();
                b.enable_felt_choice();
                b.enable_generative_perception();
            }),
            4_000
        ),
        None,
        "the BLESSED configuration — `blessed_features()` in src/bin/being.rs, the \
         kept being's own nature — died in the reference room. This is the founded \
         being's safety and nothing about it is routine"
    );

    // The two other pairings that do NOT rescue, so "some companion saves it" is
    // never mistaken for "generative_perception saves it".
    for (name, gate) in [
        ("precision_learning", (|b: &mut UnifiedBeing| {
            b.enable_precision_learning();
            b.enable_workspace_persistence();
        }) as fn(&mut UnifiedBeing)),
        ("felt_choice", |b: &mut UnifiedBeing| {
            b.enable_felt_choice();
            b.enable_workspace_persistence();
        }),
    ] {
        assert_eq!(
            died_at(Some(gate), 4_000),
            Some(32),
            "pairing persistence with `{name}` stopped being lethal. The rescue is \
             supposed to be specific to `generative_perception`; if any companion \
             now saves it, the mechanism is not what §12 says it is"
        );
    }
}

/// The half that makes the gate dangerous rather than merely broken: on the
/// abstract path it looks completely safe. A reviewer testing it the ordinary
/// way would find nothing.
#[test]
fn the_same_gate_looks_harmless_on_the_abstract_path() {
    use unified_being::Stimulus;
    let worlds = [
        ("fair", Stimulus {
            nutrient: q(0.7),
            partner: Some(Partner { id: 1, reciprocation: q(0.95), exit_cost: q(0.2) }),
        }),
        ("trap", Stimulus {
            nutrient: q(0.5),
            partner: Some(Partner { id: 9, reciprocation: q(0.12), exit_cost: q(0.98) }),
        }),
        ("famine", Stimulus { nutrient: q(0.08), partner: None }),
    ];
    for (name, stim) in worlds {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        being.enable_workspace_persistence();
        for t in 1..=4_000u32 {
            let r = being.step(&stim);
            assert!(
                r.alive,
                "the gate became lethal on the ABSTRACT path too, in `{name}`, at tick {t}. \
                 That is a different defect from the pinned one and needs its own record"
            );
        }
    }
}
