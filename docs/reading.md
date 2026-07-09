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

---

*Access note: some sources 403 through the agent proxy (arXiv abs pages, Medium).
Fetch PDFs/DOIs directly or via an institutional mirror when citing verbatim.*
