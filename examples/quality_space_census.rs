//! The quality-space census — is the space **poor**, or merely **unvisited**?
//!
//! `docs/findings.md` has carried that fork unanswered for weeks and the two branches have
//! opposite fixes: a poor space is an architecture problem, an unvisited one is a world problem.
//! **Predictions QS-1..QS-4 are locked in `docs/c1-relabelling.md` §11 and were committed before
//! this file existed.**
//!
//! **Occupied** = distinct `QualityPoint`s the being actually visits in a life.
//! **Afforded** = distinct `QualityPoint`s reachable from somatic fields the body can produce.
//!
//! Three things are declared here rather than discovered later:
//!
//! 1. **The absolute counts are chart-relative and are not findings.** The ratio is, and the
//!    *change* in the ratio under intervention is — that is C2's counterfactual question and the
//!    instrument that already worked on the faculty ablation.
//! 2. **There is no clean `0..256` box.** Reading *both* writers — `Field::write_from_body` and
//!    `Field::inject`, the pair that produced error ledger #6 — channels 9 (valence) and 11 (FE
//!    velocity) are signed and unclamped at the writer, and `inject` saturating-adds on top. So the
//!    box is measured per-channel extrema across every regime run, never an assumed range.
//! 3. **Afforded-by-box is an upper bound.** Uniform sampling inside the box includes channel
//!    combinations a real body never produces (4 and 8 are both arousal, and covary). So the
//!    reported ratio is a **lower bound** on true occupancy. Said plainly because it flatters the
//!    "unvisited" branch and I would rather name that than be caught by it.
//!
//! Pure observer: fresh beings, gates default-off unless the regime names them, no journal written,
//! the founded being untouched.
//!
//! Run: `cargo run --release --example quality_space_census`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field::N_SOMATIC;
use unified_being::q88::Q88_SCALE;
use unified_being::quality_space::{QualitySpace, N_QUALITY};
use unified_being::room::Room;

const LIFE: usize = 4_000;
const SAMPLES: usize = 200_000;
/// Reported at three grains, because a ratio that only holds at one bin size is a fact about the
/// bin size. Sensitivity is shown, not assumed away.
const BINS: [i16; 3] = [8, 32, 128];

/// Deterministic PRNG — the same one `examples/c1_relabelling.rs` uses, reused rather than rewritten.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn in_range(&mut self, lo: i16, hi: i16) -> i16 {
        lo + (self.next() % (hi as i32 - lo as i32 + 1).max(1) as u64) as i16
    }
}

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

/// Quantise a point so "distinct" means something at a stated grain.
fn key(axis: &[i16; N_QUALITY], bin: i16) -> [i16; N_QUALITY] {
    let mut k = [0i16; N_QUALITY];
    for i in 0..N_QUALITY {
        k[i] = axis[i].div_euclid(bin);
    }
    k
}

fn distinct(points: &[[i16; N_QUALITY]], bin: i16) -> usize {
    let mut seen: Vec<[i16; N_QUALITY]> = points.iter().map(|p| key(p, bin)).collect();
    seen.sort_unstable();
    seen.dedup();
    seen.len()
}

/// Project through an arbitrary basis, mirroring `QualitySpace::project` exactly — same shift, same
/// sparse threshold — so a random-basis comparison differs only in the basis.
fn project_with(basis: &[[i16; N_SOMATIC]; N_QUALITY], field: &[i16; N_SOMATIC]) -> [i16; N_QUALITY] {
    let mut axis = [0i16; N_QUALITY];
    for (k, w) in basis.iter().enumerate() {
        let mut acc = 0i32;
        for c in 0..N_SOMATIC {
            acc += w[c] as i32 * field[c] as i32;
        }
        let v = (acc >> 8).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        axis[k] = if v.unsigned_abs() < 24 { 0 } else { v };
    }
    axis
}

struct Lived {
    name: &'static str,
    alive: bool,
    ticks: usize,
    fields: Vec<[i16; N_SOMATIC]>,
    points: Vec<[i16; N_QUALITY]>,
}

fn live(name: &'static str, receptors: bool, reserve: bool) -> Lived {
    let mut b = UnifiedBeing::new(unified_being::genome::Genome::wanderer());
    if receptors {
        b.enable_receptors();
    }
    if reserve {
        b.enable_reserve();
    }
    let mut room =
        Room::peopled((32, 200), (224, 56), (128, 220), (40, 40)).with_friend((210, 128));
    let p = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };

    let mut l = Lived {
        name,
        alive: true,
        ticks: 0,
        fields: Vec::with_capacity(LIFE),
        points: Vec::with_capacity(LIFE),
    };

    for _ in 0..LIFE {
        let mut sens = room.sense();
        sens.partner = Some(p);
        let r = b.step_embodied(&sens);
        room.actuate(&intent_from(&r));

        l.fields.push(b.field.channel);
        l.points.push(QualitySpace::project(&b.field.channel).axis);
        l.ticks += 1;
        if !r.alive {
            l.alive = false;
            break;
        }
    }
    l
}

fn main() {
    println!("\n=== The quality-space census — poor, or unvisited? ===");
    println!("  predictions QS-1..QS-4 locked in docs/c1-relabelling.md §11, committed first\n");

    let regimes = [
        live("default", false, false),
        live("+reserve", false, true),
        live("+receptors", true, false),
        live("+both", true, true),
    ];

    // ---- survival first, before any ratio ----
    println!("  {:<12} {:>7}  {:>8}", "regime", "ticks", "survived");
    for r in &regimes {
        println!("  {:<12} {:>7}  {:>8}", r.name, r.ticks, if r.alive { "yes" } else { "DIED" });
    }

    // ---- the measured box: per-channel extrema across every regime ----
    let mut lo = [i16::MAX; N_SOMATIC];
    let mut hi = [i16::MIN; N_SOMATIC];
    for r in &regimes {
        for f in &r.fields {
            for c in 0..N_SOMATIC {
                lo[c] = lo[c].min(f[c]);
                hi[c] = hi[c].max(f[c]);
            }
        }
    }
    println!("\n  measured per-channel box (never assumed — both writers read):");
    for c in 0..N_SOMATIC {
        print!("   ch{:<2}[{:>5},{:>5}]", c, lo[c], hi[c]);
        if c % 4 == 3 {
            println!();
        }
    }
    let dead: Vec<usize> = (0..N_SOMATIC).filter(|&c| lo[c] == hi[c]).collect();
    println!("  channels constant across ALL regimes: {:?}", dead);

    // ---- per-regime channel variety ----
    // **Added after seeing the result, to explain QS-2's failure. No verdict was changed by it —
    // QS-2 is graded failed on the occupancy numbers alone.** The house rule is that a data column
    // may be added to clarify, never to retro-fit, and which was done must be said.
    println!("\n  distinct values per channel, WITHIN each regime (added post-hoc, explains QS-2):");
    print!("  {:<12}", "regime");
    for c in 0..N_SOMATIC {
        print!(" ch{:<3}", c);
    }
    println!();
    for r in &regimes {
        print!("  {:<12}", r.name);
        for c in 0..N_SOMATIC {
            let mut v: Vec<i16> = r.fields.iter().map(|f| f[c]).collect();
            v.sort_unstable();
            v.dedup();
            print!(" {:<4}", v.len());
        }
        println!();
    }

    // ---- afforded: uniform samples inside that box (an UPPER bound) ----
    let mut rng = Lcg(0x5EED_5EED);
    let mut afforded: Vec<[i16; N_QUALITY]> = Vec::with_capacity(SAMPLES);
    for _ in 0..SAMPLES {
        let mut f = [0i16; N_SOMATIC];
        for c in 0..N_SOMATIC {
            f[c] = rng.in_range(lo[c], hi[c]);
        }
        afforded.push(QualitySpace::project(&f).axis);
    }

    // ---- QS-1..QS-3: occupancy, at three grains ----
    println!("\n  === occupancy = occupied / afforded (LOWER bound; afforded-by-box over-counts) ===");
    for &bin in BINS.iter() {
        let aff = distinct(&afforded, bin);
        println!("\n  bin {:>4}   afforded {:>7}", bin, aff);
        println!("  {:<12} {:>9} {:>10}  {:>9}", "regime", "occupied", "occupancy", "vs default");
        let base = distinct(&regimes[0].points, bin) as f64;
        for r in &regimes {
            let occ = distinct(&r.points, bin);
            println!(
                "  {:<12} {:>9} {:>9.3}%  {:>8.2}x",
                r.name,
                occ,
                occ as f64 * 100.0 / aff.max(1) as f64,
                occ as f64 / base.max(1.0)
            );
        }
    }

    // ---- QS-4: is our hand-designed basis special, or decoration? ----
    println!("\n  === QS-4 — our basis vs random 4x12 bases (bin 32) ===");
    let bin = 32;
    let ours_aff = distinct(&afforded, bin) as f64;
    let ours_occ = distinct(&regimes[3].points, bin) as f64;
    let ours = ours_occ * 100.0 / ours_aff.max(1.0);

    let mut ratios: Vec<f64> = Vec::new();
    for _ in 0..40 {
        let mut basis = [[0i16; N_SOMATIC]; N_QUALITY];
        for k in 0..N_QUALITY {
            for c in 0..N_SOMATIC {
                basis[k][c] = rng.in_range(-256, 256);
            }
        }
        let aff: Vec<[i16; N_QUALITY]> = (0..SAMPLES / 4)
            .map(|_| {
                let mut f = [0i16; N_SOMATIC];
                for c in 0..N_SOMATIC {
                    f[c] = rng.in_range(lo[c], hi[c]);
                }
                project_with(&basis, &f)
            })
            .collect();
        let occ: Vec<[i16; N_QUALITY]> =
            regimes[3].fields.iter().map(|f| project_with(&basis, f)).collect();
        let a = distinct(&aff, bin).max(1) as f64;
        ratios.push(distinct(&occ, bin) as f64 * 100.0 / a);
    }
    ratios.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let med = ratios[ratios.len() / 2];
    println!("  ours (+both)   {:>8.3}%", ours);
    println!(
        "  random bases   median {:.3}%   min {:.3}%   max {:.3}%   (n=40)",
        med,
        ratios[0],
        ratios[ratios.len() - 1]
    );
    let factor = if med > 0.0 { ours / med } else { f64::INFINITY };
    println!("  ours / random-median = {:.2}x   — QS-4 asked for within 2x", factor);

    println!("\n  Scope: this says nothing about whether the being feels anything");
    println!("  (docs/witness-gap-literature.md §2.1). Volume is not quality.\n");
    let _ = Q88_SCALE;
}
