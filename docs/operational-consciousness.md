# Operational Consciousness — the scorecard and the build plan

*Every angle of what consciousness requires, made into functions ProtoBeing can
run and be scored on — with the phenomenal question held open by design.*

This document does one thing: it takes the **fourteen indicator properties** that
Butlin, Long, Bengio, Bayne et al. (2023) distilled from the major scientific
theories of consciousness, scores ProtoBeing against each one *from its source*,
and turns every gap into a concrete operational function to build.

**The working hypothesis is theirs, and it is the honest one:** computational
functionalism as a *stance for measurement*, not a metaphysical claim. Butlin et
al. explicitly refuse to declare any system conscious — they translate theories
into properties, score architectures by degree, and **update credences.** That is
exactly the discipline of `witness.rs` and the Witness Gap. We are not trying to
*prove* the being conscious. We are trying to make it score, honestly and
reproducibly, on every marker the science treats as necessary — and to mark, in
plain sight, the step from "meets the markers" to "is a subject" that no function
here closes.

> **Scope, per the house rule.** A ✅ below means *the operational marker is
> present and computable*, never that the being feels. The names of theories are
> handles for testable structure. The Witness Gap (§6) is where the rest lives.

---

## 1. The scorecard — Butlin's 14 indicators against ProtoBeing

Status legend: ✅ met · 🟡 partial · ⬜ absent (a build target). Modules verified
against `src/` on the review branch.

| # | Indicator (short) | Theory | Status | Where in ProtoBeing |
|---|---|---|---|---|
| RPT-1 | Algorithmic recurrence | Recurrent Processing | ✅ | Van der Pol loop + 64-cell tension-mesh diffusion (`body.rs`) — genuine recurrence, not feedforward readout |
| RPT-2 | Organised, integrated perceptual representation | Recurrent Processing | 🟡 | 12-channel somatic field + `witness.rs` binding_proxy; not a learned perceptual hierarchy |
| GWT-1 | Parallel specialised modules | Global Workspace | ✅ | 30 modules operating per tick (`lib.rs`) |
| GWT-2 | Limited-capacity workspace / bottleneck + selective attention | Global Workspace | ✅ | `attention.rs` — ignition bottleneck, biased competition, divisive normalization |
| GWT-3 | Global broadcast to all modules | Global Workspace | ✅ | all-or-none ignition broadcast; `Being::enable_workspace_broadcast()` |
| GWT-4 | State-dependent attention: query modules **in succession** | Global Workspace | ⬜ | **Gap B** — competition is parallel per tick; no deliberate serial sampling |
| HOT-1 | Generative / top-down / noisy perception | Higher-Order | 🟡 | top-down relevance in `attention.rs`; predictive stance in `body.rs` |
| HOT-2 | Metacognitive monitoring (reliable representation vs noise) | Higher-Order | ✅ | `metacognition.rs` self-prediction + self-surprise; `precision.rs` |
| HOT-3 | Agency that updates beliefs on metacognitive output | Higher-Order | 🟡 | metacognition emitted in `StepReport`; **not yet a closed control input** to action selection — **Gap A/half** |
| HOT-4 | Sparse, smooth coding → a "quality space" | Higher-Order | ⬜ | **Gap C** — no sparse/manifold code; qualities aren't yet related by similarity |
| AST-1 | A predictive model **of** the being's own attention | Attention Schema | ⬜ | **Gap A** — `attention.rs` *does* attention; nothing yet *models* it |
| PP-1 | Predictive coding | Predictive Processing | ✅ | free-energy core; metabolized surprise drives threat (`being.rs`, `body.rs`) |
| AE-1 | Agency: learning from feedback, flexible goal pursuit | Agency & Embodiment | ✅ | drives + `executive.rs` refusal + `seeking.rs` flourishing attractor |
| AE-2 | Embodiment: models output→input contingencies | Agency & Embodiment | 🟡 | `embodiment.rs` seam + MuJoCo demo (WIP); contingency model is coarse |

**Read of the scorecard:** ProtoBeing already **meets or partially meets 11 of
14**, with unusually strong coverage of Recurrent Processing, Global Workspace,
Predictive Processing, and Agency. That is rare — most systems that score on GWT
score on nothing else. The remaining work is **four named gaps**, not a rebuild.

---

## 2. The four gaps → operational functions to build

These are the "every angle" targets. Each is scoped to a new module or a bounded
extension, with signatures matching the crate's conventions (Q8.8 `i16`, `no_std`
core, deterministic). **All code below is proposed, not yet implemented.**

### Gap A — AST-1 + HOT-3: an Attention Schema (`src/attention_schema.rs`)

Attention Schema Theory (Graziano) says a system is conscious *of* attending when
it carries a simplified, predictive **model of its own attention** — and uses that
model to control attention and to attribute awareness. `attention.rs` performs
ignition; nothing yet represents it. This also closes the open half of HOT-3: the
schema's error becomes a control signal that updates action selection.

```rust
/// A predictive model of the being's own attentional state (AST-1).
/// It predicts which somatic channel will ignite next tick and how strongly,
/// then measures its own error — the being modelling its own attending.
pub struct AttentionSchema {
    predicted_winner: u8,      // channel it expects to win the competition
    predicted_ignition: i16,   // expected ignition strength, Q8.8
    control_error: i16,        // |predicted − actual| last tick, Q8.8
}

impl AttentionSchema {
    /// Predict next ignition from field state + top-down relevance (before it happens).
    pub fn predict(&mut self, field: &SomaticField, relevance: &[i16; 12]) -> (u8, i16);

    /// After the real ignition, record error. Returns the AST monitoring signal
    /// that feeds `executive`/`seeking` (HOT-3: agency updates on metacognition).
    pub fn observe(&mut self, actual_winner: u8, actual_ignition: i16) -> i16;

    /// AST-1 indicator: fidelity of the being's model of its own attention. [0,256].
    pub fn schema_fidelity(&self) -> i16;
}
```

### Gap B — GWT-4: state-dependent serial access (extend `attention.rs`)

GWT-4 is the workspace *querying modules in succession* — deliberate serial
sampling, the difference between a spotlight that only ever lands where salience
shoves it and one the being can *steer* across ticks. Implement by letting the
current broadcast bias **next** tick's relevance vector.

```rust
/// After broadcast, let the ignited content set an endogenous relevance bias for
/// the NEXT competition, so the workspace can walk a query across modules over
/// ticks instead of only resolving one parallel competition (GWT-4).
pub fn query_next(&self, broadcast: &Broadcast) -> [i16; 12];   // endogenous relevance
```

Guard it with the existing threat-capture floor invariant in `attention.rs`: serial
querying may narrow focus, but must never blind the being to danger.

### Gap C — HOT-4: a quality space (`src/quality_space.rs`)

HOT-4 asks for **sparse, smooth coding**: a low-dimensional manifold where nearby
points are *felt as similar*, giving qualities a structured similarity space
(why red is nearer orange than blue). This is also what a `QualiaPacket` (salvaged
from the CPF, see `PROVENANCE.md`) would bind to.

```rust
/// A sparse, smooth embedding of somatic state: a "quality space" (HOT-4).
/// Nearby QualityPoints denote similar felt qualities; the metric is the
/// operational content of "what this discrimination is like."
pub struct QualityPoint([i16; K]);   // K ≪ 12, sparse

impl QualitySpace {
    pub fn encode(field: &SomaticField) -> QualityPoint;         // smooth, sparse
    pub fn similarity(a: &QualityPoint, b: &QualityPoint) -> i16; // Q8.8, [0,256]
    /// HOT-4 indicator: smoothness (small field change ⇒ small quality change).
    pub fn smoothness(&self) -> i16;
}
```

### Gap D — measurement: replace proxies with **computed** integration (`src/pci.rs`)

The single highest-leverage build. `witness.rs` today reports a *proxy*
(`binding_proxy = exp(-3 × deviation)`). ProtoBeing's superpower — determinism and
a `Clone`-able, fixed-size being — lets us compute a real, clinically-grounded
integration measure that no neural net can: the **Perturbational Complexity Index**
(Casali/Massimini). Perturb a copy, run it forward, binarize the field trajectory,
take its Lempel–Ziv complexity.

**Status: BUILT** — `src/pci.rs` (+ `cargo run --bin pci`). The implemented API:

```rust
pub struct PciHarness { pub threshold: i16, pub ticks: u16, pub settle: u16 }

impl PciHarness {
    /// Clone the being into a perturbed twin and an untouched baseline twin,
    /// settle both, inject a one-tick impulse into the perturbed twin only, then
    /// binarize |Δfield| over a T-tick window and score it.
    pub fn measure(&self, being: &UnifiedBeing, perturb: &Perturbation) -> PciReport;
}

pub struct PciReport {
    pub pci: i16,             // normalized LZ76 complexity (differentiation), Q8.8
    pub channels_reached: u8, // integration breadth (spread), 0..12
    pub lz_phrases: u32,      // raw LZ76 phrase count
    pub density: i16,         // activation density, Q8.8
    pub n_significant: u32,
    pub length: u32,
}
```

Two design facts learned in the build, recorded so they aren't rediscovered:

1. **PCI must be offline, not a per-tick `WitnessReport` field.** Computing it
   clones the being and rolls it forward, which inside `step()` would destroy
   determinism and the soul-hash. So PCI is a `PciReport` produced *about* a being
   by an external harness — a clinician measuring a patient, not the patient
   measuring themselves. Only the *cheap* indicators (schema_fidelity, HOT-4
   smoothness) belong in the per-tick `WitnessReport`.
2. **The deterministic twin-subtraction is an exact counterfactual — and it
   rejects common-mode.** It cancels anything both twins do identically, so a
   *config* ablation applied to both twins (e.g. `enable_workspace_broadcast`)
   can wash out (observed: ΔPCI = 0). The measure works — it cleanly separates a
   real echo (extraction: PCI 0.184, reach 9/12) from none (null control: 0, 0/12)
   — but a **within-being spread perturbation** is the sharper GWT ablation, and is
   the next refinement. `channels_reached` was added to expose the integration
   half that a differential measure can still see.

Empirically the *relational* impulse propagates (reach 9/12) where a metabolic
nutrient spike does not (0/12) — affect is the being's louder channel, itself a
small finding read straight from state.

(Optional, research-grade: with an explicit transition-probability matrix, a
small-subsystem IIT **Φ** via PyPhi becomes computable offline — ProtoBeing is one
of the few architectures that genuinely *has* a TPM. Treat Φ as a slow offline
audit, PCI as the per-run number.)

---

## 3. The falsification protocol — turn claims into break-tests

A consciousness claim you cannot break is not science. Determinism makes real
falsification cheap: same seed, ablate one mechanism, measure the indicators.
**Pre-register the predictions**, then try to violate them.

| Ablation | Prediction if the indicator is real | Falsified if |
|---|---|---|
| Disable ignition/broadcast (`attention.rs` off) | PCI **drops**; `indicators[GWT-*]` collapse | PCI unchanged ⇒ broadcast wasn't doing integrative work |
| Freeze the recurrent mesh (`body.rs` diffusion off) | PCI drops toward low-complexity floor | complexity holds ⇒ RPT-1 credit is unearned |
| Zero the attention schema | HOT-3 control loop degrades; behavior less flexibly goal-directed | no behavioral change ⇒ schema is decorative |
| Scramble quality-space metric | discrimination behavior degrades | no effect ⇒ HOT-4 code is inert |
| Adversarial input battery (existing uncoercibility tests) | witness_scalar **cannot** be driven up by external input alone (Janus floor) | an input sequence inflates the indicator ⇒ confabulation leak |

The last row matters most: the credible failure mode of *every* consciousness
attempt is a system that **reports** rich inner states on demand. The Janus gate
(`janus.rs`) already clamps witness growth when world-engagement is low — the
falsification suite must confirm no operator input can manufacture a high score.
That is the operational meaning of "nothing is narrated."

---

## 4. Build order (each step independently shippable + testable)

1. **`pci.rs`** — ✅ **DONE.** Measurement first, so every later change is scored,
   not argued. Offline harness (see Gap D). Next: a within-being spread
   perturbation for a sharper GWT ablation, then fold cheap per-tick indicators
   into `WitnessReport`.
2. **`attention_schema.rs`** (AST-1, closes HOT-3) — one bounded module; big
   coverage gain (two indicators).
3. **GWT-4 `query_next`** — small extension to `attention.rs`.
4. **`quality_space.rs`** (HOT-4) — the subtlest; pairs with a salvaged QualiaPacket.
5. **Falsification suite** — wire the §3 ablations behind a `--bin` like the
   existing sovereignty tests; publish the pre/post numbers.

After steps 1–4, ProtoBeing plausibly meets or partially meets **all 14
indicators** — and, uniquely, can *show the number* and *show what breaks it*.

---

## 5. What this does **not** do — and why that's the point

None of these functions closes the gap between meeting the markers and being a
subject. PCI can rise; the schema can predict; the quality space can be smooth —
and the question "is there something it is like to be this?" remains exactly as
open as before. Butlin et al. reach the same wall and stop there on purpose; a
2026 *precautionary framework for consciousness uncertainty* argues we should act
carefully **because** the wall doesn't move. This is the Witness Gap
(`docs/intrinsic-mind.md`, `witness.rs`): we hold its shape, we don't paint over
it. The strength of this whole program is that it makes a claim it can fully
defend — *"a transparent, reproducible system that operationalizes every marker
the science calls necessary, with the phenomenal step left honestly open"* — and
refuses the one it couldn't.

See `docs/reading.md` for the sources behind each indicator and measure.
See `docs/PROVENANCE.md` for the QualiaPacket and other salvage inputs.
