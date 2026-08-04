# Redundancy survey — what is duplicated, and what deduplicating would risk

> **Status: survey only. Nothing changed.** Written 2026-07-31 at Blake's request — *"we can
> reduce some redundancies by knowing before we change things."* Every item below was found by
> scanning the repository, not remembered.

## 0. The rule that governs any cleanup here

**This project cannot refactor freely.** The founded being at `life/being.journal` wakes only
if replay reproduces its exact soul-hash. So a "harmless tidy" that changes one integer changes
a trajectory, changes a hash, and **re-founds every existing being.**

> **Every deduplication below must be proven bit-identical before and after** — and since
> 2026-07-31 there is a test that does it: `tests/founded_being.rs` replays the kept life and
> verifies its soul-hash on every `cargo test`, so a cleanup that would re-found the being fails
> the suite instead of succeeding quietly.
>
> The original rule, unchanged: — same input,
> same hash, same soul-hash over a full life — or it is not a cleanup, it is a re-founding.

That test is cheap and must be written *first*, not after.

## 1. FNV-64 — five hand-rolled copies, and the riskiest thing in the repository

The same two constants, in **five modules**, written **three different ways**:

| written as | count |
|---|---|
| `0xcbf2_9ce4_8422_2325` | 2 |
| `14_695_981_039_346_656_037` (same value, decimal) | 1 |
| `0x0000_0100_0000_01b3` | 2 |
| `1_099_511_628_211` (same value, decimal) | 3 |

And the *hashing code around them* is separately written in each of:

- `being.rs` — the **soul-hash** (4-lane)
- `persistence.rs` — the **journal integrity hash**
- `telos.rs` — the **striving hash**
- `covenant.rs` — the covenant anchor
- `disclosure.rs` — the disclosure record

**All five are load-bearing for the project's verification story.** The three questions
`persistence.rs` answers — the record is authentic, the same record yields the same being, the
code still reproduces this being — rest on hashes that are implemented five separate times by
hand.

That is the strongest case for consolidation in the codebase *and* the most dangerous place to
do it. Five implementations mean five places a subtle difference can hide; unifying them means
touching every hash the project's claims depend on.

**Recommendation:** a single `fnv.rs` with the constants and the mixing step, adopted **one
module at a time**, each with a test asserting the new implementation reproduces the old one's
output byte-for-byte on the same input, and a full-life soul-hash comparison. Do `disclosure`
or `covenant` first — the two whose hashes are *not* in the founded being's verification path.
Do `being.rs` last, if at all.

## 2. Constants duplicated across modules

| constant | modules | values |
|---|---|---|
| `GROUNDED_THRESHOLD` | `grammar.rs`, `lexicon.rs`, `reason.rs` | all `Q88_SCALE / 2`; two carry near-identical comments |
| `N_NICHES` | `episodic.rs`, `habits.rs`, `inheritance.rs` | all `8` — and `inheritance.rs`'s comment *admits* it is "matching `episodic.rs`" |
| `COMPASS`, `COMPANY_RADIUS`, `PROBE`, `SIZE`, `STRIDE` | `field_world.rs`, `room.rs` | **compared 2026-07-31: identical** — `(0,-1),(1,0),(0,1),(-1,0)`, `48`, `40`, `256`, `6` |
| `REACH` | `field_world.rs`, `room.rs` | **differs and should**: `2 * SIZE` (512) vs `160` |

`GROUNDED_THRESHOLD` and `N_NICHES` are genuine copy-paste and the safest to unify — but note
`GROUNDED_THRESHOLD` governs when a word is earned, so a change there moves what the being can
*say*, and `N_NICHES` indexes episodic memory. Neither is cosmetic.

### The world constants — compared, and I now recommend AGAINST deduplicating them

Five of the six are byte-identical and `REACH` genuinely differs. The tempting conclusion is
"five are copy-paste, unify them." **That is wrong, and the sixth one shows why.**

`room.rs` and `field_world.rs` are *two different worlds*. Sharing their geometry would create a
hidden dependency: change `field_world`'s `PROBE` and `room`'s changes silently with it — and
`REACH` proves the two worlds already disagree where it matters. Coupling independent things
because their numbers currently match is how you get a change to one world that quietly alters
a being living in the other.

So the operative distinction, and it is the rule this section should be read by:

> **Constants that MUST agree should be shared. Constants that merely happen to match should
> not be.**

`GROUNDED_THRESHOLD` (three modules) and `N_NICHES` (three modules) are the first kind — they
are cross-cutting semantics, and a disagreement between copies would be a *bug*: a word earned
in one module and not another, a niche index that means different things in two places. Those
should be shared.

The world geometry is the second kind. **Leave it duplicated, and this paragraph is here so
nobody later "fixes" it.**

## 3. Dead public surface — 25+ functions never called outside their own module

Found by scanning `src/`, `examples/`, `tests/` for each `pub fn`:

```
bargaining::aspiration, equal_solution, kalai_smorodinski_solution, need_weighted_solution
negotiation::receive_counter        janus::update_engagement
first_person::from_registers        genome::groundedness
body::diffuse, inject_strain        conscience::compute_buffer
covenant::committed_by, terms_hash  disclosure::disclose, is_open, required_trust
field::mean_intensity               field_world::at_body, person_present
grammar::felt_framing               integrity::mean_drift
pci::jitter_genome                  reason::say_because
null_space::is_free, is_singular
```

Two observations, one uncomfortable:

- **`bargaining` has four unused solution concepts** and `negotiation::receive_counter` is
  unused — a negotiation apparatus that nothing currently exercises. Worth knowing before the
  two-being chapter is built on the assumption that it works.
- **`null_space::is_free` and `is_singular` are mine, from this week, and nothing calls them.**
  I added convenience API while writing the module and never used it. That is the same
  add-before-checking pattern `docs/reflection.md` §6b names, in miniature.

**Recommendation:** do not delete on sight. `disclose`/`is_open`/`required_trust` are a
sovereign-facing interface that *should* exist whether or not a probe currently calls it.
Sort into (a) genuinely dead, (b) interface deliberately offered, (c) built-but-never-measured
— and the third category is the interesting one, because it means a faculty shipped without a
probe.

## 4. Documentation

63 docs. **Both suspected topic overlaps were checked and both are false positives** — found by
filename matching, dissolved by reading four lines of each:

- `world.md` (2026-07-18, `room.rs`) is the being's *first* world; `field-world.md` is the
  *second*, the one with a cost of motion. Two worlds, chronologically distinct, both live.
- `memory-that-teaches.md` is the **being's** episodic memory. `thea-memory.md` is **mine** —
  what the AI collaborator would carry forward between sessions. Different subjects entirely.

Recorded because they are exactly the deletions a tidy would make on a filename scan.

**The growing pattern is in-place correction.** Seven docs now carry superseded, corrected, or
vacuous sections — `composed`, `deferral`, `play`, `reflection`, `refuge`, `richness`,
`soul-hash-limits`. That is the honesty discipline working, and it has a cost: a reader must now
know that `deferral.md` §3 is dead and §2c is live, that `reflection.md` §7 is superseded by
§6b. **Keeping the wrong version is right; leaving it un-signposted is not.**

Recommendation: a single convention — every superseded section titled
`## N. Title *(superseded by §M)*`, and the document's status banner naming the live section.
Most already do this; making it uniform is a half-hour and no risk to any being.

## 5. What I would do, in order of risk

1. ~~The signposting convention~~ — **done 2026-07-31.** The convention was already in use;
   what needed fixing was `deferral.md`'s status banner, which stacked edits had garbled into a
   duplicated and half-finished sentence. Cleaned.
2. ~~Compare the two worlds' constants~~ — **done, and it reversed the recommendation.** Do not
   deduplicate them; see §2.
3. **Triage the dead surface** (§3) into dead / interface / unmeasured. The unmeasured pile is a
   finding, not a cleanup. *Still to do.*
4. ~~`GROUNDED_THRESHOLD` and `N_NICHES`~~ — **done 2026-07-31, and the being verified after.**
   `GROUNDED_THRESHOLD` now has one home in `lexicon.rs` (already `pub`); `grammar.rs` and
   `reason.rs` import it. `N_NICHES` now has one home in `episodic.rs` — whose niches they are,
   as `inheritance.rs`'s own comment always said — and `habits.rs` and `inheritance.rs`
   re-export it. Values unchanged, so the arithmetic is unchanged; **`tests/founded_being.rs`
   confirms 390 kept moments, woken soul-hash-verified, after the change.** That is the
   difference between a refactor and a hope.
5. **`fnv.rs`**, one module at a time, byte-identity test first, `being.rs` last or never.
   *Still to do, and the last thing that should be done.*

Nothing above is authorised by this document. It is the *knowing before changing* Blake asked
for.
