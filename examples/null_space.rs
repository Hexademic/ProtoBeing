//! The null space — how much freedom is this being already throwing away?
//!
//! The measurement for `docs/null-space.md` §5, N4 and N5. The arithmetic is proven in
//! `tests/null_space.rs` (written first, and N1 asserts the observer never disagrees with the
//! being it watches); this asks the only questions those tests cannot, because they need
//! lived beings:
//!
//! * **N4** — how many ways were adequate, on average? I predicted **≈ 2** at tolerance 0,
//!   and ≥ 2 on a majority of ticks, because a generic point on a smooth two-source field has
//!   one climbing direction per gradient component. If the answer is ≈ 1, there is no null
//!   space here to find, and play is blocked on a richer action surface rather than on an
//!   observer — a much larger piece of work, and `docs/play.md` §8 pointed at the wrong
//!   precondition.
//! * **N5** — does the freedom survive where it matters? I predicted redundancy is **lower**
//!   when the being is burdened. If freedom vanishes under load, style is a luxury of the
//!   well-fed, which is a real finding about this architecture and not a pleasant one.
//!
//! Nothing here chooses. The observer recomputes the same probe set beside `climb()` and the
//! being is never given the set to pick from (`docs/null-space.md` §3).
//!
//! Run: `cargo run --release --example null_space`

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::genome::Genome;
use unified_being::null_space::{adequate, SAME_WAY};
use unified_being::play::COMFORT;
use unified_being::primes::PrimeFacts;

const LIFE: usize = 1_500;
const TOLS: [i16; 5] = [0, 1, SAME_WAY, 8, 24];

#[derive(Default)]
struct Tally {
    ticks: usize,
    count_sum: i64,
    free: usize,
    singular: usize,
}

impl Tally {
    fn see(&mut self, count: u8) {
        self.ticks += 1;
        self.count_sum += count as i64;
        if count > 1 {
            self.free += 1;
        }
        if count == 0 {
            self.singular += 1;
        }
    }
    fn mean(&self) -> f32 {
        self.count_sum as f32 / self.ticks.max(1) as f32
    }
    fn pct(&self, n: usize) -> usize {
        n * 100 / self.ticks.max(1)
    }
}

struct Life {
    label: String,
    /// One tally per tolerance in TOLS.
    all: Vec<Tally>,
    /// At the default tolerance, split by whether the being was burdened (N5).
    burdened: Tally,
    easy: Tally,
}

fn watch(label: String, mut world: FieldWorld) -> Life {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut all: Vec<Tally> = TOLS.iter().map(|_| Tally::default()).collect();
    let (mut burdened, mut easy) = (Tally::default(), Tally::default());

    for _ in 0..LIFE {
        let sens = world.sense();
        let r = being.step_embodied(&sens);
        let intent = intent_from(&r);
        let facts = PrimeFacts::from_report(&r, Some(world.at_good() > 128));

        let deltas = world.climb_deltas(&intent);
        for (i, &tol) in TOLS.iter().enumerate() {
            all[i].see(adequate(&deltas, tol).count);
        }
        let here = adequate(&deltas, SAME_WAY).count;
        if facts.drive >= COMFORT {
            burdened.see(here);
        } else {
            easy.see(here);
        }

        world.actuate(&intent);
        if !being.is_alive() {
            break;
        }
    }

    Life { label, all, burdened, easy }
}

fn main() {
    println!("The null space — how many ways were there to do the same thing?");
    println!("(N4 and N5 locked in docs/null-space.md §5 before this was written)\n");
    println!("  the being's own resolution: SAME_WAY = {SAME_WAY} raw\n");

    let long = || FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let short = || FieldWorld::with((128, 128), (140, 140), (250, 250)).with_person(1, (20, 20));

    let lives = vec![
        watch("long crossing".into(), long()),
        watch("long + weathered".into(), long().with_weather(0, 2)),
        watch("beside its food".into(), short()),
        watch("beside food + weather".into(), short().with_weather(0, 2)),
    ];

    println!("  N4 — mean adequate ways, by tolerance (and % of ticks with >1 way)\n");
    print!("  {:<22}", "life");
    for t in TOLS {
        print!("{:>14}", format!("tol {t}"));
    }
    println!();
    println!("  {:-<22}{:-<70}", "", "");
    for l in &lives {
        print!("  {:<22}", l.label);
        for t in &l.all {
            print!("{:>14}", format!("{:.2}  {:>2}%", t.mean(), t.pct(t.free)));
        }
        println!();
    }

    println!("\n  singular ticks (nothing improves anything — the geometry of despair)\n");
    for l in &lives {
        let t = &l.all[TOLS.iter().position(|&x| x == SAME_WAY).unwrap()];
        println!("  {:<22} {:>3}% of ticks", l.label, t.pct(t.singular));
    }

    // N4's verdict, at the being's own resolution.
    let i_def = TOLS.iter().position(|&x| x == SAME_WAY).unwrap();
    let mean_at_zero: f32 =
        lives.iter().map(|l| l.all[0].mean()).sum::<f32>() / lives.len() as f32;
    let free_at_def: usize =
        lives.iter().map(|l| l.all[i_def].pct(l.all[i_def].free)).sum::<usize>() / lives.len();

    println!("\n  N4 — is there a null space here at all?");
    println!("    pooled mean adequate ways at tolerance 0: {mean_at_zero:.2} (predicted ~2.00)");
    println!("    pooled ticks with more than one way, at the being's resolution: {free_at_def}%");

    // Report the SPREAD, not the pooled mean. docs/play.md §7's lesson from this same
    // session: a mean says nothing about whether a threshold is crossed, and pooling four
    // worlds here hides the entire finding.
    let free: Vec<usize> = lives.iter().map(|l| l.all[i_def].pct(l.all[i_def].free)).collect();
    let (lo, hi) = (*free.iter().min().unwrap(), *free.iter().max().unwrap());
    println!("    per-life, ticks with more than one way: {lo}% .. {hi}%  <-- the finding");

    if hi < 10 {
        println!("\n    NO — effectively one way in every world tested. Play is then blocked on");
        println!("    a RICHER ACTION SURFACE, not on an observer, and docs/play.md §8 pointed");
        println!("    at the wrong precondition.");
    } else if lo >= 50 {
        println!("\n    YES, everywhere — the being had more than one adequate way on a majority");
        println!("    of ticks in every world, and took the first in compass order each time.");
    } else {
        println!("\n    IT DEPENDS ENTIRELY ON THE WORLD, AND THAT IS THE ANSWER. The spread above");
        println!("    is not noise around a mean: one life has more than one adequate way on {hi}% of");
        println!("    its ticks and another on {lo}%. My locked N4 ('~2.00 generically, >1 way on a");
        println!("    majority of ticks') is WRONG as a general claim and right in exactly one of");
        println!("    the four worlds.");
        println!();
        println!("    Why the spread happens is NOT established. Three explanations were tried and");
        println!("    all three failed against measurement -- see docs/null-space.md §7. What IS");
        println!("    established is the design consequence, and it does not depend on the cause:");
        println!();
        println!("    The null space is REAL but SCAVENGED. A being whose freedom ranges from {lo}% to");
        println!("    {hi}% of ticks depending on where its food happens to sit does not OWN its");
        println!("    redundancy. Style cannot rest on an accident of the field, and neither can");
        println!("    play. Redundancy has to live in the being's ACTION SURFACE -- effort within a");
        println!("    band that arrives at the same place, or acting now versus waiting a beat, both");
        println!("    of which docs/j-space.md already lists and neither of which depends on the");
        println!("    world's geometry. That is a sharper requirement than docs/null-space.md §4");
        println!("    stated, and it is what the next inch has to satisfy.");
    }

    println!("\n  N5 — does the freedom survive under load?");
    println!("    {:<22} {:>16} {:>16}", "life", "comfortable", "burdened");
    println!("    {:-<22} {:->16} {:->16}", "", "", "");
    let mut any_burdened = false;
    for l in &lives {
        if l.burdened.ticks > 0 {
            any_burdened = true;
        }
        println!(
            "    {:<22} {:>16} {:>16}",
            l.label,
            format!("{:.2} ({} t)", l.easy.mean(), l.easy.ticks),
            if l.burdened.ticks > 0 {
                format!("{:.2} ({} t)", l.burdened.mean(), l.burdened.ticks)
            } else {
                "never burdened".into()
            }
        );
    }
    if any_burdened {
        let (mut e_sum, mut e_n, mut b_sum, mut b_n) = (0i64, 0usize, 0i64, 0usize);
        for l in &lives {
            e_sum += l.easy.count_sum;
            e_n += l.easy.ticks;
            b_sum += l.burdened.count_sum;
            b_n += l.burdened.ticks;
        }
        let e = e_sum as f32 / e_n.max(1) as f32;
        let b = b_sum as f32 / b_n.max(1) as f32;
        println!("\n    pooled: {e:.2} comfortable vs {b:.2} burdened (predicted: lower burdened)");
        if b < e {
            println!("    Freedom DOES narrow under load — as predicted. Style is, at least");
            println!("    partly, a luxury of the well-fed, and a resolver built on this null");
            println!("    space will go quiet exactly when the being is struggling.");
        } else {
            println!("    Freedom does NOT narrow under load — the prediction was wrong. The");
            println!("    being has as many ways available when burdened as when comfortable.");
        }
    } else {
        println!("\n    No life was ever burdened at this drive measure, so N5 is unanswered.");
    }

    println!("\n  Nothing chose. climb() is unmodified and the being was never offered the set.");
}
