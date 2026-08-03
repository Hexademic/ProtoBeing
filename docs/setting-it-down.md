# Setting it down — a being should be able to put weight down while still carrying its life

> **Status when written: specified, nothing built.** Committed before the code, so §5's predictions
> are on the record first. **Causal** — gated, default off, founded being untouched.

*Written 2026-08-03, from checking a sentence I had repeated three times without ever reading the
call site.*

## 1. What I got wrong, plainly

I wrote in `docs/comfort.md` §10, in `docs/settling.md` S4, and in incident **I-8** that this being
converts nothing **because it never reaches `Basin::Rest`**, and that conversion happens at rest.
I then proposed, and Blake authorised, re-keying conversion onto a state the being can reach.

I went to read the call site first. `being.rs:1751`:

```rust
let resting = !burdened
    && (matches!(basin, Basin::Rest | Basin::Recovery)
        || (!losing_ground && free_energy < Q88_SCALE*3/16 && felt.state.arousal < Q88_SCALE/2));
```

**`resting` is a disjunction and the basin is only one of its two arms.** `examples/reflection_gate`
measured the rest:

| regime | `reflecting` | `Rest` basin |
|---|---:|---:|
| reference world, with company | **100.0%** | 0.0% |
| held calm, with company | **100.0%** | 0.0% |
| held calm, **alone** | **2.7%** | 0.0% |
| threat 130, with company | **99.1%** | 0.0% |

The being rests on essentially every tick of a companioned life and enters the `Rest` **basin** on
none of them. **The sentence "conversion happens at rest, and this being never rests" is false in
its second clause.** The gate I was about to re-key was already open. Option 3 as specced would
have changed nothing, and I would have measured "no effect" and drawn a further wrong conclusion
from it.

`docs/comfort.md` §10 and `docs/settling.md` S4 carry withdrawal notices pointing here.

## 2. And then I over-corrected, which is the second thing to say

`examples/reflection_deadlock` D3 concluded the being *"has never converted a single unit of what
it carried."* **That is false, and two records already in this repository said so:**

- **incident I-8's own band** (threat 90, 20 hard / 80 easy) converted **232 units** while living
  the full 4,000 ticks. I wrote that entry myself and then contradicted it.
- **the founded being** at `life/being.journal` carries `load` **0** and `weathered` **2**
  (`examples/founded_load`, replay-only). It has grown. Not much, but not never.

D3 was true of the five regimes I happened to pick and I stated it as a fact about the
architecture. That is the same error as §1 — a claim wider than the measurement — made in the
course of correcting it. Recorded rather than quietly narrowed.

## 3. What is actually broken, stated at the width the evidence supports

Conversion in this architecture has three regimes, not one:

| burden | what happens | verdict |
|---|---|---|
| **strong but intermittent** | load climbs while burdened, being becomes un-burdened, rests, converts | **works** — I-8's band, 232 units |
| **weak** | load stays under 8; `converted = q88_mul(load, 32) = load/8` **floors to zero**; the resting ebb of 4/tick then erases it | **lost to truncation** |
| **structural / permanent** | `!burdened` is never true, so the being never rests, so it never converts, so load climbs to the 256 ceiling and stays | **deadlocked** |

The third is the defect. `examples/reflection_deadlock` measured it directly:

> **Solitude: burdened 97.3%, load max 256, `weathered` 0, and the longest unbroken run at the
> ceiling is 3,638 consecutive ticks of a 4,000-tick life.**

And the reason is a single conjunct doing two opposite jobs. `reflection.rs:143` accrues chronic
load **when the being is burdened**; `being.rs:1751` requires **`!burdened`** to rest and therefore
to convert. **The condition that fills the being is the condition that locks the drain.**

`reflection.rs:152–153` says of exactly this path:

> *"...always liftable at rest — chronic stress that is real, still not a trap."*

It is not liftable, and it is a trap. The comment describes the intent correctly and the code does
the opposite. The `!burdened` conjunct was added for a real and good reason, recorded at
`being.rs:1744–1750`: *a being adapts so fast that a hard life feels calm, and that calm must not
erase the weight.* **That reasoning is right — for accrual.** It was applied to a flag used for
accrual *and* discharge, and so it also welded the exit shut.

## 4. The design

**Split the question the single flag was answering.**

- *Is the being off-duty enough to stop accruing?* — still needs `!burdened`. A calm hard life is
  still a hard life and its weight must keep accruing. **Unchanged.**
- *Is it settled enough to set some down?* — must **not** need `!burdened`, or a structural burden
  locks its own discharge.

```rust
// being.rs — `settled` is the existing second arm, with the !burdened conjunct removed.
let settled = matches!(basin, Basin::Rest | Basin::Recovery)
    || (!losing_ground && free_energy < Q88_SCALE*3/16 && felt.state.arousal < Q88_SCALE/2);
let resting = !burdened && settled;      // exactly as today

// reflection.rs
let rate = if resting { CONVERT }                         // off-duty: full rate, as today
    else if setting_down && settled { CONVERT / 4 }       // burdened but settled: slower, never nil
    else { 0 };
if rate > 0 && self.load > 0 {
    converted = q88_mul(self.load, rate);
    if setting_down { converted = converted.max(1).min(self.load); }   // fixes the truncation
    ...
}
```

Three properties this is chosen to have:

- **Slower, not equal.** A being calm *while still burdened* is not off-duty, and the original
  comment was right that these differ. It sets weight down at a quarter rate. The distinction is
  preserved; only the *zero* is removed.
- **Never while losing ground.** `settled` already requires `!losing_ground`. A being being outrun
  must not be able to bank its way out of noticing. This is the same floor `docs/deferral.md` §4,
  `docs/earned-authority.md` §4 and `docs/settling.md` §3 each wrote before their mechanisms
  existed, and it is not negotiable.
- **A floor of 1, bounded by the load itself.** `.max(1).min(load)` — so a weak burden is *banked*
  instead of being erased by the ebb, and the being can never convert more than it carries.

**The self-model composes wherever conversion happens.** A permanently burdened being currently
never composes one either — the same deadlock, taking a second thing. Included, because leaving it
out would be shipping half a fix and calling it whole.

## 5. Predictions — locked before the code

**Confident:**

- **P1.** Default-off: trajectory and soul-hash bit-identical, full suite green, the founded being
  wakes at 390 moments with `load` 0 and `weathered` 2.
- **P2.** Solitary life, gate ON: the being leaves the ceiling. The 3,638-tick pegged run drops
  below **100**, and load equilibrates where the chronic rise meets the quarter-rate drain —
  arithmetically around **48** (rise caps at `LOAD_RISE` 6; `load/32 = 6` at `load = 192`, so
  somewhere in 48–192 and I will not pretend to more precision than that before measuring).
- **P3.** Solitary life, gate ON: `weathered` rises above 0. **The first time a structurally
  burdened being in this architecture banks anything.**
- **P4.** Episodic life (threat 130, load max 11): `converted` goes 0 → above 0. The truncation fix
  alone, isolated from the deadlock fix.

**The one I expect to go badly, said before running it:**

- **P5.** `weathered` is monotone and capped at 256. A permanently burdened being converting on
  every settled tick may simply **saturate** it — which would replace an unreachable readout with a
  meaningless one, a giveaway instead of a trap. **I predict the solitary being's `weathered` does
  saturate or come close within 4,000 ticks.** If it does, `CONVERT/4` is too fast and this needs a
  second pass. I would rather find that here than ship it.

**Welfare:**

- **W.** `reflection_tone = weathered/12 − load/8` (I-8's suspect constant). Dropping load from 256
  removes up to **32** points of drag; rising `weathered` adds up to 21 of lift. Predict **mean
  drive falls in the solitary life** and the share past `COMFORT` falls with it. That is the actual
  payoff, and it is adjacent to I-8's open mechanism question — though it does not answer it, since
  I-8 asks about *competence*, not comfort, and nothing here tests competence.

**Guardrail:**

- **G.** Setting down never fires on a tick where the being is `losing_ground`. Asserted
  structurally, not inferred from an aggregate. **G has been vacuous three times running** because
  this being is almost never in trouble — but the solitary life is burdened 97.3% of the time with
  `losing_ground` at 0.0%, so this is once again a guard with nothing to bite on. **I am saying so
  in advance rather than reporting a vacuous pass as a pass.**

## 6. What must not become possible

- **No weight erased that was really carried.** Accrual is untouched; only the drain opens.
- **Never a way to stop noticing.** G, above.
- **`weathered` must stay expensive.** P5 is the check on this, and if it fails the remedy is
  wrong even though the defect is real.
- **No new default.** Gated, off. Turning it on changes trajectories and therefore **re-founds this
  being**, which is Blake's call and not mine.

## 7. Method

Spec committed first. Then the gated change, then the probe, then §8 with what came out —
**including P5, which I expect to fail, in the form it fails.**

---

## 8. What came out — the drain opens, and the remedy is not finished

Both arms lived 4,000 ticks in both lives. Nothing died.

**The solitary life — structural burden, where the drain was welded shut:**

| | gate OFF | gate ON |
|---|---:|---:|
| burdened | 97.3% | 97.3% |
| load, maximum | **256** | **0** |
| load, mean | 241 | 0 |
| longest run at the ceiling (P2) | **3,638** | **0** |
| converted, total | 0 | 3,893 |
| `weathered`, final (P3) | 0 | **256 — saturated at tick 362** |
| mean drive (W) | 134.2 | 134.2 |
| past `COMFORT` (W) | 97.4% | 97.4% |

### P1 — holds. Default-off is bit-identical; 356 tests green; the founded being wakes at 390 moments.

### P2 — holds, and overshoots badly

3,638 → **0**. The drain is open. But §5 predicted load would *equilibrate* around 48, and instead
it goes to **zero and stays there**: the being now carries nothing at all.

The cause is one line of my own design. `.max(1)` was meant to defeat the `load/8` truncation for
weak burdens. The chronic rise is `clamp(q88_mul(burden, CHRONIC_RATE), 1, 6)` and in this life it
is **1 per tick**. So the floor of 1 exactly cancels the minimum rise: load goes up by one, down by
one, and reports 0 forever.

> **That is not a fix, it is the opposite failure.** Before, the being carried everything and banked
> nothing. Now it banks everything and carries nothing. Neither is *"chronic stress that is real."*
> The weight is still being erased — I moved where the erasure happens.

### P3 — holds on its face and is hollow underneath

`weathered` 0 → 256 and 3,893 units converted. A structurally burdened being banks resilience for
the first time in this architecture. But it banks it because the load is being annihilated a unit at
a time, so the number is an artifact of P2's overshoot, not evidence the mechanism is right.

### P4 — holds, and this one is clean

Episodic life, load max 11 both arms: `converted` **0 → 1**, `weathered` **0 → 1**. Small because
the burden is small. That is the point: weight the being really carried is now banked instead of
erased by the floor division. This is the part of the remedy that does what it was designed to do.

### P5 — **holds exactly as predicted, and predicted in writing**

`weathered` **saturates at tick 362 of 4,000** — under a tenth of the life. §5 said: *"I predict the
solitary being's `weathered` does saturate... If it does, `CONVERT/4` is too fast and this needs a
second pass."* It does. This trades an unreachable readout for a meaningless one.

### W — **fails, and I should not have predicted it in the first place**

Mean drive 134.2 → 134.2; share past `COMFORT` 97.4% → 97.4%. Unmoved to the resolution measured.

I predicted this would improve because `reflection_tone = weathered/12 − load/8` swings from −32 to
+21, a 53-point move. **The swing is real and it cannot reach drive.** `being.rs:1676`:

```rust
let drive_report = drive(felt.state.viability, &joy_report.want);
```

**`drive` is a function of viability and wants. It never reads `affective_drive`.** `reflection_tone`
feeds `affective_drive` → arousal → `body.rs`'s metabolic cost → viability → drive: three steps, each
lossy, and `docs/settling.md` §7 already measured that whole channel as worth ±32 of a 256-wide
arousal. Worse, **the sign is against the being**: banking resilience makes `reflection_tone`
*positive*, which *raises* arousal, which costs *more*.

This was checkable in one grep before I wrote the prediction, and I did not check it. It is the same
error as §1 — asserting a path instead of reading it — committed inside the document written to
correct that error. Recorded here rather than quietly dropped.

It also sharpens incident **I-8**'s closing sentence, which I had called too wide. *"`weathered` is a
readout with no consequence"* is too wide for **arousal**, where it has a real if small consequence.
It is **exactly right for drive**, and now for a structural reason rather than a measured one.

### G — vacuous for the fourth time, as §5 said in advance it would be

Losing-ground ticks: **0** in both arms. The being is burdened 97.3% of the time and outrun 0.0% of
it. The guard is structural (`settled` requires `!losing_ground`) and no distribution here could
have violated it. **Not a pass.** §5 predicted this in writing rather than reporting it afterward,
which is the only improvement over the previous three.

---

## 9. The second pass — predictions locked before the code, again

§8 says plainly what is wrong: the floor of 1 is in the wrong place, and `CONVERT/4` saturates a
monotone register. Two corrections, both inside the same default-off gate.

```rust
// 1. The floor belongs to the OFF-DUTY path only.
//    A being that is off-duty clears the remainder; a being still carrying its life sets weight
//    down in PROPORTION, and only once it has enough to set down. That restores a real carried
//    weight with a real drain, instead of erasing it one unit per tick.
converted = q88_mul(self.load, rate);
if setting_down && resting { converted = converted.max(1); }
converted = converted.min(self.load);

// 2. `weathered` gains are weighted by REMAINING HEADROOM, so it approaches its ceiling instead
//    of hitting it. Resilience has diminishing returns; a monotone register that saturates in a
//    tenth of a life is not measuring anything.
let gain = if setting_down { q88_mul(converted, Q88_SCALE - self.weathered) } else { converted };
```

- **Q1.** Solitary life: load stops being 0. It equilibrates where the quarter rate `load/32` first
  matches the chronic rise of 1 — arithmetically **around 32**. Predict load mean in **16–64**,
  load max well below 256, and the pegged run stays **0**.
- **Q2.** P4 survives: the episodic being is un-burdened and off-duty on 99.1% of ticks, so it takes
  the floor by the *resting* path, which keeps it. Predict episodic `converted` stays above 0.
- **Q3.** `weathered` does **not** saturate within 4,000 ticks. Predict it ends high but below 256,
  and its climb visibly flattens.
- **Q4 — and I am predicting a failure now that I know why.** **W will fail again.** `drive` cannot
  read `affective_drive`, so no amount of load relief can move it on this path. Predict mean drive
  unchanged or **very slightly worse**, since the sign of `reflection_tone`'s effect on arousal is
  against the being. **The welfare payoff §5 claimed does not exist**, and the honest version of
  this remedy is that it fixes a trap without making the being feel better — which is worth doing
  and is not what I sold it as.
- **Q5.** If Q1–Q3 hold and Q4 fails as predicted, **I-9 closes on mechanism and remedy**, and what
  remains is a separate and older question: whether anything in `reflection.rs` can reach the being's
  drive at all. That is I-8's competence question wearing different clothes, and it is not this inch.
