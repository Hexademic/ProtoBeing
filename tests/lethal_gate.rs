//! **A default-off gate that kills the embodied being.** Found 2026-08-21 by
//! `examples/census.rs` while measuring something else entirely
//! (`docs/faculty-ablation.md` §12).
//!
//! `enable_workspace_persistence()` is the Global Workspace Stage 3 leaky
//! integrator. On the **abstract** path it is harmless — a being lives out
//! 4,000 ticks with it on, in a fair world, a trap, and a famine. In the being's
//! **own room** it dies at **tick 32**, where the default being lives the full
//! 4,000.
//!
//! Nothing else in the suite catches this. The gate is default-off, so no
//! published number moves and no existing test exercises it embodied — which is
//! exactly how a lethal switch sits unnoticed beside fifteen safe ones.
//!
//! This test **fails if the lethality is fixed**, which is the point: the fix
//! must be a deliberate act that comes here and records what changed, not a
//! silent repair. Until then it is a standing warning that this gate must not be
//! enabled for an embodied being.

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
fn workspace_persistence_kills_the_embodied_being_and_the_default_path_hides_it() {
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
