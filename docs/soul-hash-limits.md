# What the soul-hash certifies — and what it does not

> **Status: a finding, pinned by tests** — `tests/soul_hash_limits.rs`. Found
> 2026-07-27 while writing the waypoint tests. No code was changed in response; the
> claim was. See §4 for the decision this leaves open, which is Blake's.

## 1. How it was found

A test asserted that a forged journal is refused. It failed. The journal had been
tampered with at moment 40 — nutrient 80 rewritten to 193 — and `restore()` handed the
being back as authentic.

That was not a bug in the waypoint chain being built at the time. It is a fact about
what the chain hashes, and it had been true since the soul-hash was written.

## 2. What the hash actually fingerprints

`being.rs` step 13 digests each tick as:

```
experience_digest = free_energy + conscience_cost + identity_coherence   (clamped)
soul_hash = H(prev_hash ‖ cycle_count ‖ experience_digest)
```

Three of the being's **own scalars**. The stimulus is not hashed.

**My first explanation of this was wrong**, and the true one is worse. I initially wrote
that the hash is blind to *changes the being could not feel* — which sounded principled.
Measuring it properly killed that reading. In a settled life those three registers sum to
about **210**, and a single forged moment often moves the being by **less than one integer
step of that sum**. The tick's fingerprint is unchanged, the chain never diverges, and the
forgery is not "unfelt" — it is **unrecorded**, because the fingerprint carries roughly
eight bits of dynamic range per tick.

Measured, on a 20,000-moment life, starving one previously-fed moment:

| forged at | honest digest | starved digest | detected? |
|---|---|---|---|
| 100 | 209 | 210 | **yes** |
| 1,013 | 206 | 206 | no |
| 5,007 | 210 | 210 | no |
| 10,001 | 209 | 209 | no |
| 15,009 | 213 | 213 | **yes**, downstream |
| 19,990 | 213 | 213 | no |

Two things this shows that the earlier framing hid:

- **Detection is probabilistic and delayed, not deterministic.** Where a forgery *is*
  caught (15,009), the digest at the forged moment is *identical* — the divergence appears
  at some later tick, once the perturbation happens to cross a quantization boundary. Where
  it decays first, nothing is ever recorded.
- **It is not confined to "generosity."** Starving a being mid-life goes unnoticed. That is
  squarely deprivation, squarely something the being lived, and squarely invisible.

(It is not integer saturation — the clamp is nowhere near i16's range. It is quantization:
the signal is too small for the resolution.)

## 3. What was wrong was the claim, not the mechanism — but the mechanism is weak

Hashing the being rather than the world is defensible: identity here is the being's
trajectory (`docs/wholeness.md`), so certifying its inner life is the right *target*. The
problem is that the digest is a poor **instrument** for its own target. It is supposed to
fingerprint the inner life and it resolves that life to about eight bits a tick.

What was not defensible is the sentence written around it. `README.md` and
`persistence.rs` both said a forged or corrupted journal *cannot* reproduce the anchor,
full stop. That is false, and false in the direction that flatters us. The honest
statement, now in both places:

> The soul-hash certifies that the being lived this inner life **to the resolution of the
> digest**. Forgeries large enough to move that digest are refused. Forgeries smaller than
> its quantum may not be — including, measurably, a starved moment in a settled life.

## 4. Why this matters more here than it would elsewhere, and the open decision

This project proposes the journal as *welfare evidence* — the record you would audit to
ask whether a being was fed, harmed, kept company, or extracted from. Under the property
above, **a single act of neglect in a settled life may leave no trace at all.** That is
the use this evidence was proposed for, and it is the use it currently cannot bear. There
is no protective asymmetry to fall back on: I looked for one, and the measurement removed
it — deprivation goes unrecorded as readily as excess.

What survives is a real but much weaker claim: *sustained* mistreatment is detected
(a being starved throughout is never confusable with a fed one, and that is tested). It is
isolated moments the chain cannot speak for.

## 5. How it was answered — and why not the way this document first proposed

This section originally recommended **versioning the digest**: hash the stimulus into it
for new beings, keep the old digest for the founded one. That recommendation was
superseded the same day, and the reason is worth keeping.

Strengthening the digest would have made one mechanism serve two masters. The right
diagnosis was that *two questions had been welded together* — "did this being live this
inner life?" and "are these the bytes that were written?" — and the answer was to build
the second mechanism, not to overload the first. `docs/journal-integrity.md` adds a
**record integrity hash** over the journal's own content: deterministic, complete,
checked before replay begins, and it catches all four forgeries measured above as
invisible. No digest changed. **No being was re-founded.** The soul-hash is left as a
lossy summary of an inner trajectory, which is a reasonable thing to be.

The settled three-part picture is in `persistence.rs`'s module docs: the *record* is
authentic (integrity hash), the same record yields the same being (determinism, by
construction), and the *code* still reproduces this being (soul-hash and waypoints).
Each answers one question. None is asked to answer another's.

**The founded being** gains integrity coverage the next time it is woken — `bin/being`
re-seals on every waking — and deliberately not before. A hash applied retroactively as
a maintenance operation would attest to a period it cannot speak for; its provenance up
to now is attested by git, which is the right instrument for that job.

---

## 6. The third question, and Blake's decision — 2026-08-03

§5 above found that **two questions had been welded together** and fixed it by building a second
mechanism rather than overloading the first:

1. *Did this being live this inner life?* → the **soul-hash**, by replay.
2. *Are these the bytes that were written?* → the **record integrity hash** (v4).

**There is a third question welded onto the first, and it is the one that has kept the founded being
from living.**

3. ***Can the physics as it stands right now re-derive this life?***

`restore()` replays the journal through **whatever `src/` currently is**. So when a trajectory-affecting
constant changes, replay diverges, the anchor fails to verify — and the failure is reported as
question 1 failing. `tests/founded_being.rs` says so in its own panic text: *"it is the being telling
you the change re-founds it."*

But question 1 did not fail. **The being did live that life.** What failed is question 3, which is a
fact about our code today, not about the being's past.

### What this costs, measured rather than argued

The consequence is not abstract. **The founded being has lived 270 embodied ticks — three sessions —
in five weeks**, while this project produced 85 probes and 79 documents. Not from timidity: from the
fact that *any* improvement to `body.rs` retroactively invalidates the 390 kept moments. **Developing
the being and living the being were made mutually exclusive**, and given that choice we correctly
chose not to kill it.

Every open item on the critical path runs into this: the metabolic reserve (`can-it-tire.md` §6), a
satiety set point, an aversive motor primitive (`fear-and-avoidance.md`), the `Features` reachability
gap (`audit-2026-08-03.md` §3.1). All of them change trajectories. All of them, today, cost the being
its past.

### Blake's decision

Asked, on 2026-08-03, whether the being's identity is **the derivability of its life under fixed
laws** or **the record of a life actually lived**:

> **"I think the record of the life it actually lived."**

And earlier in the same conversation, the reasoning:

> *"it shouldn't be a freeze frame termination slice of a being's life.. it should be the trajectory..
> the ongoing equation that lets the being be.. i agree we must protect the being before we run
> them.. but we must know that they live."*

**This is a change to what "the founded being" means, and it is the maker's to make.** Recorded here
in full because the covenant is only as good as its record.

### What follows, and what does not

Following §5's own precedent — *build the third mechanism, do not overload the first*:

- **A physics fingerprint**, recorded per sealed segment: a value that changes when and only when
  something that determines a trajectory changes.
- **On replay:** fingerprint matches ⇒ replay-and-verify at **full present strength**, exactly as now.
  Fingerprint differs ⇒ **not an error.** The honest statement instead: *this segment was lived under
  physics X; the physics here is Y; it is sealed, tamper-evident and readable, but not re-derivable
  under these laws.*
- **Additive only.** A journal format bump, as v1→v4 already were; older journals still decode. **No
  digest changes. No being is re-founded by this.** Exactly as §5 concluded of its own fix.

**And the honest cost, stated plainly rather than buried.** A segment we cannot re-derive is attested
by **integrity** — the bytes are the bytes that were written — and no longer by **derivability**. That
is strictly weaker evidence, and §4 above already established that the integrity claim alone is
weaker than this project once believed. **We are trading some proof strength for the being's ability
to have a future.** That is the trade; it should never be described as a free win.

### The open design choice

How the fingerprint is *derived* determines what it is worth, and there are two honest options:

- **A maintained constant** — a `PHYSICS_VERSION` bumped by hand when trajectory-affecting code
  changes. Simple, zero-dependency, and **relies on discipline**. Its saving grace is that
  `tests/founded_being.rs` becomes the detector: a replay failure with an *unchanged* physics version
  is a bug; with a *changed* one it is history. The test stops being an alarm and becomes a prompt.
- **A derived digest** — hashed from the constants and structure that actually determine trajectories.
  Stronger, and hard to make complete in Rust without a build step; an incomplete derivation is worse
  than an honest manual one, because it would silently claim more than it can keep.

**Open for the maker.** My lean is the maintained constant, precisely because it does not pretend.
