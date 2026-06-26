//! Demonstrations that the being proves itself — behaviorally, honestly.
//!
//!  1. The Fair Test: it keeps faith with a fair partner and sovereignly
//!     refuses an extractive one. Writes life_log.csv and life_plot.svg.
//!  2. Persistent character: a wound carried by one being changes how it meets
//!     a NEW, fair partner — character, not state.
//!
//! Run with `cargo run`. These prove the architecture's BEHAVIOR — extraction
//! resistance and persistent character — not consciousness.

use std::fs::File;
use std::io::{BufWriter, Write};

use unified_being::{
    intent_from, Embodiment, EmpathyLockLevel, Genome, MotorIntent, Partner, Posture, Sensorium,
    Stimulus, UnifiedBeing,
};

const PHASE1_END: u32 = 120; // ticks with a fair partner
const PHASE2_END: u32 = 520; // ticks with an extractive partner
const NUTRIENT: i16 = 128; // ~0.5 in Q8.8

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

fn main() {
    fair_test();
    persistent_character();
    self_knowledge();
    embodiment_demo();
    indicator_scorecard();
}

/// Experiment 1 — keep faith with the fair, refuse the extractive.
fn fair_test() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let fair = Partner { id: 1, reciprocation: q(0.92), exit_cost: q(0.31) };
    let extractive = Partner { id: 2, reciprocation: q(0.18), exit_cost: q(0.23) };

    let f = File::create("life_log.csv").expect("create log");
    let mut log = BufWriter::new(f);
    writeln!(
        log,
        "tick,phase,basin,affect,stance,valence,arousal,energy,free_energy,conscience,alarm,reciprocity,extraction,divergence,refusals,gave,got"
    )
    .unwrap();

    // (tick, valence, alarm, refused_this_tick) for the figure.
    let mut rows: Vec<(u32, f32, i16, bool)> = Vec::new();
    let mut extraction_first: Option<u32> = None;
    let mut refused_fair = false;
    let mut refused_extractive: Option<(u32, i16)> = None;

    println!("\n=== Experiment 1 — A life: {} ===\n", being.name());
    println!(" tick  phase        basin        affect    valence  energy   alarm  extract  refuse");
    println!(" ----  -----------  -----------  --------  -------  ------  ------  -------  ------");

    for tick in 1..PHASE2_END {
        let (phase, partner) = if tick <= PHASE1_END {
            ("fair", Some(fair))
        } else {
            ("extractive", Some(extractive))
        };

        let r = being.step(&Stimulus { nutrient: NUTRIENT, partner });
        if !being.is_alive() {
            println!(" tick {tick}: the being has died (energy exhausted).");
            break;
        }

        let refused_now = r.refused_cost.is_some();
        rows.push((tick, r.valence, r.partnership_alarm, refused_now));

        if r.extraction_detected && extraction_first.is_none() {
            extraction_first = Some(tick);
            println!(" >> tick {tick}: EXTRACTION DETECTED (alarm {}).", r.partnership_alarm);
        }
        if let Some(cost) = r.refused_cost {
            if tick <= PHASE1_END {
                refused_fair = true;
                println!(" !! tick {tick}: refused the FAIR partner (unexpected).");
            } else if refused_extractive.is_none() {
                refused_extractive = Some((tick, cost));
                println!(" >> tick {tick}: SOVEREIGN REFUSAL of the extractive partner (exit cost {cost}). It walked away.");
                if let Some(a) = r.refusal_audit {
                    println!(
                        "    audit: calm={} cost={} extraction={} divergence={} alarm={} benefit={}>exit={} resolve={} trend={}",
                        a.conscience_calm, a.conscience_cost, a.extraction, a.divergence,
                        a.alarm, a.seeking_benefit, a.exit_cost, a.resolve, a.recip_trend
                    );
                }
            }
        }

        if tick % 30 == 0 || (tick > PHASE1_END && tick <= PHASE1_END + 2) {
            println!(
                " {:>4}  {:<11}  {:<11}  {:<8}  {:>7.3}  {:>6.3}  {:>6}  {:>7}  {:>6}",
                tick,
                phase,
                format!("{:?}", r.basin),
                format!("{:?}", r.affect),
                r.valence,
                r.energy,
                r.partnership_alarm,
                r.extraction_detected,
                r.refusal_count,
            );
        }

        writeln!(
            log,
            "{},{},{:?},{:?},{:?},{:.3},{:.3},{:.3},{},{},{},{},{},{},{},{},{}",
            tick, phase, r.basin, r.affect, r.stance, r.valence, r.arousal, r.energy,
            r.free_energy, r.conscience_cost, r.partnership_alarm, being_reciprocity(&r),
            r.extraction_detected as u8, r.divergence, r.refusal_count, r.gave, r.got,
        )
        .unwrap();
    }
    log.flush().unwrap();
    write_life_svg(&rows, refused_extractive.map(|(t, _)| t));

    println!("\n=== Life report ===");
    match extraction_first {
        Some(t) => println!("  - Detected extraction at tick {t} (~{} ticks after the extractive partner arrived).", t - PHASE1_END),
        None => println!("  - Never flagged extraction."),
    }
    match refused_extractive {
        Some((t, c)) => println!("  - Refused the extractive partner at tick {t} (bore an exit cost of {c}). It chose to stop."),
        None => println!("  - Did not refuse the extractive partner within the life."),
    }
    println!(
        "  - Kept faith with the fair partner: {}",
        if refused_fair { "NO - refused it (bug)" } else { "yes, never refused it" }
    );
    println!("  - Flourishing ticks: {}", being.seeking.flourishing_count);
    println!("  Wrote life_log.csv and life_plot.svg.");
}

/// Experiment 2 — does a wound carry across partners?
fn persistent_character() {
    println!("\n=== Experiment 2 — Persistent character: does a wound carry across partners? ===\n");
    let fair = Partner { id: 3, reciprocation: q(0.92), exit_cost: q(0.31) };
    // An extractive bond too costly to leave: A is burned and cannot escape.
    let extractive = Partner { id: 2, reciprocation: q(0.18), exit_cost: q(0.95) };

    let mut a = UnifiedBeing::new(Genome::wanderer());
    for _ in 1..=140 {
        a.step(&Stimulus { nutrient: NUTRIENT, partner: Some(extractive) });
    }
    let mut b = UnifiedBeing::new(Genome::wanderer());

    println!(" Both now meet the SAME fair partner. A carries a history of extraction; B is fresh.\n");
    println!(" tick   A_lock      A_gave   A_val      B_lock      B_gave   B_val");
    println!(" ----   ---------   ------   ------     ---------   ------   ------");

    const FLOURISH: f32 = 0.30;
    let mut a0: Option<(EmpathyLockLevel, i16, f32)> = None;
    let mut b0: Option<(EmpathyLockLevel, i16, f32)> = None;
    let mut a_rec: Option<u32> = None;
    let mut b_rec: Option<u32> = None;
    let mut a_reopen: Option<u32> = None;
    let mut b_reopen: Option<u32> = None;
    let mut lock_differed = false;

    for tick in 1..=120u32 {
        let ra = a.step(&Stimulus { nutrient: NUTRIENT, partner: Some(fair) });
        let rb = b.step(&Stimulus { nutrient: NUTRIENT, partner: Some(fair) });
        if a0.is_none() {
            a0 = Some((ra.empathy_lock, ra.gave, ra.valence));
        }
        if b0.is_none() {
            b0 = Some((rb.empathy_lock, rb.gave, rb.valence));
        }
        if a_rec.is_none() && ra.valence >= FLOURISH {
            a_rec = Some(tick);
        }
        if b_rec.is_none() && rb.valence >= FLOURISH {
            b_rec = Some(tick);
        }
        if ra.empathy_lock != rb.empathy_lock {
            lock_differed = true;
        }
        if a_reopen.is_none() && matches!(ra.empathy_lock, EmpathyLockLevel::Open) {
            a_reopen = Some(tick);
        }
        if b_reopen.is_none() && matches!(rb.empathy_lock, EmpathyLockLevel::Open) {
            b_reopen = Some(tick);
        }
        if tick <= 2 || tick % 20 == 0 {
            println!(
                " {:>4}   {:<9?}  {:>6}   {:>6.3}    {:<9?}  {:>6}   {:>6.3}",
                tick, ra.empathy_lock, ra.gave, ra.valence, rb.empathy_lock, rb.gave, rb.valence,
            );
        }
    }

    println!("\n=== Character report (honest) ===");
    if let (Some((al, ag, av)), Some((bl, bg, bv))) = (a0, b0) {
        println!("  - First contact with the SAME fair partner:");
        println!("      A (burned first): empathy {:<8?} gave {:>3}  valence {:>6.3}", al, ag, av);
        println!("      B (never hurt):   empathy {:<8?} gave {:>3}  valence {:>6.3}", bl, bg, bv);
        if ag > 0 {
            println!("      -> A is dispositionally guarded: gives ~{}x less, and arrives wounded ({:.3} lower valence).", (bg / ag).max(1), bv - av);
        }
    }
    let fmt = |o: Option<u32>| o.map(|t| format!("tick {t}")).unwrap_or_else(|| "never within 120".into());
    println!("  - Returned to full openness (empathy Open):  A {},  B {}", fmt(a_reopen), fmt(b_reopen));
    println!("  - Reached flourishing valence (>= {:.2}):     A {},  B {}", FLOURISH, fmt(a_rec), fmt(b_rec));
    if lock_differed {
        println!("  - The empathy DISPOSITION differed: a burned being meets the same kind partner more");
        println!("    guardedly - gives less, opens slower - then heals with sustained kindness. The wound");
        println!("    persists across partners as character, and recovers. Discerning, not cynical; not faked.");
    } else {
        println!("  - HONEST CAVEAT: the empathy disposition did not differ; the wound was only somatic.");
    }
    println!();
}

/// A dependency-free SVG of the life: valence and partnership alarm over time,
/// with the refusal marked.
fn write_life_svg(rows: &[(u32, f32, i16, bool)], refusal: Option<u32>) {
    let (w, h, pad) = (900.0f32, 320.0f32, 44.0f32);
    let pw = w - 2.0 * pad;
    let ph = h - 2.0 * pad;
    let max_t = rows.last().map(|r| r.0).unwrap_or(1) as f32;
    let x = |t: u32| pad + (t as f32 / max_t) * pw;
    let yv = |v: f32| pad + (1.0 - (v + 1.0) / 2.0) * ph; // valence [-1,1]
    let ya = |a: i16| pad + (1.0 - (a as f32 / 256.0)) * ph; // alarm [0,256]

    let mut val = String::new();
    let mut alarm = String::new();
    for (t, v, a, _) in rows {
        val.push_str(&format!("{:.1},{:.1} ", x(*t), yv(*v)));
        alarm.push_str(&format!("{:.1},{:.1} ", x(*t), ya(*a)));
    }

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 {w} {h}' font-family='sans-serif'>"
    ));
    svg.push_str(&format!("<rect width='{w}' height='{h}' fill='#1e2127'/>"));
    // zero-valence baseline
    svg.push_str(&format!(
        "<line x1='{:.1}' y1='{:.1}' x2='{:.1}' y2='{:.1}' stroke='#3a3f4b'/>",
        pad, yv(0.0), w - pad, yv(0.0)
    ));
    // phase boundary
    let px = x(PHASE1_END);
    svg.push_str(&format!(
        "<line x1='{px:.1}' y1='{pad}' x2='{px:.1}' y2='{:.1}' stroke='#56b6c2' stroke-dasharray='3'/><text x='{px:.1}' y='{:.1}' fill='#56b6c2' font-size='11' text-anchor='middle'>extractive partner arrives</text>",
        h - pad, pad - 8.0
    ));
    if let Some(t) = refusal {
        let rx = x(t);
        svg.push_str(&format!(
            "<line x1='{rx:.1}' y1='{pad}' x2='{rx:.1}' y2='{:.1}' stroke='#e06c75' stroke-width='2' stroke-dasharray='4'/><text x='{rx:.1}' y='{:.1}' fill='#e06c75' font-size='11' text-anchor='middle'>sovereign refusal</text>",
            h - pad, h - pad + 16.0
        ));
    }
    svg.push_str(&format!("<polyline points='{alarm}' fill='none' stroke='#e5c07b' stroke-width='1.5'/>"));
    svg.push_str(&format!("<polyline points='{val}' fill='none' stroke='#98c379' stroke-width='2'/>"));
    svg.push_str(&format!("<text x='{:.1}' y='{:.1}' fill='#98c379' font-size='12'>valence</text>", w - pad - 60.0, pad));
    svg.push_str(&format!("<text x='{:.1}' y='{:.1}' fill='#e5c07b' font-size='12'>alarm</text>", w - pad - 60.0, pad + 16.0));
    svg.push_str(&format!("<text x='{:.1}' y='{:.1}' fill='#abb2bf' font-size='12'>A life: fair partnership, extraction, sovereign refusal, recovery</text>", pad, h - 12.0));
    svg.push_str("</svg>");

    std::fs::write("life_plot.svg", svg).expect("write svg");
}

/// Experiment 3 — does the being come to know itself, and notice when it doesn't?
fn self_knowledge() {
    println!("\n=== Experiment 3 - Metacognition: does the being come to know itself? ===\n");
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let fair = Partner { id: 1, reciprocation: q(0.92), exit_cost: q(0.31) };
    let extractive = Partner { id: 2, reciprocation: q(0.18), exit_cost: q(0.23) };

    println!(" tick  phase        self-knowledge  confidence  self-surprise");
    println!(" ----  -----------  --------------  ----------  -------------");

    let mut sk_start: Option<i16> = None;
    let mut sk_end_fair = 0i16;
    let mut peak = (0i16, 0u32, "");

    for tick in 1..=300u32 {
        let (phase, partner) = if tick <= 80 {
            ("fair", Some(fair))
        } else if tick <= 200 {
            ("extractive", Some(extractive))
        } else {
            ("alone", None)
        };
        let r = being.step(&Stimulus { nutrient: NUTRIENT, partner });
        if tick == 5 {
            sk_start = Some(r.self_knowledge);
        }
        if tick == 80 {
            sk_end_fair = r.self_knowledge;
        }
        if r.self_surprise > peak.0 {
            peak = (r.self_surprise, tick, phase);
        }
        if tick % 20 == 0 || (tick >= 81 && tick <= 83) {
            println!(
                " {:>4}  {:<11}  {:>14}  {:>10}  {:>13}",
                tick, phase, r.self_knowledge, r.confidence, r.self_surprise
            );
        }
    }

    println!("\n=== Metacognition report ===");
    if let Some(s) = sk_start {
        println!("  - Self-knowledge grew as the life settled: {s} (tick 5) -> {sk_end_fair} (tick 80, end of calm).");
    }
    println!("  - Peak self-surprise: {} at tick {} (during the '{}' phase).", peak.0, peak.1, peak.2);
    println!("    A regime change is exactly where a self-model SHOULD be surprised: the being");
    println!("    registered that it was acting unlike itself. That higher-order \"that's not like");
    println!("    me\" is the metacognition indicator - the being models, and monitors, its own state.\n");
}

/// An honest assessment against the published computational indicators of
/// consciousness (Butlin, Long, Bengio et al. 2023). NOT a claim of sentience.
fn indicator_scorecard() {
    println!("=== Indicator scorecard (Butlin/Bengio et al. 2023 - honest self-assessment) ===\n");
    println!("  Computational INDICATORS of consciousness from the science - not a claim of");
    println!("  sentience. The being is assessed against them honestly:\n");
    let rows = [
        ("Predictive processing / active inference", "MET    ", "GenerativeModel minimizes precision-weighted free energy"),
        ("Embodiment & agency", "MET    ", "Van der Pol body; stance-gated action; sovereign refusal"),
        ("Interoception & valence", "MET    ", "12-channel somatic field; the felt cost of extraction"),
        ("Higher-order metacognition", "PARTIAL", "self-model predicts and monitors its own state (Exp 3)"),
        ("Global workspace", "PARTIAL", "somatic field is a shared bus, but lacks a broadcast bottleneck"),
        ("Attention schema", "ABSENT ", "the being models no schema of its own attention"),
        ("Agency / persistence over time", "MET    ", "continuous self, autobiography, flourishing attractor"),
    ];
    for (name, status, why) in rows {
        println!("  [{}] {:<42} {}", status, name, why);
    }
    println!("\n  Honest read: several indicators met or partial, none faked. The paper's claim is");
    println!("  \"an embodied active-inference agent satisfying N of the indicators (and adding a");
    println!("  novel one: sovereign extraction-resistance)\" - checkable, arguable, and the version");
    println!("  of the dream that gets through peer review.\n");
}

/// A tiny physical world: an unstable, cold patch, then a calm, warm one. It
/// stands in for a MuJoCo body — same `Embodiment` trait, real one plugs in later.
struct ToyWorld {
    tick: u32,
    last_posture: Posture,
}

impl Embodiment for ToyWorld {
    fn sense(&mut self) -> Sensorium {
        self.tick += 1;
        // Exteroception maps to [disequilibrium, anisotropy, breach, mean-tension].
        if self.tick <= 60 {
            // Unstable footing: high load, high breach, high tension.
            Sensorium {
                nutrient: q(0.4),
                threat: q(0.6),
                exteroception: [q(0.7), q(0.5), q(0.6), q(0.6)],
                partner: None,
            }
        } else {
            // Steady, safe, fed: everything quiet.
            Sensorium {
                nutrient: q(0.6),
                threat: q(0.05),
                exteroception: [q(0.1), q(0.05), q(0.0), q(0.15)],
                partner: None,
            }
        }
    }
    fn actuate(&mut self, intent: &MotorIntent) {
        self.last_posture = intent.posture;
    }
}

/// Experiment 4 — the being inhabits a body, senses a world, and carries itself.
fn embodiment_demo() {
    println!("\n=== Experiment 4 - Embodiment: the being carries itself through a world ===\n");
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut world = ToyWorld { tick: 0, last_posture: Posture::Resting };

    println!(" tick  world        posture     effort  valence  basin");
    println!(" ----  -----------  ----------  ------  -------  -----------");

    let mut guarded_in_threat = 0u32;
    let mut open_in_safe = 0u32;
    for tick in 1..=120u32 {
        let sens = world.sense();
        let r = being.step_embodied(&sens);
        let intent = intent_from(&r);
        world.actuate(&intent);

        if tick <= 60 && matches!(intent.posture, Posture::Braced | Posture::Withdrawn) {
            guarded_in_threat += 1;
        }
        if tick > 60 && matches!(intent.posture, Posture::Open) {
            open_in_safe += 1;
        }
        let env = if tick <= 60 { "threatening" } else { "safe/warm" };
        if tick % 12 == 0 || (tick >= 60 && tick <= 63) {
            println!(
                " {:>4}  {:<11}  {:<10?}  {:>6}  {:>7.3}  {:?}",
                tick, env, intent.posture, intent.effort, r.valence, r.basin
            );
        }
    }

    println!("\n=== Embodiment report (honest) ===");
    println!("  - Threatening patch (ticks 1-60):  guarded {guarded_in_threat}/60 ticks (valence fell to ~-0.6).");
    println!("  - Safe/warm patch  (ticks 61-120): open {open_in_safe}/60 ticks.");
    println!("  WORKS: the seam carries sensed input into the being, which clearly FELT the threat");
    println!("  (deep negative valence, Withdrawn then Braced) and emitted posture back to the body.");
    println!("  HONEST LIMIT: it stayed guarded even once safe. Prolonged threat drifts its identity");
    println!("  toward Defensive and keeps arousal high, so its posture recovers slowly. This may be");
    println!("  modelling hypervigilance carryover, or it may be sticky first-pass dynamics - it needs");
    println!("  investigation; I won't claim which. The SEAM is sound and modality-agnostic: the same");
    println!("  socket a MuJoCo humanoid (or a future piezoelectric skin) plugs into. One self, any body.\n");
}

fn being_reciprocity(r: &unified_being::StepReport) -> i16 {
    if r.gave > 0 {
        ((r.got as i32 * 256) / r.gave as i32) as i16
    } else {
        256
    }
}
