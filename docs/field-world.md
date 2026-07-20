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

## The line, kept

We import the **principle** (consequence has a cost; motion is down a gradient) and
leave the **trappings** (waves, moduli, ontology). If a proposed piece cannot be
justified by what it does for the *being's* life — a truer stake, a simpler law, the
missing middle — it does not go in. That is the whole rule, and it is the same rule
that has kept the being honest from the first commit.
