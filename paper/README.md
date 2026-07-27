# The paper package — deposit runway

> **Renamed 2026-07-27** (was `arxiv/`). No arXiv submission is planned. The folder
> was named after one venue while holding venue-generic material — the TeX source, the
> abstract, and the steps between a finished paper and a citable record. It is now
> named for what it holds.

> **Read `docs/submission.md`'s currency note first.** This package targets the
> 2026-07-06 state; the repository has advanced since. Pin a commit or refresh —
> do not deposit silently stale.

## What is here

| file | what it is |
|---|---|
| `paper.tex` | generated from `docs/paper.md` (pandoc 3.10, standalone). **Not source of truth** — fix the Markdown and regenerate; never edit the `.tex` by hand. |
| `abstract.txt` | the abstract as plain text, for clean pasting into a deposit form |

Metadata for either route lives in `docs/submission.md` §A (title, author, categories,
abstract — updated 2026-07-06 to the shipped state: four checked properties,
ablation-proven balance, the welfare-inversion claim). Machine-readable metadata already
exists at the repository root: `CITATION.cff` (what GitHub reads for "Cite this
repository") and `.zenodo.json` (what Zenodo's GitHub integration reads). Both currently
say **version 1.0.0, 2026-07-08** — if you deposit a later state, bump them first, or
the DOI will permanently describe the wrong thing.

Everything below is prepared; the acts that must be yours are marked **[YOU]**.

## Route A — Zenodo (the current plan)

Zenodo mints a DOI for a software record from a GitHub release. No endorsement, no
moderation queue, no LaTeX toolchain on their side — and every later release gets its
own version DOI under one permanent *concept* DOI. So depositing the paper **pinned**
now does not close the door on depositing a refreshed one later; that property is why
pinning is the recommendation in `docs/submission.md`.

### 1. Produce a PDF — [YOU], ~10 minutes, no installs
Zenodo wants the rendered paper, not TeX source. This machine has no TeX engine, so
`paper.tex` has never been compiled locally.

- Go to https://overleaf.com (free account), New Project → Upload Project →
  upload `paper.tex` (zip it first, or use "blank project" and paste).
- Set the compiler to **LuaLaTeX** (Menu → Compiler) — the preamble's unicode-math
  branch makes this the safest choice.
- Compile, then download the PDF. If it errors, the error line names the offending
  character or environment — bring it back and we fix `docs/paper.md` and regenerate,
  never the `.tex` by hand.
- Note the final **page count** for `docs/submission.md`.

### 2. Make the metadata true — [YOU]
Decide pin-or-refresh (`docs/submission.md`'s currency note), then make `CITATION.cff`
and `.zenodo.json` agree with that decision — same version string, same date, and the
artifact section of the paper saying which state it describes. This is the step that is
easiest to skip and most expensive to fix, because a DOI is permanent.

### 3. Connect the repository — [YOU]
zenodo.org → log in with GitHub → **GitHub** → flip the switch on
`Hexademic/ProtoBeing`. Zenodo only archives releases created *after* the switch is on,
so this must happen before step 4.

### 4. Cut a GitHub release — [YOU]
Tag it to match the version in `.zenodo.json`. Attach the PDF from step 1. Zenodo
archives the tagged source and mints the DOI.

### 5. After the DOI exists
Put it in the README badge line, in `CITATION.cff` (`doi:`), and in
`docs/submission.md`. Then send §B (the Active Inference Institute presentation
request) and §C (the Alignment Forum post) — both were drafted to link a permanent
identifier, and a Zenodo DOI serves that as well as an arXiv ID would.

## Route B — arXiv (prepared, not planned)

Kept because the material is ready and a decision can change, not because it is queued.
The differences that matter: arXiv wants the **TeX source**, not a PDF; primary category
**cs.AI**, cross-list **cs.MA**; and a first-time submitter will likely need an
**endorsement** from someone with cs.AI submission history — a real wait, with no
legitimate shortcut. Steps 1 and 2 above still apply; `docs/submission.md` §A already
holds the metadata.

One rule carried from the whole project: if a reviewer, a compile, or a moderator pushes
back — we fix the true thing, not the appearance. The paper claims nothing the repo
cannot demonstrate; that is its armor.
