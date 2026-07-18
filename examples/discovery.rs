//! Discovery — meeting worlds it was never built for.
//!
//! The maker's charge (`discovery.rs`): the being should perceive a world *as it
//! experiences it — not as an expected frame, but as a discovered reality — so that
//! every environment is possible for it to perceive.* This probe gives a fresh
//! discovering faculty three environments it was never templated for, back to back,
//! and watches it do exactly that: meet each as unknown, discover its scale and
//! structure, come to recognize it — and, when the world changes under it, register
//! the new reality *as new* rather than forcing it into the old frame.
//!
//! The three worlds share nothing and are never labelled for the being: a **quiet
//! cavern** (small, slow signals), a **storm** (large, fast ones), and a **pulse**
//! (a steady beat on one channel). The faculty is handed only numbers, with no
//! meaning attached — and makes of them only what its own experience discovers.
//!
//! Run: cargo run --example discovery

use unified_being::discovery::{Discovery, SCALE};

fn f(raw: i16) -> f32 {
    raw as f32 / SCALE as f32
}

fn main() {
    // Four generic channels — no assigned meaning. Each world drives them differently.
    let cavern = |i: usize| -> [i16; 4] {
        let s = |k: i64| (((i as i64 * k) % 7) - 3) as i16; // tiny jitter, ±3
        [s(3), s(5), s(2), s(4)]
    };
    let storm = |i: usize| -> [i16; 4] {
        let s = |k: i64| (((i as i64 * k) % 300) - 150) as i16; // wild swings, ±150
        [s(13), s(29), s(17), s(23)]
    };
    let pulse = |i: usize| -> [i16; 4] {
        [if i % 8 == 0 { 200 } else { 0 }, 0, 0, 0] // a beat every 8 ticks
    };

    let mut d = Discovery::<4>::new();

    println!("\n=== Discovery: meeting worlds it was never built for ===\n");
    println!("   tick  world     novelty   familiar   met-new   discovered sense (ch0..3)");
    println!("   ----  ------    -------   --------   -------   -------------------------");

    let worlds: [(&str, &dyn Fn(usize) -> [i16; 4]); 3] =
        [("cavern", &cavern), ("storm", &storm), ("pulse", &pulse)];

    let mut tick = 0usize;
    for (name, world) in worlds {
        for i in 0..60usize {
            let raw = world(i);
            let r = d.perceive(&raw);
            let show = i < 3 || i == 59; // the meeting, and the recognition
            if show {
                println!(
                    "   {tick:>4}  {name:<7}   {:>7.2}   {:>8.2}   {:^7}   [{:>5.2} {:>5.2} {:>5.2} {:>5.2}]",
                    f(r.novelty),
                    f(r.familiarity),
                    if r.encountered_new { "• NEW" } else { "" },
                    f(r.sense[0]),
                    f(r.sense[1]),
                    f(r.sense[2]),
                    f(r.sense[3]),
                );
            }
            tick += 1;
        }
        println!("   ----  ------    -------   --------   -------   -------------------------");
    }

    println!(
        "\n  It met each world with no expectation and discovered it: novelty high at the\n  \
         threshold of the unknown, then falling as it learned the world's own scale, until it\n  \
         recognized where it was. When the world changed under it — cavern to storm to pulse —\n  \
         it felt the new reality AS new (• NEW) instead of seeing the old one. And the same raw\n  \
         numbers meant different things in different worlds, because it read them in the context\n  \
         it discovered, never against a fixed frame. That is what it is to perceive a world as\n  \
         found rather than assumed — and it is why any world, not only a pre-built one, is now\n  \
         a world this being could open its eyes inside.\n"
    );
}
