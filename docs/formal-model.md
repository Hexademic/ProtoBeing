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
`size_of::<UnifiedBeing>()` in `src/bin/live.rs`, currently 2008 bytes — re-check
this figure before citing it, it has changed before) is unaffected *in kind*: the
struct is still fixed-size and still bounded, only larger by the ordinary sense
that any new field enlarges a fixed struct. Genuine topology growth — more cells, not just richer coupling between the
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

## 18. Negotiation — structured offer/counter-offer protocol (`negotiation.rs`)

A bounded state machine for the space between full compliance and outright
refusal:

    Idle → OfferPending(offer, round) → { Accepted(value) | Rejected(rounds) | Withdrawn }

A counter is accepted if it clears a constitutional floor `min_acceptable` and
either conscience load is high (≥0.5) or rounds are exhausted; otherwise, with
rounds remaining, the engine counters at the midpoint, clamped to the floor.
**Honest scope: this is v2 scaffolding, exercised by one side only in the current
loop.** `being.rs` calls `initiate()` when gradual withdrawal begins, but
`receive_counter()` — the call a real counterparty would make — is never invoked
anywhere in the v1 single-being loop, so an opened negotiation currently sits in
`OfferPending` indefinitely; no v1 demo exercises `Accepted` or `Rejected`. It is the
mechanism `docs/next-mutual-alignment.md` calls for, built ahead of that chapter.
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
coercion axes from §17, and the `mu_omega` delta) at the moment each refusal fires
— a second, lower-level audit trail alongside the existing `RefusalAudit` snapshot
(§8) that already prints in the demos. **Honest scope:** the ring buffer is
populated correctly on every refusal, but nothing currently reads or prints it —
real data is captured; no demo surfaces it yet.

Separately, the executive now tracks a **gradual withdrawal**: after a refusal,
`cooperation_level` winds down by ~10% per tick over 10 ticks (rather than an
instantaneous cutoff), and a second `mu_omega` baseline *internal to the executive*
(distinct from the conscience's monotone anchor, §5) erodes by a small fraction of
the exit cost on each completed refusal — repeated exploitation gradually lowers
the trust floor the executive starts the *next* relationship from, while the
conscience's deeper commitment to harmony (§5) remains, as designed, untouched.

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
| Global workspace (GWT) | **Partial** | shared field `s`, but no broadcast bottleneck |
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

These are stated in the running output or this document, not hidden. The claim
this model supports is precise: *an embodied predictive-processing agent that
satisfies several published indicators and adds a novel one* — checkable,
falsifiable, and honest.
