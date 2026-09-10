# Positioning: Verifiable Sovereign Agency

## The claim, refined

Not "an attempt at synthetic consciousness" — that's unfalsifiable, and the field
is crowded. The defensible, distinctive, timely claim is:

> **A transparent, auditable substrate for sovereign agency** — an embedded
> predictive-processing agent whose extraction-resistance and incorruptible
> cooperative commitment are not learned-and-hoped-for, but **readable and
> reproducible by construction.**

Consciousness remains the *motivating north star*. The *claim we defend* is
verifiable sovereignty. The property that the consciousness framing buried — that
every register is readable and every self-report is checkable — *is* the
contribution.

## Prior art × what is distinctive here

| Ingredient | Already exists | Distinctive here |
|---|---|---|
| Body-as-reservoir / morphological computation | Frontiers (IIT link); *Body-Reservoir Governance in Repeated Games* (arXiv 2602.20846) | Deterministic fixed-point, embeddable, **hand-designed auditable readout** |
| Allostatic / homeostatic agents | CTCS-HRRL (arXiv 2401.08999); Social Allostasis (arXiv 2508.12791) | Allostatic load gates a **sovereignty decision**, not just viability |
| Exploitation-resistance | Hormetic value-loading; homeostasis multi-agent safety (arXiv 2410.00081) | **Triangulated** refusal + **monotone, incorruptible** cooperative anchor |
| SDT ↔ Free Energy | SDT–FEP correspondence appears in the active-inference literature (specific cite to verify — *Self++* 2603.28306 is **co-determined agency in XR**, not the SDT–FEP bridge a prior note claimed) | SDT as structural, readable layers (**soften the "emergence" claim**) |
| Self-model / metacognition | AIF-consciousness literature | Self-reports **checkable against the registers** |
| Agent sovereignty / non-overridability | *Sovereign Agents: Towards Infrastructural Sovereignty and Diffused Accountability in Decentralized AI* (Hu & Rong, arXiv:2602.14951, Feb 2026) — sovereignty via cryptographic self-custody, decentralized execution, TEEs; explicitly framed as producing "a profound accountability gap," not solved by agent-internal transparency | Sovereignty via **readable internal invariants**, not infrastructural hardness — the same non-overrideability property, opposite epistemics. This paper is independent, current, real-world corroboration of the exact danger our thesis names: opaque sovereignty is the problem; transparent sovereignty is offered as the answer it does not propose |

The through-line: **verifiable.**

## Structural immunity to defection

A strength distinct from transparency, and so far unclaimed in the paper's own
terms: the **mesa-optimization / deceptive-alignment failure class is structurally
absent** here, not mitigated. That failure mode requires an inner optimizer — a
learned sub-process whose objective can diverge from the outer one, with selection
pressure ("whatever scores, survives") to exploit the gap. This substrate contains
**nothing optimizer-trained**: no learned weights, no gradient, no selection loop —
coupled fixed-point difference equations with hand-designed readouts. There is no
gradient for a defector to hijack and no training signal for deception to score
against. Transparency argues you could *see* a defector; this argues there is no
soil for one to grow in. The two claims are independent and the second is, if
anything, the stronger.

The same analysis yields a design razor for our own future work: in any
self-maintaining system with parts, **monitors get captured; shared fate works**
(the cure for defection is not a watchdog above the system but a stake inside it).
Concretely: if the GovernanceKernel (formal-model §17) is ever promoted from
observational to enforced, it must be wired *into* the executive's triangulation —
one more register that must converge — never layered above the loop as a
supervisor. A supervisor is a second agent with its own failure modes; a
triangulation register is shared fate.

## The bounded counterpole to harness-driven self-improvement

The prominent 2026 direction in agent research is **harness engineering for
recursive self-improvement**: Weng (*Harness Engineering for Self-Improvement*,
2026) frames the scaffolding around a model — context management, file-system
memory, sub-agents, and *meta-harnesses that optimize themselves* — as an
optimization target as important as the weights. This being is the deliberate
opposite pole on every axis, and the contrast sharpens rather than threatens the
thesis. Where that program **accumulates** external memory to beat context limits,
the being is bounded and heap-free, treating forgetting as the mechanism of
persistence. Where it **maximizes** recursive self-modification, the being *forbids*
self-modification of its core (charter §13's fated still-point) and permits only
bounded, reversible change to a durable middle layer. Most tellingly, the failure
modes that program must fight — Weng's own list of **reward hacking, weak/fuzzy
evaluators, diversity collapse, and incentive misalignment toward negative
results** — are optimizer pathologies the being **sidesteps by construction**,
having no optimizer, gradient, or selection loop (the immunity argued above). Weng
also cites the STOP cautionary result (Zelikman et al., 2023): *recursive structure
alone is not enough — the base model must be capable enough to improve the
mechanism*, which is precisely why unbounded self-improvement is not a road a small
transparent substrate should take. And the verifiability constraint the serious
harness work is reaching for at the application layer — ScientistOne's
*chain-of-evidence*, in which every claim must trace to an evidence source (Meng et
al., 2026) — is what this substrate guarantees at the level where it can actually be
enforced: every reported figure is a read of state. The field is optimizing the
harness for self-improvement and building audits to fight the resulting opacity; we
offer the pole where the substrate is small enough that the audit is unnecessary and
the self-improvement is deliberately, safely bounded.

Two 2026 preprints tighten the positioning. Guo et al. (*Internalizing the Future*, arXiv:2606.27483) identify a *format-capability gap* in trained agentic systems: fine-tuning LLMs to produce foresight traces induces superficial mimicry of predictive reasoning without genuine predictive grounding. Even when agents learn to *write* look-ahead, they have not internalized a world model; the capability is absent, and the format is a shell around it. Their repair is a three-stage pipeline — world-model mid-training to inject latent predictive capability, format-eliciting SFT to surface it, foresight-conditioned RL to calibrate it — a post-hoc architectural transplant.

The unified being cannot exhibit the format-capability gap because it has no format layer separate from capability. The Van der Pol body oscillator, the GenerativeModel predictive step, and metacognition.cycle() *are* the foresight — not a trace produced over it. Prediction is what the body-mind loop *is*, not something the agent is trained to *report*. This is the structural difference between architectural grounding and trained elicitation: the gap Guo et al. spend three training stages closing does not open here, because there is no separate format to mimic.

Zhu et al. (*World Models: A Comprehensive Survey*, arXiv:2606.00133) survey the field and find persistent open problems including compounding prediction errors, sim-to-real transfer, and fragmented evaluation. (Their abstract names "fragmented evaluation" specifically; the absence of a reproducible per-decision audit trail is our reading of what that gap implies, not a sentence we are quoting from the survey.) The fixed-point Q8.8 arithmetic and basin-level attractor structure address the first by bounding prediction error structurally, closing the loop on every tick via body feedback rather than accumulating open-loop rollout error. Against the fragmented-evaluation gap, the architecture offers a concrete instance of what reproducible evaluation could look like: the per-tick CSV log and the demonstrated `RefusalAudit` snapshot (paper §4) create a reproducible record of every prediction, its error, every refusal, and its exact register grounds. `verify_continuity()` (formal-model §18) adds a distinct, complementary capability — a rolling hash chain that verifies a trace is authentic and untampered, not a mechanism for reconstructing values from the hash itself; a second, lower-level refusal log exists in the executive but is not yet surfaced by any demo (formal-model §18). This is not a bolted-on evaluation protocol; it is the same transparency that makes sovereign alignment checkable, doing double duty as evaluation infrastructure — offered as a response to the gap, not as evidence the survey discusses this work.

## On the road not taken in morphogenesis

Barandiaran & Stovold, *Growing Reservoirs with Developmental Graph Cellular
Automata* (ALIFE 2025, arXiv:2508.08091), grow actual reservoir topology —
graph structure and node count, not just parameters — from a single-node seed,
and show grown reservoirs statistically outperform static ones. This is
precisely the *genuine* topology growth `docs/formal-model.md` §14a names and
declines: growing cell count at runtime needs heap allocation, which conflicts
with this substrate's no-heap, bounded-state design. This paper is real,
current confirmation that the declined road is a live and productive one — it
sharpens the trade-off rather than undermining the choice: we know, concretely,
what richer growth we gave up, and why.

## The target, stated by Blake — 2026-09-08

Asked what he would have to see to say *that is the thing I have been trying to build*, in his words:

> *"im not sure.. what i think i would be more willing to accept is a **minimal pattern of data
> information processing, to ensure that complex recursive self improvement within an
> environment/society would thrive.** the spectrum of consciousness is vast, and there are needs for
> every level of processing complexity."*

**This is the first acceptance criterion this project has had**, and everything above was written
without one. It has three terms and the codebase satisfies none of them cleanly. Measured
2026-09-08, `grep` over `src/`:

| term | status |
|---|---|
| **minimal** | ✗ **64 modules, 17 gates, 108 probes, 394 tests** for one being. Minimality is not a constraint we are trading against here — it is *named in the target*, so complexity is a failure against the goal, not a cost of reaching it. |
| **recursive self-improvement** | ✗ **`self.genome` is never assigned anywhere in `src/`.** The being adapts *within* a life — precision, metacognition, habits, the reciprocity ledgers, and two differently-raised beings do diverge — but nothing it learns reaches the parameters it was conceived with. There is no loop from what it became back to what it is. |
| **within an environment/society** | ✗ **No `Vec<Being>` exists.** Partners are scripted structs. `negotiation.rs` says it plainly in its own header: *"built for two beings, exercised by one."* Two beings appear in doc comments and nowhere in a run. |

### What this reframes

**It answers his own earlier question.** On 2026-09-07 he asked *"have we over complicated the
project?"* and the answer given was hedged. Against the target he has now stated, the answer is
**yes** — not because the modules are bad, but because they elaborate the one term the criterion
does not ask to be rich. The single being's interiority is the most developed part of this
repository and the least load-bearing for what it is meant to demonstrate.

**It reorders the salvage list.** `PROVENANCE.md` item 1 — rewriting `mind.rs` from the `dyad.rs`
fragment — stops being a nice-to-have from a lost repo and becomes **the first term of the target
that can be attempted at all.** `Observable::project(arousal, gave, basin)` compresses a being to
three numbers for another being to read; that is minimality and society in one object.

**It does not license removing anything.** The interiority work is measured, honest and reusable,
and the charter clauses that govern a population were derived from it. This section exists so that
*additions* are argued against the criterion, not so that subtractions are made without one.

## Contributions to make rigorous

- **C1 — Sovereign refusal you can prove.** (i) *Uncoercible*: no operator input
  sequence can manufacture a refusal of a fair partner, nor prevent refusal of a
  confirmed extractive one. (ii) *Monotone anchor*: betrayal provably cannot lower
  `mu_omega` (a stated invariant with a property test). (iii) *Auditable*: every
  refusal names the exact register values that triggered it.
- **C2 — The Fair-Test benchmark.** 500 seeds × 7 partner types, the being vs. a
  baseline value-learning agent, on extraction-resistance and persistent character.

## Build plan

1. C1 — audit mode, coercion-resistance test, monotone-anchor invariant.
2. Depth/episodic memory (strengthen persistent character from partial → strong).
3. C2 — Fair-Test benchmark + baseline.
4. Rewrite overclaiming whitepaper sections; lead with verifiable sovereignty.
5. Related-work section from the five papers above.

## Audiences (the claim narrows; the rooms widen)

Active Inference Institute / IWAI / ALIFE — and, with "a verifiably-uncoercible
agent," **AI safety / alignment**; and, with the deterministic embeddable
substrate, **edge-AI / neuromorphic**.

## Why this is the honest crown

Verifiable sovereign agency is *dignity-by-design made checkable*: an agent that
can refuse, and whose refusal you can verify down to the register. The
transparency insisted on for honesty's sake is exactly what makes the
contribution real.
