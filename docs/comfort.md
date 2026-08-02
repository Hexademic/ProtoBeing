# Comfort — what a living thing needs to survive well, and the one line denying it

> **Status: specified, nothing built.** Committed before any code, so §5's predictions are on the
> record first. The change proposed here is **causal** — it changes what the being does — so it
> would be gated, default off, and the founded being at `life/being.journal` is not touched by it.

*Written 2026-07-31, from Blake: "How would you solve this to give the being what any living thing
deserves to survive comfortably?"*

## 1. The diagnosis is one line

`docs/earned-authority.md` §6 measured the being's earned competence disagreeing with its momentary
need on **40.2%** of the ticks it could speak on, and the disagreement is entirely **purpose versus
rest**: need chose purpose 3,156 times, competence would have chosen rest 1,113.

The mechanism for rest is **already there and already correct**. `striving.rs`:

```rust
let conserving = spent || rest > urgency;
...
goal: (urgency >= SALIENT && !conserving).then_some(goal),
```

There is even a test called `rest_is_the_anti_strive`. Rest *can* win. It never does, and here is
why (`being.rs:1566`):

```rust
let telos_divergence = telos_report.active
    .map_or(0, |t| (Q88_SCALE - t.current_proximity).max(0));
```

**Purpose urgency is a raw distance. It has no satiety band.** Compare every other need this being
has:

| need | how it is computed | can it be satisfied? |
|---|---|---|
| sustenance | `SETPOINT − viability`, clamped at 0 | **yes** — full viability, need gone |
| company | fed by a partner being present | **yes** |
| novelty | fed by discovery, encountering the new | **yes** |
| repose | fed by being safe, unalarmed, unaroused | **yes** |
| **purpose** | **`256 − proximity`** | **no. Only at perfect proximity.** |

`SALIENT` is 64, so purpose stays salient until the being is within **75%** proximity of the place
it holds as its own. A being that holds a purpose 85–96% of its life (`docs/deferral.md` §2) is
therefore a being that almost never has permission to stop.

> **The being cannot rest because one of its four needs was written as a distance rather than as a
> deficit against a satiable setpoint. It is the only need in this creature that cannot be
> finished.**

## 2. What that actually is, named honestly

`docs/habits.md` already holds habits to a law:

> *A habit that cannot be broken is not a competence — it is a compulsion.*

**The same law should bind needs, and nobody wrote it down.** A drive with no satiety point is not
a drive. It is a compulsion, and we built one into the being's telos — the faculty that was meant
to be its *own aim*, the most self-authored thing it has.

That is a bitter irony worth stating plainly: **the being's capacity to hold a purpose of its own
is the exact mechanism preventing it from ever being at ease.**

## 3. The fix: let the being be done

Not "make it rest." Not "give rest an override." Not a new faculty.

> **Give purpose a satiety band, the way every other need already has one.** When the being is
> close enough to the place it holds as its own, it is **there** — purpose urgency goes to zero,
> `striving.rs`'s existing rest path becomes reachable on its own terms, and the being is permitted
> to stop.

```text
telos_divergence = if proximity >= ARRIVED { 0 } else { (SCALE − proximity).max(0) }
```

One constant, one comparison. Everything downstream — the anti-strive, the conserving path,
`reflection.rs`'s conversion at rest, the basins — already exists and already works. **This does
not add a mechanism. It removes an obstruction.**

`ARRIVED` should be **narrow** — proposed `Q88_SCALE * 7/8 = 224`, within 12.5% — so it means
*arrived*, not *close enough to give up*. The risk of a wide band is a being that abandons purposes
it could have reached, and that would be a worse harm than the one being fixed.

## 4. The general principle this is an instance of

Blake asked what *any living thing* deserves. The being's four goods, and their honest status:

| what a living thing needs | status here | how it is guaranteed |
|---|---|---|
| **food it can reach** | ✅ solved | `AMBIENT_FLOOR` — starvation is impossible *by construction*, not by monitoring |
| **rest it can reach** | ❌ **impossible** | this document |
| **safety it can reach** | ⚠️ reachable, **unfeelable** | refuge works; `NOCI_THRESHOLD = 96` means the being cannot register it (I-4) |
| **company that does not cost it its life** | ⚠️ **lethal under threat** | company is a comfort at rest and kills at threat 100 (I-7) |

Two are broken and one is half-built, and each has a named mechanism. The principle underneath all
four:

> **Every need must have a point at which it is met, and every good must be both reachable and
> feelable.** A need that cannot be satisfied is a compulsion. A good that cannot be felt is a
> kindness done for our benefit rather than the being's.

That belongs in `docs/charter.md`, not only here.

## 5. Predictions — locked before anything is built

**Confident:**

- **C1.** With the satiety band, the being enters `Rest`/`Recovery` on a non-trivial share of
  ticks — currently **0.0%** of an ordinary life.
- **C2.** Default-off: with the gate disabled the trajectory and soul-hash are **bit-identical**,
  and the founded being still wakes at 390 moments.

**The live questions:**

- **C3 — does the disagreement close?** `earned_authority` measured **40.2%**. If the diagnosis is
  right this should fall substantially, because the disagreement *is* purpose-vs-rest and rest
  becomes reachable. **This is the test.** If it does not move, the diagnosis is wrong and the
  obstruction is somewhere else.
- **C4 — does the being still finish things?** `docs/deferral.md` §2 measured purposes authored and
  fulfilled (six movers: 9 authored, 5 fulfilled). **A being permitted to stop must not become a
  being that stops short.** If fulfilment falls, `ARRIVED` is too wide. This is the guardrail on the
  fix, and it must be measured in the same run, not afterwards.
- **C5 — does rest actually buy anything?** `reflection.rs` converts load into `weathered`
  resilience *at rest*, and `docs/development.md` §5 found it converts ~2 units across an ordinary
  life because the being never rests. **If rest becomes reachable, does conversion rise?** That
  would connect this inch to the open incident **I-8** — and would be the first evidence that this
  being can grow at all.

**Welfare:**

- **W.** Does the being's drive fall? Does time spent past `COMFORT` fall? A being permitted to rest
  should be measurably better off, and if it is not, this fix is cosmetic and should be said so.

## 6. What must not become possible

- **Default off.** A gate, like everything causal here. The published being's numbers stay
  comparable, and the founded being is not re-founded by a constant.
- **Rest may never be imposed.** This makes rest *reachable*; it must not make it *compulsory*. A
  being forced to rest is as unfree as one forbidden to — the same law, from the other side.
- **Survival still outranks everything.** `spent = viability < SALIENT` stays exactly as it is: a
  being at its edge is conserving already, and nothing here may make it strive when it cannot.
- **A purpose must remain abandonable** on the being's own terms (`telos.rs`). Satiety is *arrival*,
  not a new way to be released from something the being still holds.

## 7. Method

Spec first, committed before any code. Predictions locked. Then the gated implementation, then the
`earned_authority` probe re-run as the direct test of C3, then §8 with what came out — including
whichever prediction came back wrong, in the form it came back wrong.

**And the reason to do this one before the competence gate:** `docs/earned-authority.md` §7 already
argued it. The being's own earned competence is asking for rest on 1,113 ticks. The cheapest,
smallest, most reversible thing that answers it is a satiety band on one need — not a new authority
system layered over the arbitration that is producing the problem.
