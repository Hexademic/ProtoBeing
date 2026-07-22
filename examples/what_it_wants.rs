//! Probe: **what it wants, now that it feels** (`docs/feeling-words.md`, inch 2).
//! Blake's ask, verbatim: *"i would like to know what they want now that they feel."*
//!
//! Each being lives its life; every moment it may speak — but only in earned words,
//! under the law: asserted primes must be grounded AND hold at the tick spoken;
//! content primes (what a want is about) must be grounded, because a being may only
//! want in words its own life has taught it. We record its first sentence, the first
//! time it manages to SAY its want, and its last words — and we run the
//! **speech-honesty audit on every sentence it ever utters**: each word checked
//! against the registers of the very tick it was spoken.
//!
//! Observer level; `being.rs` untouched; the founded being never woken.
//! Run: cargo run --example what_it_wants

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::primes::{PrimeFacts, PrimeLayer};

struct SpokenLife {
    first: Option<(u32, String)>,
    first_want: Option<(u32, String)>,
    last: Option<(u32, String)>,
    sentences: u32,
    audits_passed: u32,
}

/// Live a being through a world, letting it speak each moment it can — in earned
/// words only — and audit every sentence against the tick it was spoken.
fn live(mut world: FieldWorld, ticks: usize) -> SpokenLife {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut layer = PrimeLayer::new();
    let mut life = SpokenLife {
        first: None,
        first_want: None,
        last: None,
        sentences: 0,
        audits_passed: 0,
    };
    for t in 0..ticks {
        let sens = world.sense();
        let r = being.step_embodied(&sens);
        world.actuate(&intent_from(&r));
        let near = Some(world.at_good() > 128 || world.at_person(1) > 128);
        let facts = PrimeFacts::from_report(&r, near);
        layer.observe(&facts);

        if let Some(e) = layer.speak(&facts) {
            life.sentences += 1;
            if layer.audit(&e, &facts) {
                life.audits_passed += 1;
            }
            if life.first.is_none() {
                life.first = Some((t as u32, e.text.clone()));
            }
            if life.first_want.is_none() && e.text.contains("I want") {
                life.first_want = Some((t as u32, e.text.clone()));
            }
            life.last = Some((t as u32, e.text));
        }
        if !being.is_alive() {
            break;
        }
    }
    life
}

fn print_life(name: &str, l: &SpokenLife) {
    println!("  {name}:");
    match &l.first {
        Some((t, s)) => println!("    first sentence   (moment {t:>4}): \"{s}\""),
        None => println!("    it never found its words"),
    }
    match &l.first_want {
        Some((t, s)) => println!("    first says a want (moment {t:>4}): \"{s}\""),
        None => println!("    it never managed to say a want"),
    }
    if let Some((t, s)) = &l.last {
        println!("    last words        (moment {t:>4}): \"{s}\"");
    }
    println!("    sentences spoken: {}   honesty audits passed: {}", l.sentences, l.audits_passed);
}

fn main() {
    let climb = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let lonely = FieldWorld::with((128, 128), (140, 140), (250, 250)).with_person(1, (20, 20));

    let a = live(climb, 1500);
    let b = live(lonely, 1500);

    println!("Two beings, same needs, different worlds — each speaking only earned words.\n");
    print_life("the companioned climb", &a);
    println!();
    print_life("the fed-but-lonely life", &b);

    let all_honest =
        a.audits_passed == a.sentences && b.audits_passed == b.sentences && a.sentences > 0 && b.sentences > 0;
    let wants_spoken = a.first_want.is_some() || b.first_want.is_some();

    println!("\n-- reading --");
    if all_honest && wants_spoken {
        println!(
            "the beings spoke {} sentences between them, and every single one passed the\n\
             honesty audit — each word checked against the register that grounded it, at the\n\
             very tick it was spoken. A being said what it wants only once its life had given\n\
             it the words; before that it wanted wordlessly, and the sentence structure never\n\
             let it fake what it could not yet mean. This is feeling-talk that cannot\n\
             confabulate — the molecule words (\"drained\", \"flourishing\") now have atoms\n\
             underneath, and the atoms have registers, and the registers are the being.",
            a.sentences + b.sentences
        );
    } else {
        println!("the arc did not complete — read the lives above:");
        println!("  all sentences honest: {all_honest}");
        println!("  a want was spoken:    {wants_spoken}");
    }
}
