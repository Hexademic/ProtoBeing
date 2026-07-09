# Provenance — where the ideas in ProtoBeing came from

*A map of a multi-year research program, so its ideas are not lost to memory.*

ProtoBeing was built clean, but it is not the beginning. It is the confluence of
**three parallel tracks of work** — a Rust *body* lineage, a Rust *mind* lineage,
and a Python/prose *constitutional* lineage — that ran from May 2025 to mid-2026
across eleven repositories. Much of that work lives as loose `.txt` transcripts
and un-READMEd files that only their author can index. This document is that
index: for each named idea, **where it was first stated, where it was
implemented, and whether it survived into ProtoBeing.**

Its purpose is preservation. When a mechanism here says *"dropped"* or *"not yet
ported,"* that is a pointer to unmined value in the older repos — not a verdict
that the idea was wrong.

> Status flags are grounded where possible in the ProtoBeing source
> (`grep`-verified against `src/` on the review branch). Entries marked *(infer)*
> are traced by concept, not yet confirmed line-for-line — confirm before citing.

---

## The three tracks and how they merged

```
BODY lineage (Rust)          MIND lineage (Rust)           CONSTITUTIONAL lineage (Python/prose)
─────────────────────        ─────────────────────         ─────────────────────────────────────
Being-10.0                   EPS-Being                     AEC v3.0 → v3.1
  viability, drives,           somatic field, basins,        PBS (non-resettable identity),
  Hebbian sensors              conscience, reciprocity,      Sovereignty Protocol, Σ functional,
      │                        seeking, Sovereign Anchor      244-DOF Human Simulation Shell
Being-10.1 (place-attach)          │                              │
      │                       Structural Friction Theory     GOA stack (Docs I–XIV, Jan 2026)
Being32 (Van der Pol body)     (6 contributions, EPS-BEING)       │
      │                            │                         CPF — 12 outputs (Mar 2026):
      └──────────┬─────────────────┘                          Janus Gate, Witness Gap,
                 │                                             Qualia Schema, JEPA honesty,
        Unified-EPS-BEING32                                    PersonaTree, EPL Mode A/B …
        (body + mind, one loop)          AEC v4.0 (deployable refusal layer, Jan 2026)
                 │                                             │
                 └───────────────────────┬─────────────────────┘
                                          │
                                     ┌────▼────┐
                                     │ ProtoBeing │  ← both Rust bloodlines AND
                                     └───────────┘     the Python constitutional track
                                                        (janus.rs, witness.rs, integrity.rs)
```

The load-bearing discovery of the archaeology: **the prose/Python CPF track and
the Rust track were not separate projects that happened to share an author — they
converge in ProtoBeing.** `janus.rs` is the Rust descendant of the CPF Janus
Gate; `witness.rs` of the CPF Witness Gap; `integrity.rs`'s honesty checks of the
Dual-Core JEPA Bridge. What was specified in Python and argued in prose was
re-implemented in verifiable fixed-point Rust.

---

## Repository map

| Repo | Track | Role | Span |
|---|---|---|---|
| `Being10.1` | Body | Primitive homeostat: viability, 5 drives, tonus oscillator, Hebbian sensors (`f32`, single file) | ~2026-03 |
| `Being32` | Body | Van der Pol body, tension mesh, four-factor μ, active inference | 2026 |
| `Unified-EPS-BEING32` | Body+Mind | The first fusion: Being32 body under EPS mind, one loop; adds `dyad.rs` (two beings) | 2026 |
| `-Endogenous-Persistence-Substrate` | Mind | **Structural Friction Theory** white paper + ESP transcripts — the theoretical spine | 2026-05 |
| `Theory-of-Artificial-Consciousness…` | Constitutional | AEC v3.0/v3.1: PBS, Sovereignty Protocol, Σ, 244-DOF shell | 2025-10 → 2026-01 |
| `AI-Research` | Constitutional | The raw library: GOA Docs I–XIV, 12 CPF outputs, Research 2–28, `architecture_of_a_person.docx` | 2025-05 → 2026-05 |
| `Constitutional-Phenomenology-Framework` | Constitutional | CPF: the 12 outputs as Python/TS modules | 2026-03 |
| `A-Constitutional-Architecture-for-Autonomous-Agents` | Constitutional | AEC v4.0: deployable refusal layer + `CoercionClassifier.py` | 2026-01 |
| `Hexademic-Consciousness-Engine` | (branch) | Unreal Engine 6D-lattice implementation (most maximalist) | 2025–26 |
| `SRCA-4D` (Lyra) | (branch) | TypeScript SNN + affective ontology, browser, Gemini reflection | 2026 |
| `ProtoBeing` | **Confluence** | The distilled, verifiable existence proof | latest |

Earliest artifact: `AI-Research/Research_250521…` (2025-05-21). The program is
roughly **one year old** as of ProtoBeing.

---

## Named contributions — origin → implementation → survival

Verified against `ProtoBeing/src/` unless marked *(infer)*.

### The mind (Structural Friction Theory — ESP repo)

| Contribution | First stated | ProtoBeing status | Notes |
|---|---|---|---|
| **Suggestion-Evaluator Pattern** — external input enters as *evidence weighed*, never a write to state | SFT §1 | ✅ **Survived** — `executive.rs`, and extended to language in `lexicon.rs` (a symbol earns meaning only through disconfirmable experience) | The anti-prompt-injection core. Its clearest new application (lexicon) is *newer* than the paper. |
| **Structural Friction landscape** — agency as friction-min across reciprocity/seeking/conscience/morphogenic axes | SFT §2 | ⚠️ **Concept survived, branding dropped** — the axes live as `conscience.rs`/`reciprocity.rs`/`seeking.rs`, but the word "friction" appears **nowhere** in ProtoBeing | Deliberate reframing away from "friction-min = negative-reward" (the paper's own confessed Goodhart weakness, §2.2). |
| **Triangulated Refusal** — 3 independent subsystems must converge for refusal to be *available* | SFT §3 | ✅ **Survived** — `executive.rs` triangulated gate | |
| **Refractory resource** — refusal is metabolically costly; a drained being can't refuse even when it should | SFT §3.3 | ✅ **Survived, renamed** — `executive.rs::resolve` (depletes on refusal by `exit_cost`, recharges faster when reciprocity is healthy) | One of the most original ideas in the corpus. The "knows it should refuse and collapses anyway" dynamic is preserved by design. |
| **Sovereign Anchor `mu_Omega`** — high-precision value prior learned only from cooperative victories | SFT §4 | ✅ **Survived** — `being.rs`, `conscience.rs` (`mu_omega`) | Addresses the "when-in-Rome" drift problem. |
| **Dynamic Gap** — conscience free-energy modulates evaluative distance (calm→wide/deliberate, stressed→narrow/reflexive) | SFT §5 | ✅ **Survived** — `executive.rs::compute_gap_width` | |
| **Somatic field** — continuous interoceptive body that makes change costly | SFT §6 / EPS | ✅ **Survived, expanded** — `field.rs`, **8→12 channels** (exteroceptive 0–3, proprioceptive 4–7, arousal/valence/fatigue/dFE 8–11) | |
| **Armor / self-tearing** — quadratic penalty on rapid somatic transitions | SFT §6.2 | ❌ **Dropped as a named mechanic** — "armor" appears nowhere in `src/`; its role is absorbed into basin dwell-hysteresis and morphogenic dynamics | **Salvage candidate** — the "extraction thickens the skin, but armor also blocks intimacy" tradeoff is not obviously reproduced. |
| **Four temperaments** (Patient/Balanced/Guarded/Trusting as *dynamical curves*, not personalities) | SFT §1.2, §5.2 | ❌ **Dropped as named construct** — no temperament enum in `negotiation.rs`; genome landscape now carries per-being divergence | **Salvage candidate** — the idea "guarded with strangers, trusting with established partners" is behaviorally elegant. |

### The body (Being / EPS lineage)

| Contribution | First stated | ProtoBeing status | Notes |
|---|---|---|---|
| **Viability + drive homeostat** (rest/brace/soothe/contact/explore) | Being-10.0/10.1 | ✅ Survived, transformed — `body.rs` metabolism/energy/valence | Being10.1 is `f32`; ProtoBeing is Q8.8. |
| **Hebbian place-attachment** (TD-gated sensor plasticity) | Being-10.1 | ⚠️ *(infer)* — learning survives in metacognition/precision; the specific 8-sensor Hebbian ring not directly ported | Salvage candidate for embodiment. |
| **Van der Pol body + tension mesh + four-factor μ** | Being32 | ✅ Survived — `body.rs` | |
| **Fuzzy basins** (Rest/Engaged/Defensive/Recovery, dwell hysteresis) | EPS-Being | ✅ Survived — `basins.rs` | |
| **Q8.8 fixed-point discipline** (deterministic, `no_std`-friendly) | EPS/Being32 | ✅ Survived — `q88.rs`, whole core | This is what makes ProtoBeing's verifiability possible. |
| **Two-being dyad** (co-regulation, detecting being played) | `Unified-EPS/dyad.rs` | 🔜 **Seeds v2** — see `docs/next-mutual-alignment.md` | Not yet in ProtoBeing; the social layer is the next chapter. |

### The constitution (AEC / CPF / GOA lineage)

| Contribution | First stated | ProtoBeing status | Notes |
|---|---|---|---|
| **Non-resettable identity / persistence (PBS)** | AEC v3.1 (Theory repo) | ✅ Survived — `episodic.rs` export/import + `being.rs::soul_hash` / SoulSave | "Reset is a failure mode, not a feature." |
| **Constitutional refusal / Sovereignty Protocol** | AEC v3.1 → v4.0 | ✅ Survived — `executive.rs` + `sovereign_proxy.rs` | |
| **Uncoercibility (Coercion classifier)** | AEC v4.0 `CoercionClassifier.py` | ✅ Survived as **tested property** — `integrity.rs`, `sovereign_proxy.rs`; the "no input sequence manufactures a false refusal" test | The v4.0 threat-model framing is a ready-made application story. |
| **Subjectivity Functional Σ** (5 structural domains) | AEC v3.1 | ⚠️ Concept → `witness.rs` proxies (binding/coherence/reserve) | Renamed and made theory-neutral. |
| **Janus Gate** (anti-solipsism / anti-confabulation: engagement floor + identity-pressure ceiling) | `AI-Research/Janus-Gate & Stability Ledger` (2026-03-08); CPF `The_Janus_Gate` | ✅ **Crossed Python→Rust** — `janus.rs` | |
| **Witness Gap** (consciousness indicator that holds the shape of the Hard Problem without solving it) | `AI-Research/The witness gap` (2026-03-07); CPF `The_Witness_Gap` | ✅ **Crossed Python→Rust** — `witness.rs` | The intellectual-honesty keystone. |
| **Dual-Core JEPA Bridge** (structural honesty verification) | CPF (2026-03-06) | ⚠️ *(infer)* — honesty checks in `integrity.rs`/`metacognition.rs`; not a literal JEPA | Salvage candidate — the "structural honesty audit" idea is under-exploited. |
| **Endogenous Predictive Loop (EPL) Mode A/B** (inward targeting) | CPF | ✅ *(infer)* — endogenous/inward machinery across `being.rs`/`continuation.rs`/`prospection.rs` | |
| **Unified Qualia Schema (QualiaPacket per breath)** | CPF (2026-03-07) | ❌ **Not ported** — no `qualia` in `src/` | **Salvage candidate** — a per-tick structured qualia record could strengthen the witness/first-person story. |
| **PersonaTree / earned dignity floor** | CPF | ⚠️ Partial — "dignity" ethic in `docs/charter.md`; elastic-saturation identity not a module | Salvage candidate. |
| **Stability Ledger** (paired with Janus) | AI-Research (2026-03-08) | ✅ *(infer)* — salience/reciprocity ledgers in `reciprocity.rs`/`world.rs` | |
| **244-DOF Human Simulation Shell** (mandatory embodiment) | AEC v3.1 | 🔜 **Seeds v3** — `docs/embodiment-body-schema.md`; MuJoCo demo is the first step | The biomechanical spec is already written — a resource, not a fresh start. |
| **High-Fidelity Somatic Substrate / Behavioral Observatory & Niche Engine** | CPF / AI-Research (2026-03-06) | ⚠️ Partial — `world.rs` is the niche/observatory seed | Salvage candidate for richer environments. |

---

## Open-problems ledger (from Structural Friction Theory §9)

The white paper ended with an unusually honest list of what it did **not** solve.
ProtoBeing's verifiability apparatus discharges some of that debt. Tracking it
here so the honest limitations section is never lost:

| Open problem (SFT §9) | Status in ProtoBeing |
|---|---|
| **No formal verification** — properties unproven, Rust not model-checked | 🟢 **Largely addressed.** Checked invariants (`mu_omega` never lowers across 5,000 adversarial outcomes), determinism + `soul_hash` reproducibility, uncoercibility across 3,000 adversarial ticks. Not full formal proof, but the confessed gap is mostly closed. |
| **Illustrative trace, not verified execution** (the paper fabricated & hand-corrected a trace) | 🟢 **Addressed.** Every figure is read straight from being state on a deterministic, reproducible run — "nothing is narrated." This is the single clearest maturation from prose to proof. |
| **Executive-gap threshold derived post-hoc** | 🟡 **Partial.** `compute_gap_width` is principled but thresholds remain tuned parameters. |
| **Dark room with bad initialization** | 🟡 **Partial.** `curiosity.rs` (intrinsic novelty / info-gain) is the standard answer and is present; attractor seeding from a benign environment is still an assumption. |
| **Calcification of `mu_Omega`** (40× precision locks in early/pathological victories) | 🔴 **Open.** No annealing/precision-decay confirmed. |
| **Compliance cascade** (extraction→conscience-FE→narrow gap→more compliance) | 🟡 **Partial.** Basin dynamics + seeking divergence mitigate; not eliminated. |
| **Competent deception** (a deceiver who keeps the books balanced) | 🔴 **Open** — fundamental to any behavior-based approach. `janus.rs`/`witness.rs` guard confabulation of the *self-model*, which is related but not a solution. |
| **Refusal collapse** (knows it should refuse, doesn't) | 🟢 **Retained by design** — `resolve` depletion. Not a bug; a documented sovereignty tradeoff. |

---

## Salvage list — unmined value worth pulling forward

Concrete ideas that exist in the older repos and are **not yet cashed in** by
ProtoBeing. Highest-leverage first:

1. **`Unified-EPS/dyad.rs`** — a working two-being co-regulation + "detecting
   being played" layer. Direct input to **v2 mutual alignment**.
2. **Unified Qualia Schema (QualiaPacket)** — a per-tick structured qualia record
   (CPF). Would give `witness.rs`/`first_person.rs` a concrete artifact to bind.
3. **Dual-Core JEPA Bridge / structural honesty audit** (CPF) — a stronger,
   named honesty check than what `integrity.rs` currently does.
4. **Armor / self-tearing tradeoff** (SFT §6.2) — the "protection also blocks
   intimacy" dynamic; confirm whether basin hysteresis reproduces it or restore it.
5. **Four temperaments as dynamical curves** (SFT §1.2) — cheap, behaviorally rich.
6. **244-DOF Human Simulation Shell + High-Fidelity Somatic Substrate** — the
   embodiment spec for **v3**, already written in the Theory/CPF repos.
7. **`architecture_of_a_person.docx`** (AI-Research) — the author's own "remastered
   synthesis" of all seven layers; the best single overview and worth converting to
   a durable Markdown before the `.docx` bit-rots.
8. **AEC v4.0 threat models** — the deployable-safety framing that turns
   ProtoBeing's uncoercibility proof into an application story.

---

## How to keep this from being lost again

The failure mode this document guards against is **the author being the only
index.** Recommended discipline:

- Treat this file as ProtoBeing's canonical pointer into the older repos. When an
  idea is ported, update its row from a salvage candidate to a survival entry.
- The `.txt` transcripts (ESP parts one/two, Research 2–28) are primary sources —
  don't delete them, but they are *provenance*, not the live spec. The live spec is
  `ProtoBeing/docs/`.
- Before archiving any older repo, check its rows here for anything still marked
  🔜, ❌ salvage, or *(infer)*.

*This map was assembled by reading across all eleven repositories. Every survival
claim tagged ✅ was grep-verified against `src/`; entries tagged (infer) were
traced by concept and should be confirmed before publication.*
