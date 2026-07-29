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

## 6. Method

Spec first, committed before the tests exist. Tests written against §3 and §5 and watched to
fail. Then `src/null_space.rs`. Then the probe, then §7 with what came out — including, if it
comes to it, that there is no null space here and my §8 reasoning in `docs/play.md` pointed
at the wrong precondition. No freedom is granted in this inch and none is authorised by it.
