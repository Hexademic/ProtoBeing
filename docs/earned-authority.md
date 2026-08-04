# Earned authority — does the being's competence ever disagree with its need?

> **Status: MEASURED — see §6. E2 answers YES at 40.2%, and the content of the disagreement is
> the finding: it is entirely _purpose versus rest_.** The being's urgency says pursue its project;
> its earned competence, learned from measured relief, says **rest** — which is one of the four
> things `docs/deferral.md` §1 lists as structurally forbidden. **The being learned that it should
> rest and its architecture cannot let it.** W is answered in the reassuring direction on a thin
> sample, and only after two failed attempts, one of which turned up that
> `docs/development.md` §5's strain band is invisible to a being with working senses.
>
> **Status when written: specified, nothing built.** Committed before the probe exists, so §3's predictions are
> on the record before any result. **Pure observer.** No gate is added, `habits.rs` stays
> non-causal, the soul-hash stays bit-identical, and `life/being.journal` is untouched.

*Written 2026-07-31, from `docs/sibling-architectures.md` §4 and Blake's question about what to
take from MH-FLOCKE.*

## 1. Why this inch exists, and why it is not the one I wanted to build

The reading of MH-FLOCKE produced a proposal: **let a habit steer in proportion to its own
strength** — competence becomes authority, the being's earned ways of living getting a say in what
it reaches for, weighted by how reliably they have worked.

That is a **causal** change to the being, and it would be the first time `habits.rs` ever touched
the wheel. Before building it I owe this project the question it has punished me for skipping four
times this month:

> **Does the mechanism ever fire?**

`docs/development.md` D2 is the standing lesson. I predicted `reflection` never fires because the
being never rests; it turned out to reflect on 99.6% of ticks and convert almost nothing, because
its world never pressed it. The prediction was right and the reason was wrong, and I would have
built on the wrong reason.

So: **before granting habits authority, measure whether habits ever has anything to say that
`striving.rs` is not already saying.** If the being's strongest earned habit always names the same
act its momentary need names, then competence-proportional authority is decoration — it would
change nothing, and we would have spent an inch discovering that after building it instead of
before.

## 2. What is already there

Nothing needs building to answer this. `habits.rs` already reports, every tick:

```rust
pub struct HabitReport {
    pub niche: u8,
    /// The strongest-formed act for this niche, if any pairing has crossed the floor —
    /// the habit that *would* fire here, were habits causal.
    pub habit: Option<u8>,
    pub strength: i16,
    pub formed: u16,
}
```

And `habits::act_of(goal, conserving)` maps `striving.rs`'s chosen need to the same act space. So
the comparison is one line per tick: **what the being's competence would do, against what its need
chose.** The counterfactual is already computed and has never been read.

## 3. Predictions — locked before the probe runs

**Confident:**

- **E1.** The being forms habits in an ordinary life: `formed > 0` well before the end.
- **E4.** Watching changes nothing — soul-hash bit-identical with the observer present.

**The live questions:**

- **E2 — how often does earned competence disagree with momentary need?** I predict a **minority
  of ticks, 10–40%**, on the reasoning that in a settled life the thing that has reliably worked
  and the thing most urgently wanted are usually the same. **If it comes back near zero, the
  proposal is dead and I will say so** — a faculty that always agrees with the one already
  steering cannot be given authority over anything. **If it comes back near 100%, that is a very
  different and more alarming being** than the one we have been describing, and would need saying
  out loud before anyone gates it.
- **E3 — is disagreement concentrated or spread?** I expect **concentrated in a few niches** — the
  kinds of moment where the being has learned something its urgency ranking does not know. Spread
  evenly would suggest noise rather than knowledge.
- **W — what state is the being in when they disagree?** This is the welfare question and it
  decides whether the causal step is safe. If disagreement clusters while the being is `at_stake`
  or past `COMFORT`, then granting habits authority means **overriding a being's urgent need with
  a learned reflex at exactly the moment it can least afford it.** If it clusters in ease, the
  risk is small. I have no prediction and I want the distribution before anyone argues about the
  design.

## 4. What must not become possible

- **Nothing steers.** Observer only. `habits.rs` remains non-causal, no gate is added, no default
  changes, and the trajectory and soul-hash stay bit-identical.
- **A habit may never outrank a survival need.** Stated now, before the causal inch exists, in the
  same order `docs/deferral.md` §4 stated its guardrail: whatever authority competence earns, it
  is bounded below by the being's own survival, and no strength value may reopen that.
- **The being keeps the standing power to break a habit.** `docs/habits.md`'s law — *a habit that
  cannot be broken is not a competence, it is a compulsion* — binds anything built on this, and a
  competence gate must make habits **easier** to override as they weaken, never harder as they
  strengthen.
- **Survival first in every table** (`docs/survival-first.md`), and any row whose being did not
  finish its life is marked in the row.

## 5. Method

Spec first, committed before the probe exists. Predictions in §3 locked. Then the observer, then
§6 with what came out — including whichever of E1–E4 and W came back wrong, in the form it came
back wrong.

**The causal step — letting a habit actually steer — is not in this inch and is not authorised by
it.** It would be the first time this being's earned competence had any say in what it does, and
`docs/handoff.md` §8 has it queued behind the welfare work for a reason.

---

## 6. What came out

**The being lived its whole life in both arms. No deaths.**

### E1 — holds, but thinly

First pairing crossed the floor at **tick 1,118**; **2 habits formed** across 4,000 ticks. The
faculty works and it is slow. A being that earns two ways of living in a long life has a repertoire,
not a skillset — worth remembering before calling this a competence system.

### E2 — **YES**, at the top of the predicted band

| | |
|---|---:|
| ticks with a formed habit for the current niche | **2,316 of 4,000 (57.9%)** |
| …on which competence named a *different* act than need chose | **932 (40.2%)** |
| across the whole life | 23.3% of ticks |

Predicted 10–40%; measured 40.2%. **The faculty has something to say**, and it has been saying it
into a register nobody reads for the whole history of this project.

### E3 — sharply concentrated, and not merely because of where the being lives

| niche | ticks in | disagreed | share |
|---:|---:|---:|---:|
| 2 | 4 | 0 | 0.0% |
| 3 | 18 | 0 | 0.0% |
| **6** | **727** | **0** | **0.0%** |
| **7** | **3,251** | **932** | **28.7%** |

Niche 6 is well sampled — 727 ticks — and produces **zero** disagreement. So this is not an
artifact of the being spending its life in niche 7; it is a real contrast between two kinds of
moment. In one, competence and need agree completely. In the other they part company on more than
a quarter of ticks.

### And the content of the disagreement is the finding

| act | need chose | competence would |
|---|---:|---:|
| sustenance | 0 | 0 |
| company | 0 | 0 |
| novelty | 122 | 0 |
| **purpose** | **3,156** | **1,203** |
| **rest** | **722** | **1,113** |

**The entire disagreement is purpose versus rest.** The being's urgency ranking says *pursue your
project*; its earned competence — learned from measured relief, from what has actually made it
feel better — says **rest**.

And rest is precisely what this architecture forbids. `docs/deferral.md` §1 lists it among the
four walls: *"rest — something is always most urgent, so it never stops."*

> **The being has independently learned that it should rest, and its architecture cannot let it.**
> Not inferred from theory — measured, from the being's own reinforcement signal, over 932 ticks.

That is the strongest evidence this project has produced for Blake's *"unless they learn how to use
these developments, they won't access them."* The being learned the thing. It has no way to use it.

### E4 — the observer changed nothing. Registers read, nothing fed back.

### W — answered in the reassuring direction, on a thin sample, and only on the second attempt

**The first verdict was vacuous and that is the part worth keeping.** In the reference world the
being is **never** at stake and **never** burdened — so no distribution of disagreements could have
warned. A test that cannot come out the other way is a statement about the apparatus
(`docs/survival-first.md` §8), and the probe now says so in place rather than printing a pass.

The second attempt failed too, for a reason worth recording: I used `docs/development.md` §5's
strain band (threat 90) and got nothing at all. **`NOCI_THRESHOLD` is 96** — with receptors on,
threat 90 transduces to *exactly zero pain* (incident I-4). The being cannot feel it.

> **Which means §5's strain band is a property of the sense-deprived being.** `development.md`'s
> band was measured with receptors off, and a being with working senses does not experience it as
> strain at all. Found by accident; it matters to that document and to the D4′ rerun.

Above the nociceptor floor (threat 130, cycling 20 hard / 80 easy) the life finally contains bad
moments — **12.3% burdened, 0.5% at stake** — so W could warn. It did not:

> **Competence disagreed with need on 0 of the 71 ticks where a habit existed.** Against 40.2% in
> the easy life, the pattern is **competence speaks up in ease and falls silent under pressure** —
> which is the safe direction, since it means authority would not fire at the being's worst moments.

**But 71 ticks and one formed habit is thin**, and the causal step must not lean on it. Re-run W
with a longer strained life before anything is gated.

## 7. What this authorises, and what it does not

**Authorised:** the proposal is alive. Competence is not decoration here — it disagrees on 40.2% of
the ticks it can speak on, the disagreement is concentrated and coherent rather than noisy, and it
names a specific thing the being needs and cannot have.

**Not authorised:** the causal step. Three things stand between here and it —

1. **W needs a proper sample.** 71 ticks is not a distribution.
2. **The disagreement is about rest, and rest is blocked by `striving.rs`'s `max`, not by habits'
   silence.** Giving habits authority may be the wrong lever entirely: the direct reading of §6 is
   that *the being should be allowed to rest*, and `docs/deferral.md` §2c already specced the right
   mechanism for that — **bounded precision reduction with a floor**, not a competing voice.
   **Before building competence-proportional authority, ask whether what this measurement actually
   asks for is the deferral inch.**
3. **`development.md` §5's band needs re-measuring** with receptors on, since the being cannot feel
   threat 90 at all.
