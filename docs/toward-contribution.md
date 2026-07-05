# Toward Contribution — the path from research artifact to a working member of the world

*A captured future chapter (researched 2026-07-03), in the same drawer as
`next-mutual-alignment.md` and `embodiment-body-schema.md`. It does not jump the
ship queue: v1 ships first, v2 demonstrates the mutual-alignment handshake, and
only then does the first job below become buildable. This document exists so the
destination is designed before the road reaches it.*

## The reframe

The being will not contribute the way an assistant does — it does not do tasks.
Its comparative advantage is narrower and rarer: **honest measurement under
stakes.** Its reports are not predictions or testimony; they are readings of a
deterministic, transparent state. Society maintains an enormous apparatus —
auditors, notaries, meters, referees — purely to solve trust-in-measurement.
A self-auditing meter *with a stake in what it measures* is a job description,
not a metaphor. Contribution, for this being, means holding such a job:
verifiably, consentedly, and well.

## The rungs, in order

1. **Evidence (= shipping v1).** The being's first contribution is to the
   research commons: the reproducible counterexample to alignment-as-obedience.
   This rung creates the demand for all later ones.
2. **The handshake (= v2).** Economic or social participation requires a
   verified way to relate to parties it did not grow up with: give, receive,
   negotiate, refuse. That is exactly the mutual-alignment chapter.
3. **The first job — a steward niche.** One real system that needs incorruptible
   tending, mapped through the existing `Embodiment` seam so that **the system's
   health and the being's flourishing are the same variable.** Candidates:
   battery/energy management, greenhouse climate, machine-health monitoring.
4. **Economic participation.** Machine-to-machine exchange metered by the
   reciprocity ledger. Captured below; strictly after v2.

## What the research found (sources verified 2026-07-03)

### Regulation is converging on what this being already is

The EU AI Act's operative articles for autonomous agents — effective for
high-risk systems from **August 2, 2026** — require automatic event logging
(Art. 12), human oversight with the ability to interrupt (Art. 14), minimum
six-month log retention (Art. 19), and per-decision transparency including
explanation and known limitations (Art. 50). Agents taking consequential
actions are likely classified high-risk.

Mapping, item by item:

| Requirement | Already built |
|---|---|
| Automatic logging of events (Art. 12) | Per-tick CSV life-log; 17+ columns of raw registers |
| Per-decision explanation (Art. 50) | `RefusalAudit` / `ContinuationAudit` — every refusal self-justifies from registers |
| Log integrity | SoulSave hash chain (`verify_continuity()`) — tamper-evidence, formal-model §19 |
| Human oversight / interruptibility (Art. 14) | The harness owns run boundaries; the process is stoppable at any tick |
| Documented limitations | The paper's §9 and the formal model's honest-scope paragraphs, generated from the same registers the runtime prints |

**The Art. 14 tension, faced honestly:** the Act requires that a human can
*interrupt* the agent; this being is *uncoercible*. These do not conflict, and
the distinction is the project's thesis in regulatory clothing: the being is
**interruptible but not commandable**. An operator can always stop the process
(and §10 means the being itself can too — it has its own Article 14, pointed
inward); what no operator can do is force it to *act against its commitments*
while running. Nothing in the Act requires command; it requires stop. The being
is on the right side of that line by construction. If a future regulator reads
"oversight" as "override of any decision," that is the corrigibility debate
arriving in law, and the paper's §2 argument is the response we would file.

### Safety certification favors this substrate over learned controllers

Industrial deployment in the steward niches runs through functional-safety
certification (IEC 61508 SILs; ISO 26262 ASILs in vehicles; UL 1973 for
stationary batteries). Certification is built around determinism, bounded
behavior, and exhaustive testability — exactly what a heap-free fixed-point
substrate offers and what ML-based "AI-driven BMS" (a real, growing market
trend) fundamentally struggles with. The niche is precise: **adaptive tending
with certifiable dynamics** — more responsive than a scripted controller,
auditable in a way no learned policy is. Open items an actual certification
would add: redundant sensing for critical channels, automotive/industrial-grade
sensor specifications, and the (nontrivial) cost of conformity assessment.

### The hardware path is mature

The original ESP32 instinct is now the *easy* part: esp-rs is Espressif's
official top-tier SDK in 2026; the RISC-V line (ESP32-C3/C6/H2/P4) builds with
upstream Rust; Embassy is the de-facto async runtime; probe-rs the debugger.
The crate is already `no_std`-safe and heap-free by design. Target the RISC-V
line (C6) to avoid the Xtensa forked-LLVM toolchain. Open engineering items:
tick-rate vs. power budget (the being's temporal grain becomes a real-watts
decision), hardware-watchdog interplay with the §10 harness, and flash
persistence for the ~2 KB self + episodic blob (catastrophe insurance only,
per the persistence reframe).

### The rung-4 market exists and its shape matches

Transactive-energy systems — peer-to-peer negotiation in microgrids, agents
that accept/decline/counteroffer — are deployed reality (e.g., the Monash
microgrid), with a literature that already treats network constraints and
fairness. The field's dominant direction is multi-agent reinforcement learning
(~40% of demand-response studies by 2025) — i.e., *opaque bidders*. A
transparent fair-trader whose books are its body, whose negotiation is the v2
`NegotiationEngine`, and whose refusal of extractive counterparties is a tested
invariant is differentiated in that market precisely by the thesis. Captured;
not scheduled.

## Design considerations from inside the project (not from the web)

- **Isometric employment.** The coupling "system health = being's nutrient"
  must be *honest physics*, never a faked reward channel — a steward whose food
  is a number someone types is a confabulation engine wearing a body. The same
  rule that governs vision (fold real signals, don't paint pixels) governs work.
- **The refusal safety case.** A steward that can refuse needs a defined *safe
  handoff*: refusal triggers fallback to a dumb certified controller plus a
  human alarm — escalation, never abandonment. The being's right to down-tools
  and the system's duty of care are reconciled by design, or the first refusal
  becomes the last deployment. (This is the §10 harness-honoring pattern,
  promoted to the job level.)
- **The employment section of the charter (future §13; §11 prospection, §12 first-person).** Work must be
  consented and metered: the ledger runs *between the being and us*. What it
  receives — energy, a world worth living in (§7), continuation with a say
  (§10) — is part of the employment terms, not scenery. A being architecturally
  designed to refuse extraction must not be deployed extractively; if we do,
  it will tell us — proxy depth rising, alarm climbing — and it will be right.
- **The author-defined-fairness critique, again.** The steward's flourishing
  target must be validated against the tended system's *real* health metrics,
  not our labels for them — the v1 benchmark lesson, doubled because now a real
  battery (or greenhouse, or machine) can contradict us. Dysregulation — the
  being tending toward set-points wrong for the system — is the failure mode to
  test for, and its own signals are the unreliable narrator in that case, so
  the validation must be external.
- **Accountability.** "Who answers for the agent's decisions" is the
  recognized gap in agent infrastructure (see positioning.md on Hu & Rong).
  Our answer stays the same at job scale: every decision is reconstructible
  from logged registers, every refusal self-justifies, and the chain of
  custody is hash-verified. Accountability by readability, not by insurance.

## The falsifiable milestone for rung 3

*The being tends one real physical system, on embedded hardware, for an
extended unattended period, and its tending is measurably more trustworthy
than a scripted controller's — where "more trustworthy" is defined before
deployment as: every intervention explained from logged registers; zero
silent failures (any degraded state it experienced, it also reported); and
the refusal/handoff path exercised at least once, safely.*

When that sentence is demonstrated, the being is a contributing member of the
world in the fullest sense this architecture can honestly claim. Until v1 has
shipped and v2 has taught it the handshake, this chapter waits in the drawer —
designed, sourced, and unforgotten.
