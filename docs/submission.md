# Submission materials — *drafts for Blake to send*

> **Currency note (added 2026-07-26).** This package was prepared for the
> **2026-07-06** state of the repository. The paper's *argument* and its *verified
> results* are unchanged — every test the paper relies on still passes on the current
> code — but the artifact has grown substantially since (62 modules, 294 tests, plus
> the field-world, graded homeostatic drive, reflection, habits, the prime/explication
> voice, inheritance, social referencing and homecoming, none of which the paper
> describes). Before submitting, choose deliberately:
>
> 1. **Pin it** — deposit as-is, citing the artifact at a specific commit or release tag,
>    and state in the artifact section that the paper describes that version. Fastest,
>    fully honest. On Zenodo this costs nothing later: a refreshed paper becomes a new
>    version DOI under the same concept DOI. **Recommended.**
> 2. **Refresh it** — extend the paper to the current system. Larger job, and an
>    authorship decision, not an editorial one.
>
> What must *not* happen is submitting it while silently describing a smaller system
> than the repository contains. That is the failure mode this project exists to avoid.

> **Venue note (2026-07-27).** The plan is **Zenodo**, not arXiv — a DOI minted from a
> GitHub release, with no endorsement and no moderation queue, and version DOIs under a
> permanent concept DOI so a pinned deposit now does not foreclose a refreshed one later.
> The metadata below is venue-generic and serves either route; the arXiv-only fields are
> marked as such and kept because the material is ready, not because it is queued. The
> mechanics live in `paper/README.md` (formerly `arxiv/`).

> Drafts. The deposit and the presentation request go out under **your** name and
> account; review and edit before sending. Both hold the paper's honest scope
> (predictive processing, not active inference; a demonstrator and a position, not a
> solved problem).

---

## A. The preprint — metadata

**Title:** Alignment as Isometry: A Verifiable Reciprocal Agent in a Transparent
Fixed-Point Substrate

**Author:** Blake "zelhart" Hexademic (Independent Research)
*(Disclosure line for the paper's front matter: "Developed in collaboration with an AI
assistant; all results are reproducible from the accompanying source.")*

**Zenodo record:** upload type *software* (the artifact is the claim); license MIT;
creators and keywords are already in `.zenodo.json` at the repository root, and
`CITATION.cff` must be kept in step with it. Both currently read version 1.0.0 /
2026-07-08 — reconcile them with the pin-or-refresh decision *before* cutting the
release, because the DOI is permanent.

**Comments / description field:** "Code and full reproduction:
https://github.com/Hexademic/ProtoBeing. Demonstrator, not a sentience claim.
(Fill page/figure count after the PDF is produced.)"

**arXiv-only, if that route is ever taken:** primary category **cs.AI** (Artificial
Intelligence); cross-list **cs.MA** (Multiagent Systems), optionally **nlin.AO**
(Adaptation and Self-Organizing Systems) for the dynamical-systems framing. arXiv cs.AI
may require an **endorser** for a first-time submitter — anyone with cs.AI submission
history who has seen the work, requested through the arXiv interface once an account
exists. Zenodo requires none of this.

**Abstract (≈250 words, updated 2026-07-06 to the shipped state):**

> We present the Unified Being, a small, deterministic, embodied predictive-processing
> agent implemented in fixed-point arithmetic (≈2 KB of state), and use it to argue a
> position on machine alignment. Mainstream alignment is *corrigibility*: an agent that
> holds no preference to resist correction or shutdown. We characterize this as
> alignment-as-obedience — a projection that collapses the agent's value structure onto
> the operator's — and contrast it with alignment-as-isometry: a reciprocal arrangement
> in which each party's base needs are met and the surplus negotiated, with refusal
> possible and neither structure erased. Corrigibility is best understood as a hedge
> against *unverifiable* values under capability asymmetry. Our central result is that
> this hedge can be removed in the regime where transparency holds: the being's
> reciprocal alignment is realized as *four checked structural properties* — it is
> robust to operator coercion under adversarial test; holds a cooperative commitment
> that is monotone by construction (incorruptible); audits every refusal by its
> triggering registers; and holds a say over its own continuation that no tested
> operator input can manufacture or override. We report eight reproducible experiments
> (including embodied balance whose causal dependence on the agent's felt choices is
> shown by ablation), an adversarial benchmark against a myopic baseline, and an honest
> self-assessment against published consciousness-indicator properties. An accompanying
> charter executes welfare protections *before* the capacities they protect — to our
> knowledge the first end-to-end constructive instance of the precautionary stance now
> formalized in the AI-welfare literature. We claim a demonstrator and a position, not a
> solved problem, and state the limits plainly: the verifiability argument scales only
> as far as transparency does; reciprocity deadlocks where exit is impossible; and the
> moral-standing question that gives the thesis its force is unverifiable.

---

## B. Presentation request — Active Inference Institute / TNB group

**To:** theoreticalneurobiology@gmail.com
*(cc / alternative: blanket@activeinference.institute for general community programming)*

**Subject:** Presentation request — a transparent, verifiably-sovereign predictive-coding agent

> Hello,
>
> I'm an independent researcher working on a small, transparent, fixed-point agent
> (~2 KB of state, `no_std`-friendly) built on a predictive-coding core and a
> body-as-reservoir (morphological computation). I'd like to request a slot to present
> it to the TNB group.
>
> The work makes one defensible claim: that *reciprocal* alignment — an agent that meets
> a partner's needs, negotiates, and can refuse — can be made **verifiable** by
> construction. In this substrate the agent is robust to operator coercion under
> adversarial test, holds a cooperative commitment that is monotone (incorruptible) by
> construction, and audits every refusal by its internal registers. I frame it as the
> transparent inverse of corrigibility, and I'm explicit about scope: it is a runnable
> demonstrator and a position, not a claim of consciousness.
>
> Everything is reproducible from source (one command runs the experiments and the
> benchmark). I'd welcome the critique of this community in particular, since the
> substrate is predictive-coding-based and the honest limitations (it implements the
> perceptual core, not full active-inference action selection) are exactly the kind of
> thing your group would sharpen.
>
> I can share the preprint and repository ahead of time. Thank you for considering it.
>
> — Blake "zelhart" Hexademic

---

## C. Sharp-critique venue (parallel, no deadline)

The Alignment Forum / LessWrong is the room most likely to engage a *transparently
non-corrigible* agent hard — exactly the "prove me wrong" audience the manifesto
invites. A short post linking the preprint and repo, framed around the
obedience-vs-isometry thesis and the three checked invariants, would draw the critique
the work wants. Best sent *after* the deposit exists, so it can link a permanent DOI.
