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
