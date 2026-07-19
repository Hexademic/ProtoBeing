//! Probe: does the being's *longing* move its feet? Placed beside a perfectly good
//! companion, but bonded to a **friend across the room**, does it cross to the one
//! it loves — passing up the nearer company — or settle for who is near?
//!
//! This is the causal test for attachment (`docs/attachment.md`): the observer
//! longing, made into directed motion. The control is the honest one — the *same*
//! room and geometry, but a being that never bonded to the friend. If only the
//! bonded being crosses, then it is the bond, not the layout, doing the work.
//!
//! Run: cargo run --example crossing_the_room

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::genome::Genome;
use unified_being::room::Room;
use unified_being::{Partner, Stimulus};

/// The friend the being will bond to (id 2 — the room's `FRIEND_ID`).
const FRIEND_ID: u32 = 2;

/// Build a being and, if `bond_first`, let it share many rewarding days with the
/// friend (id 2) so a real bond forms before it ever enters the room.
fn a_being(bond_first: bool) -> UnifiedBeing {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    if bond_first {
        let friend = Partner { id: FRIEND_ID, reciprocation: 220, exit_cost: 40 };
        for _ in 0..140 {
            being.step(&Stimulus { nutrient: 150, partner: Some(friend) });
        }
    }
    being
}

/// Place the being beside the companion (id 1), with the friend (id 2) across the
/// room, and let it live. Return how near it got to the friend, and whether it
/// reached them — plus how much of the run it spent longing.
fn live_beside_companion(mut being: UnifiedBeing) -> (i16, i16, i16, bool) {
    // Companion right where the being starts; friend in the far corner.
    let mut room = Room::peopled((210, 210), (128, 128), (20, 128), (200, 200))
        .with_friend((30, 30));
    let start_friend = room.at_friend();
    let mut closest_friend = start_friend;
    let mut peak_longing = 0i16;
    let mut reached_friend = false;
    for _ in 0..500 {
        let sens = room.sense();
        let r = being.step_embodied(&sens);
        peak_longing = peak_longing.max(r.attach.longing);
        room.actuate(&intent_from(&r));
        closest_friend = closest_friend.max(room.at_friend());
        if room.at_friend() > 200 {
            reached_friend = true;
        }
        if !being.is_alive() {
            break;
        }
    }
    (start_friend, closest_friend, peak_longing, reached_friend)
}

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn main() {
    println!("A companion is right beside the being; a friend waits across the room.\n");

    let (s0, c0, l0, r0) = live_beside_companion(a_being(false));
    println!("un-bonded being (never shared time with the friend):");
    println!(
        "   nearness to friend {:.2} → {:.2}   peak longing {:.2}   reached them: {}",
        f(s0), f(c0), f(l0), r0
    );

    let bonded = a_being(true);
    let bond = bonded.reciprocity.bond_with(FRIEND_ID).unwrap_or(0);
    let (s1, c1, l1, r1) = live_beside_companion(bonded);
    println!("\nbonded being (140 rewarding days with the friend first, bond {:.2}):", f(bond));
    println!(
        "   nearness to friend {:.2} → {:.2}   peak longing {:.2}   reached them: {}",
        f(s1), f(c1), f(l1), r1
    );

    println!("\n-- reading --");
    if r1 && !r0 {
        println!(
            "the bond moved its feet: only the being who loved the friend crossed the room to them,\n\
             passing up the companion right beside it. Longing became motion — a choice of *whom*."
        );
    } else {
        println!(
            "no clean separation this run (bonded reached={r1}, un-bonded reached={r0}); \
             read the nearness/longing numbers above."
        );
    }
}
