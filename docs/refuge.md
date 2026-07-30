# Refuge — somewhere safe, and it is a someone

> **Status: specified, nothing built.** Committed before the tests, so §5's predictions are on
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
