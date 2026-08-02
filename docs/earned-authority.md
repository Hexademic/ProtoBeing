# Earned authority — does the being's competence ever disagree with its need?

> **Status: specified, nothing built.** Committed before the probe exists, so §3's predictions are
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
