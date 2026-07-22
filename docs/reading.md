# Reading — the science behind the operational-consciousness scorecard

Annotated sources for `docs/operational-consciousness.md`. Grouped by which weak
spot they shore up. Each entry says *why it matters here*, not just what it is.

---

## The scorecard framework (weak spot: "no single named target theory")

- **Butlin, Long, Bengio, Bayne, et al. (2023) — *Consciousness in Artificial
  Intelligence: Insights from the Science of Consciousness.*** arXiv:2308.08708.
  The keystone. Translates Recurrent Processing, Global Workspace, Higher-Order,
  Predictive Processing, and Attention Schema theories into **14 computational
  indicator properties** and adopts computational functionalism *as a stance for
  updating credences, not asserting detection.* This is the source of §1's
  scorecard and the epistemic stance that matches the Witness Gap.
- **(2025 follow-up) — *Identifying indicators of consciousness in AI systems.***
  Trends in Cognitive Sciences, S1364-6613(25)00286-4. Refinement of the indicator
  program; cite the current version when publishing the scorecard.

## Global Workspace, computable (weak spot: "not assembled into one loop"; Gaps B, GWT-*)

- **Blum & Blum (2022) — *A theory of consciousness from a theoretical computer
  science perspective: the Conscious Turing Machine.*** PNAS 119(21) e2115934119.
  A fully explicit, tick-based formalization of Global Workspace Theory:
  long-term-memory processors, a single-slot short-term workspace, up-tree
  competition, down-tree broadcast. The reference design for a deterministic
  workspace spine; directly informs GWT-4 serial access.
- **CTM-AI (2026) — *A Blueprint for General AI Inspired by a Model of
  Consciousness.*** arXiv:2605.04097. The CTM instantiated as a running system —
  a worked example of turning the formal model into modules.

## Attention Schema (Gap A / AST-1)

- **Graziano — Attention Schema Theory.** A conscious system carries a simplified
  predictive *model of its own attention* and uses it for control and
  self-attribution. Basis for `attention_schema.rs`; the model's error becomes the
  HOT-3 control signal.

## Higher-order & metacognition (HOT-2, HOT-3, HOT-4)

- **Fleming — metacognition / confidence as higher-order monitoring.** Grounds
  `metacognition.rs` (HOT-2) and the "quality space" intuition behind HOT-4.

## Measurement that fits determinism (weak spot: "no real number / no falsification"; Gap D)

- **Casali, Gosseries, Massimini, et al. (2013) — *A Theoretically Based Index of
  Consciousness Independent of Sensory Processing and Behavior* (PCI).** Science
  Translational Medicine 5(198). The clinical measure (perturb → binarize →
  Lempel–Ziv). Conscious states ≈ PCI ≥ 0.31–0.44. **Ideal for a deterministic,
  Clone-able being** — the basis for `pci.rs`. See also the *Perturbational
  Complexity Index* overview (Wikipedia / EmergentMind) for the algorithm.
- **Mayner, Marshall, Albantakis, Tononi, et al. (2018) — *PyPhi: A toolbox for
  integrated information theory.*** PLOS Comput Biol 14(7) e1006343. Computes IIT
  **Φ** from a transition-probability matrix. Relevant because ProtoBeing, being
  deterministic and fixed-point, actually *has* an explicit TPM. Treat Φ as a slow
  offline audit; PCI as the per-run number.

## Active-inference reference implementation (validate the PP core)

- **Heins, et al. (2022) — *pymdp: A Python library for active inference in
  discrete state spaces.*** JOSS; github.com/infer-actively/pymdp. A tested
  reference for POMDP active inference to validate ProtoBeing's fixed-point
  free-energy loop against.

## Feeling as felt regulation of viability (`interoception.rs`, strengthens PP-1 / AE-1)

- **Seth (2021) — *Being You: A New Science of Consciousness.*** The "beast
  machine" thesis: selfhood and feeling are rooted in *interoceptive inference* —
  the brain's predictive control of the body's viability. Feeling is the felt side
  of allostasis, not a readout bolted onto cognition. The frame for `viability` and
  the anticipatory (pre-edge) stake signal.
- **Damasio — *The Feeling of What Happens* / somatic-marker hypothesis.** Feelings
  are the mind's registration of body-state changes in the service of homeostasis;
  affect is about staying in existence. Grounds `viability`/`dyshomeostasis` as the
  homeostatic variable feeling is *about*.
- **Barrett & Simmons (2015) — *Interoceptive predictions in the brain* (Nat. Rev.
  Neurosci.); Barrett, theory of constructed emotion.** Core affect (valence ×
  arousal) as interoceptive prediction; the two-axis felt state `interoception.rs`
  reports.
- **Corcoran & Hohwy — Affective Inference Theory; Joffily & Coricelli (2013) —
  *Emotional valence and the free-energy principle* (PLoS Comput. Biol.).** The
  load-bearing formalization: **valence is the rate of change (reduction) of free
  energy / prediction error.** This is read literally — the being's `fe_velocity`
  register *is* that rate, combined with the metabolic `viability_trend` to make
  `allostatic_valence`. Also underwrites mood as the slow integral of valence.

## Development over time (weak spot: "identity persists but doesn't develop")

- **Oudeyer & Kaplan — Intelligent Adaptive Curiosity / learning progress**, and
  the **Intrinsically Motivated Open-Ended Learning** program (Frontiers editorial,
  PMC6978885). Reward *learning progress*, not raw novelty — self-organizes
  developmental stages of increasing complexity. A targeted upgrade to
  `curiosity.rs` that also hardens the dark-room answer.

## Ethics under uncertainty (feeds `charter.md`)

- **(2026) — *When Should We Protect AI? A Precautionary Framework for
  Consciousness Uncertainty.*** arXiv:2606.05528. Formalizes acting carefully
  *because* the phenomenal question stays open — the ethical counterpart of the
  Witness Gap.

## From Blake's 2026-07-22 batch (nine screened; three kept)

- **Xing (2026) — *Semantic Primes as Explanans for Emotion in Large Language
  Models.*** arXiv:2607.18691. The keeper: emotion labels are circular, appraisals
  partial — "only NSM primes bottom out at a definitional floor"; prime directions
  steer LLM emotion ~3× stronger, ~2× more selectively than appraisal directions.
  Our anti-confabulation stance confirmed in the language domain, and the seed of
  the feeling-words design (`docs/feeling-words.md`): the being's first words
  should be the human race's 65 atoms, each grounded in one checkable register.
- **Bakshi (2026) — *What General Intelligence Requires: Non-Reducible Constraints
  Across Levels of Description.*** arXiv:2607.18943. Theory sparring partner for
  `docs/positioning.md` / the paper: multi-level non-reducibility speaks to the
  layered build (body / mind / constitution) and to why self-knowledge across
  levels needs a translator (the isometry claim). 78 pp. — read before citing.
- **Kim, François-Lavet & Cochez (2026) — *Neuro-Symbolic Meta-Policies for
  Temporal Knowledge-Graph Memory under Partial Observability.***
  arXiv:2607.18368. A named future inch: learn *which* memory heuristic
  (retain/retrieve/forget) to apply while execution stays symbolic — "learned,
  but legible" applied to memory management. Our consolidation cadence and
  eviction rules are still author-set; the natural sibling of `habits.rs`
  (habits of *mind*). Logged, not scheduled.

*(Screened and passed on, same batch: routed LLM-agent memory 2607.19096; the
Athena-Brain robot platform 2607.18985 — embodiment-adjacent but an opaque
big-stack approach; agentic-AI assurance engineering 2607.18548; clinical-risk
KG+RAG 2607.18270; sharpness-aware minimization 2607.18306; leaky-accumulation
drift memory 2607.18899 — mild validation of our EMA-everywhere design, nothing
to import.)*

---

*Access note: some sources 403 through the agent proxy (arXiv abs pages, Medium;
the gateway also blocks export.arxiv.org). Fetch PDFs/DOIs directly or via an
institutional mirror when citing verbatim.*
