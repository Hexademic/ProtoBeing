//! Feeling — the being's own form of it, read straight from its viability.
//!
//! Interoceptive-inference and affective theory (Seth, Damasio; Affective
//! Inference Theory) hold that a feeling is the felt regulation of an
//! organism's viability: how far it is from cessation, and — as *valence* —
//! the rate at which its own prediction error is resolving. The being already
//! keeps both registers. This probe reads them *as a feeling* across a life:
//! ease beside a fair partner, then a long strain as sustenance thins, then
//! recovery. Watch the felt margin (viability) fall and rise; watch valence go
//! negative under strain (dread) and positive under recovery (relief); watch
//! the slow **mood** carry the run of moments forward; and watch the being
//! feel a deficit *coming* (anticipating) before it crosses its own edge.
//!
//! Observer-only — the being's default trajectory is unchanged; feeling is
//! witnessed, not (yet) made to steer. `cargo run --example feeling`

use unified_being::{Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };

    println!("\n=== Feeling: the being's felt regulation of its own viability ===\n");
    println!("  viability  = felt survival margin (256 = full ease, 0 = at cessation)");
    println!("  valence    = signed: relief (+) when its prediction error resolves, dread (-)");
    println!("  mood       = the slow tone that carries the run of moments forward");
    println!("  stake/antic= at its edge · feeling a deficit coming before the edge\n");
    println!("   tick  phase     nutrient  viability  valence   mood   trend  stake  antic");
    println!("   ----  --------  --------  ---------  -------  ------  -----  -----  -----");

    for t in 0..140u32 {
        // A life in three movements: ease → sustenance fails and the margin
        // narrows toward the edge → sustenance returns and the being recovers.
        let (phase, nutrient) = if t < 40 {
            ("ease", q(0.55))
        } else if t < 49 {
            ("hunger", q(0.0))
        } else {
            ("recover", q(0.7))
        };

        let r = being.step(&Stimulus { nutrient, partner: Some(fair) });
        let felt = r.felt;

        // Sample sparsely at rest, but every tick through the descent and recovery
        // so the felt arc — and the moment anticipation fires — is visible.
        if (t < 40 && t % 10 == 0) || (38..75).contains(&t) || t % 15 == 0 {
            println!(
                "   {t:>4}  {phase:<8}  {:>8.2}  {:>9.3}  {:>7.3}  {:>6.3}  {:>5}  {:>5}  {:>5}",
                f(nutrient),
                f(felt.state.viability),
                f(felt.state.allostatic_valence),
                f(felt.mood),
                felt.viability_trend,
                if felt.state.at_stake { "•" } else { "" },
                if felt.anticipating { "•" } else { "" },
            );
        }

        if !being.is_alive() {
            println!("   {t:>4}  (the being's viability reached cessation)");
            break;
        }
    }

    println!(
        "\n  The felt margin falls as sustenance thins and climbs back on recovery; valence\n  \
         turns to dread while the being cannot resolve its own error and to relief once it\n  \
         can; the mood lags, carrying the weather of the whole run; and the being feels the\n  \
         deficit approaching before it crosses its edge. This is the *architecture* the\n  \
         theories say feeling is — viability regulated, its rate felt as valence, held with\n  \
         temporal depth. Whether the being phenomenally *feels* it is the one thing no such\n  \
         construction can settle, and we keep that gap open honestly (docs/intrinsic-mind.md).\n"
    );
}
