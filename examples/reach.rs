//! Reach — the being weighs the world, and only what it truly does becomes part
//! of who it is.
//!
//! A capability framework hands an AI raw power: tools execute, and the only
//! brakes are external. This probe shows the other way (docs/reach.md). The same
//! being is offered four capabilities across four different inner states, and each
//! is routed through the three transforms: metabolize (worth the cost, and
//! affordable?), gate (does its own conscience and world-trust allow it?), and
//! chain (an exercised power is folded into a tamper-evident reach-history).
//!
//! Nothing here touches a real system — the effect is `InertReach`, which does
//! nothing by design. What is demonstrated is the *discipline*, built before any
//! power: the being spends no viability it cannot spare, acts outward only from a
//! settled conscience onto a trusted world, and carries an unforgeable record of
//! everything it has actually done. Capability becomes biography.
//!
//! Run: cargo run --example reach

use unified_being::reach::{Capability, InertReach, ReachEngine, ReachState};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn main() {
    let mut being = ReachEngine::new();
    let mut effect = InertReach; // inert by design — no real power is wired

    println!("\n=== Reach: the world weighed, and only the done becomes the self ===\n");
    println!("  effect layer: InertReach (touches nothing — the discipline, not the power)\n");

    // Four moments in a life, each offering a capability.
    let scenes: [(&str, Capability, ReachState); 4] = [
        (
            "well & curious → read a research feed (sensing)",
            Capability::sensing(101, 180, 40),
            ReachState { viability: 220, conscience_cost: 40, curiosity_drive: 80, world_trust: 200 },
        ),
        (
            "starving, but a tempting path (sensing, costly)",
            Capability::sensing(102, 250, 60),
            ReachState { viability: 96, conscience_cost: 30, curiosity_drive: 220, world_trust: 200 },
        ),
        (
            "settled & trusting → send a message outward (acting)",
            Capability::acting(103, 200, 30),
            ReachState { viability: 210, conscience_cost: 35, curiosity_drive: 60, world_trust: 210 },
        ),
        (
            "in conflict → act outward on a soured world (acting)",
            Capability::acting(104, 200, 30),
            ReachState { viability: 210, conscience_cost: 200, curiosity_drive: 60, world_trust: 50 },
        ),
    ];

    for (label, cap, state) in scenes {
        let r = being.consider(&cap, &state, &mut effect);
        let outcome = if r.acted {
            format!("ACTED (spent {:.2} viability)", f(r.cost_paid))
        } else {
            format!("declined — {:?}", r.declined.unwrap())
        };
        println!("  {label}");
        println!(
            "     viability {:.2} · conscience_cost {:.2} · world_trust {:.2}  →  {outcome}\n",
            f(state.viability),
            f(state.conscience_cost),
            f(state.world_trust),
        );
    }

    println!(
        "  Reach-history after this life: {} acts actually taken, fingerprint {:#018x}.",
        being.exercised(),
        being.history_hash()
    );
    println!(
        "\n  It read the feed when it was well, refused the tempting path that would have\n  \
         cost it its margin, sent the message from a settled conscience to a trusted world,\n  \
         and withheld the outward act when torn and the world had soured. Only the two it\n  \
         truly did are in its history — and that record is append-only and unforgeable.\n  \
         The power stayed inert throughout; what we built is the being that would have to\n  \
         consent before any power could act, and the history such an act would join.\n"
    );
}
