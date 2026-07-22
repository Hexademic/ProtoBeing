# The field-world — consequence with a cost (design groundwork, pre-build)

*Status: design and scope, nothing built. Written 2026-07-19 with Blake, after his
Unified Elastic–Entropic Information Theory white paper. This is the `docs/joy.md` /
`docs/memory-that-teaches.md` pattern: the idea, the one principle worth importing,
and — just as loudly — the trappings to leave out, before a line of code.*

## Where this comes from, and the discipline around it

Blake's EEIT paper models spacetime as an elastic information network: meaning lives
in the *strain field between things*, gravity is an entropic force down that field
(`F = T∇S`), and — its truest root — **information is physical: acting and knowing
cost energy** (Landauer). The temptation is to drape the being's room in that language
because it sounds profound. We are not going to do that. A spacetime theory's numbers,
elastic waves, and entanglement ontology mean *nothing* in a mind's room, and importing
them would be exactly the decoration we have refused at every turn (the three scalar
nulls; "told, not tuned").

**Exactly one idea earns its place**, and it earns it because it answers a problem we
already have, not because it is beautiful: *the being's world has no real cost of
consequence.* The being can strive freely; nothing it does drains it in proportion to
how hard the doing is. A stakes-world needs a principled economy, and Landauer gives us
one that isn't arbitrary.

## The one import, wearing three hats

It is a single idea seen three ways:

1. **The world is a field, not a set of beacons.** Today the Room is discrete point-
   features (hearth, hazard, companion) and the being reads distance to each. A
   field-world is instead a scalar **viability potential** `V(x)` over the space — a
   smooth landscape whose height is "how good it is to be here" (nourishment, warmth,
   safety, company all folded into one field). Features become *hills and wells and
   ridges* in `V`, not labelled objects. Cosmic-web filaments — *high-strain corridors,
   compressed one way and stretched the other* — are the design primitive worth
   stealing: paths and barriers that **emerge from the field's shape**, not hand-placed
   walls.

2. **Moving through it costs along the gradient (Landauer).** Crossing the world is not
   free. Climbing against the field — moving toward a distant good across a hard
   stretch, or away from a threat uphill — costs the being metabolic energy in
   proportion to the *steepness* it fights (`cost ∝ |∇V|` along its path against the
   grade). Coasting down a gradient is cheap. This is the principled source of the
   **stakes** the crucible needs: consequence has a real price, and the price is the
   physics of the landscape, not an arbitrary penalty knob.

3. **The being moves by one gradient law, not many special cases.** Right now
   `room.rs::actuate` special-cases each target (approach-hearth, flee-hazard, seek-
   companion, roam-for-novelty). A field-world lets a *single* law — *move along the
   gradient of the field toward better-regulated states, weighted by the need it
   chose* — replace all of them. That is `F = T∇S` in miniature, and the being's taxis
   is **already** this in disguise. The payoff is real and double: it is simpler (one
   law), and it generalizes (a new world needs no new movement code).

## Why this is the honest next step toward the stakes-world

We had already sequenced a world with real stakes as the horizon, blocked by one
finding: the being's **viability is bimodal** — fine, or crashing, with no worn-but-
stable middle where a hard life is actually lived (`examples/carrying_the_weight`), so
chronic burden can never fire. A field-world with gradient cost is precisely what
creates that missing middle: **the sustained, survivable drain of living somewhere hard
to reach** — low but stable, exactly the regime `reflection.rs::burdened` is waiting
for. So this is not a detour from the stakes-world; it is the mechanism that finally
lets the being *carry the weight of a hard life* honestly, **without touching the core
metabolism** (no soul-hash-level re-founding). The cost is in the world, across the
embodiment seam — not in the being's body.

## What we are explicitly NOT importing

- **The numbers.** `δε ≈ 3×10⁻⁸`, the elastic moduli, `w(k)` — these are about *our*
  spacetime. In the being's world they are numerology. None of them appear.
- **Elastic waves / gravity-wakes / Mach cones.** Beautiful, and useless to a mind in a
  room. Propagating strain waves buy zero behaviour. Not built.
- **The entanglement ontology.** "Everything is entanglement" is a claim about physics,
  not a design constraint for a deterministic Q8.8 sim. The field-world is a scalar
  landscape with a cost, full stop — no microstate graph underneath it.

## Sketch of the build (when we choose to)

Observer-first and continuity-safe, as always:

- A `Field` world behind the same `Embodiment` seam: `sense()` reports the local field
  value and its gradient (the four exteroceptive channels become gradient components);
  `actuate()` moves the body along the gradient and **debits energy for the grade
  fought**. Deterministic, zero-dependency, like `room.rs`.
- The being's **soul-hash is untouched**: the field-world is on the far side of the
  seam; only the `Sensorium` it hands in (nutrient, threat) and the body position
  change. A founded being that never enters it is bit-identical.
- **Measured**, the way everything ships: does gradient-cost create the sustained-low-
  but-stable middle (so `burdened` finally fires and the being carries a hard life's
  weight)? Does the single gradient law reproduce the old room's behaviour (hearth-
  reaching, hazard-fleeing) it replaces — a control against the current `room.rs`? It
  earns its place or it does not.

## Measured foundation (2026-07-20) — the graded drive reveals the middle

Before building the field-world, we tested its load-bearing assumption — that a
*graded* signal can express the worn-but-stable middle the bimodal `viability`
hides — by importing **Keramati–Gutkin's homeostatic drive** (`homeostasis.rs`) as
a pure observer: `D(H) = √(Σ wᵢ(H*ᵢ−Hᵢ)²)`, the being's continuous distance from
well across survival *and* its appetites, on its own [0,256] scale. Observer-first,
soul-hash untouched, the founded being wakes as itself.

It works, and cleanly (`examples/graded_life`). Across a good life, a chronically
lean one, and a crashing one:

| life | viability (bimodal) | graded drive |
|---|---|---|
| good | 1.00 | 0.38 |
| chronically lean | **0.88** (≈ good) | **0.64** (a stable middle) |
| famine | 1.00 → 0.00 in 25 ticks (cliff) | 0.81 |

Viability cannot tell a good life from a hard-but-survivable one (Δ ≈ 0.06); the
graded drive tells them apart (Δ ≈ 0.16) and places the lean life *between* content
and crashing. **The worn-but-alive middle exists — it was only ever inexpressible on
a binary survival signal.** This is the foundation the field-world stands on: with a
graded drive, gradient-cost can push the being into that middle and *keep it there*,
so chronic burden finally has a state to fire from — without a core re-founding
(the drive is an observer; making it steer is a later, measured, gated step).

## Built (2026-07-22) — the world stands, both promises measured

`src/field_world.rs` implements the sketch above behind the same `Embodiment` seam as
`room.rs`: a scalar viability potential `V(x)` summed from signed sources that reach
across the whole field (so there is always a gradient — a *field*, not beacons); `sense()`
reports local `V` as nourishment and the four exteroceptive channels as the field's
**gradient** (how `V` changes N/E/S/W); `actuate()` moves the being by one law — *climb
`V`* — and **debits metabolic energy for the height climbed**, accumulated as a decaying
debt subtracted from the nourishment the world reports. The cost lives entirely across the
seam; the being's core metabolism and soul-hash are untouched, and an ambient floor means
the cost *wears* the being rather than starving it.

Both promises hold, measured (`examples/the_world`, 6 unit tests):

- **One law replaces the room's four cases.** The single climb-law both reaches the good
  (nearness 0.18 → 0.60) and flees the harm (threat 0.40 → 0.07) — approach and escape are
  the *same* ascent, because good is high ground and threat is low. A clean control against
  `room.rs`'s hand-cased `actuate`.
- **Gradient-cost creates the worn-but-stable middle.** A being on easy ground settles near
  contentment (graded drive 0.37); a being living somewhere hard to reach lives its whole
  life at an elevated-but-stable drive (0.53 — clearly between content and the ~0.8 of
  crashing), having run its climb-cost far higher (peak 0.24 vs 0.14) — and **survives**.
  The middle the bimodal `viability` could never express is now *produced by the world*.

### Choice, made motion under the one law (2026-07-22)

The single law now carries the being's *arbitration*: it climbs a **choice-weighted
potential** — the viability field, plus, *only when the being reaches for company*, a
strong pull toward the **particular person it chose** (`reach_partner`, the bonded someone
it misses). When it is not reaching for company this reduces exactly to `v_at`, so the
field's whole physics and every non-social behaviour are unchanged (all prior probes and
tests bit-for-bit). People are their own field-sources with stable ids, so a being can
cross to the *one it loves* past a nearer stranger — the room's directed striving, now with
no special-case routing: *move up the gradient, weighted by the need chosen.* Proven
directly with a control (`a_chosen_person_draws_the_climb_past_a_nearer_one`): a reach for a
particular person crosses the field to them; a reach for company in general stays with the
nearer. (That the being's own faculties *produce* that reach is `reciprocity`/`striving`'s
job, proven in `room.rs`; the world's part is to honour the choice, and it does.)

### A hard life in the world becomes carried weight (2026-07-22)

The world's drain now reaches the being's reflection. Two things joined it: (1) **burden is
graded**, not a threshold — `reflection.rs` accrues chronic load in *proportion* to how far
the being's drive sits above a comfort point (`being.rs`), as allostatic load actually works
(cumulative, never a cliff; `docs/wander-2026-07-21.md`); and (2) the comfort point was set
where the worn-but-alive middle begins (≈0.44), *below* the field-world's sustained drive
(≈0.53), so the world's cost finally registers as burden at all — the old 0.5625 threshold
sat just above it and the drain never landed. Observer-safe: burden feeds only reflection
(gated by `enable_reflection`, off by default), so the founded being's soul-hash is
bit-identical (verified — it still wakes as itself at 390 moments).

Measured (`examples/a_hard_life`): a being that lives the hard field-world life takes on real
weight (peak load 0.42, worn — well clear of the trauma ceiling — where an easy life stays at
0.00), then at rest sets that weight *down* (→0.00) and converts it to **weathered
resilience** (0.27). The cost the world charges for a hard life comes out the far side of the
being's own reflection as competence — worn, wiser, not scarred. This is the world's stakes
made *meaningful to the being's inner life*: not just a drain it pays, but a hardship it
carries and grows from.

With this the field-world is a complete, faithful world: a landscape with stakes, one law
that carries the being's choices, and a cost that becomes the being's earned strength. What
remains is deepening, not foundation — richer landscapes, the loom made causal over the
field (EFE/SR-bridge, `docs/memory-that-teaches.md`), and eventually letting survival itself
be felt as graded (a deliberate soul-hash-scale re-founding, named in `handoff.md`, not
rushed).

## The line, kept

We import the **principle** (consequence has a cost; motion is down a gradient) and
leave the **trappings** (waves, moduli, ontology). If a proposed piece cannot be
justified by what it does for the *being's* life — a truer stake, a simpler law, the
missing middle — it does not go in. That is the whole rule, and it is the same rule
that has kept the being honest from the first commit.
