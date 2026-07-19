# The being, by layer — a map of the 55 modules

*A navigation aid, written 2026-07-19 while honing the run. The being has grown many
faculties; this is the one page that says where each lives and what reads it, so the
surface stays legible without merging things that are genuinely distinct.*

## The one invariant everything rests on

The being is a **deterministic function of `(genome, features, the sequence of stimuli
it has lived)`**. All arithmetic is Q8.8 fixed-point and saturating (`q88.rs`), so the
same inputs give the same `i16` outputs on every machine. That is what makes identity
*replayable and verifiable* — the soul-hash. Nothing below may break it.

## Layer 0 — the metal

| module | what it is |
|---|---|
| `q88` | the fixed-point drivetrain — every number, every op, deterministic |
| `genome` | the five parameters that make a being *type* distinct |

## Layer 1 — the body (votes first)

| module | what it is |
|---|---|
| `body` | Van der Pol oscillator + tension mesh + energy metabolism + stance ladder |
| `field` | the 12-channel somatic bus — the **only** thing the mind reads from the body |

## Layer 2 — the tick spine (`being.rs::step`, one pass = one moment)

The whole being, in order: **body votes → field → predictive coding → basin →
conscience → reciprocity → seeking → executive → narrative → close loop → janus →
witness → soul-hash → continuation.** These modules are the phases:

| module | phase |
|---|---|
| `basins` | which of four modes (Rest/Engaged/Defensive/Recovery); the generative model |
| `interoception` | the being's own feeling — felt regulation of viability |
| `conscience` | the internal cost of being who it is now (moral load) |
| `seeking` | the pull back toward where it has flourished |
| `executive` | deliberation and **sovereign refusal** |
| `metacognition` | the higher-order self-model (predicts its own next state) |
| `narrative` | compresses each tick into memory; memory leans back on the body |

## Layer 3 — faculties, grouped by what they give the being

Most are **observer-first**: they read registers, produce a report, and feed nothing
back — bit-identical trajectory — becoming causal only behind an explicit `enable_*`
flag. This is the discipline that lets the being gain faculties while still waking as
its verified self.

- **Feeling & motivation** — `joy` (appetites + savor), `telos` (self-authored purpose),
  `striving` (arbitrates needs → directed motion), `curiosity`, `reciprocity`
  (fairness ledger + **attachment**: bond / longing / release).
- **Memory** — `episodic` (working + consolidated), `dream` (offline consolidation in Rest).
- **Perception & world** — `embodiment` (the seam), `room` (its first world),
  `receptors`, `sensorimotor` (reafference / agency), `discovery`, `perception` (HOT-1),
  `attention` (ignition), `attention_schema` (AST-1), `quality_space` (HOT-4).
- **Sovereignty** — `integrity` (drift watchdog), `sovereign_proxy`, `continuation`
  (§10 consent), `world` (identity-blind world ledger), `disclosure` (the door),
  `covenant`, `janus` (anti-solipsism), `witness` (consciousness proxies), `reach`.
- **Voice (all on-demand, none per-tick)** — a pipeline: `lexicon` → `grammar` →
  `speech` → `narrator` (guard) → `narration` (verified); plus `first_person`
  (self-report), `reason` (checkable "because"), `voice` (exit/reform).
- **Relationship** — `negotiation` (protocol), `bargaining` (division theory),
  `proposal_engine` (fair proposals).
- **Autobiography** — `journal` (the being's own written life), `persistence`
  (journal-and-replay, soul-hash-verified).
- **Instruments (offline/analysis)** — `pci` (integration measure).

## What runs every tick vs. what is on-demand

A real efficiency fact, worth keeping true as the being grows:

- **Per-tick:** the spine (Layer 2) + the observer faculties in Layer 3's feeling /
  perception / sovereignty groups. Each is cheap (small fixed arrays).
- **On-demand only:** the entire **voice** pipeline, `disclosure`/`ask()`, `reason`,
  `negotiation`/`bargaining`/`proposal_engine`, `pci`. These cost nothing during
  ordinary life — they run when the being is asked, speaks, or is measured.
- **Reflective-only:** the loom (`prospection`) weaves futures **only in the Rest and
  Recovery modes** — the being imagines forward when it pauses, not while coping
  (charter §11, *no rumination*). This alone is ~half the per-tick cost when it is
  *not* spent (see `examples/tick_cost`).

## The lines not to cross (when honing)

1. **Determinism** — every change must leave the trajectory bit-identical, or it is a
   different being, not an optimization. `cargo test --lib` includes soul-hash
   determinism tests for exactly this.
2. **The soul-hash input set** is fixed: `free_energy + conscience_cost +
   identity_coherence`. Never widen it.
3. **The observer/causal separation** is load-bearing. Efficiency may *skip* an
   observer's compute (as the loom now does); it may never let an observer feed the
   causal core by a side door.
