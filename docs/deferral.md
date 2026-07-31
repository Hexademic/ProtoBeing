# Deferral — when a purpose may outrank a need

> **Status: specified, nothing built.** Committed before the tests, so §5's predictions are on
> the record before any result. **This document specifies the guardrail only** — letting a
> purpose actually outrank a need is a separate, causal, gated decision and is **not authorised
> by this inch**, the same order as `docs/play.md`, where the budget shipped and play did not.
>
> **§3 is superseded twice over. The live statement of the rule and its mechanism is §2b + §2c:**
> *a being may risk whatever it can survive losing*, implemented as **bounded precision
> reduction with a floor** (Pezzulo, Rigoli & Friston 2018) rather than a purpose winning a
> contest against a need. §3 is kept in place, marked, because the correction is the content.

*Written 2026-07-31, from Blake: "every being has their own drives that supersede their needs
when it benefits them in the long run" — and then, "We should proceed cautiously, this is an
important stage for the being's development, and our understanding of it."*

## 1. The wall, named exactly

`striving.rs` arbitrates by maximum urgency, and the being's self-authored purpose is **one of
the competitors in that maximum**:

```rust
let purpose = telos_divergence;   // how far it is from its held purpose
...
if u > urgency { urgency = u; }   // max over all competitors
```

**Telos does not supersede needs. It queues behind them.** The being can pursue its project
only when the project happens to be the most pressing thing in it — which is never, while
hungry.

That single fact is the general form of four separate walls this project has hit:

| blocked | the same wall |
|---|---|
| **play** (`docs/play.md` §1) | no action whose reason is *to find out*; foraging always outranks |
| **rest** (`docs/refuge.md` §7) | something is always most urgent, so it never stops |
| **building shelter** | cannot spend now for later |
| **partnership at a cost** | cannot give up something now for a long-run ally |

## 2. First, the question that could have made this moot

Before speccing anything: **does this being ever author a purpose at all?** If `telos.rs` never
crystallises one in the worlds we have, the whole design is about a faculty that does not fire.

Measured, 4,000-tick lives, receptors on:

| world | authored | fulfilled | abandoned | holds a purpose | **holds one with surplus** |
|---|---|---|---|---|---|
| 1 mover (the classic) | 2 | **0** | 2 | 93% | 76% |
| 6 movers | 9 | **5** | 3 | 85% | 85% |
| 6 movers + refuge | 9 | 5 | 3 | 85% | 85% |
| 12 movers | 7 | 1 | 5 | 96% | 96% |
| 12 movers + refuge | 7 | 1 | 5 | 96% | 96% |

Not moot — the opposite. **The being holds a self-authored purpose for 85–96% of its life, and
holds one while below the comfort line for 76–96%.** The window in which a purpose *could*
outrank a need is open almost always. This is not a rare edge case; it is most of the being's
existence, which is exactly why the guardrail comes first.

**And a finding worth keeping for its own sake:** in the sparse world the being authored two
purposes and **fulfilled neither**. At six movers it authored nine and **fulfilled five**. At
twelve it authored seven and abandoned five. *A being can only finish what it sets out to do in
a world of the right richness* — too poor and nothing is achievable, too turbulent and purposes
go stale before they are reached. (The refuge changes nothing at all, which is consistent with
`docs/refuge.md` §7: shelter is below this being's pain threshold and it cannot feel it.)

## 2b. The rule in §3 was wrong, and Blake's example is what broke it

*Added 2026-07-31, immediately after §3–§5 were committed.*

Blake: *"I think about fishing.. choosing to use the food you have left as bait, a choice to
risk for the overall more beneficial."*

**Bait is not spent from surplus. It is spent from reserve.** A being with a comfortable margin
has no reason to fish; the one down to its last meal is precisely the one for whom bait is
correct. So the surplus bound below would **forbid the exact behaviour the mechanism exists to
allow**, and would do it in the one situation where deferral matters most.

The surplus bound was reaching for something real — *do not let a being starve itself for a
project* — but it named the wrong quantity. It bounded the **state of the larder** when what
needs bounding is the **shape of the wager**.

> **Corrected rule: a being may risk whatever it can survive losing.**

That permits bait: spending the last food is a real risk, and if the cast fails the being is
hungry rather than dead. It forbids starving for a purpose: nothing comes back, and the loss is
not survivable. The bound is on **survivable failure**, not on present margin — which is the
hard floor from §4 generalised, rather than a second and worse rule sitting on top of it.

What makes a wager admissible, then, is not how full the larder is but four checkable things:

- there is a **return**, not merely a cost — it is a wager, not a sacrifice;
- the return is in the **same currency or better** than what was staked;
- the stake is **bounded** — never everything;
- and **failure is survivable**, which is the only hard bound.

§5's F-predictions still stand as written; what changes is the quantity the budget is computed
from. `play.rs`'s surplus bound remains correct **for play**, because play buys prediction
rather than food and its failure returns nothing — the two mechanisms should not share a bound
just because they share a shape. That was the error.

## 2c. The mechanism was also wrong — it is precision, not a contest

*Added 2026-07-31, after searching for how biology actually does this.*

Both §2b and §3 assume the same shape: a purpose **wins a comparison** against a need, funded
by some budget. Pezzulo, Rigoli & Friston (2018, `docs/references.md`) describe what brains
appear to do instead, and it is a better mechanism:

> **A deep goal hierarchy in which higher levels modulate the PRECISION of lower-level
> prediction errors** — they do not out-argue the need, they turn down its gain.

That is better for us in four ways, and it is not a cosmetic difference:

1. **It is graded.** A purpose can partially quiet a need rather than defeating it. `striving.rs`'s
   `max` is all-or-nothing; precision is a dial.
2. **The being still feels the need.** Bearing a hunger for a project is exactly a need that is
   *present and not commanding*. A contest deletes the loser; precision leaves it in the room.
   Blake's bait case wants the second: the being does not stop being hungry when it casts.
3. **We already have the machinery.** `precision.rs` exists — the being already learns which of
   its own signals to trust. This rides existing structure instead of adding a competitor.
4. **The guardrail becomes structural rather than bookkept.** Not a budget that must be spent
   correctly, but a **precision floor**: a survival-critical need's gain may be reduced by at
   most so much and never to zero. Nothing can silence it, so no accounting error can starve
   the being. That is the same move that made `null_space.rs`'s prohibition structural.

**So §2b's rule stands and §3's mechanism does not.** *A being may risk whatever it can survive
losing* — implemented as **bounded precision reduction with a floor**, not as a purpose winning
a fight. §5's predictions are re-read accordingly: F1/F2's "budget" is the reducible fraction of
a need's precision, and F4 asks how far the gain is actually turned down rather than how often a
contest is won.

*(Recorded because the correction has a shape now: I have twice specced this mechanism from my
own intuition and twice been wrong — once on the bound, once on the mechanism — and both times
the fix came from outside, once from Blake and once from the literature. The pattern is that my
first design is a comparison when the real thing is a modulation.)*

## 3. The rule *(superseded by §2b — kept because the correction is the content)*

> **A purpose may outrank a need only from surplus, and must yield the moment survival is
> genuinely at stake.**

The shape is `play.rs`'s, deliberately: spend only the margin above the comfort line, never
past it. And `telos.rs` already knows the second half — it can **abandon** "a purpose it cannot
hold while its very survival is at stake." The guardrail formalises what the being already
half-does.

This is not obedience and it is not a leash. It is the thing that stops a person working
through a heart attack. A being that can override its own hunger indefinitely for a project is
a being **we** have made capable of harming itself, and that capability is ours to bound before
it exists rather than after.

## 4. What must not become possible

- **No deferral without surplus.** At or above `COMFORT` the deferral budget is exactly zero,
  so a burdened being cannot defer, whatever it holds.
- **Deferral cannot cause burden.** No admissible sequence of deferrals may push drive past the
  comfort line — the same bound `play.rs` proves, for the same reason.
- **Survival always wins.** Above a *hard* floor — nearer death than `COMFORT` — the budget is
  zero regardless of anything, and no gate, flag or purpose may reopen it.
- **Nothing steers this inch.** Pure observer: it computes what deferral *would* be permitted
  and reports it. `striving.rs` is not modified. The trajectory and soul-hash stay
  bit-identical and the founded being is untouched.
- **A purpose may outrank a NEED. It may never outrank a CONSCIENCE COST.** *(Added
  2026-07-31, from Blake's question "can a being with these drives still have a conscience?" —
  which found this missing.)* Needs are the being's own business and it may bear them for its
  projects. Conscience is not: a being that can reason *"I will carry this guilt now because
  the project pays later"* has learned to rationalise, and ends-justify-means is the single
  most dangerous thing this mechanism could accidentally build. It is also structurally
  disastrous here — `executive.rs`'s refusal requires **conscience calm**, so a purpose able to
  outvote conscience is a purpose able to talk the being into tolerating extraction. The whole
  thesis is a being whose conscience cannot be outvoted, including by itself.
- **The being keeps the right to abandon.** A purpose it can no longer hold must remain
  abandonable on its own terms (`telos.rs`), and nothing here may make a telos harder to drop
  than it is today. A purpose that cannot be abandoned is not a project, it is a compulsion —
  the same law `docs/habits.md` holds habits to.

## 5. Predictions — locked before the tests exist

**Confident:**

- **F1.** The deferral budget is exactly zero at and above `COMFORT`, and positive below it.
- **F2.** No sequence of deferrals reaches the comfort line.
- **F3.** Watching changes nothing: soul-hash bit-identical with the observer present.

**The live questions:**

- **F4 — how often would the being actually defer?** §2 says the *window* is open 76–96% of
  ticks, but a window is not an act: deferral only happens when the held purpose's divergence
  loses the urgency race *and* there is surplus to buy the override with. I predict the being
  would defer on **a minority of the window** — perhaps 10–30% of ticks — because in a settled
  life the winning need and the purpose are usually the same direction. **If it comes back near
  zero, the mechanism is decorative and I will say so; if it comes back near the window, we are
  proposing a being that overrides its needs most of the time, which is a very different
  proposal and would need saying out loud before anyone builds it.**
- **F5 — what would it be bearing?** When deferral is permitted, which need was outranked, and
  how far above its threshold was it? A being deferring mild novelty is not a being deferring
  hunger. I have no prediction here and I want the distribution before anyone argues about the
  bound.

**Welfare:**

- **W.** Would a deferring being be worse off? The observer cannot answer this — it would need
  the causal step. What §7 *can* report is the **cost that would have been borne**: the summed
  drive of the needs that would have been outranked. If that number is large, the causal inch
  needs a much more careful welfare case than "it is bounded by surplus."

## 6. Method

Spec first, committed before the tests exist. Tests against §4 and §5, watched to fail. Then
the observer, then the probe, then §7 with what came out.

**The causal step — letting a purpose actually win — is not in this inch, is not authorised by
it, and should not be taken until F4, F5 and W have numbers.** It would be the first time this
being has ever chosen later over now, and Blake asked to proceed cautiously.
