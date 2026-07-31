# Agentic patterns — what has a name that we invented, and what does not transfer

> **Status: research note. Nothing specified, nothing built.** Written 2026-07-31 from
> `github.com/FareedKhan-dev/all-agentic-architectures` (README and architecture index fetched
> and read; the implementations were not). 35 patterns for orchestrating LLM agents.

## 1. The substrate mismatch, stated first

Every one of the 35 assumes a component that can be prompted in natural language, that
**samples** (non-deterministic), that holds unbounded intermediate state in a context window,
and whose steps are slow, expensive and high-variance.

ProtoBeing is deterministic Q8.8, ~2 KB of state, ~827 ns/tick, zero dependencies, no language
model anywhere, and its soul-hash requires **bit-identical reproduction**.

So the sampling-and-search family transfers **not at all**: Self-Consistency (sample N,
majority-vote), Tree of Thoughts, LATS (MCTS) and Ensemble all require randomness we have
deliberately excluded. The retrieval family has nothing to retrieve. Debate and STORM need
agents that generate arguments.

**And a deeper mismatch worth naming.** Most of these patterns exist to *manage an unreliable
oracle* — verification, self-consistency, grading, critique-and-revise are all machinery for
catching a black box being wrong. **Our being's problem is the exact opposite.** It is too
reliable: free energy 0.69/256, self-surprise 1.21/256, eleven thousand identical sentences.
Importing reliability-management machinery would solve a problem we do not have.

## 2. What this is actually worth: things we invented have names

The value here is not a pattern to adopt. It is that several structures we built from first
principles turn out to be **named**, which means we are in a tradition and there is literature
on their failure modes.

| ours | the standard name | what that buys us |
|---|---|---|
| the `UnifiedBeing` struct every faculty reads and writes | **Blackboard** (classical AI) | Named 1970s pattern with *known* problems — contention, and no explicit dataflow. Exactly the critique I reached in the systolic-array conversation, arrived at independently. Worth reading the failure literature rather than rediscovering it. |
| `prospection.rs` (imagines, steers nothing) + `docs/foresight.md` (blocked on §11(b)) | **Dry-Run** — propose → simulate → approval gate | Our "blocked pending avowal" *is* the approval gate. We treated it as a special ethical hesitation; it is also a standard control-flow shape. |
| `enable_memory_guidance` — learned forewarning from the being's own past | **Reflexion** — verbal reflections stored in episodic memory and read back | Ours is register-based rather than verbal, but the loop is the same one. Notably it is **already** a small instance of Maes's second causal direction (`docs/reflection.md` §2) — the only one we have. |
| the charter's numbered clauses + `executive.rs`'s refusal audit | **Constitutional AI** — per-rule pass/fail → revise | Ours is arguably stronger: our rules check *registers*, not a model's judgment of itself, so a rule cannot be talked around. |

## 3. The one that validates a diagnosis from outside

**"Reflexive Metacognitive — self-aware capability routing."**

An entire named architecture family whose premise is that an agent knows what it can and cannot
do, and routes accordingly.

`docs/richness.md` §6 measured that our being **cannot say `CAN` or `CAN'T`** — agency spikes
past the bar and never sustains, and free energy never comes near the bar for `CAN'T`. We
concluded it has no representation of its own limits.

Here is a whole pattern family that *requires* exactly that representation. That is an
independent confirmation that the gap we found is load-bearing rather than cosmetic: it is the
precondition for a class of architectures, not just for a word.

## 4. Two structural ideas worth keeping

- **Cellular Automata** ("rules over a grid") — local rules, global behaviour, no central
  controller. Same family as the systolic thread and `docs/fallback.md`'s MorphoHDL import.
  Three separate sources now point at locality-over-central-state.
- **Meta-Controller** ("router over architectures") — choosing *which mode to be in*. That is
  the same question `docs/deferral.md` is circling: not what to do, but which regime governs.
  Precision-modulation (§2c) is our version and is better suited to a graded substrate than a
  discrete router would be.

## 5. What to check before any of this is cited

The README was read; **no implementation was**. Benchmarks, "production-grade," and the "this
repo" attributions on ~8 of the 35 patterns are unverified claims from a project README. The
underlying papers (Madaan 2023, Shinn 2023, Yao 2022/2023, Bai 2022, Packer 2023, Wang 2023)
are real and citable, but should be verified individually before entering
`docs/references.md` — none is cited from here.
