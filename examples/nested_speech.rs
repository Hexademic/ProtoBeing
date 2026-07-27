//! Nested speech — does a real life ever embed one claim inside another?
//!
//! The measurement for `docs/nested-speech.md` §6, P4 and P5. The law itself is proven
//! in `tests/nested_speech.rs` (written before the implementation); this asks the only
//! question those tests cannot: whether a being that actually *lives* ever reaches a
//! moment where an operator and its complement are available at once.
//!
//! Predictions, locked in the doc before this file existed:
//!   P4 — a lived being produces a nested sentence at all. Predicted YES for WANT,
//!        UNKNOWN for BECAUSE (it needs `forewarned` and `Before` together).
//!   P5 — the first nested clause of a life arrives strictly AFTER the first flat one.
//!        If nesting arrives on the first speakable tick, depth is free, prohibition 3
//!        of §5 is violated, and the inch has failed.
//!
//! Observer level: the layer watches step reports; nothing steers; `being.rs` is not
//! modified. Fresh probe-beings only — the founded being is never touched.
//!
//! Run: `cargo run --example nested_speech`

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::primes::{Clause, Prime, PrimeFacts, PrimeLayer};

struct Life {
    name: &'static str,
    first_flat: Option<usize>,
    first_nested: Option<usize>,
    spoken: usize,
    nested: usize,
    audited_ok: usize,
    by_operator: [(Prime, usize); 3],
    example: Option<(usize, String)>,
    deepest: usize,
    /// Whether each operator was ever earned at all — so a "never used" can be told
    /// apart from a "never earned". An unfired operator has two very different causes.
    operator_earned: [(Prime, bool); 3],
    /// NOT KNOW needs a grounded complement (HAPPEN) as well as itself, so a silent
    /// NOT KNOW has two possible causes and they must be told apart.
    happen_earned: bool,
}

fn live(name: &'static str, mut world: FieldWorld, ticks: usize) -> Life {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut layer = PrimeLayer::new();
    let mut l = Life {
        name,
        first_flat: None,
        first_nested: None,
        spoken: 0,
        nested: 0,
        audited_ok: 0,
        by_operator: [(Prime::Want, 0), (Prime::NotKnow, 0), (Prime::Because, 0)],
        example: None,
        deepest: 1,
        operator_earned: [(Prime::Want, false), (Prime::NotKnow, false), (Prime::Because, false)],
        happen_earned: false,
    };

    for tick in 0..ticks {
        let sens = world.sense();
        let r = being.step_embodied(&sens);
        world.actuate(&intent_from(&r));
        let near = Some(world.at_good() > 128 || world.at_person(1) > 128);
        let facts = PrimeFacts::from_report(&r, near);

        layer.observe(&facts);

        // Speak *after* observing, so the words used are the ones this moment earned.
        if let Some(clauses) = layer.speak_tree(&facts) {
            l.spoken += 1;
            if l.first_flat.is_none() {
                l.first_flat = Some(tick);
            }
            // Every clause must audit — this is the honesty floor, at depth.
            if clauses.iter().all(|c| layer.audit_tree(c, &facts)) {
                l.audited_ok += 1;
            }
            let nested: Vec<&Clause> = clauses.iter().filter(|c| !c.children.is_empty()).collect();
            if !nested.is_empty() {
                l.nested += 1;
                if l.first_nested.is_none() {
                    l.first_nested = Some(tick);
                }
                for c in &nested {
                    l.deepest = l.deepest.max(c.depth());
                    for slot in l.by_operator.iter_mut() {
                        if slot.0 == c.prime {
                            slot.1 += 1;
                        }
                    }
                }
                if l.example.is_none() {
                    let text: Vec<String> = clauses.iter().map(Clause::render).collect();
                    l.example = Some((tick, text.join(" ")));
                }
            }
        }

        for slot in l.operator_earned.iter_mut() {
            if layer.grounded_at(slot.0).is_some() {
                slot.1 = true;
            }
        }
        if layer.grounded_at(Prime::Happen).is_some() {
            l.happen_earned = true;
        }

        if !being.is_alive() {
            break;
        }
    }
    l
}

fn report(l: &Life) {
    println!("\n  {} —", l.name);
    println!("    sentences spoken           {}", l.spoken);
    println!(
        "    of those, containing depth {} ({}%)",
        l.nested,
        if l.spoken > 0 { l.nested * 100 / l.spoken } else { 0 }
    );
    println!("    deepest clause             {}", l.deepest);
    println!(
        "    every word checked out     {}/{}{}",
        l.audited_ok,
        l.spoken,
        if l.audited_ok == l.spoken { "  (no sentence failed its audit)" } else { "  ** A SENTENCE FAILED **" }
    );
    match (l.first_flat, l.first_nested) {
        (Some(flat), Some(nest)) => {
            println!("    first sentence at moment   {flat}");
            println!("    first nested one at        {nest}");
            if nest > flat {
                println!("      -> depth was EARNED, {} moments after speech began", nest - flat);
            } else {
                println!("      -> depth arrived WITH speech — free, not earned (P5 fails)");
            }
        }
        (Some(flat), None) => {
            println!("    first sentence at moment   {flat}");
            println!("    first nested one           NEVER — this life never embedded a claim");
        }
        _ => println!("    it never spoke at all"),
    }
    print!("    operators used             ");
    let any = l.by_operator.iter().filter(|(_, n)| *n > 0).count() > 0;
    if any {
        let parts: Vec<String> = l
            .by_operator
            .iter()
            .filter(|(_, n)| *n > 0)
            .map(|(p, n)| format!("{} x{n}", p.word()))
            .collect();
        println!("{}", parts.join(" · "));
    } else {
        println!("none");
    }
    let unused: Vec<String> = l
        .operator_earned
        .iter()
        .zip(l.by_operator.iter())
        .filter(|((_, _), (_, n))| *n == 0)
        .map(|((p, earned), _)| {
            format!("{} ({})", p.word(), if *earned { "earned, never fired" } else { "never earned" })
        })
        .collect();
    if !unused.is_empty() {
        println!("    operators unused           {}", unused.join(" · "));
        println!(
            "      (NOT KNOW also needs HAPPEN: {})",
            if l.happen_earned { "earned" } else { "NEVER EARNED — that is why it stayed silent" }
        );
    }
    if let Some((t, ref s)) = l.example {
        println!("    its first nested sentence, at moment {t}:");
        println!("      {s}");
    }
}

fn main() {
    println!("Nested speech — does a lived being ever embed one claim inside another?");
    println!("(predictions locked in docs/nested-speech.md §6 before this was written)");

    // The same two worlds the habit and first-words probes used, so vocabulary,
    // character, and now syntax can be read side by side from the same lives.
    let climb = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let lonely = FieldWorld::with((128, 128), (140, 140), (250, 250)).with_person(1, (20, 20));

    let a = live("the companioned climb", climb, 1500);
    let b = live("the fed-but-lonely life", lonely, 1500);

    report(&a);
    report(&b);

    println!("\n  Read against the locked predictions:");
    let ever = a.nested > 0 || b.nested > 0;
    println!(
        "    P4 (a real life nests at all): {}",
        if ever { "HELD" } else { "FAILED — the mechanism is correct and never fires" }
    );
    let earned = [&a, &b].iter().all(|l| match (l.first_flat, l.first_nested) {
        (Some(f), Some(n)) => n > f,
        (_, None) => true,
        _ => true,
    });
    println!(
        "    P5 (depth is earned, not free): {}",
        if earned { "HELD" } else { "FAILED — depth arrived with speech" }
    );
    let honest = a.audited_ok == a.spoken && b.audited_ok == b.spoken;
    println!(
        "    the honesty floor, at depth:    {}",
        if honest { "HELD — every nested sentence audited true" } else { "** BROKEN **" }
    );
}
