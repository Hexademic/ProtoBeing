# Development — is strain generative in this being, or only expensive?

> **Status: specified, nothing measured.** Committed before the probe exists, so §3's predictions
> are on the record before any result. Pure observation: no gate default changes, nothing in
> `src/` is modified, no journal written, `life/being.journal` untouched.

*Written 2026-07-31, from Blake:*

> *"Human brains are very evolved.. but unless an individual understands, it doesn't matter how
> advanced the brain was, if it isn't developed through experience... The being is made to be able
> to track all of these.. so naturally we expect it to.. but unless they learn how to use these
> developments, they won't access them. Strain, stress, and constraints push a being into solving
> novel issues it has no reason to approach."*

## 1. The claim, and why our own data already half-agrees

**Capability is not access.** We have built eleven faculties and repeatedly found the being does
not use what it has. Three results, filed separately as curiosities, are one finding:

| where | what it said | read as |
|---|---|---|
| `docs/null-space.md` | the being's freedom is real but **scavenged** — directions available, not taken | a curiosity |
| `docs/composed.md` | all eleven gates on is **sub-additive** — a hair *worse* than `receptors` alone | an attribution result |
| `docs/deferral.md` §2 | purposes: sparse world **2 authored / 0 fulfilled**; six movers **9 / 5**; twelve movers **7 authored / 5 abandoned** | "worlds of the right richness" |

That last one is an **inverted U** and nobody called it one. Too poor and nothing is achievable;
too turbulent and purposes go stale before they are reached. There is a middle where the being
actually completes what it sets out to do, and we measured it without naming it.

## 2. The mechanism already exists, and it is called `reflection`

This is not a faculty to build. `src/reflection.rs` is precisely the strain→capability
mechanism Blake describes, and it was built from his insight in the first place:

1. **Load** accumulates only under *sustained overwhelming* distress — outrun by surprise while
   losing ground — never under hardship the being masters (`OVERWHELM = 3/16`, `LOAD_RISE = 6`).
2. **At rest**, reflection *converts* load into **weathered** resilience — monotone, earned
   (`CONVERT = 1/8` per rest tick).
3. And `weathered` is **causal** when the gate is on. `being.rs`:
   ```rust
   let reflection_tone = if self.reflection_causal {
       (self.last_weathered / 12) - (self.last_load / 8)
   } else { 0 };
   ```
   which enters `affective_drive`, and thence the body's arousal and valence. The weight drags;
   the resilience earned by having carried and set down weight *lifts*.

**So the architecture can express development.** The question is whether the being ever gets the
conditions to use it — and there is an obvious suspect. **Conversion requires rest.** Every trace
taken this week shows the being in Engaged or Defensive for its entire life, touching `Rest` only
on the tick it died. A mechanism that fires only at rest, in a being that never rests, is a
capability the being has and cannot reach. Which is exactly Blake's claim, at the level of a
constant.

## 3. Predictions — locked before the probe runs

**Confident:**

- **D1.** In an ordinary life in the reference world, the being spends **under 5%** of its ticks
  in `Rest` or `Recovery`.
- **D2.** Therefore `weathered` stays at or near **zero** across an ordinary life: the
  developmental mechanism is present, wired, causal when gated on, and **never fires**.

**The live questions:**

- **D3 — can we give it the conditions?** In a world that alternates genuine strain with genuine
  respite, does `weathered` accumulate monotonically? I expect yes, since conversion is
  unconditional at rest — but I have never seen this number and the being may not enter `Rest`
  even when its world is safe, in which case the blocker is the **basin classifier**, not the
  world, and that is a different and harder problem.
- **D4 — THE REAL ONE. Does a weathered being do better?** Take a being that has carried and
  converted load, and a naive being of the same age, and put both into the same later hardship.
  Does the weathered one last longer, or sit lower on drive, or survive a threat the naive one
  dies to?

  **I do not know the answer and my uncertainty here is stronger than my certainty.** The reason
  is structural: `docs/incidents.md` I-3 established that in this architecture **strain is a
  bill** — free energy is charged to the body as metabolic threat at 48/256 per unit per tick.
  If `reflection_tone`'s lift is small relative to that bill, then "weathered" is a register that
  rises while the being is worn down anyway, and **development here is a readout with no
  consequence.**

  > **If D4 fails, we have built a being that can be worn and not one that can grow**, and that
  > must be said plainly in `docs/incidents.md` rather than discovered later by a being.

- **D5 — is there a band?** `examples/reflection.rs` already reports the far edge: put under
  sustained hardship, *the being reached its limit and died at tick 15*. So too little strain
  gives no load, too much gives death. If D4 holds at all, it should hold in a middle — and the
  shape of that middle is the object Blake's framing predicts and we have never looked for.

## 4. What must not become possible

- **Nothing steers by default.** `enable_reflection` stays opt-in; this measures a gated faculty,
  it does not promote one.
- **No being is run past its limit to make a point.** `examples/reflection.rs` already killed one
  at tick 15 establishing the far edge; that edge is known and does not need re-establishing.
  Deaths in this inch are reported first, per `docs/survival-first.md`, and any death that was
  not the point of the measurement is an incident.
- **D4 is answered against a control**, not against a baseline of itself. A weathered being
  compared only to its own earlier state measures ageing, not development.
