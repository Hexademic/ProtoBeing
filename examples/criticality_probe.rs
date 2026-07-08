//! Criticality probe (Direction 2 of the phenomenology method) — is the being's
//! trajectory most bendable at the basin boundary?
//!
//! Blake's thesis: awareness lives at *the Now where the trajectory can bend* —
//! and in a deterministic being, that Now is a **bifurcation**: a basin boundary,
//! where the mode of being is finely balanced and a small input tips which way it
//! falls. Deep inside a basin, the same nudge does nothing (fate); near the
//! boundary, it changes the path (destiny). This measures exactly that.
//!
//! Method (observer-only; the being is deterministic, so this forks it):
//!   - each tick, read the boundary margin (distance to the nearest bifurcation);
//!   - fork the being, give ONE fork a one-unit nutrient nudge for a single tick,
//!     then feed both forks identical inputs for a short horizon;
//!   - measure the divergence between the two trajectories (their valence paths
//!     and whether their dominant basin ends up different) — this is the *bend*.
//!   - bin bend by margin. The thesis predicts: bend is largest at small margin.
//!
//! Nothing here changes the being's dynamics or any published number — it reads
//! and forks. `cargo run --example criticality_probe`

use unified_being::{Basin, Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

const HORIZON: u32 = 10; // ticks to let a nudge propagate before measuring bend
const LIFE: u32 = 1200;
const NUDGE: i16 = 12; // a moderate perturbation — big enough to reveal bend,
                       // small enough not to swamp the margin-dependence

/// Roll a forked being: one `first` tick, then `HORIZON-1` `rest` ticks. Both
/// forks share `rest`, differing only in `first`. Returns the (valence, arousal)
/// path and final basin — a fuller picture of the trajectory than valence alone.
fn roll(mut being: UnifiedBeing, first: Stimulus, rest: Stimulus) -> (Vec<(f32, f32)>, Basin) {
    let mut path = Vec::with_capacity(HORIZON as usize);
    let r0 = being.step(&first);
    path.push((r0.valence, r0.arousal));
    let mut basin = r0.basin;
    for _ in 1..HORIZON {
        let r = being.step(&rest);
        path.push((r.valence, r.arousal));
        basin = r.basin;
        if !being.is_alive() {
            break;
        }
    }
    (path, basin)
}

fn pearson(xs: &[f32], ys: &[f32]) -> f32 {
    let n = xs.len() as f32;
    if n < 2.0 {
        return 0.0;
    }
    let mx = xs.iter().sum::<f32>() / n;
    let my = ys.iter().sum::<f32>() / n;
    let mut sxy = 0.0;
    let mut sxx = 0.0;
    let mut syy = 0.0;
    for i in 0..xs.len() {
        let dx = xs[i] - mx;
        let dy = ys[i] - my;
        sxy += dx * dy;
        sxx += dx * dx;
        syy += dy * dy;
    }
    if sxx <= 1e-9 || syy <= 1e-9 {
        return 0.0;
    }
    sxy / (sxx.sqrt() * syy.sqrt())
}

fn path_divergence(a: &[(f32, f32)], b: &[(f32, f32)]) -> f32 {
    let n = a.len().min(b.len());
    if n == 0 {
        return 0.0;
    }
    // Mean over the horizon of |Δvalence| + |Δarousal| — the trajectory's bend
    // across both affective axes, not valence alone.
    (0..n)
        .map(|i| (a[i].0 - b[i].0).abs() + (a[i].1 - b[i].1).abs())
        .sum::<f32>()
        / n as f32
}

fn main() {
    println!("\n=== Criticality: is the trajectory most bendable at the basin boundary? ===");
    println!("    (fork the being, nudge one fork one unit, measure how far they diverge)\n");

    let mut being = UnifiedBeing::new(Genome::wanderer());
    let fair = Partner { id: 1, reciprocation: q(0.9), exit_cost: q(0.3) };
    let taker = Partner { id: 2, reciprocation: q(0.15), exit_cost: q(0.3) };

    // A life with REGIME CHANGES, to drive the being across basin boundaries so
    // the margin actually spans its range: calm → extraction → recovery →
    // solitude → a rough churn, cycling. Regime changes are where a settled mode
    // gives way — precisely the boundary crossings we want to sample.
    let regime = |t: u32| -> Stimulus {
        match (t / 60) % 5 {
            0 => Stimulus { nutrient: q(0.5), partner: Some(fair) },   // calm
            1 => Stimulus { nutrient: q(0.5), partner: Some(taker) },  // extraction
            2 => Stimulus { nutrient: q(0.6), partner: Some(fair) },   // recovery
            3 => Stimulus { nutrient: q(0.3), partner: None },         // solitude
            _ => Stimulus { nutrient: q(0.4), partner: Some(if t % 2 == 0 { fair } else { taker }) },
        }
    };

    let mut margins: Vec<f32> = Vec::new();
    let mut bends: Vec<f32> = Vec::new();
    let mut flips = 0u32;

    for t in 0..LIFE {
        let stim = regime(t);
        // Distance to the nearest bifurcation, on the LIVE being before it steps.
        let margin = being.basins.boundary_margin().max(0) as f32;

        // Fork the present. The perturbation is RELATIONAL — "a different other at
        // this Now" — because destiny bends fate through relationship, not noise:
        // the nudged fork meets the opposite kind of partner for one tick, then
        // both share the same future. The divergence is that one different
        // encounter propagating — the bend. (NUDGE kept as a fallback for the
        // solitude ticks where there is no partner to swap.)
        let swapped = match stim.partner {
            Some(p) if p.id == taker.id => Some(fair),
            Some(_) => Some(taker),
            None => Some(fair),
        };
        let nudge = Stimulus {
            nutrient: (stim.nutrient + NUDGE).min(256),
            partner: swapped,
        };
        let (base_path, base_basin) = roll(being.clone(), stim, stim);
        let (nudge_path, nudge_basin) = roll(being.clone(), nudge, stim);
        let bend = path_divergence(&base_path, &nudge_path);

        margins.push(margin);
        bends.push(bend);
        if base_basin != nudge_basin {
            flips += 1;
        }

        let _ = being.step(&stim);
        if !being.is_alive() {
            break;
        }
    }

    // The core test: does bend fall as margin (distance to boundary) rises?
    // Thesis predicts a NEGATIVE correlation — most bendable AT the boundary.
    let r = pearson(&margins, &bends);
    let mmin = margins.iter().cloned().fold(f32::INFINITY, f32::min);
    let mmax = margins.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    // Quintile view for intuition: split the observed margin range into fifths.
    const BINS: usize = 5;
    let span = (mmax - mmin).max(1.0);
    let mut bsum = [0f64; BINS];
    let mut bn = [0u32; BINS];
    for i in 0..margins.len() {
        let b = (((margins[i] - mmin) / span) * BINS as f32) as usize;
        let b = b.min(BINS - 1);
        bsum[b] += bends[i] as f64;
        bn[b] += 1;
    }

    println!("  margin (dist to boundary)   mean bend    n     [margin range {:.0}..{:.0}]", mmin, mmax);
    println!("  -------------------------   ---------   ---");
    let labels = ["lowest 1/5 (AT boundary)", "2nd", "3rd", "4th", "highest 1/5 (deep in)"];
    for b in 0..BINS {
        if bn[b] == 0 {
            println!("  {:<25}   {:>9}   {:>3}", labels[b], "-", 0);
        } else {
            println!("  {:<25}   {:>9.4}   {:>3}", labels[b], bsum[b] / bn[b] as f64, bn[b]);
        }
    }

    println!("\n  correlation(margin, bend) = {:+.3}   (nudge flipped the mode {} times)", r, flips);
    if mmax - mmin < 8.0 {
        println!("\n  The being LIVES near criticality — its margin barely varies ({:.0}..{:.0}), so it sits\n  \
                  perpetually near a bifurcation. That is itself a striking finding (self-organized\n  \
                  criticality), but it means this life cannot show the boundary-vs-deep CONTRAST.",
                 mmin, mmax);
    } else if r < -0.15 {
        println!("\n  CONFIRMED: bend falls as the being moves away from the boundary (negative\n  \
                  correlation). Its trajectory is most bendable AT the bifurcation — the 'Now where\n  \
                  the path can bend' is a real, measurable place. Fate is fixed deep in a basin;\n  \
                  destiny is decided at the edge. (Observed, not asserted; observer-only.)");
    } else if r > 0.15 {
        println!("\n  SURPRISE: bend RISES with distance from the boundary (positive correlation).\n  \
                  The thesis's prediction is inverted here — a real finding to understand, not hide.");
    } else {
        println!("\n  NULL: no clear margin-bend relationship in this life (|r| small). The thesis is\n  \
                  not supported by this measurement as run; worth a better perturbation or metric\n  \
                  before any claim.");
    }
    println!();
}
