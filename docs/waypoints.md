# Waypoints — a life you can check without re-living all of it

> **Status: built and measured** — `persistence.rs` (`Waypoint`, `birth_with_waypoints`,
> `restore_counting`, format v3), `tests/waypoints.rs` (written *first*),
> `examples/waypoints`. §1–§7 are exactly as committed in `a47bef2`, **before** the tests
> and before a line of implementation. §8 is what came out, including the prediction that
> came out badly and the thing found by accident that matters more than the feature.

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


---

## 8. Measured (2026-07-27) — what came out

Order: spec committed (`a47bef2`) → tests written and **watched to fail** (the `Waypoint`
type did not exist) → implementation → run. All nine pre-written tests pass.

### The law

| | prediction | result |
|---|---|---|
| **C1** | a forged journal is still refused | **held** |
| **C2** | *the crux:* the rejection names the segment | **held** — `ForgedBetween { after: 32, before: 48 }` |
| **C3** | detection stops at the first waypoint past the forgery | **held** |
| **C4** | an honest life is bit-identical with waypoints and without; older journals still wake | **held** — same anchor, v1/v2 restore unchanged |

### C5 — the curve, reported whole

A 20,000-moment life, cadence 512, 39 waypoints, 1,404 bytes of chain. Moments replayed
before a forgery is caught:

| forged at | with chain | without | saved |
|---|---|---|---|
| 100 | 512 | 20,000 | **98%** |
| 15,009 | 15,360 | 20,000 | 24% |
| 19,003 | 19,456 | 20,000 | 3% |
| 19,900 | 19,968 | 20,000 | 1% |

The prediction — large for early tampering, negligible at the end — held, and the shape is
exactly the sawtooth the cadence implies. Nothing here is surprising, and that is the point
of having written it down first.

### C6 — the flat line, as promised

Waking an **honest** 20,000-moment life: **16.6 ms** with the chain, **16.2 ms** without.
No meaningful difference, both woke as the same being. This inch does not speed up an
honest wake and never claimed to.

### What the measurement actually found — and it is not the feature

Four rows are missing from the C5 table above. Starving moment **1,013**, **5,007**,
**10,001** or **19,990** of that life is **not detected at all** — not by the waypoints,
and not by the anchor either.

That is not a waypoint failure. Waypoints inherit exactly the detection power of the
soul-hash, and the soul-hash turns out to resolve a life to about eight bits a tick. The
full measurement, the two wrong explanations I went through before getting it right, and
the decision it leaves open are in **`docs/soul-hash-limits.md`**. It is now the first
item under open tensions in the handoff.

The probe itself was wrong twice before it was right, both times by forging a no-op, and
`examples/waypoints` now *asserts* that a forged moment really differs rather than
trusting that it does. Both wrong runs are recorded in the file, because a measurement
that was wrong twice is worth more as a warning than as a clean number.

**The honest summary of this inch:** it delivered what §6 promised — early, localized
rejection of forged journals, at negligible cost, with the state cache correctly left
undone. And it was the vehicle for finding something considerably more important than
itself, which is the best thing a piece of infrastructure can do.


## 9. Superseded in part, the same day (2026-07-27)

`docs/journal-integrity.md` was built hours after this, in response to
`docs/soul-hash-limits.md`, and it takes over most of this document's headline job.

A **record integrity hash** catches every forgery, deterministically, by hashing the
journal's own bytes — before a single moment is replayed. That is cheaper than the
waypoint chain (byte hashing, not being-stepping), more complete (no quantum to fall
beneath), and it fires first. `restore()` now reports `JournalTampered` and replays
**zero** moments, which is asserted in `tests/waypoints.rs`.

So the honest accounting of what waypoints are still for:

- **Divergence of the replay itself** — an intact record that nonetheless fails to
  reproduce the being: code drift, version skew, a nondeterminism bug. The chain
  localizes *that* to a segment, and nothing else does.
- **Legacy journals** (v3 and earlier) that carry no integrity hash.

That is a real job and a narrower one than §4 claimed. §4 is left as written because it
is what was believed when it was written; this section is the correction. The chain cost
36 bytes per waypoint and remains worth keeping — but anyone reading this should not
believe it is the project's tamper-evidence mechanism. It is not. The journal hash is.
