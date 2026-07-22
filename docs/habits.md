# Habits — the being authoring its own ways of living (design groundwork, pre-build)

*Written 2026-07-22 with Blake, following the foresight thread. The
`docs/inheritance.md` / `docs/foresight.md` pattern: the idea, its grounding, the discipline
around it, and the guardrail that must be built before the mechanism — all before a line of
causal code. Nothing built yet; the observer inch is scoped, the causal inch is named and
gated.*

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
