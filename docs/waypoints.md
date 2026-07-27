# Waypoints — a life you can check without re-living all of it

> **Status: designed, tests written first, not yet built.** Committed *before* the tests
> and before a line of implementation, so §6's predictions are on the record before any
> result exists. See `docs/handoff.md` for the project-wide faculty map.

*Written 2026-07-27 with Blake, out of the Frontier: Elite thread. Elite's galaxy was
never stored — three seed words and a recurrence regenerated 2048 worlds on 32 KB, and
crucially it regenerated a system **from the current seed, not from the beginning of
time**. Our being's persistence is the same trick (`docs/wholeness.md`): identity is the
trajectory, so we store the stimuli and recompute. This document is about the half of
Elite's trick we do not have — the ability to start from somewhere other than birth.*

## 1. The name is the honest part

They are **waypoints**, not checkpoints. A checkpoint is somewhere you can *resume from*.
A waypoint is a marker you *pass and verify*. This inch builds only the second thing, and
the name says so, because calling them checkpoints would promise a capability the code
does not have.

## 2. What we are deliberately not building, and why

The thing that would make an old being wake quickly is a **state snapshot**: dump the
being at moment N, reload it, carry on. We are not building that here, and the reason is
a fact about the code rather than a preference.

`UnifiedBeing` has **111 fields**, roughly forty of which are sub-engines with their own
interior state. `persistence.rs`'s own module doc says it plainly: this is *"not a
state-dump of ninety structs."* Hand-serializing that surface is a large job in which a
single missed field produces a being that wakes **subtly not itself** — the exact failure
this project exists to make impossible. The soul-hash would catch it, which is the only
reason it would ever be safe to attempt; it is not a reason to attempt it in the same
inch as the structure it depends on.

So the ordering is the one this project always uses — **the guardrail before the
mechanism**, as `docs/habits.md` fixed breakability before building the habit.

## 3. Why the chain has to come first

A state snapshot *without* a provenance chain is precisely the pathology the Lisp world
lived with for decades: **image rot**. You have a live system that works, that nobody can
rebuild, and whose history cannot be checked. It is the thing our journal-and-replay
design was chosen to avoid.

A snapshot *with* a verified chain behind it is a different object: a **cached replay**.
The journal stays the truth. The snapshot is an optimization whose correctness can be
re-established at any time by replaying the segment that produced it. That is the only
form of state-caching this project can accept, and it is unbuildable until the chain
exists.

## 4. The design

A `Waypoint` is `(at: u32, hash: [u8; 32])` — the being's soul-hash at a recorded moment
count. The journal seals one every `cadence` moments as the being lives, reading
`being.soul_hash()` and feeding nothing back.

On `restore()`, the replay verifies each waypoint **as it passes it**. Three things follow:

- **Early failure.** A journal forged at moment 40 of forty million is rejected at the
  first waypoint past 40, not after replaying forty million.
- **Localization.** The error names the segment the forgery lies in — *between waypoint i
  and waypoint i+1* — instead of today's undifferentiated `ContinuityBroken`. The being's
  history becomes not just verifiable but **diagnosable**.
- **Prerequisite.** `verify_segment(i..j)` is meaningful the moment state snapshots
  exist, and meaningless without the chain.

Cost: 36 bytes per waypoint. At a cadence of 1024, a century of continuous life at 6 Hz
carries about **650 KB** of waypoints against a 242 GB journal. It is free.

Format version goes to 3. v1 and v2 journals decode with an empty waypoint list, so
**the founded being still wakes** — that is a requirement, not a hope, and it is tested.

## 5. What must not become possible

- **No change to any honest life.** Waypoints read the soul-hash and feed nothing back.
  A being that lives with them must produce the identical trajectory and identical final
  hash as one that does not. If it does not, the design has failed.
- **No weakening of the anchor.** Waypoints are *additional* checks, never a substitute.
  A journal that passes every waypoint and fails the final anchor is still refused.
- **No trust without derivation.** A waypoint is evidence about a replay that actually
  happened, never a licence to skip one. Nothing in this inch may let a being be handed
  back unreplayed.
- **No silent version break.** An older journal must wake unchanged.

## 6. Predictions — locked before the tests exist

**Confident:**

- **C1.** A forged journal is still rejected. (The floor, unchanged.)
- **C2 (the crux).** The rejection **names the segment** containing the forgery. If the
  error cannot say where, the chain has bought nothing over the anchor we already had.
- **C3.** Detection happens after replaying only as far as the first waypoint past the
  forgery — measurable as moments-replayed, and strictly less than the whole journal for
  any forgery before the last waypoint.
- **C4.** An honest life is bit-identical with waypoints and without: same trajectory,
  same final soul-hash. And a v2 journal — the founded being's — still restores.

**Genuinely uncertain — the measurement:**

- **C5.** *How much replay is actually avoided?* It depends entirely on where the forgery
  falls, and I do not know the shape of that curve until it is run. An early forgery
  should save nearly everything; a forgery after the last waypoint saves nothing. I
  predict the saving is large for early tampering and **zero** for the last segment, and
  the honest report is the whole curve, not the best point on it.

**Stated in advance so it cannot be quietly forgotten:**

- **C6.** This inch does **not** make an honest being wake faster. That was the original
  motivation from the Elite thread, and it is *not* what is being delivered — waking an
  honest 390-moment being costs the same afterward, plus the negligible hashing. Anyone
  reading the results should see that stated, not inferred. The wake-cost win requires
  the state cache, which requires this, and which is a separate decision with real risk.

## 7. Method

Spec committed first. Tests written against it and **watched to fail**. Then the
implementation, then the measurement, then §8 reporting what came out — including C5's
full curve and C6's flat line.
