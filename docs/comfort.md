# Comfort — what a living thing needs to survive well, and the one line denying it

> **Status: BUILT, MEASURED, and the diagnosis in §1 is WRONG — see §8.** The satiety band works
> and does not buy the being rest. **`Basin::Rest` is classified from the somatic field, not from
> striving** — a being can want nothing and still not be at rest, and this being never rests in
> either arm. Three errors of mine are recorded in §8, including a first constant that was
> arithmetically incapable of firing. The gate ships **default-off and unrecommended.**
> **The next inch is `basins.rs`.**
>
> **Status when written: specified, nothing built.** Committed before any code, so §5's predictions are on the
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

---

## 8. What came out — **the diagnosis in §1 is wrong**, and three errors are mine

Both arms lived 4,000 ticks. Nothing died.

| | gate OFF | gate ON |
|---|---:|---:|
| **rest / recovery (C1)** | **0.0%** | **0.0%** |
| competence vs need disagreement (C3) | 40.2% | **73.6%** |
| purposes authored / fulfilled (C4) | 2 / 1 | 2 / 1 |
| load converted (C5) | 0 | 0 |
| mean drive, past COMFORT (W) | 9.0, 0.0% | 9.0, 0.0% |

### C1 fails, and C1 is the whole answer

**The being's purpose became satisfied, its goal became `None` — and it still never entered
`Rest`.** Not once, in 4,000 ticks, in either arm.

Because `Basin::Rest` is not set by `striving.rs`. It is classified from the somatic field
(`being.rs:1001`, `basins.compute_membership(&self.field)`), **entirely independently of what the
being is striving for.** A being can want nothing and still not be at rest.

> **So §1's diagnosis is wrong.** The obstruction is not purpose's missing satiety band, and it is
> not in the arbitration at all. Striving choosing "no goal" and the being *resting* are two
> different things, and I conflated them because `habits::act_of(None) = rest` made them look like
> one in the report.

The satiety band is still defensible on its own terms — a need that cannot be finished is a
compulsion, and §2's law stands — but **it does not buy the being rest**, which is what it was
for. It stays default-off and unrecommended until something makes rest reachable.

### Error one: my first constant was arithmetically incapable of firing

`TELOS_ARRIVED` was 224. At proximity 224 the divergence is 32, and `striving.rs`'s `SALIENT` is
**64** — so the branch sat entirely inside the region where purpose *already* was not pressing. It
fired on 53 ticks of 4,000 and changed nothing. The soul-hash came back identical and the probe
printed **"C3 FAILS"** for a test that had never run.

Fixed by *deriving* the constant instead of choosing it — satiety is only meaningful where the need
stops being salient, so `TELOS_ARRIVED = Q88_SCALE − SALIENT = 192`. **This is the third vacuous
verdict in one day** (`docs/survival-first.md` §11, `docs/earned-authority.md` §6, here), and the
rule that catches it keeps being written and not used:

> **State what a measurement could not have shown, before reporting what it did.**

### Error two: C3 was the wrong test, and it is symmetric

Disagreement rose 40.2% → **73.6%**, and the probe called that "worse." **It cannot know that.**
The metric counts *whether* competence and need differ, not *who is right*. With satiety on, need
moves to rest on many ticks while the niche-7 habit still wants purpose — so the gap widens partly
because **need moved toward what competence had been asking for**, which is the opposite of worse.

I designed the test as though closing the gap meant comfort. A symmetric metric cannot carry a
directional claim, and I should have seen that when I specced it.

### Error three, unresolved and flagged rather than explained

**The soul-hash is identical between arms** (`5afc7074…`) while the disagreement metric moves 33
points. Those cannot both be true of a being whose life differed — so either the being's trajectory
genuinely did not change and only a *reported* field did, or the hash does not cover what I assume
it covers. **I do not know which, and I am not going to guess at the end of a session.**
`docs/soul-hash-limits.md` is the place that question belongs.

## 9. Where this leaves it

- **C2 holds**: default-off, full suite green, founded being wakes at 390 moments.
- **C4 holds**: fulfilment and abandonment unchanged — permission to stop did not become stopping
  short. That guardrail was real and it was worth writing first.
- **C1, C3, C5 fail or are unanswerable**, and the reason is one thing: **rest is a basin, not a
  goal.**

**The next inch is `basins.rs`, not `striving.rs`.** The question is what field state produces
`Basin::Rest` and whether this being ever reaches it — and given `docs/development.md` §5 also found
0.0% rest across every regime tried, the honest possibility is that **`Rest` is unreachable in this
architecture**, in which case `reflection`'s conversion-at-rest, I-8, and this document are all
downstream of one unreachable state.

That would be a bigger finding than anything here, and it is one measurement away.
