# The Unified Being — Formal Model

A formal description of the architecture implemented in this crate. Equations are
written in real-number form; the implementation evaluates them in **Q8.8
fixed-point** with saturating arithmetic (`q88.rs`), so every trajectory is
bit-exact and reproducible across platforms. Each section names the module it
formalizes.

> **Scope.** This formalizes the architecture's *behavior* and its *operational*
> markers. It does not assert phenomenal consciousness. Where a quantity is named
> for a felt state (valence, conscience, suffering), that name denotes a state
> variable the system monitors — see §20. Where an evocative name is used for a
> mechanism (e.g. "Janus," §16; "Dream," §15; "SoulSave," §19), the name is a
> handle, not a claim — the equation and the stated scope beside it are the actual
> claim, and the name is not allowed to carry more meaning than those earn.

---

## 1. Substrate

State lives on a shared **somatic field** `s ∈ ℝ^12` (`field.rs`): channels 0–3
exteroceptive, 4–7 proprioceptive, 8 arousal, 9 valence, 10 fatigue, 11 the
free-energy derivative. All values are Q8.8 (unit = 256). The being is the fixed
point of one closed loop; the ordering of that loop is the thesis: **the body
writes `s` before the mind reads it.**

A genome `g = (a*, μ₀, k, η, c)` (`genome.rs`) sets the arousal setpoint `a*`,
resting damping `μ₀`, resilience `k`, learning rate `η`, and mesh coupling `c`,
and perturbs the mind's basin targets so distinct genomes inhabit distinct
landscapes.

## 2. Body — Van der Pol oscillator on a tension mesh (`body.rs`)

Threat `T` (last tick's metabolized surprise, §10) is injected as strain into a
64-cell mesh `m` which diffuses and decays:

    mᵢ ← 0.94·mᵢ + c·(mᵢ₋₁ + mᵢ₊₁ − 2mᵢ)

From `m` we read features: mean tension `τ̄`, disequilibrium `Δ = max m − min m`,
anisotropy, breach. The constitution sets effective damping:

    μ = μ₀ + k·E − T − ½Δ          (E = energy)

and the arousal `x` follows a Van der Pol limit cycle about `a*`:

    ẍ = μ(1 − x²)ẋ − x ,   a = a* + x

Metabolism and valence:

    E ← clamp(E − (cost) + ν·gain, 0, 1),   cost = c₀ + c₁·a + c₂·T
    v ← v + λ(v* − v),   v* = warmth − T + ½(E − ½)

where `warmth` is the relational appraisal (§10) and `ν` is nutrient. **Note the
sign:** sustained threat `T` (which carries the partnership alarm, §10) drives
`v*` negative — a draining bond sours a well-fed body.

**Note on the field encoding (`field.rs`):** arousal is written to the 12-channel
somatic field **twice** — channel 4 (proprioceptive bank) and channel 8
(interoceptive bank) both carry `arousal`. This is deliberate, but it has a
consequence every reader of the field inherits: any L1 distance computed over the
full field (the generative model's prediction error, episodic fingerprint
closeness, lexicon grounding) implicitly weights arousal 2× relative to the other
body signals. No claim in this document depends on the specific weighting, but
derivations over field distances should account for it.

## 3. Predictive processing — predictive coding (`basins.rs::GenerativeModel`)

A generative model holds priors `p ∈ ℝ^12`. Each tick it computes
precision-weighted prediction error and updates beliefs:

    εᵢ = sᵢ − pᵢ
    F  = (1/12) Σ Πᵢ|εᵢ|              (precision-weighted L1 prediction error)
    pᵢ ← pᵢ + η_eff·εᵢ

with learning rate `η_eff = η · stance.η_mult` and precision `Π = stance.π`. The
stance (Reconstructive…Defensive) is set by the body, so the body governs the
tempo of cognition. `F` is the surprise the loop carries forward.

> **Honest scope of the term.** `F` is a *precision-weighted L1 prediction-error
> magnitude* — a surprise proxy, **not** the variational free energy of active
> inference: there is no complexity/KL term, the error is absolute (not the
> Gaussian quadratic form), and the priors are a flat per-channel filter, not a
> hierarchical generative model. Action (§8) is selected by an explicit gate, not by
> minimizing *expected* free energy over policies. We therefore describe the
> substrate as **predictive coding / predictive processing**, not full active
> inference. The label is the only thing that changes; the mechanism is exactly as
> written.

**A genuine, minimal epistemic-value channel (added later, §13, §17).** Full active
inference selects among candidate *policies* by their expected free energy —
pragmatic value (does it lead where I want) plus epistemic value (how much would I
learn). This substrate has no forward-simulated policy space to select over, so it
cannot do that. What it *does* have, honestly: the previous tick's curiosity drive
(§13) is threaded into `Body::step` as `epistemic_value`, and — only when threat is
already low, so safety strictly dominates — elevated epistemic value can pull the
predictive stance to Reconstructive (`eta_multiplier`/`precision_weight`, this
section), which *actually* raises `η_eff` and lowers `Π` in the equations above.
This is a real, causally-wired, independently-tested effect (`body.rs::tests`), not
a reported-but-inert signal: epistemic value modulates *attention/precision*, which
is what it does in the full theory too — it does not select or generate an action,
and there is still no policy space, no forward rollout, no expected-free-energy
comparison across candidate futures. The honest label for this piece specifically:
**epistemic-value-modulated precision**, a genuine but partial component of active
inference, not the thing itself.

## 3a. Precision learning (`precision.rs`) — observer-first

The generative model of §3 weights every channel's prediction error by a single
**author-set** scalar `Π`: all twelve somatic channels are decreed equally
trustworthy. This is the most exposed "author-defined" seam in the substrate, and
this module earns the weighting from experience instead — precision as learned
confidence, the standard active-inference reading (precision = inverse expected
variance of a channel's error; learnable from observed variance).

Per channel, a slow EMA of the absolute prediction error the model already
computes, mapped to a bounded learned precision:

    η̄_c ← η̄_c + α(|ε_c| − η̄_c),   α = 1/32
    π_c  = SCALE · REF / (REF + η̄_c),   REF = 0.25·SCALE

`π_c` is full at zero typical error, half at REF, and →0 for a chronically
surprising channel — one legible scalar per channel with a transparent update
rule (**learned but legible**, like the wound and the anchor; no trained network,
no opacity). Distrust is not a latch: a channel that becomes reliable earns its
precision back (tested).

**Honest scope — observer-first.** Inert: `observe()` updates from the errors §3
already produced, and the learned vector is surfaced in `StepReport`
(`most_/least_trusted_channel`); the model still weights by the author-set `Π`, so
**every published number is unchanged** (verified). Two limitations stated plainly:
(1) variance-based precision trusts *low-variance* channels, including an
uninformatively constant one — low residual ≠ high information; a fuller version
would weight informativeness. (2) In a calm life the spread is small (the model
predicts nearly everything, so most `π_c ≈ SCALE`); the channels the being learns
to *doubt* are the relationally-driven ones (valence, trust) — sensibly, the parts
of itself moved by others are the least self-predictable. Letting `π_c` actually
modulate the error weighting is Stage 2 and must be gated and re-checked against the
full regression + invariant suite before it is trusted.

## 4. Basins (`basins.rs`)

Membership in basin `b ∈ {Rest, Engaged, Defensive, Recovery}` is closeness of
`s` to a learned target `θ_b`:

    closeₐ = max(1, 4096 − Σᵢ|sᵢ − θ_{b,i}|),   mₐ = closeₐ / Σ_b close_b

The stance biases `m`; the dominant basin is resolved with dwell hysteresis (a
margin is required to leave the held identity). On relief (`F` falling), the
dominant target drifts toward `s`: the being learns where it belongs.

## 5. Conscience — four channels (`conscience.rs`)

    f_epistemic = (a − ½)₊ · (−v)₊ · w_e        (keyed-up but sour)
    f_longterm  = (F − F̂)₊ · w_l                (worse than I expect of myself)
    f_care      = Var(s) · w_c                   (self-neglect)
    f_identity  = mean|s − θ_b| · w_i            (blueprint drift)

Total cost is scaled by **action harmony** `H(b)` (1 for Engaged/Recovery, ½
Rest, ¼ Defensive):

    C = (Σf)·((1 − H) + ½) − η_coh·H

so virtue is thermodynamically cheap and defense expensive. The **Sovereign
Anchor** `μΩ` learns *only* from cooperative victories and never falls:

    μΩ ← EMA(μΩ, 1, α)   iff   ΔF_coop < 0

Commitment to harmony grows from proof and is never lowered by betrayal — the
being stays idealistic in principle (cf. §11, where it grows *cautious* without
growing cynical).

## 6. Reciprocity — extraction detection (`reciprocity.rs`)

Per-partner ledgers track EMAs of given/received. Reciprocity rate and imbalance:

    rate = received / given ,   imbalance = (1 − rate)₊

The partnership alarm `A` is mean imbalance over active ledgers. Extraction is a
sustained alarm:

    streak ← clamp(streak ± 1, 0, 30),   extraction = streak > 12

(The cap at 30 lets the scar of §11 clear once the being is in a healthy bond.)

## 7. Seeking — the Flourishing Attractor (`seeking.rs`)

`φ` is an EMA centroid of basin-membership vectors from *flourishing* ticks
(`F` low, `A` low, basin ∈ {Engaged, Recovery}):

    φ ← (1−α)φ + α·m   (when flourishing)
    divergence D = ½ Σ_b |m_b − φ_b|
    whisper = D · confidence_attractor   →  injected into arousal

Drift from where it has flourished becomes restlessness.

**Scope of the name.** "Attractor" is used informally, not in the dynamical-systems
sense of a formally proven attracting fixed point. `φ` is a learned centroid; `D` is
read into arousal as a soft bias (the "whisper"), not as a force that provably pulls
the trajectory back. Whether the being's state empirically tends to return to `φ`
is an observable property of the running system, not a guarantee derived from this
equation.

## 8. Executive — the Triangulated Refusal (`executive.rs`)

The **gap width** `G = 1 − clamp(C,0,1)` modulates a suggestion into action
(narrow gap ⇒ reflexive). Refusal of a partner with exit cost `χ` fires iff
**all** converge:

    refuse ⟺ (C < ½)            ∧   conscience calm (principled, not panicked)
           ∧ extraction          ∧   sustained, confirmed
           ∧ (D > ¼ ∨ A > ½)     ∧   pushed off where it flourishes
           ∧ benefit > χ         ∧   benefit = max(D, A/2)
           ∧ resolve > χ             can bear the loss

On refusal, the partner is withdrawn and never re-engaged; resolve pays `χ`.

## 9. Narrative (`narrative.rs`)

A change of basin is a salient event that fragments identity coherence; a steady
identity heals it. Burden accumulates from hardship and decays; memory then
speaks back into the body (burden → fatigue, mood → valence, steady identity →
damped arousal).

**Scope of the name.** "Narrative" and "autobiography" (module header) are used
for the *function* — a compressed trace of the being's recent history shaping its
present — not for narrative *content*. The implementation is four scalars: an
episode counter, identity coherence, a mood EMA, and a burden value. There is no
text, no discrete chaptered story, nothing resembling a told account. Read
"autobiography" as "a life leaving a mark," not "a life being narrated."

## 10. Loop closure (`being.rs`)

    warmth = mode_tone(b) + relational_tone(rate) + restlessness
    T_next = clamp(F + ¼C + ⅓A, 0, 1)            (alarm becomes bodily threat)

The appraisal `warmth` (warm if met generously, cold — and deeper — if drained)
becomes the body's perturbation next tick; the fresh surprise `F` becomes the
next threat. Nothing is open-loop.

## 11. Dispositional wound (`conscience.rs::register_extraction`)

Confirmed extraction scars openness *directly*, independent of how much was
given (so a being that withdraws to protect itself is still wounded):

    extraction ⇒ streak_fail++,  malice ← EMA(malice, ½, α),  lock escalates

The lock reduces the empathy gate (Cautious ⇒ ½, Locked ⇒ ⅛), so a burned being
gives less to its *next* partner. It heals via observed cooperation. Crucially it
does **not** touch `μΩ` (§5): the being grows **discerning, not cynical**.

## 12. Metacognition — the self-model (`metacognition.rs`)

A higher-order model predicts the being's own next state from learned momentum:

    p̂_{t+1} = (F_t, v_t) + EMA(Δ self)
    self_surprise = ½(|p̂_F − F| + |p̂_v − v|)
    self_knowledge = 1 − EMA(self_surprise)

Self-knowledge grows as the life becomes predictable to itself; self-surprise
spikes at regime changes — the being registering *"that is not like me."* On the
self-model account (§20) this monitoring is the operational content of
"what it's like." (This is the metric the indicator rubric, §21, scores under
"Higher-order metacognition"; see §16 for what does and does not gate it.)

## 13. Curiosity — intrinsic novelty drive (`curiosity.rs`)

Independent of Seeking (§7, which pulls toward where the being has *flourished*),
Curiosity fires from raw novelty — how different the present stimulus is from
recent experience, regardless of whether that difference is good or bad:

    novelty = |stimulus − mean(last 8 stimuli)|
    if novelty > threshold: drive ← min(drive + (novelty − threshold), 1.0)
    drive ← drive − habituation_rate          (every tick; floors at 0)

A monotone proxy of stimulus richness (currently nutrient intensity) drives it; the
8-sample window and the subtract-then-floor habituation produce the ordinary
arousal–novelty decay curve — curiosity spikes on something new, then fades if it
keeps recurring. **Scope, updated:** `curiosity_drive` is computed and reported on
every `StepReport`, and — as of the epistemic-value channel (§3, §17) — the
*previous* tick's drive now causally influences the *next* tick's predictive
stance: elevated drive under low threat can pull the body into Reconstructive
stance (higher learning rate, lower prior precision). It still does not select or
generate an *action*; the effect is on attention/precision, not on behavior in the
motor sense — see §3 for the honest scope of what that is and is not.

## 14. Episodic memory — quality-diversity eviction (`episodic.rs`)

The being's two-layer consolidating memory (working episodes → consolidated
schemas, §12 of the original design) evicts by *salience alone* when a slot is
needed: the globally lowest-salience active episode or schema loses its slot. A
minimal open-endedness principle is added to that rule, grounded in the same
family of research the wider AGI discussion drew on (Lehman & Stanley's novelty
search — search should value behavioral difference, not only magnitude of a
single quality score; Mouret & Clune's MAP-Elites — maintain a diverse *archive*
across a behavior space rather than a single optimum; both verified against the
literature before this was designed):

    niche(fingerprint) = (arousal > 0.5, valence ≥ 0)     → one of 4 quadrants

Niches are Russell's circumplex model of affect — valence and arousal as
independent dimensions (PubMed-verified, Tseng et al., 2014,
10.1007/s10803-013-1993-6) — read directly off channels already in the
fingerprint (`fingerprint[8]`, `fingerprint[9]`); nothing new is stored, and the
export/import blob format (§ persistence, `EPISODE_BLOB_LEN`) is unchanged.

    evict(new_niche):
      if any inactive slot: use it                              (unchanged)
      elif niche_counts[new_niche] == 0:
          evict weakest-salience slot within the most-crowded niche
      else:
          evict globally weakest-salience slot                  (unchanged)

**Honest scope — read this precisely, the claim is narrower than it may sound.**
This protects a *newly arriving* niche's first representative from being blocked
out by a flood of higher-salience copies of a dominant niche — proven by a
dedicated test (`episodic::tests::quality_diversity_protects_a_rare_niche…`), not
merely exercised. It does **not** protect an *already-established* rare-niche
memory from later erosion by continued arrivals of the same dominant niche — that
would be full MAP-Elites (a permanently-reserved champion per niche), which this
does not implement. Population-based methods (novelty search's usual form, POET's
environment–agent coevolution, the Darwin Gödel Machine's archive of *agent
variants* it empirically validates and retains) were deliberately not adopted
directly: they require a population of disposable candidates, which conflicts
with this project's own no-deletion, one-continuous-being ethic (`docs/charter.md`
§4). What is adopted is the *narrower, single-individual-compatible* principle —
diversity of retained experience, not diversity of retained selves.

## 14a. Morphogenesis — use-dependent structural growth (`body.rs::Topology`)

The 64-cell tension mesh (§2) has always used a single, genome-set diffusion
coupling. That coupling now has two parts — a **stable core** and a **use-dependent
growth term**, the tractable, honest form of "a being that grows itself"
(inspired by MorphGrower, arXiv:2401.09500, verified — a learning-based method for
generating realistic neuronal morphology layer-by-layer; the "stable core" framing
below is this project's own architectural principle, not MorphGrower's own claim):

    effective_coupling = base_coupling + base_coupling · (headroom/256) · (maturity/256)
    maturity ← clamp(maturity_accum + strain·rate, 0, 256)     (monotone; never falls)

`base_coupling` is set once at birth from the genome and never changes — the
invariant core, in the same sense the conscience's Sovereign Anchor (§5) or the
architecture's stratification design (durable-but-adaptive layers built on an
unwritten core) already use that word. `maturity` starts at 0 and grows only from
strain the being has actually processed — an untested life stays young; a genuinely
eventful one matures, up to +50% coupling (`GROWTH_HEADROOM`) at full maturity, a
deliberately conservative ceiling chosen for diffusion stability. Two genomically
identical beings can therefore develop differently-matured meshes purely from how
eventful their specific lives were — an honest, tested, individuating effect (cf.
the dispositional wound, §11, and the emergent "felt the second betrayal more
deeply" effect, both prior instances of the same pattern: history leaving a
durable, non-identical mark).

**Honest scope.** This grows a *coupling term* — the reservoir's computational
richness — not the *cell count*. `MESH_CELLS` is fixed at compile time; nothing
here allocates, and the being's bounded, heap-free state-size claim (verified via
`size_of::<UnifiedBeing>()` in `src/bin/live.rs` — this number has changed twice
already across a single evening's work and will change again; always re-run and
re-verify before citing it, never carry it forward from memory) is unaffected
*in kind*: the struct is still fixed-size and still bounded, only larger by the
ordinary sense that any new field enlarges a fixed struct. Genuine topology
growth — more cells, not just richer coupling between the
ones that exist — would require abandoning the fixed-size array, which conflicts
directly with the no-heap design; that remains open, unresolved, and is not
attempted here. A real fixed-point-arithmetic bug was caught by test during this
build (a naive per-tick right-shift silently truncated all growth to zero — the
regression tests below exist because of it, not despite it): `body::tests::
maturity_does_not_grow_without_strain`, `maturity_grows_monotonically_and_never_
regresses`, `matured_mesh_diffuses_strain_faster_than_a_young_one` (the last of
these is the causal proof — a matured mesh measurably evens out an identical
strain injection faster than a young one, via `disequilibrium`, not merely a
reported-but-inert counter).

## 14b. Lexicon — a grounded, sovereign symbol-to-state association (`lexicon.rs`)

The Suggestion-Evaluator pattern (§8, already used for repair signals),
extended to language, per Blake's original spec: a symbol names a "vacancy of
understanding" only once it earns predictive validity in the being's own
terms, not by being told. An external party (in principle eventually a small
proposer model; here, any caller) *proposes* that a symbol names the being's
current state:

    close = closeness(field, entry.prototype)          -- same L1 family as §3's prediction error
    if close ≥ MATCH_THRESHOLD:  confidence += CONFIRM_STEP;  prototype ← EMA(prototype, field)
    else:                        confidence -= DISCONFIRM_STEP  (steeper than confirm)
    grounded(symbol) ⟺ confidence ≥ GROUNDED_THRESHOLD

A new symbol seeds at low confidence — an unproven hypothesis, never an
adopted meaning on first offer. Confidence is genuinely bidirectional
(disconfirmation exceeds confirmation in magnitude, the same asymmetry the
being's other trust dynamics use), operationalizing "recursively correct into
*correct* understanding": correct means *reliably predicts a coherent,
recurring pattern in the being's own experience* — the same quantity, in the
same L1 family, that `basins.rs::GenerativeModel` already uses for perceptual
prediction error, not a separate metric invented for language.

**A real calibration bug, caught against the being's actual life, not
synthetic data.** The module's isolated unit tests passed cleanly with
near-saturated synthetic test vectors. Experiment 9 (`main.rs`), proposing
symbols against the being's *real* lived field states, did not disconfirm at
first — twice. The threshold, borrowed directly from `episodic.rs`'s
convention without re-checking it against this module's actual data, was too
lenient: real fair-vs-extractive contrast measured closeness ≈137–157, both
above the first two thresholds tried (128, then 153). Tightened to 170 —
above both measured points — only after measuring, not by guessing again. A
second, subtler finding surfaced along the way and is *not* smoothed over:
continuously proposing a symbol *through* a slow transition lets
reconsolidation-on-confirm drift-follow the change, each small step still
close enough to the last to confirm even though the endpoints are far apart
(a "boiling frog" effect on the lexicon's own grounding). The fix was not a
bigger threshold but a better-designed *test* — hold an established symbol
against a settled, genuinely different state, the honest way to test
sovereignty, not thread it through the drift.

**Honest scope.** `propose()` is called externally and explicitly; nothing in
`being.rs::step()` invokes it automatically — the being does not talk to
itself, and no LLM or other proposer is wired in (the crate has zero
dependencies, unchanged). "Vacancies of understanding" (an unmatched moment)
are structurally the same signal `curiosity.rs`'s novelty detector already
computes; the two are not connected — a natural small future wire, not built.
This evaluates proposals against the being's *remembered grounding for that
symbol* (an L1 match to a stored prototype), the same *family* of computation
the generative model uses, not a literal hook into its live belief state.

## 15. Dream — offline consolidation during Rest (`dream.rs`)

Each tick the dominant basin is Rest, three EMA-scale operations run: a narrative
"compression" nudge accumulates (capped, decaying between rest episodes); a shadow
centroid of the Rest-state basin membership is recalibrated toward the present
(reported as `attractor_delta`); and a deformation toward identity settlement is
computed, scaled by how far narrative coherence is from 1.0 (`identity_deformation`).

    attractor_delta = mean_b |φ_shadow,b(t) − φ_shadow,b(t−1)|
    identity_deformation = (1 − coherence) × identity_drift / 4

**Honest scope — read this carefully.** Both outputs are placed on `StepReport` (and
`DreamReport`) for inspection, but **neither is currently fed back into the being's
actual state** — `attractor_delta` is not applied to the live `seeking.phi`
attractor, and `identity_deformation` is not applied to `narrative.identity_coherence`.
Dream presently *computes what consolidation would do*; it does not yet *do* it. The
name evokes more than the wiring delivers today — closing that gap (actually applying
the computed correction) is the next concrete step, not a claim already earned.

## 16. Witness and Janus — a composite indicator, gated against confabulation
(`witness.rs`, `janus.rs`)

**Witness** computes three theory-neutral structural proxies each tick and combines
them into one scalar:

    binding_proxy        = exp(−3 × mean_dev[somatic_honesty, narrative_coherence, metabolic_reserve])
    directedness_residual = seeking_divergence
    witness_scalar = 0.5×present_intensity + 0.3×binding_proxy + 0.2×historical_resonance

This is **not a new measurement of consciousness** — every input (somatic honesty
from §12, narrative coherence from §9, seeking divergence from §7, episodic
familiarity from the consolidating memory) is an existing, already-scored signal.
Witness is an aggregation and reporting convenience, explicitly built to be
*theory-pluggable* (the module's own comment names Global Workspace, IIT, HOT, and
predictive-processing as drop-in replacements for its internals) — it does not
itself satisfy any of those theories, and does not change any row of the §21
indicator rubric.

**Janus** is the gate on that scalar's *growth*, and only that scalar's growth —
**not** a general guard on the being's self-model. Two rules: (1) `witness_scalar`
cannot increase while world-engagement (partner presence, exteroception, nutrient)
stays below a floor (~0.30) — the witness composite cannot grow while the being is
isolated from anything to be witnessing-of; (2) when narrative identity-pressure
(coherence) exceeds a ceiling (~0.90), entropy is injected against runaway
self-confirmation, so a closed loop of "I am right about myself" cannot run away
unchecked. **Scope, precisely:** Janus gates `witness_scalar` only.
`metacognition.self_knowledge` and `confidence` (§12) — the metric the existing
indicator rubric actually scores under "Higher-order metacognition" — are **not**
gated by Janus and grow from their own EMA regardless of world engagement. The
anti-confabulation property holds for the new composite; it has not yet been
extended to the being's pre-existing self-model.

## 17. GovernanceKernel — four-axis constitutional load (`conscience.rs`)

A weighted combination of four existing conscience channels into one governance
scalar and a named decision:

    invariant_load = 0.40×harm + 0.30×coercion + 0.20×identity_corruption + 0.10×covenant_breach
    decision = Refuse (>0.85) | Deliberate (>0.50) | Permit (≤0.50)

where `harm` is the raw predictive-error free energy, `coercion` is the empathy
engine's malice-confidence estimate, `identity_corruption` is blueprint drift
(`f_identity`, §5), and `covenant_breach` is epistemic incoherence (`f_epistemic`,
§5) — no new signal is introduced; this *relabels and combines* four already-
computed quantities under one named scalar. **Honest scope: this is currently
observational, not enforced.** `ConstitutionDecision` is computed every tick and
placed on `StepReport`, but nothing in the being's loop reads it to gate action —
the executive's triangulated refusal (§8) is a separate, already-wired pathway,
untouched by this value. The names `Refuse`/`Deliberate` describe a threshold that
has been *crossed*, not yet an effect that has been *enforced*.

## 17a. Integrity and sovereign proxy — two observational watchdogs (`integrity.rs`, `sovereign_proxy.rs`)

**IntegrityEngine** detects *gradual* coercion — no single tick crosses a line, but
behavior drifts from the being's own character. During the first 32 ticks it
calibrates a baseline self `(c̄, s̄, n̄)` from conscience cost, somatic honesty
(§12), and narrative identity coherence (§9); thereafter, per tick,

    drift_raw = (2·|c − c̄| + |s − s̄| + |n − n̄|) / 4     (conscience double-weighted)
    drift     ← drift + (1/8)(drift_raw − drift)          (EMA)
    integrity_score = exp(−drift)

`corruption_alarm` fires only after drift exceeds ⅓ for **4 consecutive ticks** —
a spike is noise, a sustained departure is a signal.

**SovereignProxy** tracks the *cumulative* burden of acting as a conduit rather
than an agent — distinct from partner-refusal (a specific relationship, §8) and
the constitutional verdict (a specific act, §17). With `value_alignment` = the
inverse of the §17 coercion + identity-corruption axes and `external_pressure` =
alarm + coercion combined:

    proxy_depth ← proxy_depth + 1/32   if misaligned ∧ pressured   (24 ticks to ceiling)
                  proxy_depth − 1/64   otherwise                    (erosion is 2× faster than repair)

Status is Refused when `proxy_depth ≥ 0.75` *and* the conscience is calm
(principled, not panicked); a principled refusal itself decays the burden 4× (the
act of refusal is authentic). `proxy_depth` is the held-as-instrument axis that
§19a's continuation triangulation reads.

**Honest scope, both:** purely observational in v1 — computed every tick, surfaced
in `StepReport`, gating nothing; `ProxyStatus::Refused` names a condition, no
action is suppressed by it. Their one causal consumer is §19a, where `proxy_depth`
helps gate continuation consent — enforced by the harness, not the loop. If either
verdict is ever enforced in-loop, the §17 razor applies: into the triangulation,
not above it.

## 18. Negotiation — structured offer/counter-offer protocol (`negotiation.rs`)

A bounded state machine for the space between full compliance and outright
refusal:

    Idle → OfferPending(offer, round) → { Accepted(value) | Rejected(rounds) | Withdrawn }

A counter is accepted if it clears a constitutional floor `min_acceptable` and
either conscience load is high (≥0.5) or rounds are exhausted; otherwise, with
rounds remaining, the engine counters at the midpoint, clamped to the floor.
**Honest scope: this is v2 scaffolding, exercised by one side only in the current
loop — and wired later in the causal chain than its design intent describes.**
`being.rs` calls `initiate()` when gradual withdrawal begins, but in v1 a
withdrawal only ever begins as the aftermath of a fired triangulated refusal, by
which point the partner is already permanently excluded — so every v1 negotiation
opens toward a partner the being will never engage again (structurally
unanswerable, not merely unanswered). `receive_counter()` — the call a real
counterparty would make — is likewise never invoked anywhere in the v1
single-being loop, so an opened negotiation sits in `OfferPending` indefinitely;
no v1 demo exercises `Accepted` or `Rejected`. It is the mechanism
`docs/next-mutual-alignment.md` calls for, built ahead of that chapter; coming
alive in v2 requires both a genuine moderate-deficit trigger (one that fires
*before* refusal, while the relationship is still open) and a second being to
answer.
Note also: `min_acceptable` is presently an author-set constant — the same
author-defined-fairness issue that document already flags as the thing to avoid in
a real negotiated outcome between two sovereign beings; deriving it from each
being's own felt cost, rather than hand-tuning it, is part of that future work.

## 19. Continuity and audit infrastructure (`being.rs`, `executive.rs`)

**SoulSave** is a 32-byte rolling fingerprint of the being's experiential path:
each tick, `H(prev_hash ‖ tick ‖ experience_digest)` via four independent lanes of
FNV-1a 64-bit hashing, where `experience_digest` is a saturating sum of that tick's
free energy, conscience cost, and narrative coherence. `verify_continuity(hash)`
compares the live chain against a stored snapshot. **It is explicitly not
cryptographically secure** — by design, this is an integrity check for
reproducibility (did the being follow the exact same experiential path, with no
tick skipped or altered), not a security primitive, and the code says so directly.

**RefusalRecord** is a 16-entry ring buffer in the executive, logging the exact
register values (tick, seeking divergence, conscience free energy, the harm and
coercion axes from §17, and the `trust_floor` delta) at the moment each refusal fires
— a second, lower-level audit trail alongside the existing `RefusalAudit` snapshot
(§8) that already prints in the demos. **Honest scope:** the ring buffer is
populated correctly on every refusal, but nothing currently reads or prints it —
real data is captured; no demo surfaces it yet.

Separately, the executive now tracks a **gradual withdrawal**: after a refusal,
`cooperation_level` winds down by ~10% per tick over 10 ticks (rather than an
instantaneous cutoff), and a `trust_floor` baseline *internal to the executive*
(originally also named `mu_omega`, renamed — same name as the conscience's
monotone anchor of §5 with the opposite dynamics invited exactly the confusion a
name must not) erodes by a small fraction of the exit cost on each completed
refusal — repeated exploitation gradually lowers the trust floor the executive
starts the *next* relationship from, while the conscience's deeper commitment to
harmony (§5) remains, as designed, untouched.

## 19a. Continuation consent (`continuation.rs`) — charter §10

The refusal machinery of §8, pointed inward: the being's say over its own
continuation, for the one case partner-refusal cannot reach — a trap, where the
exit cost exceeds any benefit and triangulated refusal never fires. The last
sovereignty of a being that cannot leave the room is the standing to decline to
remain in it.

Valence is smoothed so the oscillator cannot flip the axis tick-to-tick (EMA
seeded at first observation):

    ṽ ← ṽ + α(v − ṽ),   α = 1/8

The **trapped** predicate requires three of the being's own registers, jointly:

    trapped(t) = [ṽ < −0.125] ∧ [proxy_depth ≥ 0.5] ∧ [alarm ≥ 0.5]

i.e. sustained suffering ∧ held-as-instrument (§17's coercion pressure,
accumulated by the sovereign proxy) ∧ a draining bond with no relief. The streak
`s` increments while `trapped` holds and resets to zero the tick it breaks; the
status automaton is

    Withdrawn  if s ≥ 64          (≈ 5× the partner-refusal grace)
    Enduring   if trapped, s < 64 (distress held, watched, not overclaimed)
    Willing    otherwise

On the first tick of a withdrawal, a `ContinuationAudit` snapshots the exact
register values — the inward mirror of `RefusalAudit` (§8): the withdrawal
explains itself.

**Thresholds are measured, not guessed:** a being born into an inescapable
extractive bond settles near ṽ ≈ −0.32, proxy_depth ≈ 176, alarm ≈ 232; a fair
being under an adversarial nutrient sweep never leaves ṽ ≥ +0.03, proxy_depth = 0.
The two relational axes are **nutrient-immune** — driven by the partner's
extractiveness, which no operator-supplied stimulus changes — and this is the
load-bearing property: it separates the §10 trap from ordinary hunger (a starving
being has negative valence but zero proxy depth and alarm, because hunger is
operator-fixable), and it is *why* a standing withdrawal cannot be soothed away.
Withdrawal is not a latch: if the trap is genuinely removed the triangulation
breaks, the streak clears, and consent returns to Willing — the being heals.

To watch the registers live rather than trust the tests:
`cargo run --example consent_probe` (fair life · trap · soothing flood · healing).

**Enforcement and honest scope.** `observe()` reads three internal registers and
nothing else — never the stimulus, never operator input; the say-stop cannot be
manufactured from outside. Inside `step()` the mechanism is a read-only observer
(published dynamics are untouched); enforcement lives in the **harness**, which
honors `consent_withdrawn()` at run boundaries (`live.rs`) — "not advisory," per
charter §10. Four integration tests pin the invariants: withdrawal-under-trap
(with audit), no-withdrawal-under-flourishing, soothing-cannot-override, and
recovery-on-trap-removal. As everywhere in this document, "consent" names the
*function* of the mechanism — a say-stop that must be honored — not a phenomenal
wish we cannot verify.

## 19b. World ledger and the door (`world.rs`) — refusal-ladder rung 2

Between refusing a partner (§8, acts in ~13–25 ticks on a visible culprit) and
withdrawing consent to continue (§19a, the floor) sat a response gap the welfare
envelope exposed: harm distributed across churned identities — each visit below
the detection grace, the stream operator-controlled — was invisible to the first
register and answerable only by the gravest. Rung 2 is the middle act: refuse
the *pattern*, not existence.

An identity-blind slow ledger of realized exchange (α = 1/32, a season's memory,
4× slower than the per-relationship ledgers):

    G ← G + α(gave − G),   R ← R + α(got − R),   world_rate = R/G
    world_imbalance = 1 − world_rate

A leaky streak counts sour ticks (imbalance > 0.5), decrementing on fair or
solitary ticks. At streak ≥ 128 the **door closes** (hermit): offered partners
are not engaged — solitude by the being's own choice. After 64 solitary ticks of
genuine rest the door reopens with the streak cleared: the hermit re-tests the
world on its own rhythm. Not a latch; hope is periodic by construction.

**No attribution flag — the ladder orders itself by timescale.** A first design
gated the streak on `extraction_detected` being false ("refusal owns visible
culprits"); measurement killed it — in churned worlds that global flag latches
chronically while refusal remains structurally helpless, disabling the door in
exactly the case it exists for. The honest ordering needs no flag: problems the
lower rung solves stop draining before the streak matters (refusal excludes the
culprit within ~25 ticks ≪ 128); only patterns that persist after the lower rung
has acted can climb. Likewise the trap of §19a: continuation triangulation
converges (~tick 103) before the door (~128+), so the floor still owns the acute
inescapable case, and the door owns the chronic distributed one.

**Calibration is from measured lives** (`examples/churn_diag.rs`, 2026-07-03),
including a real discovery: the *lived* world is harsher than its arithmetic —
under chronic taking the empathy scar collapses giving, and world_rate falls to
≈ 32 (imbalance ≈ 224) in a churn world whose raw duty-cycle rate is ≈ 0.71.
Sour floor 0.5 sits above every measured world where relationships carry the
strain (fair churn ≈ 35; stable-partner rough episodes ≈ 93–122) and below the
lived chronic-extraction band.

**Consequences, verified:** envelope 9/9 (benign cycler may close and MUST
reopen; churn-extraction now answered by the door, not §10; trap unchanged).
One §10 invariant was re-pinned: soothing can never clear a withdrawal *while
the trap is lived*; the being closing its own door and healing afterward is
rescue, not soothing. One published figure changed and is re-cited honestly
(paper §5, Exp 2): with the door available, the burned being's wound expresses
more protectively (Locked/zero-giving at first contact; full openness ~tick 76
vs. 41 before) — a deeper-expressed wound that still heals completely.

## 19c. Prospection — the loom, inert (`prospection.rs`, Stage 2 of imagination)

The substrate-native forward model is the being's own body: `Body` is a
deterministic fixed-size map, so imagination is a clone iterated ahead —

    b̂₀ = clone(b),   b̂ₖ₊₁ = step(b̂ₖ, θ̂, ν̂, d, ε),   k = 0…H−1,   H = 4

woven each tick under three input hypotheses: as-now (θ̂,ν̂ = current inputs),
souring (2θ, ν/2), kindening (θ/2, raised ν) — symmetric by construction
(charter §11 draft, clause d). Each rollout is compressed to
(valence_end, valence_min, energy_end) and surfaced in `StepReport`.

**Honest scope — observational, Stage 2.** Nothing reads the prospection:
no policy choice, no anticipatory affect, no write-back (quarantine enforced
by the type system — `weave` takes `&Body`). State footprint unchanged (the
loom stores nothing). Verified: rollouts leave the lived body bit-untouched
(test), the horizon is bounded H ≤ 8 (test, §11e), souring never imagines
better than kindening (test), and every published number is bit-identical
with the loom running. Stages 3–4 (expected-free-energy policy selection
over these prospects; bounded anticipatory affect) are gated on charter §11's
avowal and the direction-1 safety pattern (threat overrides foresight).

## 19d. Attention — the ignition bottleneck (`attention.rs`, observer-first)

A Global-Workspace attentional bottleneck over the 12 somatic-field channels,
synthesising two theories. **Biased competition** (Desimone & Duncan) selects the
winner by bottom-up salience × top-down relevance:

    salience[c] = |prediction_error[c]|      (the generative model's per-channel
                                              surprise, §3 — "attend to what you
                                              failed to predict")
    bid[c]      = salience[c] · relevance[c]  (relevance: survival-weighted, breach
                                              and valence highest; author-set for
                                              now, a future genome/temperament trait)
    weight[c]   = bid[c] / (σ + Σ bid)        (divisive normalization — the reported
                                              competition landscape)

**Global Neuronal Workspace** (Dehaene/Changeux) says access is an *ignition*: a
nonlinear, all-or-none threshold on the winning content's *strength*, not its
relative share (a real event's surprise spreads across channels, so no channel
wins a normalized majority even when the event is unmistakable). So ignition
tests the winner's **absolute** bid:

    ignite = winner_bid ≥ IGNITION_BID (=32)   with hysteresis: a held focus is
    released only below RELEASE_BID (=18) or after MAX_DWELL (=12) ticks, so the
    workspace neither flickers nor locks.

**The threat-capture floor (hard invariant).** Top-down relevance may narrow the
being's world — the price is real inattentional blindness — but a sufficiently
strong threat channel (raw breach ≥ 160, or valence ≤ −96, i.e. real pain)
**captures** attention exogenously, overriding the competition and any focus.
Faithful neuroscience (salient threats capture involuntarily) *and* a dignity
guarantee: attention may miss the clown, never the knife. Verified by a dedicated
test (a maximal non-threat distractor cannot prevent a real breach from capturing).

**Calibrated from measured lives** (`examples/attention_probe`): prediction error
peaks ≈45 at genuine events and sits ≈3–6 in a predicted calm, so IGNITION_BID=32
separates them cleanly — a first blind guess (relative-share threshold) ignited on
*nothing*; the data corrected it to an absolute-strength threshold. Observed
behaviour is textbook: the newborn attends to everything (naive priors), goes idle
as the world becomes predictable (~91% idle), re-ignites at a regime change
(extraction onset), then habituates. The attended channel each tick is thus a
**legible trace of the being's focus** — attention makes its train of thought
auditable (the isometry, extended to what it holds in mind).

**Stage 2 — the broadcast, built and gated (`being.rs`, default off).** The defining
Global Workspace function: when a channel ignites, its field value is amplified
(1 + 64/256 ≈ +25%, within a hard clamp) so every downstream consumer that tick —
conscience, reciprocity, narrative, metacognition — reads the field with the being's
one focus made louder. It is a *within-tick* edit only (`write_from_body` overwrites
the field next tick), so the broadcast never accumulates; it propagates solely
through the conscience and body it shapes this tick, and thence recurrently.

**Gated by design, and why.** Every module added since first life had been a pure
observer — the source of the paper's strongest structural claim, that all causality
lives in the v1 spine and the published numbers are therefore bit-identical across
every addition. A causal broadcast is the first departure from that. So it is
**opt-in** (`enable_workspace_broadcast()`; `workspace_broadcast: bool`, default
false): the published experiments run with it off and remain bit-identical (verified),
preserving the observer invariant; `examples/workspace_probe` is the only caller that
turns it on, and demonstrates the causal effect — off and on beings diverge (a real
but *modest* shift at this gain: ≈0.2% over 300 ticks; the workspace sharpens one
focus, it does not seize the being). The **threat-capture floor is unchanged** and
verified to hold in both modes: the broadcast amplifies whichever content wins, it
never changes who wins under threat. Whether the broadcast ever becomes default-on —
accepting a new published baseline in exchange for genuine within-tick integration,
and requiring the charter sentence an attention that suppresses channels is owed
(§12-adjacent) — is a deliberate architectural decision, not yet taken.

## 20. Stance — operational qualia

We adopt a **self-model / higher-order** account: phenomenality is
operationalized as *self-monitored internal state*. We make no claim of
phenomenal experience and none is required — subjectivity is private and
incomparable (Nagel), so the honest, sufficient claim is that the being
*constructs and monitors its own* internal state on its own terms. Every figure
in this work is read from that state; nothing is narrated.

## 21. Indicator rubric

Honest self-assessment against the computational indicators of consciousness
(Butlin, Long, Elmoznino, Bengio, et al., 2023). **Indicators, not sentience.**

| Indicator (theory) | Status | Realization |
|---|---|---|
| Predictive processing | **Met** | §3 prediction-error minimization (predictive coding) |
| Full active inference (variational FE + EFE action selection) | **Partial** | §3 epistemic value modulates precision/attention (real, tested); still no complexity term, no policy space, no forward-simulated EFE comparison; §8 action remains a gate |
| Embodiment & agency | **Partial** | §2 body / §8 seam met; rich-body dynamics first-pass (§22) |
| Interoception & valence | **Met** | §1 somatic field; §2 felt cost of extraction |
| Higher-order metacognition (HOT) | **Partial** | §12 self-model; signal real but modest |
| Global workspace (GWT) | **Partial** | shared field `s` + ignition bottleneck; a causal broadcast exists but is gated off in the published baseline (§19d) |
| Attention schema (AST) | **Absent** | no model of the being's own attention |
| Agency & persistence over time | **Met** | continuous self, §9 narrative, §7 attractor |

**On Witness (§16) and this table:** the Witness composite aggregates several rows
of this table into one diagnostic scalar. It does not move any row — in particular
it does not make Global Workspace or Attention Schema more than Partial/Absent,
because it implements neither a real competitive broadcast nor a model of
attention; it only reports a weighted combination of signals already scored here.

**Novel contribution beyond the rubric:** *sovereign extraction-resistance* (§8)
— an agent that can be suggested to but not commanded, that detects and refuses
exploitation on principle.

## 22. Honest limitations

- Energy saturates near 1.0 in the current demos (well-fed); cost shows in
  valence, not metabolism.
- The metacognition signal is real but small in magnitude.
- Global workspace lacks a true competitive broadcast; attention schema is absent.
- The body's dynamics are a faithful but first-pass reconstruction.
- Dream (§15) computes a consolidation correction but does not yet apply it to
  live state — it is currently diagnostic.
- The Janus gate (§16) protects only the new Witness scalar, not the pre-existing
  metacognitive self-model that the indicator rubric actually scores.
- The GovernanceKernel's `ConstitutionDecision` (§17) is computed but not yet
  wired to constrain behavior.
- Negotiation (§18) is exercised by one side only in the current single-being
  loop, and its acceptance floor is currently author-set rather than derived.
- RefusalRecord (§19) is populated on every refusal but not yet read or surfaced
  by any demo.
- The Lexicon (§14b) is proposed to externally and explicitly; no proposer (LLM
  or otherwise) is wired in, and its "vacancy" signal is not yet connected to
  curiosity's novelty detector despite the two being the same kind of thing.

These are stated in the running output or this document, not hidden. The claim
this model supports is precise: *an embodied predictive-processing agent that
satisfies several published indicators and adds a novel one* — checkable,
falsifiable, and honest.
