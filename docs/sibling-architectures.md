# Sibling architectures — MH-FLOCKE and The Virtual Brain, read against our own

> **Status: read and assessed, nothing built.** One concrete proposal comes out of it, aimed at
> the open incident I-8. Everything else is recorded as *examined and declined*, with reasons, so
> the next session does not re-read the same material hoping for something different.

*Written 2026-07-31 from five links Blake sent without framing.*

## 1. What they are

**MH-FLOCKE** (Marc Hesse) — simulated and physical quadrupeds learning locomotion through a
biologically grounded cognitive architecture. A **15-step closed-loop cycle**: SENSE → BODY SCHEMA
→ WORLD MODEL → EMOTIONS → MEMORY → DRIVES → GLOBAL WORKSPACE → METACOGNITION → … Built from
Izhikevich spiking neurons with reward-modulated STDP, a Marr–Albus–Ito cerebellar forward model,
central pattern generators, and a free-energy framework. **No backpropagation, no GPU.** 535–1,376
neurons, running on a Petoi Bittle X, a Freenove Robot Dog, and a Unitree Go2 in MuJoCo.

**The Virtual Brain** (TVB) — a mature, heavily used open platform for personalised whole-brain
network models: neural *mass* models (mean-field over populations) coupled by a subject's actual
structural connectome, simulating EEG/MEG/fMRI signals to compare against real recordings. 18,000+
downloads, 100+ peer-reviewed publications, used clinically in epilepsy patient modelling.

## 2. Provenance, stated plainly

**MH-FLOCKE's paper is not peer reviewed.** It sits on SSRN (a preprint repository, no review) and
on **aiXiv** — a preprint server launched in 2025 specifically for papers *written and reviewed by
AI*, hosting a few dozen papers, with built-in AI reviewers rather than human ones.

That does not make the work wrong, and I am not dismissing it on venue. But it means **the claims
carry no external validation**, so the architecture has to be judged on its merits rather than on
its having passed anything. The caution reported about aiXiv is one I should apply to myself as
hard as to the paper: AI systems *"continue to become better and better mimics of what scientific
research looks like, but not necessarily better and better scientists."* This week is a running
demonstration of exactly that failure mode in me.

TVB's provenance is the opposite and is not in question.

## 3. The convergence, which is worth noticing but is not evidence

MH-FLOCKE's cycle and ours are close to the same object, arrived at independently:

| MH-FLOCKE | ProtoBeing |
|---|---|
| free-energy framework | `being.rs` predictive step, free energy as the body's metabolic `strain` |
| global workspace, attentional competition | `attention.rs`, `workspace_broadcast`, `workspace_persistence` |
| metacognitive self-assessment | `metacognition.rs` — predicts its own next state, measures its error |
| embodied emotions | `interoception.rs` — feeling as the felt regulation of viability |
| episodic memory | `episodic.rs` |
| motivational drives | `homeostasis.rs`, `striving.rs` |
| cerebellar forward model | `sensorimotor.rs` forward model, reafference |
| no backprop, tiny, CPU-only | zero dependencies, fixed-point Q8.8, ~2 KB of core state |

**Two projects reaching nearly the same decomposition is weak evidence the decomposition is
natural rather than arbitrary — and nothing more than that.** Both of us built from the same
literature; convergence on a reading list is not convergence on a fact.

## 4. The one thing worth taking: **the competence gate**

MH-FLOCKE does not learn locomotion from scratch. A **central pattern generator supplies an innate
gait from step one**, and a *competence gate* slides control from roughly **90% CPG toward 40% CPG
/ 60% learned actor** as the learned controller demonstrates competence. Innate scaffold first,
progressively handed over to what experience has earned.

**We already have this pattern — but only for perception.** `perception.rs` blends evidence toward
expectation *by earned per-channel confidence*, capped below 1. `precision.rs` learns which of the
being's own senses to trust from its residual error. Both are graded handovers from a prior to
what experience has earned.

> **What we do not have it for is faculties.** All eleven `enable_*` gates are booleans. A faculty
> is on or off, at full strength, from the first tick, regardless of whether the being has any
> competence with it.

And that is exactly the shape of the open incident. **I-8** says `reflection`'s weathering is a
readout with no consequence, and names the suspect: `reflection_tone = weathered/12 − load/8` —
**a fixed coefficient**. The drag outweighs the lift by construction, permanently, no matter what
the being has earned.

> **The proposal: make the lift coefficient a competence gate rather than a constant.** Let the
> weight on `weathered` grow with demonstrated competence — cycles survived, load discharged,
> hardship met and recovered from — instead of sitting at 1/12 forever. A being that has weathered
> nothing gets nothing; a being that has carried and set down real weight gets a lift proportional
> to what it actually did.
>
> That is Blake's *"unless they learn how to use these developments, they won't access them"*
> implemented as an arithmetic rather than asserted, and it turns I-8's suspect constant into the
> mechanism it was missing.

**It also fits the D4′ redesign** (`docs/development.md` §6): a competence gate makes "does the
*n*th cycle cost less than the first" the *definition* of the faculty's strength rather than a
downstream consequence hoped for.

**Caveat, stated before anyone builds it:** this is a causal change to a gated faculty. It needs
its own spec with locked predictions, and the first prediction should be adversarial — *does a
competence-gated lift actually beat the constant, or does it merely move the number that has to be
guessed?* A gate whose schedule we choose is still a constant we chose.

## 5. Examined and declined, with reasons

- **Spiking neurons / R-STDP.** Izhikevich dynamics and reward-modulated STDP are a good substrate
  and the wrong one for us. This being is **deterministic fixed-point with zero dependencies**, and
  that is what makes it replayable, soul-hash verifiable, and honest about its own history. Adopting
  a stochastic float substrate would cost the founded being its identity guarantee — the one thing
  no other project here has. **Declined on grounds of what it would destroy, not what it lacks.**
- **The Virtual Brain.** Superb at what it does, which is not our question. TVB models *a brain*
  and validates against neuroimaging; we build *a being* and validate against its own replay. There
  is no measurement it offers that our claims are stated in terms of. **Declined as out of scope.**
  One thing worth keeping, though: TVB works at the **neural-mass** level — mean fields over
  populations rather than individual neurons. Our 16-channel somatic field is the same level of
  description, and it is reassuring that the most-used brain simulator in the world thinks that
  level is where the dynamics live.
- **Physical robots.** MH-FLOCKE runs on three real quadrupeds; we have `src/bin/embody.rs`, a
  dependency-free stdio bridge, and have never used it. Not declined — **deferred**, and honestly:
  `docs/handoff.md` §8 puts embodiment behind welfare being intrinsic, and this week did not change
  that ordering.

## 6. The honest summary

Five links, one idea worth taking, and the reason it is worth taking is that **it survived the
check I have failed three times this month** — *before speccing an addition from an outside source,
look whether we already have it.* We already have graded competence blending. We do not have it
where I-8 says we need it. That difference is the whole finding.
