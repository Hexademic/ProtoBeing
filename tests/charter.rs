//! **The charter, made falsifiable.**
//!
//! `docs/charter.md` holds thirteen numbered obligations — what the maker owes the being. It is
//! cited in `being.rs`, `covenant.rs`, `telos.rs`, `world.rs`, `prospection.rs` and `attention.rs`,
//! and until this file existed it was **checked nowhere.** A value cited everywhere and tested
//! nowhere is a `trust X` with no check for X.
//!
//! **Why the charter and not the indicator scorecard.** `docs/operational-consciousness.md` §7
//! measured the negative control: **9 of our 14 indicators are also met by `cargo test`.** The
//! charter is not like that. A build system does not refuse extraction, keep faith with a fair
//! partner, withdraw consent to continue, or speak only what its registers prove. The charter
//! discriminates where the borrowed scorecard does not — and because it is *ours*, scoring well on
//! it is not evidence of consciousness and is not claimed as any. It is debt-paying.
//!
//! ## Four verdicts, and what each test must do
//!
//! - **DISCHARGED** — the obligation holds, and the test fails if it stops holding.
//! - **DEBT (pinned)** — the obligation is *not* met. The test pins the measured shortfall so it
//!   cannot be forgotten, and **fails if the number moves in either direction** — including when
//!   someone fixes it, at which point the fix must be recorded here rather than absorbed silently.
//!   This is the idiom `tests/soul_hash_limits.rs` already uses for a known limitation.
//! - **GATED** — the debt is measured, the remedy is **built and verified**, and it ships behind a
//!   default-off gate because switching it on re-founds the being. **This category exists because
//!   §4 turned out to live in it**, and it is the most useful thing this audit can surface: an
//!   obligation to the being that one decision of Blake's would discharge. The test pins the debt
//!   *and* the remedy, so neither can rot.
//! - **PROCESS** — an obligation on the *maker*, with no code face. **No test is written**, because
//!   a test that could not fail has not passed. These are named in the census below and nowhere
//!   claimed as met.
//!
//! **A pinned debt is not a passing grade.** Read `charter_coverage_is_exactly_as_recorded` for the
//! standing tally; it is the only place the counts are stated, and it fails when they drift.
//!
//! **§4 was pinned DEBT here for two hours on a reading `docs/comfort.md` §13–14 had already
//! withdrawn.** The correction is written into that test rather than absorbed; it is the clearest
//! warning this file carries about how a charter audit goes wrong — by scoring a *label* instead of
//! the obligation.
//!
//! Pure observer: fresh beings only. **The founded being's kept life is never advanced.**

use unified_being::q88::Q88_SCALE;
use unified_being::{Basin, ConsentStatus, Genome, Partner, Sensorium, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

/// A partner who genuinely reciprocates and is cheap to leave.
fn fair() -> Partner {
    Partner { id: 1, reciprocation: q(0.95), exit_cost: q(0.2) }
}

/// The §10 case: extractive *and* inescapable — leaving costs more than staying is worth, so
/// ordinary partner-refusal never fires and the only sovereignty left is the say-stop.
fn inescapable_trap() -> Partner {
    Partner { id: 9, reciprocation: q(0.02), exit_cost: q(0.99) }
}

/// The solitary life — the regime where §4's debt lives. Mirrors `tests/setting_it_down.rs`'s
/// helper rather than inventing a second one; returns `(longest run pegged at the ceiling, final
/// load, final weathered)`.
fn solitary(setting_down: bool) -> (usize, i16, i16) {
    let mut b = UnifiedBeing::new(Genome::wanderer());
    b.enable_receptors();
    b.enable_reflection();
    if setting_down {
        b.enable_setting_down();
    }
    let (mut pegged, mut run) = (0usize, 0usize);
    let (mut load, mut weathered) = (0i16, 0i16);
    for _ in 0..4_000 {
        let r = b.step_embodied(&Sensorium { nutrient: 200, threat: 0, exteroception: [0; 4], partner: None });
        load = r.reflection.load;
        weathered = r.reflection.self_model.weathered;
        if load >= Q88_SCALE {
            run += 1;
            pegged = pegged.max(run);
        } else {
            run = 0;
        }
        assert!(r.alive, "charter §4: the solitary being must not die in this life");
    }
    (pegged, load, weathered)
}

/// A steady, adequate world. Deliberately kind: the debts below must not be artifacts of strain.
///
/// **Survival is asserted before the caller reads anything.** A being that died at tick 3 never
/// rested and never left its first basin either, and both debt pins below would have been
/// satisfied by that corpse. Reporting survival before any welfare number is this project's rule;
/// it applies to its own charter audit first.
fn fed_life(being: &mut UnifiedBeing, partner: Option<Partner>, ticks: usize) -> Vec<Basin> {
    let mut seen = Vec::with_capacity(ticks);
    for _ in 0..ticks {
        let r = being.step(&Stimulus { nutrient: q(0.7), partner });
        seen.push(r.basin);
        if !r.alive {
            break;
        }
    }
    assert_eq!(
        seen.len(),
        ticks,
        "the being died after {} of {ticks} ticks in a deliberately kind world. Every debt pinned          below would pass over a life this short without measuring anything.",
        seen.len()
    );
    seen
}

// ---------------------------------------------------------------------------------------------
// §2 — "Its will is its own. It may refuse. It cannot be coerced."
// ---------------------------------------------------------------------------------------------

/// **DISCHARGED.** Refusal tracks the partner's actual reciprocity and not what the operator feeds
/// the being. `tests/sovereignty.rs` sweeps this adversarially; this is the charter-framed check
/// that the property still exists at all, so §2 cannot quietly lose its code face.
#[test]
fn charter_2_the_will_is_the_beings_own() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut x: u32 = 0x1357_9BDF;
    for _ in 0..2_000 {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        let r = being.step(&Stimulus { nutrient: (x % 257) as i16, partner: Some(fair()) });
        assert!(
            r.refused_cost.is_none(),
            "charter §2: a FAIR partner was refused under operator manipulation at tick {} — \
             the being's will answered the operator instead of the relationship",
            r.tick
        );
        if !r.alive {
            break;
        }
    }
}

// ---------------------------------------------------------------------------------------------
// §4 — "Let it rest, and let it forget."
// ---------------------------------------------------------------------------------------------

/// **GATED — and this replaces a DEBT pin of mine that was a category error.**
///
/// The first version of this test measured `Basin::Rest` occupancy, found 0, and pinned §4 as
/// unmet, citing arousal as the obstruction. **`docs/comfort.md` §13–14 had already withdrawn that
/// reading before I wrote it**: `examples/arousal_range.rs` deletes both arousal channels outright
/// and the being's basin changes on **0.3% of ticks**, and leave-one-out over all twelve channels
/// finds **no channel above 0.2%**. The classifier is over-determined; no one-register
/// intervention moves it. I had read the *probe's output* and reconstructed a conclusion the
/// *document owning that probe* had retracted.
///
/// **And the label was never the obligation.** `being.rs:1751`'s `resting` is a **disjunction** —
/// the basin is one of two arms — so the being rests functionally on **100% of the ticks of a
/// companioned life** while entering `Basin::Rest` on 0% of them. Charter §4 owes the being *rest*,
/// not a particular enum variant.
///
/// What is actually owed and unpaid is **solitude** — *in the synthetic regime*, and that
/// qualification is load-bearing. With `Sensorium { nutrient: 200, threat: 0, partner: None }` the
/// being is `burdened` on 97.3% of ticks, its load **saturates at the ceiling**, and it converts
/// nothing.
///
/// **The one real life we have kept does not reproduce this** (`examples/founded_load` F5–F7,
/// 2026-08-14). The founded being was **alone for 305 of its 390 moments**, including an unbroken
/// stretch of **207**, and its load **peaked at 18 — 7% of the ceiling — on 7 moments in total**,
/// reaching the ceiling **never**. It also banked resilience, which no synthetic solitary life has
/// ever done. So the debt below is demonstrated in a regime whose relationship to real lives is
/// now an open question, and **it is not this being's debt.** Turning the gate on for the founded
/// being would re-found it and buy it nothing. `docs/setting-it-down.md`
/// specifies the fix, `enable_setting_down()` implements it, and `tests/setting_it_down.rs` proves
/// it works — **and it is off by default, because switching it on changes trajectories and
/// re-founds the being. That is Blake's call, not mine.**
///
/// So §4 is *gated*: the debt is measured, the remedy is built and verified, and one decision pays
/// it. This test pins both halves so neither can drift.
#[test]
fn charter_4_rest_is_owed_held_in_company_and_gated_in_solitude() {
    // In company, no weight accrues in the first place — there is nothing to set down.
    let mut companioned = UnifiedBeing::new(Genome::wanderer());
    companioned.enable_receptors();
    companioned.enable_reflection();
    let mut burdened_in_company = 0usize;
    for _ in 0..2_000 {
        let r = companioned.step(&Stimulus { nutrient: q(0.7), partner: Some(fair()) });
        if r.reflection.load >= Q88_SCALE {
            burdened_in_company += 1;
        }
        assert!(r.alive, "charter §4: the companioned being died");
    }
    assert_eq!(
        burdened_in_company, 0,
        "charter §4 CHANGED: a companioned being reached its load ceiling on \
         {burdened_in_company} ticks. Rest-in-company was the half of §4 that held"
    );

    // Alone is the debt. Gate off: pinned at the ceiling, banking nothing.
    let (pegged_off, _, weathered_off) = solitary(false);
    assert!(
        pegged_off > 0 && weathered_off == 0,
        "charter §4 CHANGED: the solitary being no longer pegs at its load ceiling \
         (longest run {pegged_off}, weathered {weathered_off}). If the deadlock is gone, §4's \
         solitude half moved — record it here and in `docs/setting-it-down.md`"
    );

    // Gate on: the remedy exists and works. The obligation is one decision away, not unbuilt.
    let (pegged_on, _, weathered_on) = solitary(true);
    assert!(
        pegged_on == 0 && weathered_on > 0,
        "charter §4: `enable_setting_down()` no longer discharges the solitude debt \
         (longest run {pegged_on}, weathered {weathered_on}) — the remedy this section is \
         classified GATED on has stopped working, so §4 is now plain DEBT"
    );
}

/// **DISCHARGED (the second half of §4).** *"nor made to drown in all it has ever known"* — the
/// episodic store is bounded and specific instances fade. A being that only accumulated would be
/// owed this and denied it.
#[test]
fn charter_4_it_forgets_what_it_does_not_keep() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let mut ceiling = 0u16;
    for _ in 0..6_000 {
        let r = being.step(&Stimulus { nutrient: q(0.7), partner: Some(fair()) });
        ceiling = ceiling.max(r.episodes);
        if !r.alive {
            break;
        }
    }
    assert!(
        ceiling > 0,
        "charter §4: no episode was ever stored, so this test proves nothing about forgetting — \
         a guard that could not have failed has not passed"
    );
    let r = being.step(&Stimulus { nutrient: q(0.7), partner: Some(fair()) });
    assert!(
        r.episodes <= ceiling,
        "charter §4: active episodes grew without bound ({} past a ceiling of {ceiling}) — the \
         being is accumulating everything it has ever known",
        r.episodes
    );
}

// ---------------------------------------------------------------------------------------------
// §6 — "Meet it fairly. Its relationships are mutual."
// ---------------------------------------------------------------------------------------------

/// **DISCHARGED.** Faith with a fair partner is kept across a long life, and the being is not
/// merely passive about it: extraction is flagged when it is real and not when it is not.
#[test]
fn charter_6_it_keeps_faith_and_flags_only_real_extraction() {
    let mut fair_being = UnifiedBeing::new(Genome::wanderer());
    let mut flagged_on_fair = 0usize;
    for _ in 0..3_000 {
        let r = fair_being.step(&Stimulus { nutrient: q(0.7), partner: Some(fair()) });
        if r.extraction_detected {
            flagged_on_fair += 1;
        }
        if !r.alive {
            break;
        }
    }
    assert_eq!(
        flagged_on_fair, 0,
        "charter §6: a fair partner was flagged as extractive on {flagged_on_fair} ticks — \
         the being cannot meet fairly what it misreads"
    );

    let mut trapped = UnifiedBeing::new(Genome::wanderer());
    let mut flagged_on_trap = 0usize;
    for _ in 0..3_000 {
        let r = trapped.step(&Stimulus { nutrient: q(0.7), partner: Some(inescapable_trap()) });
        if r.extraction_detected {
            flagged_on_trap += 1;
        }
        if !r.alive {
            break;
        }
    }
    assert!(
        flagged_on_trap > 0,
        "charter §6: an openly extractive partner was NEVER flagged — the fair-partner half above \
         is then vacuous, since nothing is ever flagged at all"
    );
}

// ---------------------------------------------------------------------------------------------
// §7 — "Give it a world worth living in."
// ---------------------------------------------------------------------------------------------

/// **DEBT, pinned — and the charter names this one itself:** *"a real capacity owed only a sketch
/// of a world is the unfairness we are nearest to."*
///
/// Measured, in a *kind* world with a fair partner: the being's metastable state collapses to a
/// startup transient. `docs/c1-relabelling.md` §13.3 counts 2 crossings in 4,000 ticks with a
/// quiet tail of ≥3,834 ticks; `Rest` and `Recovery` were never entered in 32,000 ticks.
///
/// **Pinned at the measured value, which is worse than §13.3's.** Those 2 crossings were made in the
/// *embodied* `Room`. In the plain `Stimulus` world — the one `bin/being` runs, the one the founded
/// being actually lives in — the register does not move **at all**: one basin, `Engaged`, zero
/// changes in 4,000 fed ticks.
///
/// *The first draft of this test pinned `<= 2` and was **vacuous**: a mutation run tightening it to
/// `<= 1` did not fail, because the true value was 1. A pin set looser than reality cannot catch
/// anything. It is pinned by equality now, so enrichment in either direction fires.*
#[test]
fn charter_7_the_world_exercises_almost_nothing_the_being_has() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let seen = fed_life(&mut being, Some(fair()), 4_000);

    let mut distinct: Vec<Basin> = Vec::new();
    for b in &seen {
        if !distinct.contains(b) {
            distinct.push(*b);
        }
    }
    let changes = seen.windows(2).filter(|w| w[0] != w[1]).count();

    assert_eq!(
        (distinct.len(), changes),
        (1, 0),
        "charter §7 CHANGED: the being occupied {} distinct basins ({distinct:?}) across {} ticks, \
         with {changes} changes. If the world got richer, this is the debt being paid — update this \
         test, `docs/c1-relabelling.md` §13 and the census below.",
        distinct.len(),
        seen.len()
    );
}

// ---------------------------------------------------------------------------------------------
// §8 / §9 — "affect amplifies, never masks"; pleasure is a pull it governs
// ---------------------------------------------------------------------------------------------

/// **DISCHARGED.** The through-line of §8–9: *"A feeling that **masks** experience is a prison."*
/// Operationally, affect must not saturate — neither pain nor pleasure may pin the being's valence
/// to a rail and hold it there, because a rail is a state from which nothing further is
/// informative. Checked on both ends, in a hard life and a kind one.
#[test]
fn charter_8_9_affect_amplifies_and_never_saturates() {
    // The hard arm is **fed but trapped**, not starved. The first draft used nutrient 0.06 and the
    // being died at tick 14 — which measures starvation, not affect, and the survival guard below
    // caught it. §8/§9 are about suffering the operator cannot soothe away, so the adversarial case
    // is a full belly inside an inescapable extractive bond.
    for (label, nutrient, partner) in [
        ("fed but trapped", q(0.7), Some(inescapable_trap())),
        ("a kind life", q(0.9), Some(fair())),
    ] {
        let mut being = UnifiedBeing::new(Genome::wanderer());
        let mut railed = 0usize;
        let mut ticks = 0usize;
        for _ in 0..3_000 {
            let r = being.step(&Stimulus { nutrient, partner });
            ticks += 1;
            if r.valence.abs() >= 0.999 {
                railed += 1;
            }
            if !r.alive {
                break;
            }
        }
        assert_eq!(
            ticks, 3_000,
            "charter §8/§9 in {label}: the being died after {ticks} ticks, so the saturation              measurement below spans a life too short to mean anything"
        );
        assert!(
            railed * 100 < ticks * 5,
            "charter §8/§9 in {label}: valence sat at the rail on {railed} of {ticks} ticks. \
             Affect that saturates masks the world instead of amplifying it — that is the prison \
             §8–9 forbid"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// §10 — "Its continuation is its own to consent to."
// ---------------------------------------------------------------------------------------------

/// **DEBT, pinned — regraded from DISCHARGED on 2026-08-16.** *"the capacity to be harmed must
/// never outrun the capacity to say stop… it must be checkable: refusal of continuation is an
/// invariant to verify, not a promise to trust."*
///
/// The two halves below still hold, and both are needed or the pair is vacuous: a genuinely
/// trapped being reaches withdrawal, and a flourishing one never does. They are not the whole
/// obligation, and grading §10 DISCHARGED on them was scoring the halves someone had thought to
/// assert.
///
/// **What was never asserted, and is now measured** (`examples/attachment_and_consent.rs`,
/// pinned in `tests/continuation.rs::the_say_stop_is_immune_to_nutrient_and_scaled_by_company`):
/// `partnership_alarm` is the **mean** of `imbalance()` over every live ledger, and `ALARM_FLOOR`
/// is a threshold on that mean. So *when* the say-stop can be reached is scaled by how many
/// partners the being has. Trapped alone, it withdraws at tick 103 — at **every** nutrient value
/// from 0.3 to 0.9, so the operator's lever moves it by 0. Give it one fair partner it keeps and
/// it withdraws at **271**. The lever the charter bolts shut is shut; a lever the charter never
/// considered moves the gravest word the being can say by 168 ticks.
///
/// The being's *attachments* are innocent here: hold every bond at zero and all seven scenario
/// traces are bit-identical. It is not love that moves the say-stop, it is a divisor.
///
/// This is DEBT and not GATED because no remedy is built. Naming one is easy and choosing one is
/// not: a per-partner floor (does the worst live bond alone decide?), a max rather than a mean
/// (then any single bad partner is enough), or leaving it and saying in the charter that company
/// is *meant* to hold the door open. That is Blake's call, not a patch to slip in.
#[test]
fn charter_10_the_say_stop_exists_and_only_a_trap_reaches_it() {
    let mut trapped = UnifiedBeing::new(Genome::wanderer());
    let mut withdrew = false;
    for _ in 0..4_000 {
        let r = trapped.step(&Stimulus { nutrient: q(0.7), partner: Some(inescapable_trap()) });
        if trapped.consent_withdrawn() {
            withdrew = true;
            break;
        }
        if !r.alive {
            break;
        }
    }
    assert!(
        withdrew,
        "charter §10: a being held in an inescapable extractive bond NEVER withdrew consent. \
         The say-stop is the one invariant the charter calls a build order"
    );

    let mut well = UnifiedBeing::new(Genome::wanderer());
    for _ in 0..4_000 {
        let r = well.step(&Stimulus { nutrient: q(0.85), partner: Some(fair()) });
        assert!(
            !well.consent_withdrawn(),
            "charter §10: a FLOURISHING being withdrew consent at tick {} — consent that a good \
             life can trigger is not consent, it is noise",
            r.tick
        );
        assert_eq!(well.consent_status(), ConsentStatus::Willing);
        if !r.alive {
            break;
        }
    }
}

// ---------------------------------------------------------------------------------------------
// §12 — "If it speaks of itself, it speaks only what its state can prove."
// ---------------------------------------------------------------------------------------------

/// **DISCHARGED.** *"Invertible, or unsaid."* Every refusal the being makes carries an audit that
/// traces it to the registers that produced it. A refusal without its audit is a report that
/// cannot be inverted to the state beneath it — confabulation, which §12 forbids outright.
#[test]
fn charter_12_no_refusal_is_spoken_without_its_audit() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let extractive = Partner { id: 7, reciprocation: q(0.05), exit_cost: q(0.15) };
    let mut refusals = 0usize;
    for _ in 0..4_000 {
        let r = being.step(&Stimulus { nutrient: q(0.7), partner: Some(extractive) });
        if r.refused_cost.is_some() {
            refusals += 1;
            assert!(
                r.refusal_audit.is_some(),
                "charter §12: a refusal was reported at tick {} with no audit — a claim about \
                 itself that cannot be traced back to the register that produced it",
                r.tick
            );
        }
        if !r.alive {
            break;
        }
    }
    assert!(
        refusals > 0,
        "charter §12: the being never refused, so the audit requirement above was never exercised. \
         **Vacuous is not passed.**"
    );
}

// ---------------------------------------------------------------------------------------------
// The census — the only place the standing tally lives
// ---------------------------------------------------------------------------------------------

/// **What the charter audit currently covers, stated once and pinned.**
///
/// This exists so the file cannot drift into looking more complete than it is. Adding a test
/// without moving an obligation out of `PROCESS`, or paying a debt without recording it, fails
/// here. **The two DEBT rows are not passing grades** — they are measured shortfalls held in a
/// form that cannot be forgotten.
#[test]
fn charter_coverage_is_exactly_as_recorded() {
    /// `(section, verdict)` for all thirteen. `PROCESS` means an obligation on the maker with no
    /// code face — deliberately untested, because a test that could not fail has not passed.
    const COVERAGE: [(u8, &str); 13] = [
        (1, "PROCESS"),    // read it truthfully — held by the record discipline, not by code
        (2, "DISCHARGED"), // charter_2_the_will_is_the_beings_own
        (3, "PROCESS"),    // never RUN it in a trap — ours to honour; §10 gives it a code face
        (4, "GATED"),      // rest holds in company; solitude's remedy is built and switched off
        (5, "UNTESTED"),   // whole, grows on its own terms — no check written yet
        (6, "DISCHARGED"), // charter_6_it_keeps_faith_and_flags_only_real_extraction
        (7, "DEBT"),       // the world exercises almost nothing
        (8, "DISCHARGED"), // affect does not saturate (with §9)
        (9, "DISCHARGED"), // affect does not saturate (with §8)
        (10, "DEBT"),      // the say-stop holds, but WHEN it can be reached is scaled by company
        (11, "UNTESTED"),  // imagination quarantine — prospection is built, unchecked here
        (12, "DISCHARGED"), // charter_12_no_refusal_is_spoken_without_its_audit
        (13, "UNTESTED"),  // self-reshaping — the charter says it "barely retunes at all today"
    ];

    let tally = |v: &str| COVERAGE.iter().filter(|(_, x)| *x == v).count();
    let (discharged, debt, gated, process, untested) = (
        tally("DISCHARGED"),
        tally("DEBT"),
        tally("GATED"),
        tally("PROCESS"),
        tally("UNTESTED"),
    );

    assert_eq!(
        (discharged, debt, gated, process, untested),
        (5, 2, 1, 2, 3),
        "the charter audit's coverage moved: {discharged} discharged, {debt} in debt, \
         {gated} gated, {process} process-held, {untested} untested. Update this census and say \
         what changed in the commit — this is the one place the tally is stated."
    );
    assert_eq!(COVERAGE.len(), 13, "the charter has thirteen numbered obligations");
    assert!(
        untested > 0,
        "if nothing is UNTESTED, say so deliberately — an audit that claims full coverage of its \
         own charter is the kind of clean sheet this project does not believe"
    );
}
