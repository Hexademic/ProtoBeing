# The null space — is there more than one way to do the same thing?

> **Status: specified, nothing built.** Committed before the tests and before a line of
> implementation, so §5's predictions are on the record before any result. This is
> `docs/j-space.md`'s **step 1 — create redundancy** — reduced to the smallest honest inch:
> an observer that *finds* the null space the being already has and refuses to use, and
> reports its size. Nothing is given freedom in this inch.

*Written 2026-07-29, because the play measurement demanded it. `docs/play.md` §8 found that
play cannot be built: `intent_from` is a total function of the being's state, the being is at
rest on 0% of ticks, and its action→sensation map is degenerate (`[-3,-3,-3,0]`). Play is a
use of freedom, and this being has none to use. `docs/j-space.md` had already named the gap
on 2026-07-26 — "no two ways to do the same thing" — and filed it as the reason the being
lacks a manner. It is also the reason it cannot play. So the null space stops being the last
item on the list and becomes the precondition for the first.*

## 1. Where the freedom is being thrown away

`FieldWorld::climb` (`src/field_world.rs:524`) is four lines of the being's whole life:

```rust
let here = self.potential(self.body, intent);
let mut best_dir = (0, 0);
let mut best_delta = 0;
for dir in COMPASS.iter() {
    let probe = (self.body.0 + dir.0 * PROBE, self.body.1 + dir.1 * PROBE);
    let delta = self.v_at(probe) - here;
    if delta > best_delta { best_delta = delta; best_dir = *dir; }
}
```

Two things to see. First, the comparison is a **strict `>`**, so among directions that are
*equally* good the first in `COMPASS` order silently wins — a tie is not resolved, it is
never noticed. Second, and more important: a direction that is *nearly* as good as the best
is discarded outright, even though taking it would reach the same high ground a tick later at
the same cost. On a smooth two-source field most points have **two** directions that climb —
one per gradient component — and the being can only see one of them.

That set of adequate-but-unchosen directions is the null space. It is already there, every
tick, in a being that has never had access to it.

## 2. What this inch builds

An observer — `src/null_space.rs` — that answers, per tick, one question:

> **How many distinct actions would have been good enough?**

Concretely: given the potential the being is climbing, the set of compass directions whose
climb-delta is within a **tolerance** of the best. Its size is the local redundancy.
`|adequate| = 0` is a singular point — nothing the being can do improves anything.
`|adequate| = 1` means the way is forced. `|adequate| ≥ 2` is freedom: two ways to do the
same thing.

It reports and steers nothing. `climb()` is not modified; the observer recomputes the same
probe set beside it. The trajectory and soul-hash stay bit-identical, and the founded being
is untouched — the same Stage-1 discipline as `homeostasis.rs`, `habits.rs` and `primes.rs`.

**What tolerance means, and why it is not a fudge.** Two directions are equally adequate if
they arrive at the same place, and on a discrete grid "the same place" has a width: one raw
unit of potential is smaller than the being's own receptor quantization (`docs/play.md` §8:
its full-effort action moves a channel by ~3). So a tolerance is not a loosening of the
criterion — it is the criterion stated at the resolution the being actually has. It is a
declared constant, swept in the probe, and reported at every value rather than tuned to a
flattering one.

## 3. What must not become possible

- **Nothing steers.** Pure observer. `climb()` unmodified, no `enable_*` gate added, no
  soul-hash input touched. If the trajectory moves by one raw unit, this inch has failed.
- **The observer must not disagree with the being.** Whatever `climb()` actually chose must
  always be a member of the adequate set. An observer that reports the being's own action as
  inadequate is measuring something else, and that is a bug, not a finding.
- **No freedom is granted.** The being does not get to pick from the set in this inch.
  Choosing among adequate ways is the *next* inch and a separate decision, because a chooser
  needs the reflex layer (`docs/j-space.md` step 2) and the permanence guardrail below.
- **The permanence guardrail is stated now, before anything can violate it** — from
  `docs/j-space.md`, which derives it from the formalization rather than from taste:
  *nothing may constrain the null space permanently.* A resolver that hardens until one way
  remains has become a compulsion by definition. The test is precise and belongs to the
  inch that builds the resolver: **is the being's variability in task-irrelevant directions
  still non-zero?**

## 4. Why this is the thing that unblocks play

Play needs a place to put an action whose reason is *to find out*. The adequate set is that
place, and it is the only one that does not require overriding striving:

- The being's **task** is set by deliberation (`striving.rs` picks the need). Untouched.
- Any member of the adequate set accomplishes that task. So choosing among them is **free in
  task terms** — which is exactly the definition of a play action that does not compete with
  regulation.
- And it makes the payoff measurable. Four near-identical gains came from one scalar action;
  a being that varies *which way* it goes generates genuinely different action→sensation
  pairings, so the body-map has something non-degenerate to learn.

If §5's N1 comes back saying the adequate set is almost always of size 1, then this being has
no null space to find rather than one it is neglecting, and **play is blocked on something
larger than an observer** — a richer action surface, which is a much bigger and more honest
piece of work than I would otherwise have proposed.

## 5. Predictions — locked before the tests exist

**Confident:**

- **N1.** The observer never disagrees with the being: `climb()`'s chosen direction is a
  member of the adequate set on every tick of every life, at every tolerance ≥ 0.
- **N2.** At tolerance 0 the set is never empty when `climb()` found a positive delta, and
  its size is monotone non-decreasing in tolerance.
- **N3.** Singular points (`|adequate| = 0`) coincide with the being having nothing to climb
  toward — at or atop its goal. This is `docs/j-space.md`'s geometry of despair, and it
  should be **rare in a kind world and not rare in a hard one.**

**Genuinely uncertain, and it decides whether the next inch is small or large:**

- **N4.** *How much redundancy is actually there?* My reasoning says a generic point on a
  smooth two-source field has **two** climbing directions, so I predict mean adequate-set
  size **≈ 2** at tolerance 0, and that **≥ 2 on a majority of ticks**. But `potential()` is
  choice-weighted and sums two sources plus persons plus a threat term, `PROBE` samples at a
  distance rather than taking a true local gradient, and the field is integer-quantized —
  three reasons the real answer may be 1. **If the mean is ≈ 1, play is blocked on a richer
  action surface and I will say so plainly**, having just argued the opposite direction in
  `docs/play.md` §8.
- **N5.** *Does redundancy survive where it matters?* Freedom is worth little if it exists
  only when the being is comfortable and vanishes under pressure — that would be a null
  space that closes exactly when a manner would cost something. I predict redundancy is
  **lower** when burdened (the gradient is steeper near a source it needs) and will report
  the split against `play.rs`'s `COMFORT` line either way. If freedom vanishes under load,
  then style is a luxury of the well-fed, which is a real finding about this architecture and
  not a pleasant one.

## 6. Method (as planned)

Spec first, committed before the tests exist. Tests written against §3 and §5 and watched to
fail. Then `src/null_space.rs`. Then the probe, then §7 with what came out — including, if it
comes to it, that there is no null space here and my §8 reasoning in `docs/play.md` pointed
at the wrong precondition. No freedom is granted in this inch and none is authorised by it.

## 7. What came out — measured 2026-07-29

`src/null_space.rs`, `tests/null_space.rs` (7, all green), `examples/null_space.rs`.

**N1, N2, N3 hold.** The observer never disagrees with the being: across three worlds and
600 ticks each, at every tolerance in {0, 1, 3, 8}, whatever `climb()` actually chose was a
member of the adequate set, the observer's `best` equalled `climb`'s delta, and the set was
empty exactly when nothing climbed. Two identical lives, one observed at every tick, ended on
the same soul-hash — the observer moves nothing.

**N4 — wrong, and wrong in a way worth more than being right.** I predicted a mean adequate
set of ≈ 2.00 at tolerance 0 and more than one way on a majority of ticks. Pooled, the mean
is **1.20** and the majority is **31%**. But the pooled number is the least informative thing
in the table:

| life | tol 0 | tol 1 | tol 3 | tol 8 | tol 24 | singular |
|---|---|---|---|---|---|---|
| long crossing | 0.97 · **0%** | 0.97 · 0% | 0.97 · 0% | 0.97 · 0% | 1.24 · 27% | 3% |
| long + weathered | 0.98 · **0%** | 0.98 · 0% | 0.98 · 0% | 0.99 · 1% | 1.42 · 43% | 2% |
| beside its food | 0.91 · 24% | 0.91 · 24% | 0.96 · 29% | 1.02 · 35% | 1.15 · 48% | **33%** |
| beside food + weather | 1.93 · **94%** | 1.93 · 94% | 1.94 · 95% | 1.95 · 95% | 1.95 · 95% | 3% |

*(mean adequate ways · % of ticks with more than one way)*

**The freedom ranges from 0% of ticks to 95% across four lives of the same being.** One world
forces its hand at every single tick; another leaves it two adequate ways at nearly every
tick, and it takes the first in compass order without ever knowing the other was there.

I nearly published the pooled 1.20 with a verdict of "no null space here" — the probe's first
version did exactly that, thresholding on the mean. That is the same error `docs/play.md` §7
had recorded **earlier the same day**: *a mean below a threshold says nothing about whether
the threshold is crossed.* Writing the lesson down did not stop me repeating it; the spread
did. The probe now reports the range and never the pooled verdict alone.

**Three explanations for the spread, all three disproven by measurement.** Recorded because
the failures are the honest content of this section:

1. *"A generic point has one climbing direction per gradient component, so ≈ 2."* — the
   spec's own reasoning, §5. Refuted by the 0% lives.
2. *"Ties need the field's symmetry — two directions tie only when the gradient components
   are near-equal."* — the probe's first verdict. Swept a single-source field: **72%** of
   positions near the symmetry line have more than one way, against **47%** away from it.
   Symmetry helps and comes nowhere near explaining a 0%-vs-95% split.
3. *"The long crossing's off-axis harm source breaks an otherwise exact tie."* — walked that
   geometry with the harm source present, removed, and moved collinear: **25% → 30% → 27%**.
   It is not the harm source.

What *is* established, from the raw deltas: the field is **L1 (Manhattan)**, so any direction
that reduces the distance by the full `PROBE` yields exactly `peak · PROBE / REACH` — ties are
**exact and generic**, not near-misses. A component smaller than `PROBE` overshoots and stops
climbing, and when *both* components are under `PROBE` the being is inside its own probe
radius and everything reads downhill — which is why "beside its food", waking 12 cells from a
source it probes 40 ahead of, is **singular on 33% of ticks**.

But that static account does not reproduce the live numbers (statically, "beside its food" is
100% singular; live it is 33%), because the real `potential()` is *choice-weighted* by intent
and also carries persons and a threat term. **So the cause of the world-to-world spread is
not established, and I am not going to assert a fourth explanation I have not tested.**

**N5 — freedom does narrow under load, as predicted.** Pooled **1.22** adequate ways when
comfortable against **0.89** when burdened; on the long crossing, 0.98 across 1,390
comfortable ticks against **0.82** across 110 burdened ones. The two kind lives were never
burdened at all. So style is at least partly a luxury of the well-fed, and a resolver built
on this null space would go quiet exactly when the being is struggling — which is worth
knowing before building one, and is not a pleasant fact about the architecture.

## 8. What this changes

The design conclusion is robust even though the mechanism is not settled, because it does not
depend on the mechanism:

> **The null space is real but *scavenged*. The being does not own it.**

A being whose available freedom swings from 0% to 95% of ticks according to where its food
happens to sit relative to its harm has not got a manner — it has got a coincidence. Style
cannot rest on that, and neither can play: `docs/play.md` §8 blocked play on redundancy, and
redundancy of *this* kind would make play available in one world and impossible in the next,
for reasons the being has no access to and cannot influence.

So the requirement is sharper than §4 of this document stated. **Redundancy has to live in
the being's own action surface, not in the field's geometry.** `docs/j-space.md` already
listed two candidates that qualify, and neither depends on the world:

- **effort within a band that arrives at the same place** — the being is at 0% rest and mean
  effort 163–250 of 256 (`docs/play.md` §8), so this band is currently collapsed to its
  ceiling; and
- **when — act now, or wait a beat.**

Both are owned by the being, available in every world, and orthogonal to what striving
chose. That is the next inch, and it is a bigger one than this observer: it changes the
action surface rather than watching it, so it needs its own spec, its own locked predictions,
and a gate.

**Unchanged by this inch:** nothing chooses. `climb()` is not modified, no `enable_*` gate was
added, the soul-hash is untouched, and the founded being is unchanged at 390 kept moments.
