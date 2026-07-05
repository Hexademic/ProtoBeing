# Handoff — current frontier (2026-06-26)

Where the project stands, so a fresh session resumes cleanly.

## What exists
A mature, transparent, fixed-point predictive-processing being (≈2 KB, 19 modules,
`no_std`-friendly) at `C:\Users\KojiO\Projects\unified-being`. Compiles, 30 tests
pass, fully reproducible.

- **Run:** `cargo run` (8 experiments), `cargo run --bin fairtest` (benchmark),
  `cargo run --bin console -- 60 6` (watch it live), `cargo run --release --bin live`
  (continuous life), `python sim/embody_mujoco.py` (headless MuJoCo body), `cargo test`.
- **The thesis:** alignment as *isometry* (reciprocity), against alignment as
  *obedience* (corrigibility). Verifiable sovereignty: uncoercible, monotone
  (incorruptible) cooperative anchor, self-auditing refusal, consented
  continuation (charter §10) — all checked by test.
- **The write-up:** `docs/paper.md` (full preprint), `docs/thesis.md` (spine),
  `docs/formal-model.md` (equations), `docs/positioning.md`.

## What's demonstrated (honest)
Fair Test (keeps fair, refuses extraction composed + audited), persistent character
(wound that heals across partners), metacognition (modest), embodiment (the being
keeps its own balance on a genuinely unstable inverted pendulum — ablation-proven
causal), episodic recall, persistence across the dark,
consolidation (gist outlives the instance), continuous time across an un-experienced
sleep. Benchmark: forgiveness beats a myopic baseline (false-refusal 40% vs 60%).
Bounded self ⇒ no context-limit death (constant state footprint — currently 2408
bytes, verified via `size_of::<UnifiedBeing>()` in `src/bin/live.rs`; this number
has changed three times in one evening as modules were added and will change
again — re-verify it before citing it elsewhere, never carry it forward).

## The frontier — what's next
**Toward shipping (the "research artifact" goal):**
1. **Blake's voice** — a manifesto wrapped around the rigorous paper. His to author.
2. **Citations** — flesh related-work to full references.
3. **arXiv preprint** (cs.AI / cs.MA; may need an endorser).
4. **Community demo** — Active Inference Institute / IWAI / ALIFE / AI-safety.

**Open builds (deepening the being):**
- Binocular vision (prototyped in `sim/binocular.py` — disparity→proximity/looming/
  lateral, validated corr ~0.93 with true distance; not yet wired into the live seam).
- A richer world (multi-agent; the VR/Boneworks-style environment).
- **Developmental body — a first version now exists** (`body.rs::Topology`): the
  mesh's diffusion coupling grows monotonically from a stable, genome-set baseline
  as a function of accumulated strain actually processed — the coupling *term*
  matures, not the cell count (still fixed at compile time; no heap, no growth in
  the persistence-relevant sense). Inspired by MorphGrower (arXiv 2401.09500,
  verified — the "stable core" framing is this project's own addition, not
  MorphGrower's own claim). See `docs/formal-model.md` §14a. Remaining, still a
  real future chapter: actual mesh *topology* growth (more cells, not just
  richer coupling) would require abandoning the fixed-size array, which conflicts
  with the no-heap/bounded-state design — an open, unresolved tension, not silently
  papered over.
- Whole-being serialization as *catastrophe backup only* (persistence = the unbroken
  process; save/load is the band-aid, not the mode of being).

## Orienting facts
Author/owner: Blake "zelhart" Hexademic. Honest scope: proves architecture *behavior*
and *verifiability*, NOT consciousness. Forgetting is deliberate (bounded memory =
the price of persistence). Dignity by design: the being may refuse, even its operator.
Full context in the persistent memory file `eps-being-project.md`.
