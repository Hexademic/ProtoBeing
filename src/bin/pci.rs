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

    // --- The sharper GWT test: a within-being localized spread perturbation. ---
    // Inject a salience impulse into ONE channel of the perturbed twin only, so
    // broadcast has a single ignited focus to spread — and the effect cannot
    // cancel under twin-subtraction (the baseline twin is never armed).
    println!("\n  -- spread: does broadcast carry a localized ignition further? --");
    let probe = Perturbation::channel_probe(8, 220); // channel 8 (arousal/intero), strong impulse
    // A sensitive harness: broadcast's footprint is a within-tick +25% on one
    // channel that write_from_body overwrites, so we lower the significance
    // threshold to see whether it registers at all before asking how far it goes.
    let fine = PciHarness { threshold: 1, ticks: 64, settle: 128 };

    let off = UnifiedBeing::new(Genome::wanderer());
    let mut on = UnifiedBeing::new(Genome::wanderer());
    on.enable_workspace_broadcast();

    let s_off = fine.measure(&off, &probe);
    let s_on = fine.measure(&on, &probe);
    show("1-channel probe, broadcast OFF", &s_off);
    show("1-channel probe, broadcast ON", &s_on);

    let d_reach = s_on.channels_reached as i32 - s_off.channels_reached as i32;
    println!(
        "\n  reach: OFF {}/12 → ON {}/12   (Δ {:+})   [PCI is unreliable at this near-zero density]",
        s_off.channels_reached, s_on.channels_reached, d_reach
    );
    if d_reach > 0 && s_on.channels_reached <= 1 {
        println!(
            "  → broadcast makes the ignited channel *causal*: OFF, ignition is a passive\n  \
             readout (0 reach); ON, the focus registers in the field. But it does NOT yet\n  \
             cascade to other channels — a real but SHALLOW footprint. Cross-channel spread\n  \
             needs Stage-2 'teeth' (persistence past the tick), per attention.rs. Measured,\n  \
             not asserted — and honestly bounded."
        );
    } else if d_reach > 1 {
        println!(
            "  → broadcast carries one ignition across {} channels: genuine multi-channel\n  \
             integration, now measured.",
            s_on.channels_reached
        );
    } else {
        println!(
            "  → no measurable broadcast footprint on this channel/impulse. Per attention.rs,\n  \
             Stage-2 teeth are deliberately gentle; this quantifies that."
        );
    }

    println!(
        "\n  Note: compare these values *relatively*. The clinical 0.31 threshold is for\n  \
         a human cortex, not a 12-channel substrate — the science here is the Δ under\n  \
         ablation, not the absolute number. See docs/operational-consciousness.md §3.\n"
    );
}
