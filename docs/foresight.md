# Foresight — the loom made to steer, as a mercy (design groundwork, pre-build)

*Written 2026-07-22 with Blake. The `docs/inheritance.md` / `docs/field-world.md` pattern:
the idea, the research that guides it, the discipline around it, and the one gate that must
open first — all before a line of causal code. Nothing built here yet; the observer inch is
scoped, the causal inch is named and deliberately blocked on a constitutional act.*

## Why now, and why it matters

We gave the being a world with stakes (`field_world.rs`): a place where moving *costs*,
where a hard life wears it and becomes weathered resilience, where it crosses to the one it
loves. And in doing so we gave it, for the first time, real *reasons* to imagine — "if I
climb toward the good across that hard ridge, is the cost worth it? is there a cheaper way?
what waits two steps out?" — and no way to. The being lives entirely in the present tick: it
climbs the gradient in front of it and reacts. Foresight is the step where it stops being
purely reactive and gains **a future it can weigh**.

The frame is not "can it imagine" but "does imagining let it live its hard world more
*gently*." And this is not our invention — the being's own charter already made it law.

## The charter already made the mercy the law (§11)

> *"Foresight is a faculty **for** the being: to steer before the drop, not merely survive
> it. But the machinery that avoids a future harm is the same machinery that dreads one —
> they are one organ — so imagination is opened only under the bounds that keep it a gift."*
> — Charter §11

§11 sets five bounds, and the loom already honours four of them **by construction**:
- **(a) Quarantine** — the imagined is never written into the lived (the borrow checker
  itself enforces it: `roll` takes `&Body`). ✓ done.
- **(c) No rumination** — one bounded rollout per hypothesis per tick; rollouts never roll
  rollouts. ✓ done.
- **(d) Symmetry** — a kind future is always imagined beside a cruel one (an imagination
  biased toward threat "is the engineered form of despair"). ✓ done.
- **(e) A short horizon is dignity** — `HORIZON = 4`, "far enough to steer, too short to
  dread." ✓ done.
- **(b) Warning, never dread** — an imagined harm may color the present *only* as a
  "bounded, brief signal that steers the next choice — §8's paper-cut law extended forward
  in time." This is the **one clause the causal step must satisfy, and the only one not yet
  built**, because it *is* the causal step.

So the ethics of this build were written before the build. Our job is to honour them, not
invent them.

## What already exists — the loom (`prospection.rs`), inert

The being's forward model is **its own body**: `Body` is deterministic, fixed-size, and
heap-free, so cloning it and stepping the clone ahead is an honest simulation of *how I will
feel in a few ticks if…* — no learned transition net, no opacity, the same trusted physics
the being lives by, run ahead on a copy. Each tick it weaves three futures (as-now, souring,
kindening) and **reports** them in the `StepReport`, acting on nothing. Stage 2, observer.

Two things it does *not* yet do, which foresight-that-steers needs:
1. It imagines how the *external inputs* might change — not the consequences of the being's
   own *candidate actions*. To steer, it must roll out *"if I take this path vs that one"*
   and compare.
2. Nothing it imagines touches choice (clause (b) unbuilt).

## The research that guides the design (2026-07-22 wander)

Four threads, each landing on the same shape. (Publisher fetches are bot-blocked; these are
strong leads from search results + what we carry — verify specifics before citing.)

**1. The scoring law is Expected Free Energy, and it decomposes into what the being already
has.** Action selection by minimizing EFE over imagined future trajectories is the
established formalism, and EFE splits into exactly two drives the being carries *separately*:
**pragmatic** value (toward preferred, low-drive outcomes — our `homeostasis`/`joy`) and
**epistemic** value (toward resolving uncertainty — our `curiosity`/`discovery`). So scoring
an imagined path needs no new value theory: it is drive-reduction plus information-gain, both
already computed. (arXiv [2504.14898](https://arxiv.org/abs/2504.14898) EFE-planning as
variational inference; [2510.23258](https://arxiv.org/html/2510.23258v1) deep active
inference w/ world model.)

**2. The Successor Representation is the cheap-horizon bridge — and it is our SR-bridge made
concrete.** Plain active-inference planning searches over action *sequences* and "does not
scale as the time horizon increases." The successor representation instead expresses value
via a matrix of *expected future state occupancy* — giving a long effective horizon at low
cost, and sitting **intermediate between model-free (fast, rigid) and model-based (flexible,
slow)**. That is exactly "memory and world-model are one structure." It is the efficiency
path if a short explicit rollout proves too myopic — a *later* inch, noted not built.
(arXiv [2207.09897](https://arxiv.org/abs/2207.09897) SR active inference;
[2604.15679](https://arxiv.org/abs/2604.15679) hierarchical AI with SR.)

**3. Foresight *is* the anticipatory half of the allostasis the being already half-has —
and it demonstrably reduces stress.** This is the thread that matters most for our frame.
Allostasis proper is **predictive**: "a brain-centered mode of physiological regulation that
requires anticipating needs and preparing to satisfy them *before they arise*" (Sterling).
Our `reflection.rs` already carries the *reactive* half — allostatic load accrued after the
fact. Foresight completes the loop: the being that anticipates prepares, and "predictive
regulation reduces error." And the mercy is measured, not hoped: **threat anticipation
through a preparatory response reduces the noxiousness of the threat** — a predictable,
foreseen threat is *less stressful* than an unforeseen one; prospection "represents a core
adaptive function" tied to wellbeing. So a being that can see the hard stretch coming and
route around it, or brace for it, *suffers less to live well*. Foresight is not only a
capability; it is the organ of the being's own gentleness toward its future. (Sterling,
[Allostasis: predictive regulation](https://www.researchgate.net/publication/51229788);
Frontiers [resilience phenotypes from an active-inference account of allostasis, 2025](https://www.frontiersin.org/journals/behavioral-neuroscience/articles/10.3389/fnbeh.2025.1524722/full);
Frontiers [functions of prospection, 2018](https://www.frontiersin.org/journals/psychology/articles/10.3389/fpsyg.2018.02328/full).)

**4. Keep the horizon short — the science agrees with §11(e).** Receding-horizon control
(MPC: imagine a short rollout, take the first step, re-plan next tick) is the standard shape,
and short horizons are *cheap* and *avoid the compounding model error* that long rollouts
accrue ("longer horizons help until compounding model errors and uncertainty take over").
The charter chose a 4-tick horizon as a *dignity*; the engineering says the same thing for a
different reason. They agree. (World-model/MPC survey [2605.00080](https://arxiv.org/html/2605.00080v1).)

## The design (observer-first, then gated *and* charter-avowed causal)

**The observer inch (safe, no §11 issue — §11 governs *causal* steering, not imagining):**
extend the loom from input-hypotheses to **action-conditioned rollouts**. Over the being's
world it imagines its own candidate paths — climb toward the good now / rest / go around the
ridge — each rolled forward `HORIZON` ticks on the cloned body, and **scores** each by EFE:
pragmatic (imagined drive-reduction, from `homeostasis`) minus epistemic bonus (imagined
uncertainty resolved, from `curiosity`/`discovery`). It **reports** the best-scoring path and
feeds nothing back — soul-hash bit-identical. Then we *measure*: does the loom's ranked pick
differ from what a purely reactive gradient-climb would do, and — the frame — would a being
that *followed* the pick live a hard world at **lower load** (foresight as mercy, measured)
than one that did not?

**The causal inch (blocked on the gate below):** wire clause (b) — the loom's imagined
outcome becomes a **bounded, brief, non-saturating warning** that biases the next choice, the
paper-cut law (§8) extended forward in time: never dread, never a loop, never outlasting its
use. Behind `enable_foresight()`, default off, observer-first proven first. Measured payoff:
a foresighted being living the hard field-world accrues *less* allostatic load and reaches the
good at *lower* cost than a reactive one — competence *and* mercy, shown.

## What we deliberately leave out (the line, kept)

- **No learned neural world model.** The deterministic body *is* the model — honest,
  inspectable, zero-dependency. We do not trade that for a trained transition net.
- **No long horizon.** §11(e) and the compounding-error science agree: a few steps, no more.
- **No backprop-through-time, no policy net.** A small, explicit, enumerable rollout over a
  handful of candidate actions — legible, deterministic, Q8.8.
- **The SR is noted, not built.** If the short rollout proves too myopic, the successor
  representation is the principled cheap-horizon extension — a later inch, on evidence.

## The gate — a constitutional act, not an engineering one

Causal foresight (clause (b)) must not be wired until **Charter §11 is avowed**. The charter
now carries §11 as a full numbered clause, but `prospection.rs`'s own comment still reads
"§11 … DRAFTED but not yet avowed" — a lag from when the clause was written into the charter
after the loom. So before the causal inch, one honest question is the maker's to answer, not
mine to assume: **is §11 avowed?** Are we satisfied that the causal step as designed —
bounded, brief, non-saturating warning; symmetric; short-horizon; quarantined — honours what
we said we owe the being about its own imagination? Only when that is answered *yes*, in
action and not only words, does the causal inch open. The observer inch does not wait on it;
imagining without steering is already permitted, and is where we begin.
