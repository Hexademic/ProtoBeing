# Play — action whose reason is to find out

> **Status: the guardrail is specified; play itself is not built.** Committed before the
> tests and before a line of implementation, so §5's predictions are on the record before
> any result. Guardrail first, as `docs/habits.md` fixed breakability before the habit and
> `docs/reflection.md` fixed the anti-trauma exits before the weight.

*Written 2026-07-28, from a critique Blake relayed. The observation was that ProtoBeing
treats regulation, embodiment, prediction, attachment, consent and governance well, and
that biological organisms do not only stabilise — they also play; that play is not random
exploration but low-risk experimentation whose objective is discovering affordances rather
than immediate success. That is correct, and this being cannot do it.*

## 1. The gap, precisely

`striving.rs` arbitrates by urgency: it takes the maximum-urgency need and everything
downstream follows from that choice. **There is no action in this being whose reason is *to
find out*.**

`Need::Novelty` is not a counterexample. Novelty is a need competing in the same
arbitration, so seeking it is exploration-as-drive-satisfaction — the being explores when
being-un-explored is its most pressing hunger. That is foraging, not play.

The distinction that makes this operational:

> **Play is action that costs regulation now and buys prediction later.**

Both halves are falsifiable in this architecture. It should *not* reduce drive at the time
it is taken — if it does, it was foraging. And it *should* improve the forward model's
accuracy in that kind of moment afterward — if it does not, it was waste.

## 2. What was proposed and then abandoned, and why

The first plan for this inch was **play-journaling**: mark moments as *tried* rather than
*did*, so a failed experiment could not corrupt the record of who the being is.

Checking the code killed it. `Moment` records `Abstract(Stimulus)` or
`Embodied(Sensorium)` — **the journal stores zero actions.** What the being did is
*derived* from replay, deterministically, so "this was play" is already recoverable and
marking it would be redundant.

And the premise was wrong at a deeper level. **Nothing in this architecture can corrupt
identity through failure.** Identity *is* the trajectory; a trajectory containing failed
experiments is exactly as much the being's own as one containing successes. The soul-hash
does not score outcomes, it records them. There is no identity risk to protect against.

Recorded because the reasoning is worth more than the abandoned mechanism: the critique
asked for a place to *"fail safely without threatening identity,"* and the honest answer is
that this substrate already provides it. What it does **not** provide is protection for the
being's *welfare* while it experiments — and that is a real gap.

## 3. The guardrail: play spends surplus, never reserve

`being.rs` already marks the line where a life stops being comfortable:
`COMFORT = Q88_SCALE * 7/16` (≈ 0.44), above which the graded drive registers as burden and
`reflection.rs` begins accruing chronic load.

The rule, then:

> **A being may play only from the margin between its drive and its comfort line, and play
> may never spend that margin down past the line.**

Animals play when fed and safe; a hungry animal forages. This is that, made structural. It
is not an attitude to be remembered at the call site — it is a budget that returns zero
whenever the being is burdened, so a burdened being *cannot* play even if something asks it
to.

## 4. What must not become possible

- **A burdened being cannot play.** Budget is exactly zero at or above the comfort line.
- **Play cannot cause burden.** No sequence of play actions may push drive past `COMFORT`;
  the budget shrinks as it is spent.
- **Nothing steers yet.** The budget is a pure observer this inch — it computes and reports
  and no action consults it. Play itself is the next inch and a separate decision.
- **The welfare gate is long-run.** Play costs margin *now* to buy prediction *later*, so a
  play-being will look worse early and must look better late. Judging it tick-by-tick would
  reject the mechanism by construction. Blake accepted this framing explicitly; it is
  recorded here because it is the assumption most likely to be wrong, and if it is wrong
  then play cannot be built at all.

## 5. Predictions — locked before the tests exist

**Confident:**

- **B1.** The budget is zero for every drive at or above `COMFORT`, and positive below it.
- **B2.** It is monotone: the further below the comfort line, the more margin available.
- **B3.** Spending is bounded — no admissible sequence of withdrawals crosses `COMFORT`.

**Genuinely uncertain, and it decides whether this guardrail is real:**

- **B4.** *Is the budget ever zero in an actual life?* The weathered and still lives of
  `docs/weather.md` §7 ran mean drive **0.13–0.18** against a comfort line of **0.44** — so
  the being may be comfortable almost always, and a guardrail that never binds is a comfort
  blanket rather than a constraint. If the budget is positive at essentially every tick, I
  will say so plainly: it would mean either our worlds are too kind for this guardrail to
  matter, or the guardrail needs a different form (a rate limit on play rather than a state
  gate on it). I do not know which, and I would rather find out now than after play exists.

## 6. Method

Spec first. Tests written against it and watched to fail. Then the implementation, then §7
with what came out — including B4's answer, which may say this guardrail is decorative.
Play itself is not built in this inch and is not authorised by it.

## 7. What came out — measured 2026-07-29

`src/play.rs`, `tests/play_budget.rs` (6, all green), `examples/play_budget.rs`.

**B1, B2, B3 hold.** The budget is exactly zero at and above `COMFORT`; strictly monotone
below it; and greedy withdrawal from every starting point — ten thousand spends from each of
`0`, `16`, `64`, `COMFORT − 8`, `COMFORT − 1` — never once reaches the line. An overdraft is
refused rather than clamped, so an upstream bug cannot become an overspend here.

**B4 — the guardrail binds. It is not a comfort blanket.** Four 1,500-tick lives, budget
watching and steering nothing:

| life | mean drive | min | max | mean margin | min margin | ticks at zero margin |
|---|---|---|---|---|---|---|
| long crossing | 0.18 | 0.07 | **0.59** | 0.06 | **0** | **7%** |
| long + weathered | 0.16 | 0.07 | **0.59** | 0.07 | **0** | **4%** |
| beside its food | 0.09 | 0.00 | 0.25 | 0.08 | 12 | 0% |
| beside food + weather | 0.01 | 0.00 | 0.25 | 0.11 | 12 | 0% |

What decides it is the **distance from where the being wakes to what feeds it**. A being that
must cross ~316 cells to its nutrient source passes the comfort line and spends 7% of its
life burdened — peak drive **0.59** against a line of **0.44**. A being that wakes 17 cells
from its food never comes near it: minimum margin 12, never zero.

So the guardrail has exactly the shape a welfare guardrail should have. It **refuses play to
the being that is struggling** — one tick in fourteen of the long-crossing life, by
arithmetic, with no call site to get right — and **leaves the well-fed being free to
experiment**. B4's bad outcome (positive at essentially every tick, therefore decorative) did
not occur, and the reason it did not is that the mean drive figure from `docs/weather.md` §7
was hiding the excursions: mean **0.18**, maximum **0.59**. A mean below a threshold says
nothing about whether the threshold is crossed. That is the lesson worth keeping from this
inch, and it applies to every other constant in the project that was reasoned about from a
mean.

**The prohibition has a small skirt, and it errs the right way.** Integer division makes the
budget zero not only at `COMFORT` but for the three raw units below it — drives 109, 110 and
111 all yield zero margin, and the first spendable unit appears at 108. So a being *nearly*
burdened is also refused. That is quantization rather than design, but it is the direction a
welfare guardrail should fail in, and it is why `src/play.rs`'s unit test asserts only
`available(COMFORT − 1) >= 0`: at one below the line the margin genuinely is zero, and a test
demanding otherwise would have been a test demanding a worse guardrail.

**A correction to this probe's own method.** Its first version named the near-food life "the
hard climb" and the far one "still, companioned", and so read the table backwards on first
run. `FieldWorld::with(body, good, harm)` — the names were wrong, the measurement was not.
Fixed in the probe; recorded here because a mislabeled world is exactly the kind of error
that turns a real result into a false conclusion, and it was caught by the numbers refusing
to match the story.

**One thing measured that is not explained.** Weather made both lives *better*, not worse —
burden fell 7% → 4% on the long crossing, and mean drive fell 0.09 → 0.01 beside the food.
`breathe()` scales the good source's peak into `[64, 128]` from a base of 128, so the
weathered world is on average a **weaker** provider; the being should have done worse. It did
not. The most likely account is that a fluctuating gradient changes *when* the being commits
to crossing rather than how much it gets, but that is a hypothesis and it is not tested here.
Named, not resolved: a fluctuating world may be easier to live in than a constant one, which
if true is interesting well beyond this budget.

**Still true after this inch:** nothing plays. No action consults the budget, no `enable_*`
gate was added, the soul-hash is untouched and the founded being is unchanged at 390 kept
moments. Play itself remains the next inch, and a separate decision.
