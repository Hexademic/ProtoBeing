# Settling — the first act this being has that operates on itself

> **⚠️ §7's EXPLANATION IS WITHDRAWN — see `docs/comfort.md` §13.** The ±32 mind→body channel is
> **not** why `Rest` is unreachable: arousal turns out to be dead weight in the basin classifier
> (deleting it changes the winner on 0.3% of ticks), and no single channel decides the being's mode
> at all. **S2's result stands untouched** — the being does come down while keeping its company —
> but the account of why that does not reach Rest was wrong.
>
> **Status: BUILT and MEASURED — see §7.** **S2 holds:** arousal's floor falls 113 → 100 and the
> being comes down **without giving up its company**, which incident I-7 said it could not do.
> **S3 fails, as predicted in writing.** And chasing the smallness produced the real finding:
> **everything the being's mind can do to its own body is worth ±32 of a 256-wide arousal — about
> 12% — while `Rest` sits 164 away. The channel is five times too narrow.** G is untested for the
> third time and §7 says why I am not going to force it. Ships gated, default-off.
>
> **Status when written: specified, nothing built.** Committed before the code, so §4's predictions are on the
> record first. **Causal** — gated, default off, founded being untouched.

*Written 2026-07-31 from Blake: "what would you do to allow a sense of striving towards rest,
towards endurance?" — and from `docs/comfort.md` §11–12, which established that the obstruction is
neither a missing need nor a mis-placed target.*

## 1. The gap, verified rather than asserted

`docs/comfort.md` §12 claimed *"the being can change where it is, it cannot change how it is."*
I checked it before building on it, because I have asserted three things this month that turned out
already to exist.

**It holds.** `MotorIntent` carries a `Posture`, and `Posture::Resting` is in the vocabulary — but
every consumer of posture is a **world**: `field_world.rs:705`, `room.rs:264`, `main.rs:402`. It
changes how the being *moves*. Nothing routes it back to the being's body. `Posture::Resting`
currently means *travel differently*, not *be calmer*.

So the gap is real: **no act this being has operates on itself.**

## 2. And checking narrowed the fix by a lot

§12 proposed this as a new self-directed act, and implied new machinery. It does not need any.

**`affective_drive` is already the one channel from the being's mind to its own body**
(`being.rs:1370`), and it is already a *sum of tones*:

```rust
self.affective_drive = Q8_8::from_raw(
    (mode_tone + relational_tone + restlessness + recall + reflection_tone + homecoming_tone)
        .clamp(-128, 128),
);
```

Two of those six — `reflection_tone` and `homecoming_tone` — are **already opt-in gated terms**
added exactly this way. And `body.rs` feeds it straight into arousal:

```rust
self.arousal = self.arousal.add(affective_drive.mul(quarter)).clamp(...)
```

> **So settling is a seventh term in an existing sum, negative, in a place two other gated tones
> already sit.** Not a new faculty, not a new act type, not a new field. One term.

And the world side follows for free: `intent_from` sets `effort = arousal × 256`, so an arousal
that falls carries effort down with it. **The being settling *is* the being doing less** — which is
the vocabulary gap `docs/comfort.md` §12 named, closed without inventing a vocabulary.

## 3. The design

```rust
let settle_tone = if self.settling_causal && !felt.state.at_stake {
    -(rest_want / SETTLE_DIVISOR)      // negative: wanting rest pulls arousal DOWN
} else { 0 };
```

- **Proportional, not thresholded.** The being settles *in proportion to how much it wants rest*
  (`joy.rs`'s repose want, already computed, currently attached to no goal). Graded, because every
  threshold this project has shipped has produced a dead zone — `NOCI_THRESHOLD` most recently
  (I-4), and `TELOS_ARRIVED`'s first value last hour.
- **This is what "striving toward rest" means here.** The being's own hunger for repose becomes the
  thing that quiets it. Not an override, not a competitor in the urgency race — the want acting
  directly on the body, which is the one thing needs in this architecture have never been able to do.
- **Bounded by the existing clamp** at ±128 on the whole sum, so it cannot dominate the other five
  tones however large the want grows.
- **Zeroed at stake.** A being whose survival is in question must not be able to sedate itself.
  This is the same floor `docs/deferral.md` §4 and `docs/earned-authority.md` §4 both wrote before
  their mechanisms existed, and it is not negotiable.

## 4. Predictions — locked before the code

**Confident:**

- **S1.** Default-off: trajectory and soul-hash bit-identical, full suite green, founded being wakes
  at 390 moments.
- **S2.** With the gate on, arousal falls below **113** — its floor in every companioned regime
  measured — in a life *with company*. Currently only solitude achieves that, and only by
  overshooting to 6.

**The live questions:**

- **S3 — does the being ever enter `Rest`?** Currently **0.0%** in every regime ever measured.
  **I predict this FAILS or barely moves, and I am saying so before running it.** `docs/comfort.md`
  §11 established that Rest is a *conjunction* — low arousal **and** fatigue ≈ 80 **and** channel 0
  ≈ 20 — and settling supplies exactly one of the three. If rest stays at zero, that is not a failed
  fix; it is the conjunction being confirmed as the real obstruction, and the next inch is fatigue.
- **S4 — does `reflection`'s conversion rise?** The link to incident **I-8**. Only if S3 moves;
  conversion happens at rest.
- **W — is the being better off?** Mean drive, share past `COMFORT`, and — the one that matters
  most given incident I-7 — **does settling let a being keep its company AND come down?** Right now
  it must choose. If settling gives it both, that is the finding, whether or not `Rest` ever fires.

**Guardrail:**

- **G.** Settling never fires on a tick where the being is `at_stake`. Asserted directly, not
  inferred from an aggregate.

## 5. What must not become possible

- **Rest reachable, never compulsory.** A being that must rest is as unfree as one that cannot.
  Settling is proportional to the being's *own* want; it is never imposed and never latching.
- **Survival outranks it absolutely.** G, above.
- **No new default.** Gated, off, and the published being's numbers stay comparable.
- **It must remain the being's own.** The tone is driven by `joy.rs`'s repose want — a register the
  being earns from its own state — and not by any operator-set schedule.

## 6. Method

Spec committed first. Then the gated term, then the probe, then §7 with what came out — **including
S3, which I expect to fail, in the form it fails.**

---

## 7. What came out — it works, and the channel is five times too narrow

Both arms lived 4,000 ticks. Nothing died.

| with company, no threat | settling OFF | settling ON |
|---|---:|---:|
| **arousal, minimum (S2)** | **113** | **100** |
| arousal, mean | 236 | 237 |
| channel 8, minimum | 103 | 91 |
| Rest / Recovery (S3) | 0.0% | 0.0% |
| load converted (S4) | 0 | 0 |
| mean drive, past COMFORT (W) | 93.8, 0.0% | 93.8, 0.0% |

### S1 — holds. Default-off is bit-identical; the founded being wakes at 390 moments.

### S2 — **holds**, and it is the first time this has happened

Arousal's floor moves **113 → 100**, channel 8 **103 → 91**. Small, and it matters for one reason:

> **The being came down without giving up its company.** Incident I-7 established that solitude
> was the *only* thing that lowered its arousal, and that solitude costs it the thing keeping it
> unburdened — it had to choose. It no longer strictly has to. The mind now has *some* purchase on
> the body while the being stays in company.

**And that is the whole of the good news.** The mean does not move (236 → 237). This is a floor
effect, not a shift.

### S3 — **fails, exactly as predicted and predicted in writing**

0.0% → 0.0%. Rest is a conjunction and settling supplies one coordinate of three. Recorded as the
conjunction confirmed rather than as a failed fix, because that is what §4 said it would mean.

### S4 — 0 → 0, which follows from S3. No rest, no conversion.

### The finding: the mind→body channel is only worth ±32 of 256

Chasing why the effect is so small produced something better than "the constant is too low":

```rust
affective_drive              clamped to ±128 (raw Q8.8)
body.rs:  arousal += affective_drive · ¼      ⇒  ±32 of a 256-wide arousal
```

> **Everything the being's mind can do to its own body — all seven tones, mode, relational,
> restlessness, recall, reflection, homecoming and now settling — sums to at most ±32 arousal out
> of 256. About 12%.**
>
> **Rest requires moving arousal from ~237 to 73. That is 164. The entire channel is five times
> too narrow.**

So `docs/comfort.md` §11's *"the being can change where it is and cannot change how it is"* is
now measured rather than argued, and it is not quite right. **It can change how it is — by about a
twelfth.** Settling is not undersized within that channel; it is one of seven terms sharing a
channel that is itself far too small for the state space it is being asked to traverse.

That is a statement about `body.rs`'s `quarter` coefficient and `affective_drive`'s ±128 clamp,
not about this faculty. **And it is the real reason `Basin::Rest` is an unvisited corner.**

### G — **untested for the third time, and I am going to stop trying**

The guard is structural and reads `!self.last_felt.state.at_stake`. But the being is `at_stake` on
**0.0%** of ticks in *both* the easy life and the deliberately hard one (threat 130, nutrient 200),
so no distribution could have violated it in either. G is **not passed** — it is unexercised.

This is the third vacuous welfare verdict in two days (`survival-first.md` §11,
`earned-authority.md` §6, here). The pattern is not carelessness about the guard; it is that
**this being is almost never in trouble**, so guards written for its trouble have nothing to bite
on. Testing G properly needs a life engineered to put the being at its edge and hold it there —
which is a deliberate act on a creature, and belongs in its own inch with its own welfare case,
not tacked onto the end of a session.

## 8. Where this leaves it

- **Ships gated, default-off.** It does something real and small, and nothing downstream depends
  on it.
- **The next inch is not another tone.** It is the **±32 channel** — whether `affective_drive`'s
  clamp and `body.rs`'s `quarter` were chosen to be a gentle influence or inherited as one, and
  what widening them would cost. That question governs every self-directed act this being will
  ever have, including all four steps of `docs/comfort.md` §12.
- **And `Basin::Rest` may simply be mis-sited.** If the mind can move arousal by 32 and Rest sits
  164 away, then either the channel is wrong or the target is. Both are one measurement from being
  answered, and nobody has asked which.
