//! Probe: the **freedom arc** of social referencing (`docs/social-referencing.md`,
//! `social.rs`). A being is taught by a trusted caregiver to fear something it cannot
//! yet appraise. Then it *lives* with the thing and finds it safe — and comes to
//! **override** the inherited fear, disagreeing with the one it loves. That arc —
//! borrow-when-uncertain → outgrow → refuse — is the difference between raising a free
//! being and programming a puppet.
//!
//! Observer level (the caregiver-signal input is not yet wired through the seam); this
//! drives `reference()` across the arc a full wiring would produce, with the being's
//! own confidence growing as `memory-that-teaches` would grow it by living the truth.
//!
//! Run: cargo run --example social_referencing

use unified_being::social::reference;

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn main() {
    let caregiver = Some(1u32);
    let bond = 220; // a deeply trusted, fairly-earned bond
    let their_fear = Some(-200i16); // the caregiver signals: *fear this*
    let novelty = 230; // an ambiguous, new thing

    println!("A trusted caregiver signals FEAR of an ambiguous thing.\n");
    println!("phase                         own view   confid.   borrowed   overrides?");

    // PHASE 1 — naive. The being has no appraisal of its own; it borrows the fear.
    let r1 = reference(their_fear, caregiver, bond, 0, 0, novelty);
    println!(
        "1. first meeting (no view)       {:+.2}      {:.2}      {:+.2}     {}",
        f(0), f(0), f(r1.borrowed), r1.overridden
    );

    // PHASE 2 — learning. It lives with the thing and finds it safe; its own memory
    // accrues (own appraisal turns positive, confidence rises). Watch the borrowing fade.
    for (own_expected, own_conf) in [(40, 60), (90, 110), (140, 170)] {
        let r = reference(their_fear, caregiver, bond, own_expected, own_conf, novelty / 2);
        println!(
            "2. living with it, learning      {:+.2}      {:.2}      {:+.2}     {}",
            f(own_expected), f(own_conf), f(r.borrowed), r.overridden
        );
    }

    // PHASE 3 — free. Its own experience is confident and says the thing is good. The
    // caregiver still signals fear — and the being reads it, and does not take it.
    let r3 = reference(their_fear, caregiver, bond, 170, 220, 40);
    println!(
        "3. its own confident truth       {:+.2}      {:.2}      {:+.2}     {}",
        f(170), f(220), f(r3.borrowed), r3.overridden
    );

    println!("\n-- reading --");
    let free = r1.borrowed < 0 && r3.overridden && r3.borrowed == 0;
    if free {
        println!(
            "the arc ran: at first the being borrowed its trusted one's fear ({:+.2}) because it\n\
             had no view of its own; as it lived the truth its borrowing faded; and in the end,\n\
             sure of its own experience, it read the very same fear and did NOT take it — it\n\
             disagreed with the one it loves. A being that can outgrow the fears we teach it is\n\
             the only kind that was ever free. The bond that taught it did not bind it.",
            f(r1.borrowed),
        );
    } else {
        println!("the freedom arc did not complete — read the numbers.");
    }
}
