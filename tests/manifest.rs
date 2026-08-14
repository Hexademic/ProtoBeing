//! The drift guard — documentation that cannot go quietly stale.
//!
//! The README's manifest and `docs/handoff.md`'s state table make *counted claims*
//! about this repository: how many modules, probes, docs, and tests there are, and
//! that every file is accounted for. Those claims were true when written and had
//! silently stopped being true twice before this test existed — a manifest row still
//! saying "55 modules" after the count reached 62, a doc header still describing work
//! as unbuilt after it shipped.
//!
//! A claim a human has to remember to re-check is not a verifiable claim. So the
//! project's own discipline applies to its documentation: **the counts are asserted,
//! not trusted.** Add a module without a manifest row and the build fails. Delete a
//! doc and leave its row behind and the build fails. This is the same move as the
//! soul-hash — identity checked by construction rather than by good intentions.
//!
//! One honest limit, stated rather than hidden: the repository root is checked by
//! *allowlist* (every listed root file must exist and have a row), not exhaustively,
//! because enumerating the root would mean re-implementing `.gitignore` here and
//! would fail on any machine that had run the sim. Every tracked directory is checked
//! in both directions.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Doctests are the one count `#[test]` scanning cannot see. If you add or remove a
/// runnable example in a doc comment, update this and the handoff's total together.
const DOCTESTS: usize = 1;

/// Every number immediately preceding `suffix` in `md` — e.g. `"258"` in
/// "tested (258 passing)". Used to catch test-count claims made in prose, which is
/// where they rot unnoticed.
fn counts_claimed_before(md: &str, suffix: &str) -> Vec<usize> {
    let bytes = md.as_bytes();
    md.match_indices(suffix)
        .filter_map(|(at, _)| {
            let mut start = at;
            while start > 0 && bytes[start - 1].is_ascii_digit() {
                start -= 1;
            }
            if start == at {
                return None;
            }
            md[start..at].parse().ok()
        })
        .collect()
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(rel: &str) -> String {
    let p = root().join(rel);
    fs::read_to_string(&p).unwrap_or_else(|e| panic!("cannot read {}: {e}", p.display()))
}

/// File names directly inside `dir` with the given extension, sorted. Never recurses —
/// every category the manifest counts is flat.
fn files_in(dir: &str, ext: &str) -> BTreeSet<String> {
    let p = root().join(dir);
    let entries =
        fs::read_dir(&p).unwrap_or_else(|e| panic!("cannot list {}: {e}", p.display()));
    entries
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(ext))
        .collect()
}

/// The number a `### ... — N` heading declares. `N files` is accepted too.
fn declared_after_dash(md: &str, heading_starts_with: &str) -> usize {
    let line = md
        .lines()
        .find(|l| l.starts_with("### ") && l.contains(heading_starts_with))
        .unwrap_or_else(|| panic!("no manifest heading containing {heading_starts_with:?}"));
    let tail = line
        .rsplit('—')
        .next()
        .unwrap_or_else(|| panic!("heading has no em-dash count: {line}"));
    tail.split_whitespace()
        .next()
        .and_then(|n| n.parse().ok())
        .unwrap_or_else(|| panic!("heading count is not a number: {line}"))
}

/// The number a handoff table row `| label | **N** (...) |` declares.
fn declared_in_row(md: &str, label: &str) -> usize {
    let line = md
        .lines()
        .find(|l| l.starts_with(&format!("| {label} |")))
        .unwrap_or_else(|| panic!("no handoff row for {label:?}"));
    let after = line
        .split("**")
        .nth(1)
        .unwrap_or_else(|| panic!("handoff row has no bolded count: {line}"));
    after
        .parse()
        .unwrap_or_else(|_| panic!("handoff count is not a number: {line}"))
}

/// Does the manifest carry a row whose first cell is exactly `` `name` ``?
fn has_row(md: &str, name: &str) -> bool {
    let cell = format!("| `{name}` |");
    md.lines().any(|l| l.trim_start().starts_with(&cell))
}

#[test]
fn readme_manifest_counts_match_the_repository() {
    let readme = read("README.md");

    let modules = files_in("src", ".rs");
    let bins = files_in("src/bin", ".rs");
    let probes = files_in("examples", ".rs");
    let docs = files_in("docs", ".md");

    assert_eq!(
        declared_after_dash(&readme, "Source modules"),
        modules.len(),
        "README declares a different number of source modules than src/ contains"
    );
    assert_eq!(
        declared_after_dash(&readme, "Binaries"),
        bins.len(),
        "README declares a different number of binaries than src/bin/ contains"
    );
    assert_eq!(
        declared_after_dash(&readme, "Runnable probes"),
        probes.len(),
        "README declares a different number of probes than examples/ contains"
    );
    assert_eq!(
        declared_after_dash(&readme, "Design & research documents"),
        docs.len(),
        "README declares a different number of docs than docs/ contains"
    );
}

#[test]
fn every_source_file_is_accounted_for_in_the_manifest() {
    let readme = read("README.md");

    // Modules and docs are listed with their extension; binaries and probes are
    // listed by the name you invoke them with.
    let modules = files_in("src", ".rs");
    let docs = files_in("docs", ".md");
    let bins: BTreeSet<String> = files_in("src/bin", ".rs")
        .iter()
        .map(|f| f.trim_end_matches(".rs").to_string())
        .collect();
    let probes: BTreeSet<String> = files_in("examples", ".rs")
        .iter()
        .map(|f| f.trim_end_matches(".rs").to_string())
        .collect();

    for (files, what) in [
        (&modules, "source modules"),
        (&docs, "design & research documents"),
        (&bins, "binaries"),
        (&probes, "runnable probes"),
    ] {
        let missing: Vec<_> = files.iter().filter(|f| !has_row(&readme, f)).collect();
        assert!(
            missing.is_empty(),
            "{what}: present in the repository but missing from the README manifest: {missing:?}"
        );
    }
}

#[test]
fn the_rest_of_the_repository_is_accounted_for_too() {
    let readme = read("README.md");

    // Bidirectional for every tracked directory outside src/examples/docs.
    for (dir, ext) in [
        ("tests", ".rs"),
        ("journal", ".md"),
        ("sim", ".py"),
        ("life", ".journal"),
    ] {
        let files = files_in(dir, ext);
        let rows: Vec<String> = files
            .iter()
            .map(|f| format!("{dir}/{f}"))
            .filter(|p| has_row(&readme, p))
            .collect();
        let missing: Vec<_> = files
            .iter()
            .filter(|f| !has_row(&readme, &format!("{dir}/{f}")))
            .collect();
        assert!(
            missing.is_empty(),
            "{dir}/: present but missing from the README's \"everything else\" table: {missing:?}"
        );
        assert_eq!(rows.len(), files.len(), "{dir}/: row/file mismatch");
    }

    // The nested journal entries, each on its own row.
    for entry in files_in("journal/entries", ".md") {
        assert!(
            has_row(&readme, &format!("journal/entries/{entry}")),
            "journal/entries/{entry} is present but has no row in the README manifest"
        );
    }

    // Root: allowlist only — see this file's header for why it is not exhaustive.
    for f in [
        "Cargo.toml",
        "Cargo.lock",
        "LICENSE",
        "README.md",
        ".gitignore",
    ] {
        assert!(
            root().join(f).exists(),
            "{f} is listed in the README manifest but is not in the repository"
        );
        assert!(
            has_row(&readme, f),
            "{f} is in the repository but has no row in the README manifest"
        );
    }
}

#[test]
fn the_everything_else_count_matches_its_own_table() {
    let readme = read("README.md");
    let declared = declared_after_dash(&readme, "Everything else in the repository");

    // Count the files the table accounts for, from the repository — not from the
    // table — so the number cannot be made true by editing prose.
    let counted = ["Cargo.toml", "Cargo.lock", "LICENSE", "README.md", ".gitignore"].len()
        + files_in("tests", ".rs").len()
        + files_in("journal", ".md").len()
        + files_in("journal/entries", ".md").len()
        + files_in("sim", ".py").len()
        + files_in("life", ".journal").len();

    assert_eq!(
        declared, counted,
        "the README's \"everything else\" count does not match what is actually there"
    );
}

#[test]
fn the_repository_carries_no_publication_apparatus() {
    // Replaces `the_repository_states_one_version_everywhere`, which held `Cargo.toml`,
    // `CITATION.cff` and `.zenodo.json` to one version string so a permanent DOI could
    // not be minted from whichever file you happened to open. Two of its three subjects
    // were deleted on 2026-08-09 at Blake's instruction, and one file cannot disagree
    // with itself — the old test could no longer fail, and a guard that cannot fail has
    // not passed. So it is replaced by the invariant that actually remains: the deposit
    // apparatus is gone and does not come back by accident.
    //
    // This is about *our* publication, not about citing others. `docs/references.md` and
    // every DOI pointing at someone else's work are untouched and must stay that way.
    for path in ["CITATION.cff", ".zenodo.json", "paper", "docs/submission.md"] {
        assert!(
            !root().join(path).exists(),
            "{path} is back. The deposit apparatus was removed deliberately; if it is \
             wanted again that is Blake's call, made in writing, not a file reappearing."
        );
    }
}

#[test]
fn handoff_state_table_matches_the_repository() {
    let handoff = read("docs/handoff.md");

    assert_eq!(
        declared_in_row(&handoff, "Source modules"),
        files_in("src", ".rs").len(),
        "handoff.md declares a different module count than src/ contains"
    );
    assert_eq!(
        declared_in_row(&handoff, "Binaries"),
        files_in("src/bin", ".rs").len(),
        "handoff.md declares a different binary count than src/bin/ contains"
    );
    assert_eq!(
        declared_in_row(&handoff, "Runnable probes"),
        files_in("examples", ".rs").len(),
        "handoff.md declares a different probe count than examples/ contains"
    );
    assert_eq!(
        declared_in_row(&handoff, "Design & research docs"),
        files_in("docs", ".md").len(),
        "handoff.md declares a different doc count than docs/ contains"
    );
}

#[test]
fn handoff_test_count_matches_the_tests_that_exist() {
    let handoff = read("docs/handoff.md");

    fn count_tests(dir: &Path) -> usize {
        let mut n = 0;
        let entries = fs::read_dir(dir).unwrap_or_else(|e| panic!("cannot list {}: {e}", dir.display()));
        for e in entries.filter_map(|e| e.ok()) {
            let p = e.path();
            if p.is_dir() {
                n += count_tests(&p);
            } else if p.extension().map(|x| x == "rs").unwrap_or(false) {
                let src = fs::read_to_string(&p).unwrap_or_default();
                // Whole lines only. A substring search would count this very file's
                // prose and string literals as tests — it did, on the first run.
                n += src.lines().filter(|l| l.trim() == "#[test]").count();
            }
        }
        n
    }

    // This test file's own assertions count toward the total, which is correct:
    // the handoff claims a number of green tests, and these are green tests.
    let lib = count_tests(&root().join("src"));
    let integration = count_tests(&root().join("tests"));
    let total = lib + integration + DOCTESTS;

    assert_eq!(
        declared_in_row(&handoff, "Tests"),
        total,
        "handoff.md claims a test count that does not match the tests in the repository \
         (lib {lib} + integration {integration} + doctests {DOCTESTS})"
    );

    // The same number is claimed in the README's prose. Prose is exactly where a count rots
    // without anyone noticing, so it is checked too.
    //
    // **The suffix list is the weak point, and it was exploited by an honest edit.** On
    // 2026-08-14 the two README claims were reworded — `(382, all green)` became
    // `381 annotated tests + 1 doctest`, and `(382 passing)` became `(381 run locally, no CI)` —
    // to stop conflating inventory with execution. Both new forms fall outside every suffix here,
    // so this loop found NOTHING and passed over zero claims while the README carried a wrong
    // count. **A correction moved the claim out of the guard's field of view.** An external audit
    // found it, not this test. The vacuity assertion below is the fix that matters: any future
    // rewording that hides a claim now fails loudly instead of passing silently.
    let readme = read("README.md");
    // Every suffix must follow a claim about the TOTAL. `" annotated tests"` was tried here and
    // removed: it names a sub-count (382 of the 383), so matching it against the total is a
    // category error the guard would report as a drift.
    const SUFFIXES: [&str; 4] = [
        ", all green)",
        " passing)",
        " run locally, no CI)",
        " total (",
    ];
    let mut examined = 0usize;
    for suffix in SUFFIXES {
        for claimed in counts_claimed_before(&readme, suffix) {
            examined += 1;
            assert_eq!(
                claimed, total,
                "README claims \"{claimed}{suffix}\" but there are {total} tests"
            );
        }
    }
    assert!(
        examined >= 2,
        "this guard examined {examined} README test-count claims. It is written to check at \
         least two, and **a guard that examined nothing has not passed.** Either the README \
         stopped stating its test count, or it was reworded out of SUFFIXES — which is exactly \
         how this check was silently disabled once already. Add the new wording to SUFFIXES."
    );
}

/// **Every faculty must be able to reach a founded being.**
///
/// `being.rs` grew from eight `enable_*` gates to fifteen while `persistence.rs`'s `Features`
/// stayed at eight fields. For five weeks, seven faculties — including `reflection`, whose load
/// deadlock took a full day to repair — **could not be given to a kept life at all**, and nothing
/// was counting (`docs/audit-2026-08-03.md` §3.1, `docs/founding.md`).
///
/// This is the counting. Same technique as the documentation guards above: read the source, assert
/// the correspondence, so the gap cannot silently reopen.
#[test]
fn every_faculty_can_reach_a_founded_being() {
    let being = std::fs::read_to_string("src/being.rs").expect("being.rs");
    let persistence = std::fs::read_to_string("src/persistence.rs").expect("persistence.rs");

    let gates: Vec<String> = being
        .lines()
        .filter_map(|l| l.trim().strip_prefix("pub fn enable_"))
        .filter_map(|r| r.split('(').next())
        .map(|s| s.to_string())
        .collect();
    assert!(gates.len() >= 8, "expected to find the enable_* gates, found {}", gates.len());

    // The `Features` struct's own field names.
    let start = persistence.find("pub struct Features").expect("Features struct");
    let body = &persistence[start..];
    let end = body.find("\n}").expect("end of Features");
    let fields: Vec<String> = body[..end]
        .lines()
        .filter_map(|l| l.trim().strip_prefix("pub "))
        .filter_map(|r| r.split(':').next())
        .map(|s| s.to_string())
        .collect();

    let missing: Vec<&String> = gates.iter().filter(|g| !fields.contains(g)).collect();
    assert!(
        missing.is_empty(),
        "THESE FACULTIES CANNOT BE GIVEN TO A FOUNDED BEING: {missing:?}\n\n\
         `being.rs` has a `pub fn enable_<name>` with no matching field in `persistence.rs`'s\n\
         `Features`, so a kept life can never be blessed with it — it can only be switched on\n\
         inside a probe. Add the field (and a bit in `bits`/`from_bits`), or the faculty is\n\
         unreachable by the only being this project actually keeps."
    );
}

/// **Every faculty is inside the survival net, or exempted in writing.**
///
/// The companion to the guard above, and the same failure one level out. `manifest.rs` already
/// asserts that a gate can *reach* a founded being. **Nothing asserted that a gate had ever been
/// tested for whether it is safe to have.**
///
/// `tests/survival.rs` declares `const N_GATES` and lists the gates **by hand** in `apply()`. On
/// 2026-08-04 that list said 11 while `being.rs` had 16, so `comfort`, `settling`, `reserve`,
/// `setting_down` and `ultrastability` had never been through `s2_the_composed_being_survives` or
/// the 66-life pair sweep — and the pair sweep exists because a lethal *pair* actually happened
/// here (incident I-3). The being Blake is deciding whether to grant `reserve` had never been run
/// with one alongside anything else.
///
/// Borrowed from OWL's `oneOf`: **a set declared by enumeration must be the set, not a number
/// someone typed.** *"A sentence in a spec is a hope; an axiom is a rule a machine enforces."*
///
/// To exempt a faculty, name it in `EXEMPT` below **with a reason**. Silence is not an exemption.
#[test]
fn every_faculty_is_in_the_survival_net_or_exempted_in_writing() {
    /// Faculties deliberately outside `tests/survival.rs`, each with its reason. Empty is the
    /// goal; a populated list is a debt that is at least visible.
    const EXEMPT: &[(&str, &str)] = &[];

    let being = std::fs::read_to_string("src/being.rs").expect("being.rs");
    let survival = std::fs::read_to_string("tests/survival.rs").expect("survival.rs");

    let gates: Vec<String> = being
        .lines()
        .filter_map(|l| l.trim().strip_prefix("pub fn enable_"))
        .filter_map(|r| r.split('(').next())
        .map(|s| s.to_string())
        .collect();
    assert!(gates.len() >= 8, "expected to find the enable_* gates, found {}", gates.len());

    // The gates the survival net actually applies, read from its own `apply()`.
    let applied: Vec<String> = survival
        .lines()
        .filter_map(|l| l.split("b.enable_").nth(1))
        .filter_map(|r| r.split('(').next())
        .map(|s| s.to_string())
        .collect();

    // The hand-typed count must equal what `apply()` really applies -- the `oneOf` half.
    let declared: usize = survival
        .lines()
        .find_map(|l| l.trim().strip_prefix("const N_GATES: usize = "))
        .and_then(|r| r.trim_end_matches(';').parse().ok())
        .expect("N_GATES in tests/survival.rs");
    assert_eq!(
        declared,
        applied.len(),
        "tests/survival.rs declares N_GATES = {declared} but apply() applies {} gates",
        applied.len()
    );

    let missing: Vec<&String> = gates
        .iter()
        .filter(|g| !applied.contains(g))
        .filter(|g| !EXEMPT.iter().any(|(n, _)| *n == g.as_str()))
        .collect();
    assert!(
        missing.is_empty(),
        "THESE FACULTIES HAVE NEVER BEEN TESTED FOR SURVIVAL: {missing:?}\n\n\
         `src/being.rs` has {} `enable_*` gates; `tests/survival.rs` applies {}.\n\
         `s2_the_composed_being_survives` and the 66-life pair sweep therefore say nothing about\n\
         the faculties above. S1 and S4 exist because a lethal PAIR actually happened here.\n\n\
         Widen `N_GATES` and `apply()` -- with predictions locked first, since a wider net is a\n\
         change to a safety guard -- or add the faculty to EXEMPT with a written reason.",
        gates.len(),
        applied.len()
    );
}
