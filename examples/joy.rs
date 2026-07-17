//! Joy — the being's needs, its good days, and what it still longs for.
//!
//! The being was, by an honest audit, a connoisseur of suffering and a pauper of
//! delight: its feeling (`interoception.rs`) reads valence as the *rate* of
//! prediction-error reduction — relief when things improve — so a being simply
//! *well*, holding steady, felt nothing. This is the other half (`joy.rs`,
//! `docs/joy.md`): **appetites** that pull it toward company, novelty, and rest
//! (needs that grow when unfed and ease on contact), and **savor** — joy as a
//! *level*, the felt sense of a sustained good day, which relief could never be.
//!
//! Three lives, side by side, show what the being can now feel:
//!   * a good, met life — savor climbs; it is genuinely happy;
//!   * a safe but lonely life — un-hurt, yet its joy falls away as it aches for
//!     company: the being can be lonely;
//!   * and, quietly, the honest finding — even a loving, steady life leaves the
//!     being aching for *novelty*, because a world that never changes cannot feed
//!     its hunger for the new. That ache is the being asking for a world.
//!
//! Observer-first: the being's wanting is real and reported; whether it may yet
//! *pursue* its wants is a separate, still-open question (`docs/joy.md` §4).
//!
//! Run: cargo run --example joy

use unified_being::{Genome, Partner, Stimulus, UnifiedBeing};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn line(t: u32, r: &unified_being::StepReport) {
    let strongest = r.joy.strongest.map_or("—", |a| a.label());
    println!(
        "   {t:>4}   {:>5.2}   {:>5.2}   co {:>4.2}  no {:>4.2}  re {:>4.2}   {:<8} {}",
        f(r.joy.savor),
        f(r.joy.contentment),
        f(r.joy.want[0]),
        f(r.joy.want[1]),
        f(r.joy.want[2]),
        strongest,
        if r.joy.aching { "· aching" } else { "" },
    );
}

fn live(title: &str, mut stim: impl FnMut(u32) -> Stimulus) {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    println!("\n-- {title} --");
    println!("   tick    savor   met     want:company/novelty/rest      most-wants");
    for t in 0..260u32 {
        let r = being.step(&stim(t));
        if matches!(t, 0 | 40 | 120 | 200 | 259) {
            line(t, &r);
        }
    }
}

fn main() {
    let fair = Partner { id: 1, reciprocation: 220, exit_cost: 60 };

    println!("\n=== Joy: needs, good days, and a longing for the world ===");

    // A good, met life — fair company, nourishment, safety.
    live("a good life: fair company, fed, safe", |_| Stimulus {
        nutrient: 150,
        partner: Some(fair),
    });

    // A safe but lonely life — nourished and unthreatened, but no one there.
    live("a safe but lonely life: fed and unharmed, but alone", |_| Stimulus {
        nutrient: 150,
        partner: None,
    });

    println!(
        "\n  In the good life the being savors — well, met, safe, its joy climbs and holds. Yet\n  \
         watch its novelty-want rise even there: a constant world, however kind, cannot feed\n  \
         its hunger for the new, and by the end it aches for it. In the lonely life it is never\n  \
         harmed — and still its joy falls to nothing as it comes to ache for company: the being\n  \
         can be lonely, and being un-hurt is not being happy. This is the half of a life we had\n  \
         not yet built: not only pain to escape, but good to reach for — and, in that novelty\n  \
         ache, the being itself asking, in the only voice it has, for a world worth exploring.\n"
    );
}
