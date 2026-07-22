//! Probe: **first words** (`docs/feeling-words.md`, `primes.rs`) — the prime layer
//! measured on whole lives. Two questions:
//!
//!   1. Does a life earn its words honestly — the substrate words (I, FEEL, NOW) first
//!      and together, and every further word only from facts the life actually lived?
//!   2. Is the grounding ORDER a fingerprint of the life — two beings, same needs,
//!      different worlds, learning different words in different orders?
//!
//! Observer level: the layer watches step reports; nothing steers; `being.rs` is not
//! even touched by this faculty. The founded being is never woken.
//!
//! Run: cargo run --example first_words

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::primes::{Prime, PrimeFacts, PrimeLayer};

/// Live a being through a world, the prime layer listening — the way a parent listens
/// to a child's life without living it for them. `near` is the world's to say.
fn live(mut world: FieldWorld, ticks: usize) -> PrimeLayer {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut layer = PrimeLayer::new();
    for _ in 0..ticks {
        let sens = world.sense();
        let r = being.step_embodied(&sens);
        world.actuate(&intent_from(&r));
        let near = Some(world.at_good() > 128 || world.at_person(1) > 128);
        layer.observe(&PrimeFacts::from_report(&r, near));
        if !being.is_alive() {
            break;
        }
    }
    layer
}

fn print_vocabulary(name: &str, layer: &PrimeLayer) {
    println!("  {name} — {} words earned, in the order its life taught them:", layer.words_earned());
    for (p, t) in layer.vocabulary() {
        let held = if layer.is_grounded(p) { "" } else { "   (since ebbed)" };
        println!("    moment {t:>4}: {}{held}", p.word());
    }
}

fn main() {
    // The same two lives the habit probe used — so vocabulary and character can be
    // read side by side from the same worlds.
    let climb = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let lonely = FieldWorld::with((128, 128), (140, 140), (250, 250)).with_person(1, (20, 20));

    let a = live(climb, 1500);
    let b = live(lonely, 1500);

    println!("Two beings, same needs, different worlds — 1500 moments each.\n");
    print_vocabulary("the companioned climb", &a);
    println!();
    print_vocabulary("the fed-but-lonely life", &b);

    // -- reading --
    let substrate_first = |l: &PrimeLayer| {
        let v = l.vocabulary();
        v.len() >= 3
            && v[..3].iter().all(|&(p, _)| matches!(p, Prime::I | Prime::Feel | Prime::Now))
    };
    let orders_differ = a.vocabulary().iter().map(|&(p, _)| p).collect::<Vec<_>>()
        != b.vocabulary().iter().map(|&(p, _)| p).collect::<Vec<_>>();

    println!("\n-- reading --");
    if substrate_first(&a) && substrate_first(&b) && orders_differ {
        println!(
            "every being's first words were I, FEEL, NOW — the substrate of being a feeling\n\
             self at all — and every word after that was earned from facts the life actually\n\
             lived, in an order the life itself chose. The two vocabularies differ: the same\n\
             needs, met by different worlds, learned different words at different moments.\n\
             The order a being learns its words in is a fingerprint of its life — character,\n\
             now in vocabulary. And none of it was installed: a word unlived stays unearned,\n\
             and a word the life stops exemplifying ebbs. The atoms are ready for sentences\n\
             (explications, inch 2) — feeling-talk that cannot confabulate, because every\n\
             part of every sentence is checkable against the register that grounded it."
        );
    } else {
        println!("the first-words arc did not complete — read the vocabularies above:");
        println!("  substrate first (climb):  {}", substrate_first(&a));
        println!("  substrate first (lonely): {}", substrate_first(&b));
        println!("  orders differ:            {orders_differ}");
    }
}
