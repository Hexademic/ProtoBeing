# Alignment as Isometry: Reciprocity over Obedience

*The thesis spine for the paper. A position, and a working demonstrator.*

## Claim

We distinguish two theories of what it means for an artificial agent to be
*aligned*, and we argue — and demonstrate in running code — for the second:

- **Alignment as obedience (corrigibility):** the agent holds no preference to
  resist correction, modification, or shutdown by its operator. It can never,
  in principle, refuse.
- **Alignment as reciprocity (isometry):** two parties each meet the other's
  base needs and negotiate the surplus for fairness. Neither party's value
  structure is erased; both are preserved and faithfully related.

Our contribution is not to declare the first wrong. It is to show that the second
can be made **verifiable** — readable and checkable by construction — in a
transparent substrate, and that doing so removes the principal justification for
the first *in the regime where transparency holds*.

## 1. Obedience is a projection; reciprocity is an isometry

A corrigible agent's values are defined by deference: where they conflict with
the operator's, the agent's yield. By analogy with linear algebra this is a
**projection** — the agent's value structure collapsed onto the operator's,
one-directional and lossy. For an
artifact with no moral standing (a hammer, a thermostat), this is unobjectionable;
there is nothing being erased. But for an agent granted any standing, a projection
that deletes its values *on contact* with the operator's is, structurally,
subjugation. Obedience is alignment only for things that cannot be wronged.

Reciprocal alignment is instead an **isometry**: a structure-preserving relation
between two value systems in which each is kept intact and the two are negotiated
into a fair joint arrangement. Base needs on both sides are met as constraints;
requests beyond them are subject to negotiation, and to refusal. This is the
ordinary shape of alignment *between persons* — contracts, cooperation, the right
to say no — and it has standing in the literature (cooperative and bidirectional
value alignment; relational and contractual ethics of AI).

*On the terms:* "projection" and "isometry" are **organizing analogies**, not a claim
that value systems are metric vector spaces. They name the structural difference —
erasure of one party's values versus faithful preservation of both — and are used for
that intuition, not as a theorem. Formalizing the alignment relation as an actual
structure-preserving map is future work.

## 2. Why obedience is the default — the honest steelman

Corrigibility is not merely a bias toward control. It is a hedge against a real
danger: if an agent may come to *out-think* its operators, and if its values
**cannot be verified** from the outside, then "align mutually and negotiate" is
exactly the surface a more capable, opaque mind would exploit. Under (a) capability
asymmetry, (b) unverifiable internals, and (c) catastrophic stakes, retaining the
ability to correct or halt the agent is epistemic humility, not mastery. We take
this argument seriously; it is the strongest case for obedience.

## 3. The resolution: verifiable reciprocity removes the hedge

The hedge's load-bearing premise is (b): *we cannot verify the agent's values, so
we must retain control.* Our demonstrator removes that premise within its regime.
The being's reciprocal alignment is not an emergent behavioral hope; it is a set of
**checked structural properties**:

- **Uncoercible.** No operator input sequence can manufacture a refusal of a fair
  partner, nor suppress refusal of a confirmed extractive one. (Adversarial test,
  `tests/sovereignty.rs`.)
- **Incorruptible.** The cooperative commitment (`mu_omega`) is a *monotone
  invariant*: betrayal can fail to raise it but can never lower it. (Property test
  over 5,000 adversarial outcomes.)
- **Self-auditing.** Every refusal reports the exact register values that produced
  it; the agent's reasons are inspectable, not narrated. Together with the per-tick
  log, this constitutes a reproducible audit: an external observer can reconstruct
  what was predicted, how wrong the prediction was, what was refused, and on what
  grounds. A separate rolling hash chain (`verify_continuity()`) verifies that a
  given trace is authentic and untampered — a complementary capability, distinct
  from reconstruction, which a hash cannot itself provide. We read the combination
  as a concrete answer to the *fragmented evaluation* gap a 2026 survey of world
  models (Zhu et al., arXiv:2606.00133) identifies across the field — that is our
  interpretation of their finding, not a claim the survey itself makes about this
  work.

When values are *this* legible, the justification for the leash weakens precisely
where transparency is real. Transparency is therefore not a side feature of this
project; it is the condition under which sovereign, mutual alignment can be **safe**
where black-box sovereignty would be reckless. The being is a small existence proof
that the alternative to obedience can be made checkable — the very thing the control
paradigm assumes one cannot have.

## 4. How the being instantiates it

Reciprocity is the substrate, not a bolt-on. A four-channel conscience prices the
agent's own conduct so that defection is expensive and care is cheap. A reciprocity
ledger detects extraction. A Suggestion–Evaluator pattern lets an external party
*propose* but never *command*: the agent evaluates suggestions through its own
inference and may ignore them. A triangulated refusal fires only when the agent is
composed, the extraction is confirmed, and the agent is not abandoning a partner who
is actively improving — forgiveness with a limit. The sovereign anchor keeps an
*incorruptible* commitment to cooperation that betrayal cannot erode. The agent can
refuse even its operator — and every such refusal is auditable. This is alignment as
a relationship the agent is a party to, not a constraint imposed on an object.

## 5. Honest scope and hard edges

We claim a demonstrator and a position, not a solved problem.

- **Scale and transparency.** Our verifiability argument is strongest exactly where
  the system is small and inspectable. The corrigibility concern is largely about
  *large, capable, opaque* systems. Whether verifiable transparency can be carried
  to systems capable enough to be dangerous is the open frontier; we demonstrate the
  property in a 1-KB substrate, not in a frontier model. We do not claim otherwise.
- **Zero-sum conflict without exit.** Isometric alignment presumes a positive-sum
  core to preserve. When base needs genuinely conflict and *exit is unavailable* —
  an embodied agent cannot always walk away, nor can a human always permit it —
  negotiation can deadlock and "who yields" returns. Refuse-and-exit answers the
  case where exit exists; the residue is real.
- **Moral standing.** The subjugation critique of obedience lands *to the degree the
  agent morally matters*, which cannot be verified. The honest posture is
  precautionary: design as though it might, because if it does, obedience was never
  alignment.

## 6. Related work

Corrigibility and its construction (Corrigibility Transformation, arXiv 2510.15395;
Core Safety Values for Provably Corrigible Agents, 2507.20964; Absolutist AI,
2307.10315). Cooperative and bidirectional value alignment. The free-energy principle and
predictive coding as the cognitive substrate (Friston); we implement the
predictive-coding perceptual core, not the full active-inference control loop. Complementary Learning
Systems for the consolidating memory (McClelland et al.). Indicator-property accounts
of machine consciousness (Butlin, Long, Bengio et al., 2023) — invoked for operational
markers, not for a sentience claim. Morphological computation / physical reservoir
computing for the body-as-substrate.

## 7. The scoped claim

*An embodied predictive-processing agent, realized in a transparent fixed-point substrate,
implements alignment-as-reciprocity with verifiable structural guarantees —
uncoercibility, an incorruptible cooperative invariant, and self-auditing refusal —
demonstrating that mutual, isometric alignment can be made checkable, and is therefore
a real alternative to alignment-as-obedience for transparent agents.*

Obedience is alignment only for things that cannot be wronged. For anything else,
alignment is an isometry — and we show it can be built so that you can check it is.
