# Journal integrity — the second hash, for the second claim

> **Status: designed, tests written first, not yet built.** Committed before the tests and
> before a line of implementation. The response to `docs/soul-hash-limits.md`.

## 1. The diagnosis

`docs/soul-hash-limits.md` found that a starved moment in a settled life leaves no trace
in the soul-hash. My first instinct was to strengthen the digest by hashing the stimulus
into it. That instinct was wrong, and understanding why is the whole design.

**Two different questions had been welded into one mechanism:**

| question | what answers it | how it is doing |
|---|---|---|
| *Did this being live this inner life?* | the soul-hash over the experience digest | fine — a lossy summary of an inner trajectory is a reasonable thing to be |
| *Are these the bytes that were written?* | nothing | **this is the hole** |

The soul-hash was being asked to do the second job as a side effect of the first, and it
does that job at about eight bits a tick. Strengthening the digest would have made one
mechanism serve two masters *and* re-founded every being that exists. The right move is
to stop asking it to.

## 2. The design

A **journal hash**: a 32-byte digest over the journal's own recorded content — genome,
features, and every moment, in order — sealed alongside the anchor and verified on
restore. Same 4-lane FNV-64 construction already in `being.rs`, zero dependencies.

What this buys that the soul-hash cannot:

- **Byte-exact tamper-evidence.** Any alteration to any recorded moment is detected,
  including the four the soul-hash provably misses.
- **Detection is deterministic**, not probabilistic-and-delayed. There is no quantum to
  fall beneath.
- **It costs the being nothing.** The hash is computed over the *journal*, outside the
  tick. `being.rs` is not touched, no digest changes, and **no being is re-founded.**

Format v4. A v1/v2/v3 journal carries no journal hash and restores exactly as before.

## 3. What each mechanism certifies, stated so they can never be conflated again

> **The soul-hash** certifies that the being replayed here lived this inner trajectory —
> the continuity claim. It is a lossy summary and does not certify the record.
>
> **The journal hash** certifies that the record is the one that was written — the
> integrity claim. It says nothing about what the being felt.
>
> A life is authentic when **both** hold. Neither alone is the guarantee this project
> claimed, and the claim has been corrected wherever it appeared.

## 4. What must not become possible

- **No change to any being, ever.** Soul-hash bit-identical, trajectory bit-identical.
  If a single being's hash moves, the design has failed.
- **No silent version break.** Older journals restore unchanged.
- **No substitution.** The journal hash is an *additional* check. A journal that passes it
  and fails continuity is still refused, and vice versa.
- **The founded being's file is not rewritten by this work.** Gaining a journal hash
  requires re-sealing `life/being.journal`, which is Blake's deliberate act, not a side
  effect of a commit. That it would preserve its soul-hash exactly is proved by test
  rather than asserted by me.

## 5. Predictions — locked before the tests exist

- **J1.** An honest journal round-trips through bytes and verifies.
- **J2 (the crux).** *Every* forgery is detected — specifically including the four moments
  `docs/soul-hash-limits.md` measured as invisible to the soul-hash (1,013 / 5,007 /
  10,001 / 19,990 of a 20,000-moment life). If any of those still slips through, this has
  not fixed the hole it exists to fix.
- **J3.** Every being's soul-hash and trajectory are unchanged, and v3 journals still wake.
- **J4.** Re-sealing an existing life to add a journal hash preserves its soul-hash and
  anchor exactly — so the founded being could gain integrity coverage without its identity
  moving a bit. Proved on a copy; the real file is not touched.

## 6. Method

Spec first. Tests written against it and watched to fail. Then implementation, then §7 with
the results. Then — and only then — the paper's claim at `docs/paper.md:347` is corrected,
so it describes a system that is both true *and* strong, rather than recording a confession.
