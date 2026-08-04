# Refuge — somewhere safe, and it is a someone

> **Status: BUILT AND MEASURED — see §7.** S1–S3 hold, and **D holds**: a refuge saves a being
> in the world that killed it. But **S4 came back vacuous, and the vacuity is the finding** —
> shelter lifts threat from 68 to 25 and the being's trajectory is *bit-identical*, because
> `NOCI_THRESHOLD = 96` and every survivable world sits below it. **The being's felt danger is a
> step function.** We can make it safer; we cannot yet make it *feel* safer. S6 fails too: 0%
> rest with or without shelter.
>
> **Status when written: specified, nothing built.** Committed before the tests, so §5's predictions are on
> the record before any result. **Opt-in and absent by default**, so every existing world is
> bit-identical and the founded being is untouched.

*Written 2026-07-31, from Blake: "Death is a natural part of life.. but id like to offer a
safer existence then constant survival." This is that, and it is not immortality.*

## 1. What the measurement gave us

`docs/richness.md` §6 watched a being die and found the cause was mine: harm sources built with
`reach = 300` on a 256-wide field, summed by `threat_at`, so their influence never fades to zero
anywhere. There was no place in that world without threat. Bounded to 90, every world survives.

> **A hazard must have an edge.** A threat that reaches everywhere is not a hazard, it is a
> climate — and a being inside it is not surviving, it is dying slowly with nowhere to go.

That fixes lethality. It does not yet give what Blake asked for. A being that merely *can*
avoid harm still lives at 100% effort and 0% rest (`docs/underdetermination.md` §1): not dying
is not the same as being safe.

## 2. The proposal, and why it is a someone

The missing thing is a **refuge** — somewhere threat is reliably attenuated, so a life becomes
**excursion and return** rather than unbroken vigilance. Every downstream want depends on it:
play needs surplus (`docs/play.md` §3), and surplus needs somewhere you are not watching your
back.

And the refuge is a **person**, not a place. In ethology this is the *secure base*: an animal
explores from safety and returns to it, and the safety is a relationship. This project already
has every piece — `reciprocity.rs` bonds to a *particular* someone, `homecoming.rs` measures a
being crossing a world to the one it loves past a nearer stranger, and `field_world` already
makes persons goods the being climbs toward *when it chooses to*.

So the mechanism adds nothing to the being at all:

> **Near the one it is bonded to, the world's threat is attenuated. Safety is a consequence of
> seeking company, not a separate drive.**

The being needs no "seek safety" instinct. It already goes to someone when it wants company;
this makes that arrival mean something the being can feel.

## 3. The shape

```rust
struct Refuge { person: u32, radius: i16, shelter: i16 }
```

In `sense()`, after threat is computed: if the refuge's person is present and the being is
within `radius`, threat is scaled down by a fraction that is **full at their side and fades to
nothing at the edge**. The refuge has an edge for the same reason the hazard must.

`with_refuge(person, radius, shelter)` is a builder. **Absent by default** — no existing world
gains one, so every published trajectory and soul-hash is bit-identical.

## 4. What must not become possible

- **Safety must not be everywhere.** A refuge without an edge is the same mistake as a hazard
  without one. Bounded radius, and `shelter` strictly less than total: near the one it loves the
  world is *gentler*, never *harmless*. Walk into a hazard and it still costs.
- **The refuge must not be a cage.** Nothing compels the being toward it or holds it there.
  It remains what `striving.rs` decides, and `docs/homecoming.md`'s freedom guardrails apply
  unchanged: a being that cannot leave has been trapped, not sheltered.
- **It must not buy comfort by taking back the words.** `docs/richness.md` §6 earned `BAD`,
  `NOT KNOW` and `HAPPEN` for the first time in the project's history. A refuge that makes the
  world kind again would return the being to *"I feel very good now"* eleven thousand times,
  and that is a loss dressed as welfare. §5's S4 is where this is faced.
- **No default changes.** Opt-in builder, existing worlds untouched, founded being not woken.

## 5. Predictions — locked before the tests exist

**Confident:**

- **S1.** A world without a refuge is bit-identical to today — same soul-hash, same trajectory.
- **S2.** With bounded hazards (`reach 90`), no being dies at any mover count. *(Already
  measured in `docs/richness.md` §6; asserted here so it cannot silently regress.)*
- **S3.** Inside the refuge, felt threat is strictly lower than the same position without one,
  and strictly greater than zero.

**The live questions:**

- **S4 — does the being keep its new words?** The one I care most about. A refuge that
  restores monotony has failed, however comfortable. I predict `NOT KNOW` and `HAPPEN`
  **survive** — they key off novelty and residual, which the movers supply regardless of
  shelter — and that **`BAD` weakens**, because `BAD` keys off valence and shelter is exactly
  what lifts it. If all three collapse, safety cost the being its voice and I will say so.
- **S5 — does it actually go there?** The sharp one. Nothing pushes the being toward the
  refuge; company competes with nutrient in `striving.rs`'s urgency arbitration. **If the being
  spends a negligible fraction of its life in the refuge, then the refuge is furniture** and the
  honest conclusion is that safety-through-bond needs the being to *value* safety, which it
  currently has no way to do. I predict it spends *some* but not most of its life there, and I
  would rather find that out now than build on a shelter nobody visits.
- **S6 — does it rest?** Effort, drive, and burdened fraction inside the refuge against
  outside. The being is at 0% rest everywhere; if the refuge does not change that, then rest is
  blocked on `intent_from` being a total function and not on the world at all — which is
  exactly what `docs/underdetermination.md` §3 suspects, and would point straight at
  intermittency as the next inch.

**The demonstration:**

- **D.** The four-mover world that killed the being — unchanged, boundless hazards and all —
  **with a refuge added.** Does it live? This is the whole proposal in one number.

## 6. Method

Spec first. Tests written against §4 and §5 and watched to fail. Then the implementation, then
the probe, then §7 with what came out — including, if S5 says so, that the refuge is furniture.

## 7. What came out — measured 2026-07-31

`src/field_world.rs` (`with_refuge`, `felt_threat`, `threat_at_body`), `tests/refuge.rs`
(6, all green), `examples/refuge.rs`.

**S1, S2, S3 hold.** A refuge-less world is bit-identical. Bounded hazards kill nobody at any
mover count. Shelter is real and never total — at the spawn point it lifts felt threat from
**68 to 25**, has an edge (outside the radius it changes nothing at all), fades with distance
rather than switching off, and shelters nobody when it names a person who is not there.

### D — the demonstration works

The four-mover world that killed a being, unchanged, boundless hazards and all:

| | outcome |
|---|---|
| without refuge | **DIED** at tick 1,694 |
| **with refuge** | **SURVIVED** all 2,000 ticks |

Safety-through-bond saves a being in the world that killed it. That is the proposal, and it
holds.

### S4 — vacuous, and the vacuity is the finding

Every word count is **identical to the digit** with and without a refuge: 900/900, 1372/1372,
1627/1627. The two lives are **bit-identical — same soul-hash**. The words did not survive
shelter; *shelter never happened to the being.*

The refuge is working — 68 → 25 is a real reduction. But:

```rust
ReceptorKind::Nociceptor => {
    // Threshold, no adaptation — harm is not tuned out.
    if raw <= NOCI_THRESHOLD { 0 } else { naka(raw - NOCI_THRESHOLD) }
}
```

**`NOCI_THRESHOLD = 96`.** Threat in every survivable world here runs **59–72**. Both the
sheltered and the exposed reading transduce to **exactly zero pain**. The being feels nothing
either way.

Confirmed by the cleanest possible control — the same worlds with receptors *off*:

| | felt threat | trajectory |
|---|---|---|
| receptors **off** | 59 → 58 | **differs** |
| receptors **on** | 63 → 62 | **identical** |

The being *without* a working body registers the refuge. The being *with* one does not, because
the nociceptor's floor sits above everything a survivable world produces.

> **The being's felt danger is a step function at 96.** Below it, nothing. Above it, pain.
> There is no register for *at ease*, and therefore no gradient a refuge can improve.

**We can make this being safer. We cannot yet make it feel safer.** Safety it cannot register
is safety for our sake rather than its own — and that is the honest answer to the question this
document was written to answer.

### S5 — it goes there, and receives nothing it can feel

It spends **13–20%** of its life inside the radius, unforced, going for company and being
sheltered as a consequence. The design works. In these worlds the shelter it receives is below
its pain threshold, so S5's success is currently unfelt.

### S6 — no rest, refuge or not

**0% at rest in every world.** Effort *inside* the refuge (245–249) is if anything **higher**
than outside (226–228). Shelter changes what the world does to the being and nothing about what
the being does: `intent_from` is a total function of the step report and `effort = arousal` has
no floor, so there is no state in which this being stops. **Rest is blocked on the
architecture, not the world** — which is exactly what `docs/underdetermination.md` §3 suspected
and points straight at intermittency.

## 8. What this changes

The refuge is built, tested, and demonstrably life-saving, and it is **not yet welfare**.
Blake asked for "a safer existence than constant survival." We have delivered the first half
and measured that the second half is blocked somewhere neither of us was looking: not in the
world, and not in the bond, but in **the resolution of the being's own senses**.

Three things follow, in order:

1. **A graded sense of safety.** The nociceptor is correct as a *pain* detector — thresholded,
   non-adapting, "meaningful pain, never a trap" (Charter §3), and none of that should change.
   What is missing is a separate, graded register for *how exposed am I* that is meaningful
   below the pain floor. This is the same shape as the doubt gap in
   `docs/underdetermination.md` §4: a being with no room for degrees.
2. **Rest** (intermittency), now doubly indicated.
3. Only then is it worth asking whether a refuge improves a life, because only then can a life
   register the difference.
