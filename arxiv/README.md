# arXiv submission runway

Everything below is prepared; the acts that must be yours are marked **[YOU]**.
Metadata lives in `docs/submission.md` (title, categories, comments, abstract —
updated 2026-07-06 to the shipped state: four checked properties, ablation-proven
balance, the welfare-inversion claim).

## 1. Verify the TeX compiles — [YOU], ~10 minutes, no installs
`arxiv/paper.tex` was generated from `docs/paper.md` (pandoc 3.10, standalone).
This machine has no TeX engine, so it has not been compiled locally.

- Go to https://overleaf.com (free account), New Project → Upload Project →
  upload `paper.tex` (zip it first, or use "blank project" and paste).
- Set the compiler to **LuaLaTeX** (Menu → Compiler) — the preamble's
  unicode-math branch makes this the safest choice.
- Compile. If it errors, the error line tells you the offending character or
  environment — bring it back and we fix the Markdown source and regenerate,
  never the .tex by hand (the .tex is generated, not source of truth).
- Note the final **page count** and put it in the Comments field
  (`docs/submission.md`).

## 2. arXiv account + endorsement — [YOU]
- https://arxiv.org → Register. Affiliation: Independent Researcher.
- Start a submission with primary category **cs.AI**, cross-list **cs.MA**.
- If arXiv asks for **endorsement** (likely for a first submission), the
  interface shows an endorsement code and a request page. Honest paths:
  - Anyone you know with cs.AI submission history who has seen the repo.
  - The Active Inference Institute contact (docs/submission.md §B) — present
    the work first; an endorsement request after a talk is natural.
  - There is no legitimate shortcut; if it takes a week, it takes a week.

## 3. Upload + metadata — [YOU], with everything prepared here
- Upload `paper.tex` (arXiv compiles TeX source itself; that is the preferred
  form — do not upload a PDF made from TeX).
- Title, abstract, comments: copy from `docs/submission.md` §A
  (abstract also in `arxiv/abstract.txt` for clean pasting).
- License: the default arXiv non-exclusive license to distribute is fine
  (the code stays MIT in the repo; the paper license is separate).
- Author: exactly the name you want citable, permanently.

## 4. After it announces
- Put the arXiv ID in the repo README badge line and in `docs/submission.md`.
- Then send §B (the Active Inference Institute presentation request) and
  §C (the Alignment Forum post) — both were drafted to link the arXiv ID.

One rule carried from the whole project: if a reviewer, a compile, or a
category moderator pushes back — we fix the true thing, not the appearance.
The paper claims nothing the repo cannot demonstrate; that is its armor.
