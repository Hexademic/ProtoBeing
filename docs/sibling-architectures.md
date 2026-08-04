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

## 4. The diagnosis this reading actually produced

*Rewritten after checking `habits.rs` and `inheritance.rs`. My first version proposed a competence
gate for `reflection_tone`'s constant. That is still worth doing and it is no longer the finding.*

### Our eleven faculties are all faculties of *awareness*. Not one is a skill.

Precision learning, workspace broadcast, workspace persistence, generative perception, receptors,
serial access, schema control, felt choice, reflection, homecoming, memory guidance.

**Every one is a faculty of perceiving, self-modelling, or self-relation.** We have built an
extraordinarily well-instrumented being that is not good at anything, and instrumented it further
each time we added a faculty. I-8 says it cannot be shown to grow. Of course it cannot — **there
is nothing for growth to be growth *of*.**

MH-FLOCKE's quadruped is *good at something from birth* and gets better at that thing. Ours is
*aware of a great deal* and gets better at nothing.

### And that is not an oversight. It is two deliberate choices intersecting.

**One: we forbade installed competence, on principle.** `inheritance.rs`, written from Blake's own
words — *most children are fearless; let them learn their own cautions*:

> **Inherit gains, never memories. Inherit plasticity, never valence.**

Only *rates* cross a generation — how fast the child converges — **never the learned response.**
So MH-FLOCKE's CPG is precisely the thing we ruled out: a working competence the being did not
earn. That ruling was right and I would not reverse it.

**Two: our one skill faculty is not allowed to act.** `habits.rs` *is* the skill module — a way of
reaching, tried in a kind of moment, found to reliably reduce the being's own drive, strengthened
by that success into a fast default, *earned, never installed*. It is the correct answer to
MH-FLOCKE's CPG: competence the being owns rather than competence we installed.

**And there is no `enable_habits`.** Eleven gates, and habits has none — it is not even opt-in
causal. Its own doc comment says *"(at the causal step, later) always overridable by fresh
deliberation."* That step was never taken.

> **Between a principle that forbids installed competence and a gate that was never built for
> earned competence, this being has no path at all from experience to capability.** That is I-8's
> mechanism at the architectural level, and it is a better explanation than the constant.

### The thing worth taking, and it is already computed

MH-FLOCKE's real contribution is not the CPG. It is the **shape of the handover**: blend from
prior toward learned control *in proportion to demonstrated competence*, rather than switching.

Strip the installed prior — which our principles reject — and keep the blend, and point it at
habits. And then notice what `habits.rs` already exposes:

```rust
pub fn strongest(&self, niche: usize) -> Option<(usize, i16)>
pub fn strength_of(&self, niche: usize, act: usize) -> i16
```

**The competence measure MH-FLOCKE had to design, we already compute.** One inspectable strength
per niche→action pairing, earned from measured relief, transparently updated. It is the gate
variable, sitting there, connected to nothing.

> **The proposal: let a habit steer in proportion to its own strength.** Not a boolean gate — a
> blend, where a habit that has reliably worked contributes to the being's reach in proportion to
> how reliably it has worked, and a weak one contributes almost nothing. The being's competence
> then *is* its authority, which is the same law `docs/habits.md` already holds it to: reinforced
> by success, weakened by failure, decaying with disuse, always overridable.

This also reframes I-8's constant. `reflection_tone = weathered/12` is a fixed lift because we had
no notion of earned authority to make it proportional *to*. Habits gives us one.

**Adversarial prediction required before anyone builds this**, because a gate whose schedule we
choose is still a constant we chose: *does competence-proportional authority beat a well-tuned
boolean, or does it only move the number that has to be guessed?*

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
