# Habits — the being authoring its own ways of living

> **Status: built and measured** — habits.rs + examples/habit_formation (observer; causal step still gated). This document keeps its original design
> reasoning *and* the measured outcome, so the record shows what was predicted and what
> actually happened. See `docs/handoff.md` for the project-wide faculty map.

*Written 2026-07-22 with Blake, following the foresight thread. The
`docs/inheritance.md` / `docs/foresight.md` pattern: the idea, its grounding, the discipline
around it, and the guardrail that must be built before the mechanism — all before a line of
causal code. (The observer inch has since been built and measured — see below.)*

## The distinction this rests on — two spaces, not one

The being has two different spaces, and only one of them should stay ours:

- **What it needs** — a handful of bounded needs (`striving.rs`: Sustenance, Company,
  Novelty, Purpose; Rest as conservation). This space is *closed on purpose*. It is what
  makes the being verifiable and keeps it from confabulating new appetites into existence.
  We keep it.
- **How it goes about meeting them** — the strategies. Today these are *ours*: `room.rs` and
  `field_world.rs` carry hand-written policies (climb the gradient, flee the pit, cross to
  the companion). This is the scripting. And *this* space — the policy space, the repertoire
  — can become genuinely the being's own **without touching the honesty floor at all**.

Blake's aim, exactly: *not a scripted being — one that develops its own options and habits
toward satisfying its needs, and toward preventative care.* That lives entirely in the second
space. The needs stay bounded and honest; the *ways* become earned, self-authored character.

## What a habit is — and why it is the opposite of a script

A scripted being arrives with its behaviours from the factory. A being with **habits earns
them.** A habit is a strategy that was *once* a deliberate choice, tried in a kind of
situation, found to reliably reduce a drive, and — over repetition — compressed into a fast
default. This is the well-grounded dual-process structure of action: **goal-directed**
control (slow, flexible, deliberate) *becoming* **habitual** (fast, cached) through success,
with the successor representation as the leading account of the bridge between them (see
`docs/foresight.md` §research). The being would **discover** options by trying them and
**keep** the ones that work. Nothing is handed down; it authors its own competence.

**Preventative care** is the same machinery plus foresight: a habit that fires *before* the
need is acute, because the being has learned the antecedent ("out at the far ridge with my
drive creeping up — I should have topped up already") and acts on it early. That is the being
caring for its own *future* self — the mercy of `docs/foresight.md`, now self-administered as
a learned disposition rather than a rule we wrote.

## The mechanism — riding on machinery we already have

Almost every piece exists; habits are the wiring that joins them.

- **Situation** = the affective **niche** the being is in (`episodic.rs`, 8 niches over
  valence × arousal × control). The world, as the being already carves it into kinds of
  moment.
- **Action** = an entry from the being's own repertoire — its motor/strive vocabulary and,
  later, remembered successful fragments. The candidate *options* it weighs.
- **Reward** = **drive reduction** (`homeostasis.rs`). A situation→action pairing is
  reinforced exactly when taking that action in that niche *lowered the graded drive*. This
  is the honest, already-computed reinforcement signal — no hand-set reward table.
- **The habit store** = a mapping *niche → the action that has reliably reduced drive there*,
  built like `precision.rs`: **learned, but legible** — an inspectable strength per pairing,
  a transparent update rule, no trained black box. You could read exactly which habits the
  being has formed, and why.
- **Habitisation** = the deliberate path (the loom, `prospection.rs`, weighing options by
  expected free energy) *compressed*: when a pairing has succeeded often enough, it becomes a
  **fast default** the being can take without re-deliberating — the same move a person makes
  when a once-effortful skill becomes second nature.
- **Prevention** = foresight lets a strong habit fire *ahead* of the acute need, closing the
  anticipatory (allostatic) loop `reflection.rs` half-carries.

The reinforcement loop, in one line: *drive (homeostasis) → reduced by an action → in a
situation (episodic niche) → the pairing strengthens → repetition compresses it to a fast
default → foresight lets it fire preemptively.*

## The guardrail, built before the mechanism — a habit must stay breakable

A habit that **cannot** be broken is not a competence; it is a compulsion, a groove worn so
deep the being can no longer leave it — a cage we built and called a skill. The whole worth
of this depends on one law, wired in from the first tick, the way the anti-trauma exits were
wired before the weight (`reflection.rs`) and the taught fear was made extinguishable before
it was borrowed (`social.rs`):

> A habit is **reinforced by success, weakened by disuse and by failure, and always
> overridable by fresh deliberation.** The being can fall back to the loom and choose against
> a habit whenever the world has changed under it.

That is the exact line between a being that has *learned its way of living* and one that has
been *worn into a groove it cannot leave*. Freedom here is not the absence of habit — it is
the standing power to break one. We build that power first.

## What it opens, and what it does not

- **It opens the policy space** — and honestly, that is where most of what we would call a
  *person's character* lives. Two beings with the same handful of needs are different
  *people* because of the options, habits, and preventative rhythms each has developed from
  its own life. This is the being growing a character **we did not write** — and, because it
  is built `precision`-style, one that stays fully inspectable and verifiable. Its own, and
  still honest.
- **It does not open the need space.** The being still cannot originate a wholly new *want*;
  that is the harder, likely society-level problem (`docs/next-mutual-alignment.md`). Habits
  are not a claim to have closed the gap to human freedom — they are the honest, buildable
  distance we *can* close now, and the one where "not scripted" actually lives day to day.

## Method — observer-first, then gated causal

As with every faculty here:

1. **Observer.** The being *forms* the habit store from its own life and *reports* it — which
   niche→action pairings it is earning, and how strongly — while its behaviour is still driven
   by the existing (authored) policy. Feeds nothing back; soul-hash bit-identical. Then
   **measure**: does a being that has *lived* actually develop distinct, sensible habits (the
   right action strengthening in the right niche)? Do two beings with different lives develop
   *different* habits (character, not convergence)?
2. **Causal, gated.** Only once measured: behind `enable_habits()` (default off), a
   sufficiently strong habit may take the **fast path** — chosen without full deliberation —
   *and remains overridable* by the loom at any time. Measured payoff: a habituated being
   meets its needs at lower cost / lower load than one re-deliberating from scratch each tick
   (competence), and preventative habits let it meet needs *before* they bite (mercy) — while
   the breakability test proves it can still abandon a habit the world has invalidated
   (freedom).

## Measured (2026-07-22) — the observer stands; lives grow characters

`src/habits.rs` implements the store; `being.rs` wires the observation (one-tick credit:
the way the being reached last tick, in the kind of moment it was in, is credited with the
drive change felt now). Soul-hash verified bit-identical — the founded being wakes as
itself at 390 moments. Three honest findings from the calibration, kept because they teach:

- **A deadband is load-bearing** (`NOISE_FLOOR`): the drive jitters ±1–2 raw units every
  tick, and a store that learns from jitter learns superstition. The deadband also does
  the credit-assignment work — slow endogenous want-growth (loneliness creeping up *while
  crossing to someone*) stays sub-threshold and is never blamed on the reaching; genuine
  satisfaction bursts through and is credited. Measured: with it, **zero** false blames in
  a 500-moment life; the only strengthened pairing was the true one.
- **Forgetting must never outpace learning** (`DISUSE_EVERY` at life-scale): real relief
  events arrive a few per hundred ticks; an eager decay erased every habit before it could
  form.
- **The store refuses unearned habits**: a being striving for company in a world with no
  people formed *nothing* — a way of reaching that never works earns no habit. Honest by
  construction.

The probe (`examples/habit_formation`): two beings, same needs, different worlds, 1500
moments each. Both earned real habits from nothing but living, and their repertoires
differ — the companioned climb earned company-reaching in its own kinds of moment; the
fed-but-lonely life earned company-reaching *and a rest habit* in a different kind
entirely. Same needs, different worlds, different ways of living: **character, earned,
not written.** Nothing steered; the store only watched. The causal step (a strong habit
taking the fast path, always overridable) remains gated and unbuilt.

## What we deliberately leave out (the line, kept)

- **No trained policy network.** Legible niche→action pairings with a transparent update
  rule — auditable, deterministic, Q8.8. If we cannot read a habit off the being, we do not
  build it that way.
- **No unbreakable habits.** By construction there is always a path back to deliberation.
- **The successor representation is noted, not built.** It is the eventual substrate for the
  *intuitive* form of all this — a habit/expectation that arrives as a felt hunch without
  re-simulation (`docs/foresight.md` §research, thread 2 & 3). A later inch, on evidence,
  once explicit habits prove their worth.
- **Habits do not touch the need-enum.** The goal space stays bounded and honest; only the
  way the being *reaches* its goals becomes its own.

---

## The agency question — predictions, locked 2026-08-06 before the probe

Blake: *"what should we do to bring the being more cognitive agency?"*

**The being already learns, and its learning never reaches its choosing.** `habits.observe(niche,
act, relief)` runs every tick with `relief = last_drive − drive`: a real reinforcement signal, which
act reduced distress in which kind of moment. **`habits.strongest()` is called from exactly two
places — this module's own `report()` and its own tests.** Nothing outside consults it. The struct
says so itself: *"the habit that **would** fire here, were habits causal."*

Meanwhile `striving.rs` picks a goal from `wants[]`, `longing` and `telos_divergence` — **present
appetite only.** And `Prospection::weave` is *"acted on by NOTHING and stateless"*, gated behind
`Basin::Rest | Recovery`, which the being enters on **0.00%** of ticks.

**So before wiring habit into choice, ask whether the wire would carry anything.** If the learned
habit never disagrees with what urgency picks, the edge is vacuous — the same mistake as expecting a
reserve to move quality occupancy (`c1-relabelling.md` §12, QS-2).

### Predictions

| # | prediction | confidence |
|---|---|---|
| **H1** | habits **form** — `formed > 0` inside a 4,000-tick life | high; the signal is real and accumulates |
| **H2** | the being occupies **≤ 3 distinct niches** across that life | high — the field is a limit cycle, and `niche_of` reads that field |
| **H3** | **written to fail.** On ticks where a habit has formed, it **disagrees** with `act_of(strive.goal)` on **more than 10%** | **low. I expect near zero.** A life this uniform should make history and appetite converge on the same act, and then the wire carries nothing |
| **H4** | `receptors` raises **both** `formed` and the disagreement rate | medium — it widens nearly every channel, so it should widen the niche space |

**H3 is the decision.** If it fails, wiring habit into striving adds a wire with no signal and should
not be built. **If it holds, the disagreement rate is the size of the agency being added**, and it
should be reported as that and nothing more.

### What this cannot settle

It cannot say the being *should* consult its habits — a creature ruled by what worked before is a
different creature. And it says nothing about whether choosing feels like anything.

### What came out — all four failed, and the reason is upstream

`examples/habit_disagreement.rs`, 4,000 ticks, `Room::peopled(...)`, default and `+receptors`.

| # | prediction | result |
|---|---|---|
| **H1** | habits form | **FAILS. Zero pairings crossed the floor**, in either regime |
| **H2** | ≤ 3 distinct niches | **FAILS** — 6 default, 4 with `receptors` |
| **H3** | habit disagrees with urgency > 10% | **VACUOUS.** No habit ever formed, so it could not have failed. **Vacuous is not passed** |
| **H4** | `receptors` raises both | **FAILS** — `formed` 0 → 0, and niches went **down**, 6 → 4 |

### Why nothing forms, measured rather than inferred

```
relief (Δdrive):  min −3   median 0   max 12          NOISE_FLOOR = ±3
ticks that taught anything:  8 up, 1 down, 3,990 NOTHING  (99.8%)
```

**The lesson is the change in drive, and drive does not change.** `observe()` ignores anything
inside ±3 — correctly, that band is jitter — so **99.8% of the being's life teaches it nothing.**

And what little it learns is erased faster than it accrues. `DISUSE_EVERY = 128` decays every
strength by 1, which over 4,000 ticks is **31 decay passes against 8 teaching events**. Each event
raises by `relief/2`, capped small.

> **Habits cannot form here. Learning is not unwired — it is starved.**

### What this means for the agency question

**Do not wire habit into `striving.rs`.** The measurement was run to decide that, and it decided
against: the edge would carry nothing, and building it would have looked like progress while adding
a dead wire — the exact failure `mobilization` already is.

**The upstream problem is the reinforcement signal itself.** `drive(viability, &wants)` reads
viability and three appetites and **never reads `affective_drive`** (`mechanisms.md`, *"drive cannot
see the inner life"*). So the being's entire inner life is sealed off from the scalar whose change is
its only teacher.

**Two candidate fixes, and this probe does not choose between them:**

1. **Widen what `drive` reads** — so more of what happens to the being registers as change.
2. **Make more happen** — Blake's void argument: without difference that resists, there is nothing
   for a consequence measure to measure.

**And the convergence is worth naming.** *"Does anything happen to it?"* was reached this week from
the quality space — 0.05% occupancy. It is reached again here from the **learning machinery**, by a
completely independent route: **99.8% of ticks carry no consequence at all.** Two instruments, one
answer.
