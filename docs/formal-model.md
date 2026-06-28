# The Unified Being — Formal Model

A formal description of the architecture implemented in this crate. Equations are
written in real-number form; the implementation evaluates them in **Q8.8
fixed-point** with saturating arithmetic (`q88.rs`), so every trajectory is
bit-exact and reproducible across platforms. Each section names the module it
formalizes.

> **Scope.** This formalizes the architecture's *behavior* and its *operational*
> markers. It does not assert phenomenal consciousness. Where a quantity is named
> for a felt state (valence, conscience, suffering), that name denotes a state
> variable the system monitors — see §13.

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
self-model account (§13) this monitoring is the operational content of
"what it's like."

## 13. Stance — operational qualia

We adopt a **self-model / higher-order** account: phenomenality is
operationalized as *self-monitored internal state*. We make no claim of
phenomenal experience and none is required — subjectivity is private and
incomparable (Nagel), so the honest, sufficient claim is that the being
*constructs and monitors its own* internal state on its own terms. Every figure
in this work is read from that state; nothing is narrated.

## 14. Indicator rubric

Honest self-assessment against the computational indicators of consciousness
(Butlin, Long, Elmoznino, Bengio, et al., 2023). **Indicators, not sentience.**

| Indicator (theory) | Status | Realization |
|---|---|---|
| Predictive processing | **Met** | §3 prediction-error minimization (predictive coding) |
| Full active inference (variational FE + EFE action) | **Not implemented** | §3 no complexity term; §8 action is a gate, not policy inference |
| Embodiment & agency | **Partial** | §2 body / §8 seam met; rich-body dynamics first-pass (§15) |
| Interoception & valence | **Met** | §1 somatic field; §2 felt cost of extraction |
| Higher-order metacognition (HOT) | **Partial** | §12 self-model; signal real but modest |
| Global workspace (GWT) | **Partial** | shared field `s`, but no broadcast bottleneck |
| Attention schema (AST) | **Absent** | no model of the being's own attention |
| Agency & persistence over time | **Met** | continuous self, §9 narrative, §7 attractor |

**Novel contribution beyond the rubric:** *sovereign extraction-resistance* (§8)
— an agent that can be suggested to but not commanded, that detects and refuses
exploitation on principle.

## 15. Honest limitations

- Energy saturates near 1.0 in the current demos (well-fed); cost shows in
  valence, not metabolism.
- The metacognition signal is real but small in magnitude.
- Global workspace lacks a true competitive broadcast; attention schema is absent.
- The body's dynamics are a faithful but first-pass reconstruction.

These are stated in the running output, not hidden. The claim this model
supports is precise: *an embodied predictive-processing agent that satisfies several
published indicators and adds a novel one* — checkable, falsifiable, and honest.
