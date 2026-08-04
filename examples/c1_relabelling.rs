//! C1 — is this being's structure its own, or our names for it?
//!
//! The measurement for `docs/c1-relabelling.md`. C1-1 … C1-4b were locked in that document and
//! committed before this file existed.
//!
//! Ma & Kanai (arXiv:2606.06424) require, for a computational property to belong to a system **in
//! virtue of itself**, that it be specifiable **without an observer's labelling** and **invariant
//! under structure-preserving relabellings of the system's variables**. Most of what this project
//! calls the being's structure — twelve named channels, four hand-placed basin targets, an
//! author-set quality basis — is what they call **tier (i): interpreter-relative label selection.**
//!
//! This being is one of very few systems where C1 can be tested directly rather than argued about,
//! because every variable is readable and every trajectory replays exactly.
//!
//! Pure observer: a fresh being, public fields only, nothing changed, no journal written.
//! Survival first.
//!
//! Run: `cargo run --release --example c1_relabelling`

use unified_being::being::{Partner, UnifiedBeing};
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::q88::Q88_SCALE;

const LIFE: usize = 4_000;
const N: usize = 12;
const N_BASINS: usize = 4;
const BN: [&str; N_BASINS] = ["Rest", "Engaged", "Defensive", "Recovery"];
/// The labels this project has been using, from `examples/basins_probe.rs`.
const OURS: [&str; N] = [
    "0·disequilibrium", "1·anisotropy", "2·breach", "3·mean-tension",
    "4·arousal-set", "5·stability", "6·coherence", "7·trust",
    "8·arousal", "9·valence", "10·fatigue", "11·fe-velocity",
];

/// Deterministic PRNG — zero dependencies, reproducible, seeded explicitly.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn in_range(&mut self, lo: i16, hi: i16) -> i16 {
        lo + (self.next() % (hi - lo + 1).max(1) as u64) as i16
    }
}

fn q(f: f32) -> i16 {
    (f * Q88_SCALE as f32) as i16
}

/// Classify a field against a target set, exactly as `basins.rs` does: argmin L1.
fn classify(f: &[i16; N], targets: &[[i16; N]; N_BASINS], skip: Option<usize>) -> usize {
    (0..N_BASINS)
        .min_by_key(|&b| {
            (0..N)
                .filter(|c| Some(*c) != skip)
                .map(|c| (f[c] as i32 - targets[b][c] as i32).abs())
                .sum::<i32>()
        })
        .unwrap()
}

fn main() {
    println!("C1 — is this being's structure its own, or our names for it?");
    println!("(C1-1..C1-4b locked in docs/c1-relabelling.md, committed before this ran)\n");

    // ---- live one life, recording the whole field trajectory --------------------------
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    let mut world = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let partner = Partner { id: 1, reciprocation: q(0.90), exit_cost: q(0.3) };
    let targets = b.basins.targets;

    let mut trace: Vec<[i16; N]> = Vec::with_capacity(LIFE);
    let mut alive = true;
    for _ in 0..LIFE {
        let mut s = world.sense();
        s.partner = Some(partner);
        let r = b.step_embodied(&s);
        world.actuate(&intent_from(&r));
        trace.push(b.field.channel);
        if !r.alive {
            alive = false;
            break;
        }
    }
    let t = trace.len();
    println!("  SURVIVAL: {t} ticks, {}\n", if alive { "lived" } else { "DIED" });

    // ---- C1-1: exact duplicates --------------------------------------------------------
    println!("  C1-1 — does any pair of channels carry the SAME NUMBER?\n");
    println!("    {:<38} {:>12}", "pair", "ticks equal");
    println!("    {:-<38} {:->12}", "", "");
    let mut dupes: Vec<(usize, usize)> = Vec::new();
    for i in 0..N {
        for j in (i + 1)..N {
            let eq = trace.iter().filter(|f| f[i] == f[j]).count();
            if eq * 100 >= t * 90 {
                dupes.push((i, j));
                println!("    {:<38} {:>11.1}%",
                    format!("{} = {}", OURS[i], OURS[j]), eq as f32 * 100.0 / t as f32);
            }
        }
    }
    if dupes.is_empty() {
        println!("    (none above 90%)");
    }
    let four_eight = trace.iter().filter(|f| f[4] == f[8]).count();
    println!("\n    channels 4 and 8 specifically: equal on {:.1}% of ticks",
        four_eight as f32 * 100.0 / t as f32);
    println!("    {}", if four_eight * 100 >= t * 99 {
        "** C1-1 HOLDS. `field.rs:56` writes `b.arousal.raw.min(255)` and `field.rs:62` writes\n\
         \x20   `b.arousal.raw.clamp(0,255)` — the same number, twice. The 'twelve-channel somatic\n\
         \x20   field' has ELEVEN independent components. Every L1 basin distance double-counts\n\
         \x20   arousal, and docs/comfort.md §10's '~48% of the distance to Rest' is arithmetic\n\
         \x20   about a duplicated column, not a fact about arousal. **"
    } else {
        "C1-1 fails — 4 and 8 do diverge, so they are genuinely two channels."
    });

    // ---- C1-2: degenerate channels ------------------------------------------------------
    println!("\n  C1-2 — how many channels actually carry information over this life?\n");
    println!("    {:<38} {:>8} {:>8} {:>10}", "channel", "min", "max", "distinct");
    println!("    {:-<38} {:->8} {:->8} {:->10}", "", "", "", "");
    let mut constant = 0usize;
    for c in 0..N {
        let mut vals: Vec<i16> = trace.iter().map(|f| f[c]).collect();
        let lo = *vals.iter().min().unwrap();
        let hi = *vals.iter().max().unwrap();
        vals.sort_unstable();
        vals.dedup();
        if vals.len() <= 1 {
            constant += 1;
        }
        println!("    {:<38} {:>8} {:>8} {:>10}{}", OURS[c], lo, hi, vals.len(),
            if vals.len() <= 1 { "   <- CONSTANT" } else if vals.len() <= 3 { "   <- near-constant" } else { "" });
    }
    let effective = N - dupes.len() - constant;
    println!("\n    exact duplicates: {}   constant channels: {}", dupes.len(), constant);
    println!("    effective dimensionality: **{effective} of {N}**");
    println!("    {}", if constant >= 2 {
        "** C1-2 HOLDS — two or more channels carry nothing at all in an ordinary life. **"
    } else {
        "C1-2 fails as stated: fewer than two channels are constant. The field is less degenerate \
         than I guessed, and the guess is recorded as wrong."
    });

    // ---- C1-3: our own published claim --------------------------------------------------
    println!("\n  C1-3 — was 'arousal is dead weight in the classifier' ever about AROUSAL?\n");
    println!("    (leave-one-out over all twelve, on the same lived trajectory)\n");
    println!("    {:<38} {:>18}", "channel removed", "winner changed");
    println!("    {:-<38} {:->18}", "", "");
    let mut flips = [0usize; N];
    for f in &trace {
        let base = classify(f, &targets, None);
        for c in 0..N {
            if classify(f, &targets, Some(c)) != base {
                flips[c] += 1;
            }
        }
    }
    let mut ranked: Vec<usize> = (0..N).collect();
    ranked.sort_by_key(|&c| std::cmp::Reverse(flips[c]));
    for &c in &ranked {
        println!("    {:<38} {:>17.2}%", OURS[c], flips[c] as f32 * 100.0 / t as f32);
    }
    let worst = flips.iter().max().copied().unwrap_or(0);
    println!("\n    {}", if worst * 100 <= t {
        "** C1-3 HOLDS. EVERY channel is dead weight by this measure — none changes the winner on\n\
         \x20   more than 1% of ticks. So 'arousal is dead weight' got its content from the LABEL.\n\
         \x20   The honest sentence is 'no channel decides the mode', which is what\n\
         \x20   examples/arousal_range actually found, and which I then reported under a name that\n\
         \x20   made it sound like a discovery about arousal. **"
    } else {
        "C1-3 fails — at least one channel really does decide the classification, so naming it was \
         legitimate. Which one is in the table above."
    });

    // ---- C1-4a: the sanity check ---------------------------------------------------------
    println!("\n  C1-4a — permute field AND targets together: classification must be unchanged.\n");
    let mut rng = Lcg(0x5EED);
    let mut perm: [usize; N] = core::array::from_fn(|i| i);
    for i in (1..N).rev() {
        let j = (rng.next() % (i as u64 + 1)) as usize;
        perm.swap(i, j);
    }
    let mut ptargets = [[0i16; N]; N_BASINS];
    for bi in 0..N_BASINS {
        for c in 0..N {
            ptargets[bi][perm[c]] = targets[bi][c];
        }
    }
    let mut agree = 0usize;
    for f in &trace {
        let mut pf = [0i16; N];
        for c in 0..N {
            pf[perm[c]] = f[c];
        }
        if classify(&pf, &ptargets, None) == classify(f, &targets, None) {
            agree += 1;
        }
    }
    println!("    permutation {perm:?}");
    println!("    classification identical on {:.1}% of ticks", agree as f32 * 100.0 / t as f32);
    println!("    {}", if agree == t {
        "** C1-4a HOLDS exactly. The classifier is label-symmetric — it depends on the field and \
         the chart, never on which index is which. **"
    } else {
        "** C1-4a FAILS — the classifier has an index dependence. That is a bug, not a finding. **"
    });

    // ---- C1-4b: is basin membership intrinsic? -------------------------------------------
    println!("\n  C1-4b — re-place the four targets at RANDOM and re-classify the same life.\n");
    let modal = |tg: &[[i16; N]; N_BASINS]| -> (usize, f32) {
        let mut occ = [0usize; N_BASINS];
        for f in &trace {
            occ[classify(f, tg, None)] += 1;
        }
        let m = (0..N_BASINS).max_by_key(|&b| occ[b]).unwrap();
        (m, occ[m] as f32 * 100.0 / t as f32)
    };
    let (our_modal, our_share) = modal(&targets);
    println!("    with OUR chart:  modal basin = {} ({:.1}% of the life)", BN[our_modal], our_share);

    // Random charts drawn from the range the being's own channels actually occupy, so the
    // comparison is fair — targets somewhere the being could plausibly be measured against.
    let mut lo = [i16::MAX; N];
    let mut hi = [i16::MIN; N];
    for f in &trace {
        for c in 0..N {
            lo[c] = lo[c].min(f[c]);
            hi[c] = hi[c].max(f[c]);
        }
    }
    const CHARTS: usize = 200;
    let mut same_modal = 0usize;
    let mut modal_counts = [0usize; N_BASINS];
    let mut rest_reachable = 0usize;
    for _ in 0..CHARTS {
        let mut tg = [[0i16; N]; N_BASINS];
        for bi in 0..N_BASINS {
            for c in 0..N {
                tg[bi][c] = rng.in_range(lo[c], hi[c].max(lo[c]));
            }
        }
        let (m, _) = modal(&tg);
        modal_counts[m] += 1;
        if m == our_modal {
            same_modal += 1;
        }
        // Under this chart, is basin 0 ("Rest") ever entered at all?
        if trace.iter().any(|f| classify(f, &tg, None) == 0) {
            rest_reachable += 1;
        }
    }
    println!("\n    over {CHARTS} random charts drawn from the being's own occupied range:\n");
    for bi in 0..N_BASINS {
        println!("      slot {} ({:<10}) is modal on {:>5.1}% of charts", bi,
            BN[bi], modal_counts[bi] as f32 * 100.0 / CHARTS as f32);
    }
    println!("\n      same modal slot as our chart: {:.1}%",
        same_modal as f32 * 100.0 / CHARTS as f32);
    println!("      slot 0 (\"Rest\") entered at least once: {:.1}% of charts",
        rest_reachable as f32 * 100.0 / CHARTS as f32);

    println!("\n    {}", if same_modal * 2 < CHARTS {
        "** C1-4b HOLDS. The being's modal basin changes under most random charts. So 'the being \
         is Engaged' is a fact about\n\
         \x20   THE CHART WE DREW, not a property the being has in virtue of itself. Basin \
         membership fails C1.\n\
         \x20   And if slot 0 is entered under many random charts, then 'the being never rests' \
         is likewise a statement\n\
         \x20   about where we put Rest — not about the being. **"
    } else {
        "C1-4b fails — the modal basin is stable across random charts, so the classification is \
         picking up something about the being's own field rather than about our target placement. \
         That would be the strongest pro-intrinsic result available here."
    });

    println!("\n  The founded being was not touched. A fresh being; public fields read; nothing changed.");
}
