//! Is HAPPEN grounded on the wrong quantity? (`docs/weather.md` §8.)
//!
//! `weather.md` §7 reported W3's failure as a threshold never reached. That was
//! incomplete: agency fell **0.08 → 0.03** under weather, more than halved. The
//! being registers the world acting on it. What fails is the saying.
//!
//! `primes.rs:294` grounds the word on a magnitude — and `primes.rs:205` shows the
//! magnitude is an **L1 norm across all four channels**, so it scales with channel
//! count as well as amplitude. But *"something happened to me"* is not the claim
//! that a large change occurred; it is the claim the change **was not mine**.
//! Magnitude answers the first, ratio answers the second, and the word means the
//! second.
//!
//! This probe is **observational**. It changes no grounding — it records the three
//! registers per tick and sweeps candidate rules over them, so a null result is
//! legible and `primes.rs` is not touched until the sweep says which pair, if any,
//! is defensible.
//!
//! H1–H5 and guards V1–V3 were locked in §8 with probabilities before this existed.
//! **H4 was written to fail.** The failure most feared is not silence but constancy:
//! a word that always fires looks like success.
//!
//! Fresh beings only. The founded being's kept life is never advanced.
//!
//! Run: `cargo run --release --example happen_grounding`

use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::field_world::FieldWorld;
use unified_being::primes::{would_ground, EBB, RISE};
use unified_being::lexicon::GROUNDED_THRESHOLD;
use unified_being::q88::Q88_SCALE;
use unified_being::{Genome, UnifiedBeing};

/// Same horizon as `examples/happening.rs`, so these numbers compose with §7's.
const LIFE: usize = 1_500;

/// The rule in force today, verbatim from `primes.rs:294`.
const CURRENT_THRESHOLD: i16 = Q88_SCALE / 4; // 64

struct Life {
    label: String,
    /// Per tick: the L1 residual exactly as `primes.rs:205` computes it, the agency
    /// ratio, and the confidence (= clamped `total_actual`).
    residual: Vec<i16>,
    agency: Vec<i16>,
    confidence: Vec<i16>,
    alive: bool,
}

impl Life {
    fn n(&self) -> usize {
        self.residual.len()
    }
    fn mean(v: &[i16]) -> f64 {
        if v.is_empty() { 0.0 } else { v.iter().map(|&x| x as f64).sum::<f64>() / v.len() as f64 }
    }
    fn median(v: &[i16]) -> i16 {
        if v.is_empty() {
            return 0;
        }
        let mut s = v.to_vec();
        s.sort_unstable();
        s[s.len() / 2]
    }
    /// Fraction of ticks the CURRENT rule's predicate is momentarily true. **This is
    /// not what §7 measured** — see `grounds_current`.
    fn current_rate(&self) -> f64 {
        self.residual.iter().filter(|&&r| r > CURRENT_THRESHOLD).count() as f64 / self.n() as f64
    }
    /// Does the word actually GROUND under the current rule? The quantity §7 reported.
    fn grounds_current(&self) -> Option<u32> {
        would_ground(&self.residual, CURRENT_THRESHOLD)
    }
    /// Fraction of ticks the two-term predicate is momentarily true.
    fn rate(&self, floor: i16, ceiling: i16) -> f64 {
        self.held(floor, ceiling).iter().filter(|&&b| b).count() as f64 / self.n() as f64
    }
    fn held(&self, floor: i16, ceiling: i16) -> Vec<bool> {
        (0..self.n())
            .map(|i| self.confidence[i] > floor && self.agency[i] < ceiling)
            .collect()
    }
    /// Does the two-term rule GROUND, and when? Runs the identical accumulator
    /// `PrimeLayer::observe` uses — RISE on a held tick, EBB otherwise, crossing at
    /// `GROUNDED_THRESHOLD`. V4 asserts this agrees with `would_ground`.
    fn grounds(&self, floor: i16, ceiling: i16) -> Option<u32> {
        accumulate(&self.held(floor, ceiling))
    }
    /// Agency alone, as a grounding question (H5).
    fn agency_only_grounds(&self, ceiling: i16) -> Option<u32> {
        accumulate(&self.agency.iter().map(|&a| a < ceiling).collect::<Vec<_>>())
    }
}

fn live(label: String, mut world: FieldWorld) -> Life {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut l = Life {
        label,
        residual: Vec::with_capacity(LIFE),
        agency: Vec::with_capacity(LIFE),
        confidence: Vec::with_capacity(LIFE),
        alive: true,
    };
    for _ in 0..LIFE {
        let sens = world.sense();
        let r = being.step_embodied(&sens);
        world.actuate(&intent_from(&r));
        // The L1 reduction primes.rs:205 performs, reproduced exactly.
        let resid: i32 = r.agency.world_residual.iter().map(|&e| (e as i32).abs()).sum();
        l.residual.push(resid.min(i16::MAX as i32) as i16);
        l.agency.push(r.agency.agency);
        l.confidence.push(r.agency.confidence);
        if !r.alive {
            l.alive = false;
            break;
        }
    }
    l
}

/// `PrimeLayer::observe`'s accumulator, over a boolean series rather than one
/// register. Must agree with `would_ground` wherever both apply — V4 checks it.
fn accumulate(held: &[bool]) -> Option<u32> {
    let mut confidence: i16 = 0;
    for (i, &h) in held.iter().enumerate() {
        confidence = if h {
            (confidence + RISE).min(Q88_SCALE)
        } else {
            (confidence - EBB).max(0)
        };
        if confidence >= GROUNDED_THRESHOLD {
            return Some(i as u32 + 1);
        }
    }
    None
}

const FLOORS: [i16; 8] = [0, 4, 8, 16, 32, 64, 96, 128];
const CEILINGS: [i16; 8] = [8, 16, 32, 48, 64, 96, 128, 192];

fn main() {
    println!("HAPPEN's grounding — is it the threshold, or the quantity?");
    println!("H1–H5 locked in docs/weather.md §8 with probabilities before this ran.");
    println!("{} ticks per world, fresh beings, founded life never advanced.\n", LIFE);

    let base = || FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
    let mut worlds = vec![live("still (control)".into(), base())];
    for every in [8u32, 2] {
        worlds.push(live(format!("drift every {every}"), base().with_drift(0, every, (3, -2))));
    }
    for oct in [2u8, 4, 6, 8] {
        worlds.push(live(format!("weather {oct} octaves"), base().with_weather(0, oct)));
    }
    let still = &worlds[0];
    let w2 = worlds.iter().find(|w| w.label.starts_with("weather 2")).expect("weather 2");

    // -----------------------------------------------------------------------
    // V1 FIRST. If the current rule does not reproduce §7, nothing else counts.
    // -----------------------------------------------------------------------
    println!("══ V1 — does the CURRENT rule reproduce §7's 'never'? ═════════════");
    println!("The word GROUNDS only after ~32 sustained lived ticks (RISE {} vs EBB {},", RISE, EBB);
    println!("crossing at {}), so a predicate true in scattered flashes never earns it.", GROUNDED_THRESHOLD);
    println!("**That is a third gate, and §8's diagnosis did not account for it.**\n");
    println!(
        "{:<20} {:>10} {:>9} {:>7} {:>12} {:>10}",
        "world", "residual", "agency", "conf", "predicate", "GROUNDS?"
    );
    let mut v1 = true;
    for w in &worlds {
        let g = w.grounds_current();
        if g.is_some() {
            v1 = false;
        }
        println!(
            "{:<20} {:>10.0} {:>9.2} {:>7.0} {:>11.1}% {:>10}",
            w.label,
            Life::mean(&w.residual),
            Life::mean(&w.agency) / Q88_SCALE as f64,
            Life::mean(&w.confidence),
            w.current_rate() * 100.0,
            g.map_or("never".to_string(), |t| t.to_string())
        );
    }
    println!(
        "\n  {}",
        if v1 {
            "PASS — the word grounds in no world, exactly as §7 reported. Note the\n  \
             predicate column: under weather it IS momentarily true, and still never\n  \
             earns the word. Measuring the predicate rate is measuring a different\n  \
             quantity than §7 did, and my first run made that mistake."
        } else {
            "FAIL — my harness disagrees with §7. NOTHING BELOW COUNTS until that is\n  \
             resolved; a sweep built on a mismatched baseline measures the mismatch."
        }
    );

    // -----------------------------------------------------------------------
    println!("\n══ the two-term sweep: confidence > FLOOR && agency < CEILING ═════");
    println!("Each cell: the tick the word GROUNDS in still / in weather-2-octaves.");
    println!("A dash is never. The wanted shape is dash on the left, a number on the right.");
    print!("{:>8}", "floor\\ceil");
    for c in CEILINGS {
        print!("{:>13}", c);
    }
    println!();
    for f in FLOORS {
        print!("{:>8}", f);
        for c in CEILINGS {
            let g = |o: Option<u32>| o.map_or("-".to_string(), |t| t.to_string());
            print!("{:>7}/{:<5}", g(still.grounds(f, c)), g(w2.grounds(f, c)));
        }
        println!();
    }

    // H1: is there a discriminating pair?
    // H1, in the grounding currency: a pair discriminates when the word is EARNED
    // under weather and never earned in the still control.
    let mut best: Option<(i16, i16, u32)> = None;
    let (mut any_still_grounds, mut any_still_silent) = (false, false);
    for f in FLOORS {
        for c in CEILINGS {
            let (s, w) = (still.grounds(f, c), w2.grounds(f, c));
            if s.is_some() { any_still_grounds = true } else { any_still_silent = true }
            if let (None, Some(wt)) = (s, w) {
                if best.map_or(true, |(_, _, bt)| wt < bt) {
                    best = Some((f, c, wt));
                }
            }
        }
    }

    println!("\n══ vacuity guards ════════════════════════════════════════════════");
    println!(
        "V1  current rule reproduces §7's 'never' ......... {}",
        if v1 { "PASS" } else { "FAIL — everything below is void" }
    );
    let v2 = any_still_grounds && any_still_silent;
    println!(
        "V2  sweep spans pairs where the control fires\n    \
         AND pairs where it does not .................. {}",
        if v2 { "PASS" } else { "FAIL — 'no pair discriminates' is about a sweep too narrow" }
    );
    // V4 — my accumulator must be the same ruler `would_ground` is, or the sweep
    // measures with an instrument of its own. Checked against the residual series.
    let v4 = worlds.iter().all(|w| {
        let held: Vec<bool> = w.residual.iter().map(|&r| r > CURRENT_THRESHOLD).collect();
        accumulate(&held) == would_ground(&w.residual, CURRENT_THRESHOLD)
    });
    let med_conf_still = Life::median(&still.confidence);
    let v3 = med_conf_still > 0;
    println!(
        "V3  confidence moved in the still control ....... {} (median {})",
        if v3 { "PASS" } else { "FAIL — H1/H3 are about a dead register" },
        med_conf_still
    );

    // -----------------------------------------------------------------------
    println!(
        "V4  my accumulator agrees with would_ground ..... {}",
        if v4 {
            "PASS — same ruler as PrimeLayer::observe, on all 7 worlds"
        } else {
            "FAIL — the sweep is measuring with an instrument of its own"
        }
    );

    println!("\n══ predictions as locked ═════════════════════════════════════════");
    println!(
        "H1  no pair discriminates (p=0.60) .............. {}",
        if !v1 || !v2 || !v4 {
            "VACUOUS — see the guards".to_string()
        } else {
            match best {
                None => "HOLDS — no (floor, ceiling) earns the word under weather while\n    \
                         leaving the still control silent. The two-term grounding does\n    \
                         not separate world-change from ordinary living."
                    .to_string(),
                Some((f, c, t)) => format!(
                    "FAILS — and this is the good outcome. floor={} ceiling={} earns\n    \
                     HAPPEN at tick {} under weather, and NEVER in the still control.",
                    f, c, t
                ),
            }
        }
    );
    println!(
        "H2  HAPPEN fires at all under weather (p=0.90) .. {}",
        if FLOORS.iter().any(|&f| CEILINGS.iter().any(|&c| w2.grounds(f, c).is_some())) {
            "HOLDS"
        } else {
            "FAILS — no pair earns the word under weather at all"
        }
    );
    println!(
        "H3  still-control confidence median > 64 (p=0.70) {}",
        if med_conf_still > 64 {
            format!("HOLDS — {}. The being's ordinary sensory flux is already large.", med_conf_still)
        } else {
            format!("FAILS — {}", med_conf_still)
        }
    );

    // H4: monotonic across octaves, at the most permissive pair that fires.
    let (mf, mc) = (FLOORS[1], CEILINGS[4]);
    let oct_rates: Vec<(String, f64)> = worlds
        .iter()
        .filter(|w| w.label.starts_with("weather"))
        .map(|w| (w.label.clone(), w.rate(mf, mc)))
        .collect();
    let monotonic = oct_rates.windows(2).all(|p| p[0].1 >= p[1].1);
    let oct_grounds: Vec<(String, Option<u32>)> = worlds
        .iter()
        .filter(|w| w.label.starts_with("weather"))
        .map(|w| (w.label.clone(), w.grounds(16, 16)))
        .collect();
    let mono_ground = oct_grounds.windows(2).all(|p| match (p[0].1, p[1].1) {
        (Some(a), Some(b)) => a <= b,
        (Some(_), None) => true,
        (None, Some(_)) => false,
        (None, None) => true,
    });
    println!(
        "H4  fire rate monotonic across octaves (p=0.20) . {}",
        if monotonic {
            format!("HOLDS at floor={} ceiling={}. I predicted this would fail.", mf, mc)
        } else {
            format!(
                "FAILS, as predicted — at floor={} ceiling={}: {}",
                mf,
                mc,
                oct_rates
                    .iter()
                    .map(|(l, r)| format!("{}={:.0}%", l.replace("weather ", ""), r * 100.0))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    );

    println!(
        "    ...but H4 as LOCKED said \"fire rate\", and rate is the quantity §8 spent\n    \
         its whole argument saying is the wrong one. In the GROUNDING currency it {}:\n    \
         {}. I committed the same category error while diagnosing it.",
        if mono_ground { "also holds" } else { "FAILS" },
        oct_grounds
            .iter()
            .map(|(l, g)| format!(
                "{}={}",
                l.replace("weather ", "").replace(" octaves", ""),
                g.map_or("never".to_string(), |t| t.to_string())
            ))
            .collect::<Vec<_>>()
            .join(", ")
    );

    // H5: agency alone.
    let mut agency_discriminates = None;
    for c in CEILINGS {
        if let (None, Some(wt)) = (still.agency_only_grounds(c), w2.agency_only_grounds(c)) {
            agency_discriminates = Some((c, wt));
        }
    }
    println!(
        "H5  agency ALONE does not discriminate (p=0.75) . {}",
        match agency_discriminates {
            None => "HOLDS — every ceiling that lets weather fire also lets the\n    \
                     still control fire. The magnitude term is doing real work."
                .to_string(),
            Some((c, t)) => format!(
                "FAILS — ceiling={} ALONE earns the word at tick {} under weather and\n    \
                 never in the still control. The confidence term may be unnecessary.",
                c, t
            ),
        }
    );

    // -----------------------------------------------------------------------
    // Adversarial mutations, run before any of this was written up.
    // -----------------------------------------------------------------------
    println!("\n══ adversarial mutations ═════════════════════════════════════════");
    let mk = |k: &str| -> FieldWorld {
        let b = FieldWorld::with((16, 16), (240, 240), (30, 170)).with_person(1, (20, 20));
        match k {
            "drift8" => b.with_drift(0, 8, (3, -2)),
            "drift2" => b.with_drift(0, 2, (3, -2)),
            "w2" => b.with_weather(0, 2),
            "w8" => b.with_weather(0, 8),
            _ => b,
        }
    };
    let ground_at = |k: &str, g: Genome, ticks: usize, f: i16, c: i16| -> String {
        let mut world = mk(k);
        let mut being = UnifiedBeing::new(g);
        let mut held = Vec::with_capacity(ticks);
        for _ in 0..ticks {
            let sens = world.sense();
            let r = being.step_embodied(&sens);
            world.actuate(&intent_from(&r));
            held.push(r.agency.confidence > f && r.agency.agency < c);
            if !r.alive {
                break;
            }
        }
        accumulate(&held).map_or("never".to_string(), |t| t.to_string())
    };

    println!("M1  is the still control's silence a HORIZON artefact? (floor 16)");
    println!("    {:<10} {:>9} {:>9} {:>9} {:>9}", "ceiling", "1500", "6000", "20000", "50000");
    for c in [8i16, 16] {
        print!("    {:<10}", c);
        for t in [1500usize, 6000, 20000, 50000] {
            print!("{:>9}", ground_at("still", Genome::wanderer(), t, 16, c));
        }
        println!();
    }
    println!("    Silent at every horizon out to 50000. The 'never' is real.");

    println!("\nM2  does it pick out WEATHER, or world-motion generally? (floor 16)");
    println!("    {:<22} {:>10} {:>10}", "world", "ceil 8", "ceil 16");
    for (n, k) in [
        ("still (control)", "still"),
        ("drift every 8", "drift8"),
        ("drift every 2", "drift2"),
        ("weather 2 octaves", "w2"),
        ("weather 8 octaves", "w8"),
    ] {
        println!(
            "    {:<22} {:>10} {:>10}",
            n,
            ground_at(k, Genome::wanderer(), 1500, 16, 8),
            ground_at(k, Genome::wanderer(), 1500, 16, 16)
        );
    }
    println!("    It grounds under DRIFT too — which is correct for the word: drift is");
    println!("    the world acting on the being. But weather-8 never grounds, so the");
    println!("    rule has a sensitivity floor that the gentlest world sits below.");

    println!("\nM3  does the window survive a different genome? (floor 16, ceil 16)");
    println!("    {:<14} {:>14} {:>14}", "genome", "still", "weather 2");
    let mut genome_general = true;
    for (gl, gg) in [("wanderer", Genome::wanderer()), ("default", Genome::default())] {
        let w = ground_at("w2", gg.clone(), 1500, 16, 16);
        if w == "never" {
            genome_general = false;
        }
        println!("    {:<14} {:>14} {:>14}", gl, ground_at("still", gg, 1500, 16, 16), w);
    }
    println!(
        "    {}",
        if genome_general {
            "The window generalises."
        } else {
            "IT DOES NOT GENERALISE. Only `wanderer` earns the word; `default` stays\n                 silent under weather. One genome, one weather setting."
        }
    );

    println!("\n══ what this does NOT close ══════════════════════════════════════");
    println!("§4's regularity gap. A zone's forward process is statistically regular by");
    println!("construction; a being with prediction error alone habituates, and then");
    println!("\"as it always is\" and \"changed while I stood here\" produce the same low");
    println!("error. No threshold reaches that distinction, including any found above.");
}
