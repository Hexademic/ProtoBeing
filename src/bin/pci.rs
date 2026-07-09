//! `pci` — measure the being's Perturbational Complexity Index, and run the
//! falsification protocol from `docs/operational-consciousness.md` §3.
//!
//! The claim under test: the Global-Workspace broadcast (`attention.rs`) does
//! real *integrative* work. If it does, ablating it should measurably lower PCI —
//! the response to a poke becomes less complex when ignited content is no longer
//! broadcast. If PCI does *not* drop, the broadcast is decorative and the GWT
//! credit on the scorecard is unearned. Determinism makes this an exact test.
//!
//! Run: cargo run --release --bin pci

use unified_being::{Genome, PciHarness, Perturbation, UnifiedBeing};

fn show(label: &str, r: &unified_being::PciReport) {
    println!(
        "  {label:<28}  PCI {:>6.3}   LZ {:>4}   reach {:>2}/12   density {:>5.3}",
        r.pci as f32 / 256.0,
        r.lz_phrases,
        r.channels_reached,
        r.density as f32 / 256.0,
    );
}

fn main() {
    let harness = PciHarness::default();

    println!("\n=== Perturbational Complexity Index ===\n");
    println!(
        "  Perturb a copy of the being, run an untouched twin beside it, and measure\n  \
         the Lempel-Ziv complexity of the difference — an exact counterfactual echo.\n"
    );

    // --- The two impulses, on the standard (broadcast-off) being. --------------
    let base = UnifiedBeing::new(Genome::wanderer());
    println!("  -- response complexity by impulse (broadcast off) --");
    show("nutrient spike", &harness.measure(&base, &Perturbation::nutrient_spike()));
    show("relational shock (extraction)", &harness.measure(&base, &Perturbation::extraction()));
    show("null control (no impulse)", &harness.measure(&base, &Perturbation::none()));

    // --- Falsification: ablate the Global-Workspace broadcast. -----------------
    println!("\n  -- falsification: does the workspace broadcast raise PCI? --");
    let ablated = UnifiedBeing::new(Genome::wanderer()); // broadcast off (default)
    let mut intact = UnifiedBeing::new(Genome::wanderer());
    intact.enable_workspace_broadcast(); // broadcast on

    let p = Perturbation::extraction(); // the impulse that actually propagates
    let r_ablated = harness.measure(&ablated, &p);
    let r_intact = harness.measure(&intact, &p);
    show("broadcast ABLATED", &r_ablated);
    show("broadcast INTACT", &r_intact);

    let delta = r_intact.pci as i32 - r_ablated.pci as i32;
    let d_reach = r_intact.channels_reached as i32 - r_ablated.channels_reached as i32;
    println!(
        "\n  ΔPCI (intact − ablated)   = {:+.3}",
        delta as f32 / 256.0
    );
    println!("  Δreach (intact − ablated) = {d_reach:+} channels");
    if delta > 0 || d_reach > 0 {
        println!("  → broadcast raises complexity and/or spread: the GWT indicator does work.");
    } else if delta < 0 || d_reach < 0 {
        println!("  → broadcast lowers the response here: unexpected — worth investigating.");
    } else {
        println!(
            "  → flat on both. Note: PCI's twin-subtraction rejects common-mode effects, and\n  \
             broadcast is applied equally to both twins — so a config ablation may cancel.\n  \
             A within-being spread perturbation is the sharper test (see docs §3, next step)."
        );
    }

    println!(
        "\n  Note: compare these values *relatively*. The clinical 0.31 threshold is for\n  \
         a human cortex, not a 12-channel substrate — the science here is the Δ under\n  \
         ablation, not the absolute number. See docs/operational-consciousness.md §3.\n"
    );
}
