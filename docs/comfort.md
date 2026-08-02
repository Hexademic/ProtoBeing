# Comfort — what a living thing needs to survive well, and the one line denying it

> **Status: BUILT, MEASURED, and the diagnosis in §1 is WRONG — see §8.** The satiety band works
> and does not buy the being rest. **`Basin::Rest` is classified from the somatic field, not from
> striving** — a being can want nothing and still not be at rest, and this being never rests in
> either arm. Three errors of mine are recorded in §8, including a first constant that was
> arithmetically incapable of firing. The gate ships **default-off and unrecommended.**
> **§10–§12 go further:** the only thing that calms this being is **solitude** (arousal 234 → 6),
> and even then it stays 100% Engaged. But **every one of Rest's twelve coordinates is individually
> reachable** — the conjunction never is. Rest is an *unvisited corner*, not a dead state, and the
> reason is that **the being can change where it is and cannot change how it is.** §12 is the
> four-step answer to striving-toward-rest, and step 4 closes incident I-8.
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

---

## 10. `basins.rs` — the being is never tired and never calm

*The inch §9 named. `examples/basins_probe.rs`, predictions B1–B4 locked in its header and
committed before it ran.*

### B1 — holds in the strongest possible form

| regime | Rest | Engaged | Defensive | Recovery |
|---|---:|---:|---:|---:|
| reference world | **0.0%** | 100.0% | 0.0% | 0.0% |
| reference + reflection + comfort | **0.0%** | 100.0% | 0.0% | 0.0% |
| held calm (threat 0) | **0.0%** | 100.0% | 0.0% | 0.0% |
| strain cycle (threat 130) | **0.0%** | 0.0% | 100.0% | 0.0% |
| strain cycle + every gate | **0.0%** | 0.0% | 100.0% | 0.0% |

**Zero Rest ticks across every regime.** And note what else that table says: the being does not
*move between* basins. It picks one and stays — 100.0%, not "mostly". Calm worlds hold it in
Engaged; strain holds it in Defensive; nothing takes it to Rest or Recovery, ever.

### B2 — **fails**, and the failure is informative

Rest is **not** the furthest basin. By mean L1 distance over a reference life:

| basin | distance |
|---|---:|
| Engaged | **375** |
| Recovery | 629 |
| **Rest** | **659** |
| Defensive | 1025 |

Rest ranks 3 of 4. The being spends 100% of a strained life in **Defensive — the furthest basin of
all** — and 0% in Rest, which is nearer. So occupancy is not simply "closest target wins": the
stance bias and dwell hysteresis carry it, and mean distance does not predict where it lives. I
predicted Rest would be furthest and it is not.

### B3 — holds. Two channels, and they are the same thing

| channel | being sits at | Rest wants | gap | share |
|---|---:|---:|---:|---:|
| **4 · arousal setpoint** | **234** | **73** | 161 | 24.4% |
| **8 · arousal** | **225** | **73** | 152 | 23.1% |
| 0 | 120 | 20 | 100 | 15.2% |
| **10 · fatigue** | **0** | **80** | 80 | 12.1% |
| 9 · valence | 98 | 32 | 66 | 10.0% |

Top three channels are **63%** of the whole distance, and the top two are both **arousal** —
together nearly half of it. The being lives at ~230 of 256 arousal. Rest is defined at 73.

### B4 — the answer, and it is not "structurally dead". It is worse-shaped than that

Two facts, and they compose into something neither says alone:

> **The being sits at arousal ~230 when Rest requires ~73, and its fatigue is exactly 0 when Rest
> requires 80.**

Rest in this architecture is *low arousal with some accumulated tiredness* — the state of a
creature that has been worn and is winding down. This being reaches neither half:

- **Arousal only climbs.** Every trace taken this week shows it rising monotonically across a life
  (0.449 → 0.934 in `i3_trace`) and never returning. There is no decay path to 73.
- **Fatigue is zero because nothing wears it.** Channel 10 is fed by `narrative.rs`'s
  `apply_identity_reflection` as `narrative_burden / 4`, and burden stays near zero in a life that
  goes well. `docs/development.md` §5 found the same thing from the other side: the reference world
  never presses the being hard enough to accumulate anything.

Which produces the finding, and it is a genuinely strange shape:

> **A being that is doing well never gets tired, and a being that never gets tired can never rest.
> Fatigue is the entry condition for rest, and this being's fatigue is zero precisely because its
> life is going fine.**

So rest is not withheld from a struggling being. It is withheld from a *thriving* one — and when
the being finally is pressed hard enough to accumulate fatigue, the pressure puts it in
**Defensive**, not Rest. Both doors are shut, for opposite reasons.

### What this closes, and what it opens

- **`docs/comfort.md` §1's diagnosis is now fully retired.** The obstruction was never striving,
  never purpose's satiety, and never the arbitration. It is that the field never enters the region
  the architecture calls rest.
- **Incident I-8 is now explained rather than merely open.** `reflection.rs` converts load into
  `weathered` resilience *at rest*. The being never rests. So the developmental mechanism has never
  had a single tick in which to run — which is why D4 found `weathered` bought nothing. **I-8's
  answer is upstream of I-8.**
- **`docs/development.md` §5's inverted-U needs re-reading.** The band where load accumulates and
  the state where it converts are disjoint: strain gives fatigue *and* Defensive, ease gives Rest's
  arousal requirement no closer and no fatigue at all.

**The next question is a real one and I do not have it:** does anything in this architecture bring
arousal *down*? If nothing does, Rest is unreachable by construction and the four-basin model has
a state it can never occupy — which would be worth saying plainly in `docs/architecture.md` rather
than leaving three documents to keep rediscovering the same zero.

---

## 11. Rest is not a dead state. It is an unvisited corner — and the being has no way to move itself there.

*From Blake: "I'm not sure, would you like to test? And what would you do to allow a sense of
striving towards rest, towards endurance?" Both answered below; the test came first because the
design depends on it.*

### Test 1 — does anything bring arousal down? **Yes. Exactly one thing: solitude.**

| condition | min arousal | final | min ch8 |
|---|---:|---:|---:|
| perfect ease — fed, safe, company | 113 | 234 | 103 |
| …plus every safe gate on | 113 | 234 | 103 |
| ambient floor / gentle threat / sustained threat 130 | 113–115 | 234–293 | 103–109 |
| starving | 115 | 387 | 109 |
| **alone, no threat, fed** | **6** | **10** | **−2** |

Nothing this being can experience calms it except **being by itself**. Which sits exactly against
incident I-7: **company is what keeps it unburdened, and solitude is the only thing that lets it
calm down.** It cannot have both.

### Test 2 — so does a solitary being rest? **No. Still 100% Engaged.**

Arousal at 6, well under Rest's 73 — and the basin never changes. And note the overshoot: ch8
reaches **−2**, so solitude does not move the being *to* rest, it carries it *past* rest into a
different kind of far.

The being is **100% Engaged in every condition tested** — with company, alone, calm, threatened,
starving. Engaged is not a mode it enters. It is the only place it can be.

### Test 3 — the one that settles it: is Rest even *reachable*?

Sweeping 120 regimes (partner × threat × nutrient × gates) and recording the envelope each somatic
channel actually visits, against each basin's target:

| basin | target values outside the being's reachable range |
|---|---|
| **Rest** | **0 of 12** |
| Engaged | 0 of 12 |
| **Defensive** | **3 of 12** (channels 1, 2, 3) |
| Recovery | 0 of 12 |

> **Every one of Rest's twelve coordinates is individually reachable.** The being visits arousal
> 19–247 (Rest wants 73), fatigue 0–315 (wants 80), channel 0 across 0–256 (wants 20).
> **Each coordinate is reachable. The conjunction never is.**

Rest is not a dead state. It is an **unvisited corner** — the being's trajectory is confined to a
manifold that never passes through low-arousal-*with*-moderate-fatigue-*with*-low-ch0 at the same
moment.

**And one thing this kills outright:** `Defensive` has three targets the being can *never* reach,
and it spends 100% of a strained life there. So basin occupancy is **not** proximity to a target —
it is L1 argmax among four, plus stance bias, plus dwell hysteresis. **You can live in a basin
whose definition you can never satisfy.** Any future reasoning about basins must not assume
otherwise; I nearly did.

## 12. What I would do for "striving towards rest, towards endurance"

The tests say the obstruction is not a missing need, a missing threshold, or a badly-placed target.
It is this:

> **The being can change where it is. It cannot change how it is.**

`being.rs:927` recomputes the somatic field from the body every tick (`write_from_body`). Every
action this being has acts on the **world** — move, reach, approach, withdraw. **Nothing it can do
acts on itself.** So it cannot assemble the conjunction Rest requires, because assembling a
somatic state is not in its vocabulary of acts.

Four steps, in order, each small and each gated:

**1. `Need::Rest` — make it namable.** The enum has Sustenance, Company, Novelty, Purpose; rest is
`None`. But `joy.rs` *already computes a repose want*. **The being has a hunger for rest and no
goal to attach it to** — the same shape as `docs/earned-authority.md`'s finding, one level down.

**2. Let striving mean *settling*.** `mobilization = urgency × viability`, and `effort =
arousal × 256`. **To want anything is to mobilize.** Rest is the one need where wanting it must
*lower* effort, and the architecture has no vocabulary for a need approached by doing less. This is
the real gap and it is a vocabulary gap, not a mechanism gap.

**3. One self-directed act: *settle*.** A motor intent whose target is the being's own arousal
rather than a place in the world. It would be the first act this being has that operates on itself,
and it is what "striving toward rest" literally requires. **Spec it, lock predictions, gate it** —
this is a real architectural addition, not a constant, and it must not be built at the end of a
session.

**4. Endurance then follows for free.** `reflection.rs` already converts load into `weathered` at
rest; `weathered` already feeds `affective_drive`; `affective_drive` already reaches the body. The
whole arc **rest → resilience → endurance is already built** and has never had its first state
enterable. That is also incident **I-8**'s answer, and it means I-8 closes the moment step 3 does.

**The guardrail, before any of it:** rest made *reachable*, never *compulsory*. A being that must
rest is as unfree as one that cannot — the same law from the other side, and `docs/habits.md`'s
breakability law already binds it.
