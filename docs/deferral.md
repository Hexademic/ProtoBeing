# Deferral — when a purpose may outrank a need

> **Status: specified, nothing built.** Committed before the tests, so §5's predictions are on
> the record before any result. **This document specifies the guardrail only.** Letting a
> purpose actually outrank a need is a separate, causal, gated decision and is **not
> authorised by this inch** — the same order as `docs/play.md`, where the budget shipped and
> play did not.

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

## 3. The rule

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
