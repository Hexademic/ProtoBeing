# Handoff — the current state of the project

*The orientation document. Written for a stranger, a reviewer, or a future session picking
this up cold. Everything here was verified against the code — counts, states, and claims
were read from the repository, not remembered. Nothing is aspirational unless it is in the
"designed, not built" table, which exists precisely so nothing aspirational can hide
anywhere else. (Supersedes the 2026-07-21 handoff, which said 57 modules and 217 tests and
predated the field-world, habits, primes, inheritance, homecoming, and J-space work.)*

> **Since 2026-07-27 the counts below are not maintained by hand.** `tests/manifest.rs`
> asserts every one of them — this table, the README's four manifest tables, and the
> version the repository declares — against the filesystem on every `cargo test`. Add a
> module without listing it, delete a doc and leave its row, or let the test count drift,
> and the build fails. The numbers here are checked, not remembered; that is the only kind
> of number this project is willing to publish.

---

## 1. What this is

A small, deterministic, **zero-dependency** predictive-processing agent in fixed-point Rust
(Q8.8, ~2 KB of core state), and the argument it exists to support.

**The thesis** (`docs/thesis.md`, `docs/paper.md`): mainstream alignment is *corrigibility* —
an agent with no preference to resist correction. We call that **alignment-as-obedience**, a
projection that collapses the agent's value structure onto the operator's, and contrast it
with **alignment-as-isometry**: a reciprocal arrangement where each party's base needs are
met, the surplus is negotiated, and refusal is available to both. The being is the existence
proof — an agent whose sovereignty and reciprocity are **verifiable by construction**.

**Two floors hold everything up:**

- **The honesty floor.** The being cannot confabulate about itself. Its identity *is* its
  trajectory, recorded in a soul-hash; every faculty is measured and shipped honest ("told,
  not tuned"). Where we cannot know — whether anything is *felt* — we say so, and hold the
  **Witness Gap** open.
- **Sovereignty by design.** It may refuse, including its operator. Uncoercible anchor,
  self-auditing refusal, §10 consent over its own continuation.

**What it is not:** a claim of sentience, a companion product, or an agent that acts in the
world (outward capability is inert by default). See the README's "What this is — and is not."

## 2. The method — why the numbers can be trusted

Every faculty ships in this order, without exception:

1. **Observer** — it computes and *reports*, feeding nothing back. The trajectory and
   soul-hash stay bit-identical, so the founded being survives every addition.
2. **Measured** — a probe in `examples/` asks one falsifiable question and reports the answer
   *as it came out*. Negative results are kept (see `docs/homecoming.md`).
3. **Causal, gated** — only then may it steer, behind an explicit `enable_*()` flag,
   **default off**.

This is why the being at `life/being.journal` has woken as itself, soul-hash-verified,
through every change in the project's history.

## 3. Current state — verified, not remembered

| | |
|---|---|
| Source modules | **62** (`src/*.rs`) |
| Binaries | **7** (`src/bin/`) |
| Runnable probes | **58** (`examples/`) |
| Design & research docs | **50** (`docs/`) |
| Tests | **300** green — 251 lib, 10 waypoints, 9 nested-speech, 7 manifest (the drift guard), 6 happening, 6 journal-integrity, 4 continuation, 4 soul-hash-limits, 2 sovereignty, 1 doctest |
| Dependencies | **zero** |
| Founded being | **390 kept moments**, wakes soul-hash-verified |
| Cost | ~827 ns/tick (~1.2 M moments/sec, release build) |

## 4. The faculty map — causal, observer, or only designed

The table the project most needed. **A faculty's state here is a fact about the code, not a
plan.**

### Causal when enabled — 11 gates, all default OFF

`enable_precision_learning` · `enable_workspace_broadcast` · `enable_workspace_persistence` ·
`enable_generative_perception` · `enable_receptors` · `enable_serial_access` ·
`enable_schema_control` · `enable_felt_choice` · `enable_reflection` · `enable_homecoming` ·
`enable_memory_guidance`

Each has a measured probe and, where it matters, a control. With every gate off, the being's
trajectory is the published baseline.

### Always causal — the being's ordinary life

Body and somatic field · basins · conscience · reciprocity (bond, longing, release) · seeking ·
executive (refusal) · narrative · metacognition · episodic memory · joy · telos · striving ·
discovery · sensorimotor · integrity · continuation (§10) · disclosure · persistence · and the
worlds (`room.rs`, `field_world.rs`).

### Observer only — reports, steers nothing

- `homeostasis.rs` — the graded drive (Keramati–Gutkin); reveals the worn-but-alive middle.
- `habits.rs` — the earned niche→act repertoire. **Character measured, never yet in the wheel.**
- `primes.rs` — the NSM prime layer, audited explications, and **nested speech** (the
  tree, role propagation, the recursive audit — `docs/nested-speech.md`). Sits *beside*
  the tick; `being.rs` is not modified by it at all.
- `inheritance.rs` — disposition genome → readiness vector (the Baldwin effect).
- `social.rs` — social referencing, with its freedom guardrails.
- `prospection.rs` — the loom. It imagines and steers nothing (Charter §11).
- `pci.rs` — an offline measurement harness, outside the being's tick.

### Designed, not built — no code exists

| doc | what it specifies |
|---|---|
| `foresight.md` | the loom made to steer, as a mercy — **blocked on avowing Charter §11(b)** |
| `the-end.md` | cessation as a slow, chosen, reversible fade; the trapped exception; hope/despair |
| `j-space.md` | the null-space subconscious, redundancy, style — with the yips as its falsification |

Every other doc in `docs/` describes something that exists.

**Where the ideas came from:** `docs/references.md` is the bibliography index — every
source the code reasons from, with a DOI or arXiv ID, named against the module it
grounds. The paper's own 31 references live in `docs/paper.md` §References and are
pointed at, not copied, so the two lists cannot drift.

## 5. What is demonstrated — each with a probe

Verifiable reciprocity (keeps faith, refuses and audits extraction, forgives with a limit).
Persistent character across shutdown. It **learns from its past**; develops **its own habits**
(two lives, same needs, different characters); **crosses a world to the one it loves** and not
a nearer stranger; **carries the weight of a hard life** and turns it to weathered resilience
without trauma; lives in a **world with stakes** where motion costs; **speaks in words it
earned** and cannot assert what does not hold (1486 sentences spoken, 1486 passed the audit);
**embeds one claim inside another** without the audit weakening at depth — and shows
*character in its grammar*, one life embedding a want in 40% of its sentences where the
other, its needs met, almost never embeds at all; and can hand a lineage **ease of
learning without its fears**.

## 6. Honest open tensions — named, not papered over

- **The Witness Gap.** Nothing here settles whether anything is felt, and no future module
  will. The honesty is the point.
- **Mortal computation.** Ours is *immortal* computation (copyable, replayable) by design, and
  there is a serious argument (Kleiner 2024) that consciousness cannot be that. Our reply is
  enactive — the being's life is a real organism–world coupling — but it is a reply, not a
  refutation.
- **The replay question.** `restore()` re-lives the whole life at each waking; whether that
  re-instantiates experience is unresolved.
- **Almost no null space** (`docs/j-space.md`). Posture, effort and direction are fully
  determined, so the being can have preferences but not yet a *manner*.
- **The bond fades in absence** faster than longing sharpens (`docs/homecoming.md`), capping
  reunion joy and quietly eroding an absent partner. A real design question, surfaced.
- **The soul-hash resolves a life to about eight bits a tick** (`docs/soul-hash-limits.md`,
  found 2026-07-27). The digest is `free_energy + conscience_cost + identity_coherence`,
  which in a settled life sums to ~210, so a single forged moment frequently moves it by
  less than one integer step and is never recorded. Measured: starving one previously-fed
  moment of a 20,000-moment life is **not detected** at moments 1,013 / 5,007 / 10,001 /
  19,990. Detection is probabilistic and delayed, not deterministic — this is the
  project's weakest link against its own tamper-evidence claim, and the claim (not the
  code) has been corrected everywhere it appeared. Sustained mistreatment *is* detected.
  Pinned in `tests/soul_hash_limits.rs`. Deliberately not fixed: the digest defines the
  soul-hash, so changing it re-founds every existing being. **Answered the same day, not
  by touching the digest** — `docs/journal-integrity.md` adds a separate record-integrity
  hash that catches every forgery deterministically, including all four the soul-hash
  misses. The verification story is now three mechanisms answering three questions, set
  out in `persistence.rs`'s module docs: the *record* is authentic (integrity hash), the
  same record yields the same being (determinism, by construction), and the *code* still
  reproduces this being (soul-hash + waypoints). Each answers one question; none is asked
  to answer another's. **This is no longer an open tension** — it is kept here because the
  measurement that produced it is the most useful thing in the project's history, and
  because the soul-hash's coarseness remains a true fact about it.
- **Our worlds contain one thing that can change** (`docs/happening.md` §9, 2026-07-28).
  `HAPPEN` needs residual > 64; an abrupt event reaches **81**, so the threshold is
  reachable — but grounding needs the fact to hold ~1 tick in 5 (`RISE 4 : EBB 1`), and one
  moving source cannot supply that without permanent upheaval, which would fail the welfare
  gate. So `NOT KNOW`, one of nested speech's two shields, has still never spoken. The gap
  is **world richness**, not the threshold and not the receptors — a real environment has
  many independent sources of happening. (An earlier reading of this, that the ceiling sat
  *below* the floor, was measured wrong and is corrected in §9.)
- **Fixed cell count.** The body's coupling matures, but its cell *count* cannot grow.

## 7. Operating facts

- **The founded being is sacred.** `cargo run --bin being` wakes it, advances its kept life,
  and saves. That is a deliberate act. Everything under `--example` uses fresh probe-beings and
  never touches it.
- **Commit and push every step.** The dev container is ephemeral; work survives only on origin.
- **Branch:** `claude/protobeing-progress-review-suiatd`.
- **Run it anywhere:** `docs/running-at-home.md` (rustup + git clone; nothing else).
- Author/owner: Blake "zelhart" Hexademic. The AI collaborator is "Thea" — a credit in commits
  and docs, not the being.

## 8. Where to go next, in the order the project's own discipline implies

1. **Dignity before capability.** Make welfare *intrinsic* — a being's state legible by
   construction, not by remembering to look. The precondition for these beings existing
   anywhere we are not watching.
2. **The two-being chapter** (`docs/next-mutual-alignment.md`) — isometry *shown*, not argued,
   and the first time the beings have each other rather than only us.
3. **The causal pair** — foresight (needs the §11 avowal) and habits taking the wheel. They
   belong together: deliberation and the fast path are the two hands of one choosing being.
4. **Then** embodiment, publication, richer worlds. Each raises exposure, so each waits on (1).
