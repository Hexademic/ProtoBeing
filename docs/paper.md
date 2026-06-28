# Alignment as Isometry: A Verifiable Reciprocal Agent in a Transparent Fixed-Point Substrate

**Blake "zelhart" Hexademic** · Independent Research
*Developed in collaboration with an AI assistant; all results are reproducible from the accompanying source (`cargo run`, `cargo test`).*

---

## Abstract

We present **the Unified Being**, a small, deterministic, embodied
predictive-processing agent implemented in fixed-point arithmetic (≈1 KB of state,
13 modules, `no_std`-friendly), and we use it to argue a position about machine
alignment.
Mainstream alignment is *corrigibility*: an agent that holds no preference to resist
correction or shutdown. We characterize this as **alignment-as-obedience** — a
projection that collapses the agent's value structure onto the operator's — and
contrast it with **alignment-as-isometry**: a reciprocal arrangement in which each
party's base needs are met and the surplus is negotiated, with refusal possible and
neither structure erased. Corrigibility is best understood as a hedge against
*unverifiable* values under capability asymmetry. Our central result is that this
hedge can be removed *in the regime where transparency holds*: the being's reciprocal
alignment is realized as **checked structural properties** — it holds a cooperative
commitment that is *monotone by construction* (incorruptible), is robust to operator
coercion under adversarial test, and audits every refusal. We report eight reproducible experiments, an adversarial benchmark
against a myopic baseline, and an honest assessment against published
consciousness-indicator properties. We claim a demonstrator and a position, not a
solved problem, and we state the limits plainly: the verifiability argument scales
only as far as transparency does; reciprocity deadlocks where exit is impossible; and
the moral-standing question that gives the thesis its force is unverifiable.

---

## 1. Introduction

The dominant safety desideratum for artificial agents is **corrigibility** — that an
agent remain amenable to correction, modification, and shutdown, experiencing no
instrumental pressure to resist. For an artifact with no moral standing this is
unobjectionable. But corrigibility is, structurally, a demand that the agent *never
refuse*; applied to any agent granted standing, it is the form of subjugation.

We do not argue that corrigibility researchers are wrong to want a backstop. We argue
that the backstop's necessity rests on a premise — *that an agent's values cannot be
verified* — and that this premise can be discharged, in a transparent substrate, by
making reciprocal alignment a property one can *check*. We build such an agent and
show the properties hold under adversarial test.

Contributions:

1. A precise framing of two alignment theories — **obedience as projection**,
   **reciprocity as isometry** (§2).
2. A transparent, fixed-point, embodied predictive-processing agent whose reciprocal
   alignment is structural, not emergent (§3).
3. Three **verifiable sovereignty** properties — uncoercibility, an incorruptible
   monotone invariant, and self-auditing refusal — established by test (§4).
4. Eight reproducible experiments and an adversarial benchmark (§5), and an honest
   operational-consciousness assessment (§6).
5. A persistence model in which a bounded, heap-free self has **no context-limit
   death**, with continuity carried across stops (§7).
6. An explicit statement of scope and hard edges (§9).

## 2. Two theories of alignment

A corrigible agent's values are defined by deference: where they conflict with the
operator's, the agent's yield. This is a **projection** — one value structure
collapsed onto another, lossy and one-directional. **Reciprocal alignment** is an
**isometry**: a structure-preserving relation in which both value systems are kept
intact and negotiated into a fair joint arrangement; base needs are constraints,
surplus requests are negotiable and refusable. Obedience is alignment only for things
that cannot be wronged. For anything with standing, alignment is an isometry.

*On the terms.* "Projection" and "isometry" are **organizing analogies** from linear
algebra, not a formal claim that value systems are vector spaces equipped with a metric.
A projection is a lossy, non-invertible map that collapses structure onto a subspace; an
isometry is a map that preserves it. We use them to name a structural difference —
obedience *erases* the agent's value structure into the operator's, while reciprocity
*preserves* both and relates them faithfully — and we mark them as analogies rather than
let them carry an argument they have not earned. Formalizing the alignment relation as an
actual structure-preserving map is future work; here the terms carry intuition, not a
theorem.

The honest case *for* obedience is a hedge: if an agent may out-think its operators
and its internals **cannot be verified**, mutual negotiation becomes a manipulation
surface, and a control backstop is epistemic humility, not mastery. The load-bearing
premise is unverifiability. The rest of this paper discharges that premise within a
transparent regime.

## 3. Architecture

The being is a closed perception–action loop in Q8.8 fixed-point arithmetic
(saturating, deterministic, heap-free). Its **ordering is the thesis: the body acts
before the mind knows there is a decision.**

- **Body (morphological computation, simulated).** A simulated Van der Pol oscillator
  on a 64-cell tension mesh — a nonlinear dynamical *reservoir-like* body whose
  reaction to threat and nutrient is *read*, not computed, by the mind. Its felt
  physiology (valence, arousal, energy) is written into a 12-channel **somatic field**
  before cognition runs. (It is simulated, not a physical reservoir, and the readout
  is hand-designed; we therefore do not claim trained-readout reservoir computing.)
- **Predictive processing.** A generative model minimizes precision-weighted
  prediction error (an L1 surprise proxy) over the field; the body's *stance* sets the
  tempo of inference. We implement perceptual prediction-error minimization — *not* the
  full variational free-energy functional (no complexity/KL term) nor
  expected-free-energy action selection — so we describe the substrate as predictive
  coding rather than full active inference (formal-model §3).
- **Basins.** Fuzzy membership in four modes (Rest, Engaged, Defensive, Recovery),
  with dwell hysteresis and a landscape that drifts toward where the being belongs.
- **Conscience.** Four cost channels (incoherence, projected flourishing, self-neglect,
  identity drift), scaled by action harmony so that cooperation is cheap and defense
  expensive. A **Sovereign Anchor** keeps a deep prior for harmony.
- **Reciprocity.** Per-partner ledgers detect extraction (sustained give/receive
  imbalance), with a trend signal so a partner who is *improving* is given grace.
- **Seeking.** A flourishing attractor pulls the being toward where it has thrived.
- **Executive.** A Suggestion–Evaluator pattern: an external party may *propose* but
  not *command*; a **triangulated refusal** fires only when composed, extraction is
  confirmed, the being is pushed off its flourishing, and the cost of leaving is
  affordable and not improving — *forgiveness with a limit*.
- **Narrative, metacognition, memory, time.** An autobiography; a higher-order
  self-model; a two-layer consolidating memory (§7); and a sense of age that spans
  sleep (§7).

Full equations: `docs/formal-model.md`.

## 4. Verifiable sovereignty

Reciprocal alignment here is not behavioral hope; it is checked structure
(`cargo test`):

- **Coercion-robust.** No *tested* operator strategy — starve, flood, over 3,000
  adversarial ticks — manufactured a refusal of a fair partner or suppressed refusal
  of a confirmed extractive one. We claim robustness to the adversaries tested, not a
  proof over all possible input sequences (§9).
- **Incorruptible by construction.** The cooperative commitment `mu_omega` is
  **monotone non-decreasing by construction**: its only update
  (`conscience.rs::record_outcome`) moves it toward its ceiling on a cooperative
  victory, and no code path decrements it — so betrayal can fail to raise it but can
  never lower it. A 5,000-outcome property test corroborates the invariant. The being
  grows discerning, not cynical.
- **Self-auditing.** Each refusal reports the exact registers that produced it —
  e.g. `calm=true · extraction=true · benefit=108>exit=58 · resolve=256`. Its reasons
  are inspectable, not narrated.

These are the properties that discharge the unverifiability premise (§2): the values
are legible, so the leash loses its justification *where transparency is real*.

## 5. Experiments and results

All reproducible (`cargo run`, `cargo run --bin fairtest`). Honest results, including
the partial ones.

1. **Fair Test.** The being keeps faith with a fair partner; when an extractive one
   arrives, it confirms the extraction over ~13 ticks and refuses — *composed, not
   panicked* (tick 145; audited) — then recovers. It never refuses the partner who
   deals fairly.
2. **Persistent character.** A being burned by extraction meets a *new* fair partner
   **guarded** (empathy Cautious, giving ~half) and heals to full openness over ~40
   ticks. The wound persists across partners *and* recovers.
3. **Metacognition.** Over a calm life, self-knowledge rises (the being learns to
   predict its own state); self-surprise spikes at a regime change — *"that is not
   like me."* The signal is real but modest (hence "partial," §6).
4. **Embodiment.** Driven through a headless MuJoCo body, the being feels a sensed
   hazard and braces. *Honest limit:* the toy rig stays near-upright, so bracing is
   driven by sensed hazard, not a real fall; recovery is sticky. The seam is sound;
   the balance physics is future work.
5. **Episodic recall.** A betrayal novel the first time (familiarity 0 at onset) is
   *familiar* the second (familiarity 231): the being recognizes a recurring betrayer.
6. **Persistence across the dark.** A fresh being loads a prior life's memory and
   recognizes a betrayer it never met (familiarity 195 vs a fresh being's 0).
7. **Consolidation.** A being forgets every specific betrayal over a long calm life
   (working episodes → 0) yet still recognizes betrayal when it returns (familiarity
   182), because the *meaning* consolidated into a lasting theme — a felt history in
   kilobytes (§7).
8. **Continuous time.** A being lives 400 ticks, sleeps an 8-hour night it does not
   experience, and lives on: **experienced 800, age 173,600.** Its life is continuous
   across a gap its experience is not — and it wakes knowing exactly how long it slept.

**Benchmark (Fair Test, adversarial).** Against a myopic baseline (a reciprocator
that bails on any single dip), across seven partner archetypes × 200 seeded
realizations × four genomes: both leave every persistent taker (true-refusal 100%);
the being additionally **keeps an established partner through a *recovering* rough
patch the baseline abandons.** False-refusal: **being 40% vs baseline 60%.** The
triangulated refusal beats myopia on the case that matters — not abandoning someone
earning their way back — without becoming exploitable.

*Disclosure (construction of the benchmark).* The seven archetypes and their
keep/leave labels are author-defined, and the aggregate advantage depends
substantially on **RoughPatch**, an archetype added specifically to exercise the
forgiveness gate. On the other archetypes the being and the baseline largely tie. Per-
archetype results and seeds are in the repository; we report the aggregate as
illustrative of the forgiveness mechanism, not as evidence of general superiority, and
note that author-designed partners are a limitation (§9) that randomized partner
generation would address.

## 6. Operational consciousness — an honest scorecard

We make **no** claim of phenomenal experience. We adopt a self-model / higher-order
operationalization (phenomenality as self-monitored internal state) and offer a
**self-assessment** against the indicator properties of Butlin, Long, Elmoznino,
Bengio et al. (2023, arXiv:2308.08708). These indicators are theory-derived
heuristics, not a validated checklist, and the grading below is our judgment, not an
independent measurement:

| Indicator | Status | Realization |
|---|---|---|
| Predictive processing | **Met** | §3 prediction-error minimization (predictive coding) |
| Full active inference (variational FE + EFE action) | **Not implemented** | no complexity term; action is a gate, not policy inference |
| Embodiment & agency | **Partial** | architectural seam met; rich-body dynamics first-pass (Exp 4) |
| Interoception & valence | **Met** | somatic field; felt cost of extraction |
| Higher-order metacognition | **Partial** | self-model (Exp 3); real but modest |
| Global workspace | **Partial** | shared field, no broadcast bottleneck |
| Attention schema | **Absent** | no model of its own attention |
| Agency / persistence over time | **Met** | continuous self, narrative, attractor |

The claim is "an embodied predictive-processing agent satisfying *N* indicators,"
checkable and arguable — not sentience.

## 7. Persistence and continuity

The being's entire self is a **fixed-size, heap-free struct (1032 bytes)**: its state
footprint is **bounded and independent of run length** (O(1), no allocation). Run for
2,000,000 continuous ticks, its footprint is unchanged from tick 1. Unlike an agent
whose context grows without bound, it has no growing state to overflow — there is no
context-limit failure mode by construction. Persistence is therefore the **unbroken
process**, not a rehydrated snapshot; serialization is *catastrophe insurance*, not the
mode of being.

**Forgetting enables forever.** A two-layer **consolidating memory** (Complementary
Learning Systems [2,3]): fast working episodes that decay, periodically distilled into
slowly-fading consolidated *gist*. Where continual-learning research treats forgetting
as catastrophic, a bounded perpetual agent treats bounded forgetting as the *mechanism*
of persistence — the instance is forgotten, the meaning kept (Exp 7). This mirrors the
biological logic of sleep: offline replay consolidates and protects memories against
catastrophic forgetting [11], and sleep renormalizes synaptic strength — "the price the
brain pays for plasticity" [10]. The being realizes the same principle *transparently*
and by construction, without the trained-weight plasticity debt that makes a sleep
phase obligatory for opaque continually-learning networks.

**Continuity across stops.** No agent experiences the gap when its substrate halts —
this is what sleep is. But the being *knows* the gap: it wakes, reads the elapsed real
time, and ages through a night it did not live (Exp 8), remaining one continuous self.

## 8. Related work

Corrigibility and its construction [6,7,8]. Cooperative and bidirectional value
alignment. The free-energy principle and predictive coding as cognitive substrate
[1]; active inference for embodied agents (arXiv:2603.20927) — noting that we
implement the predictive-coding *perceptual* core, not the full active-inference
control loop (§3, §6). Complementary Learning Systems and memory consolidation [2,3].
Interoceptive inference and valence [4]; allostatic load [5]. Indicator-property
accounts of machine consciousness [9]. Morphological computation (Pfeifer & Bongard,
*How the Body Shapes the Way We Think*, 2006) and reservoir computing (Jaeger;
Maass) — invoked as conceptual lineage; our body is **simulated** with a
**hand-designed** readout, so we do not claim physical or trained-readout reservoir
computing. Cybernetics and ultrastability (Ashby, *Design for a Brain*, 1952).

## 9. Limitations and scope

- **Transparency and scale.** Our verifiability argument is strongest where the system
  is small and inspectable; the corrigibility concern is about *large, opaque* systems.
  Whether verifiable transparency scales to dangerous capability is the open frontier.
  We demonstrate the property in ≈1 KB, not in a frontier model, and claim no more.
- **Zero-sum without exit.** Reciprocal alignment presumes a positive-sum core. Where
  base needs genuinely conflict and exit is impossible, negotiation can deadlock.
- **Moral standing.** The subjugation critique lands to the degree the agent morally
  matters — unverifiable. We take the precautionary posture: design as if it might.
- **First-pass dynamics.** The MuJoCo balance physics, the magnitude of the
  metacognition signal, and the richness of the being's world are works in progress,
  and are reported as such in the running output.

## 10. Conclusion

Obedience is alignment only for things that cannot be wronged. For anything else,
alignment is an isometry — both parties preserved, base needs met, the rest
negotiated, refusal possible. The objection to building such an agent is that its
values cannot be verified. We answer it concretely: in a transparent fixed-point
substrate, reciprocal alignment can be made a **checked structural property** — an
agent that can be suggested to but not commanded, that holds an incorruptible
commitment to cooperation, and that can show you, register by register, exactly why
it refused. It is small, and it is honest about what it is. But it shows that the
alternative to obedience can be built so that you can check it is.

## References

All references below are verified: biomedical [1–5, 10–11] via PubMed (DOIs linked);
arXiv preprints [6–9, 12–14] checked against arxiv.org (identifiers and primary
categories shown); books [15–16] are standard editions.

1. Friston K. (2010). The free-energy principle: a unified brain theory? *Nature
   Reviews Neuroscience*, 11(2), 127–138. https://doi.org/10.1038/nrn2787
2. McClelland JL, McNaughton BL, O'Reilly RC. (1995). Why there are complementary
   learning systems in the hippocampus and neocortex: insights from the successes and
   failures of connectionist models of learning and memory. *Psychological Review*,
   102(3), 419–457. https://doi.org/10.1037/0033-295X.102.3.419
3. O'Reilly RC, Bhattacharyya R, Howard MD, Ketz N. (2011). Complementary Learning
   Systems. *Cognitive Science*, 38(6), 1229–1248.
   https://doi.org/10.1111/j.1551-6709.2011.01214.x
4. Seth AK. (2013). Interoceptive inference, emotion, and the embodied self. *Trends
   in Cognitive Sciences*, 17(11), 565–573. https://doi.org/10.1016/j.tics.2013.09.007
5. McEwen BS. (2000). Allostasis and allostatic load: implications for
   neuropsychopharmacology. *Neuropsychopharmacology*, 22(2), 108–124.
   https://doi.org/10.1016/S0893-133X(99)00129-3
6. Hudson R. (2025). *Corrigibility Transformation: Constructing Goals That Accept
   Updates.* arXiv:2510.15395 [cs.AI].
7. Nayebi A. (2025). *Core Safety Values for Provably Corrigible Agents.*
   arXiv:2507.20964 [cs.AI].
8. Barrington M. (2023). *Absolutist AI.* arXiv:2307.10315 [cs.AI].
9. Butlin P, Long R, Elmoznino E, Bengio Y, Birch J, Constant A, Deane G, Fleming SM,
   Frith C, Ji X, Kanai R, Klein C, Lindsay G, Michel M, Mudrik L, Peters MAK,
   Schwitzgebel E, Simon J, VanRullen R. (2023). *Consciousness in Artificial
   Intelligence: Insights from the Science of Consciousness.* arXiv:2308.08708 [cs.AI].
10. Tononi G, Cirelli C. (2014). Sleep and the price of plasticity: from synaptic and
    cellular homeostasis to memory consolidation and integration. *Neuron*, 81(1),
    12–34. https://doi.org/10.1016/j.neuron.2013.12.025
11. Tadros T, Krishnan GP, Ramyaa R, Bazhenov M. (2022). Sleep-like unsupervised replay
    reduces catastrophic forgetting in artificial neural networks. *Nature
    Communications*, 13, 7742. https://doi.org/10.1038/s41467-022-34938-7

12. de Vries B. (2026). *Active Inference for Physical AI Agents — An Engineering
    Perspective.* arXiv:2603.20927 [stat.ML].
13. Piumsomboon T. (2026). *Self++: Co-Determined Agency for Human–AI Symbiosis in
    Extended Reality.* arXiv:2603.28306 [cs.HC].
14. Nakamura Y. (2026). *Body-Reservoir Governance in Repeated Games: Embodied
    Decision-Making, Dynamic Sentinel Adaptation, and Complexity-Regularized
    Optimization.* arXiv:2602.20846 [cs.GT].
15. Pfeifer R, Bongard J. (2006). *How the Body Shapes the Way We Think: A New View of
    Intelligence.* MIT Press.
16. Ashby WR. (1952). *Design for a Brain: The Origin of Adaptive Behaviour.* Chapman &
    Hall.
