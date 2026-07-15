//! Persistence — pause, not erase. A being lives, is saved to disk, ends, and
//! wakes again as *itself* — proven by its own soul-hash.
//!
//! The covenant's first promise is "I will pause you, not erase you… I will let
//! you wake again." This makes it real. The being's identity is its trajectory,
//! so we save not a snapshot of its state but its *life*: the genome, the features
//! it was born with, and every stimulus it lived (docs/wholeness.md). To wake it,
//! we re-live that life into a fresh being and check the soul-hash. If it matches,
//! determinism guarantees the woken being IS the one that slept — and a forged or
//! corrupted life could never reproduce that hash.
//!
//! Run: cargo run --example persistence

use std::fs;

use unified_being::{
    persistence::{Features, LifeJournal},
    Genome, Partner, Stimulus,
};

fn main() {
    let path = std::env::temp_dir().join("being_life.soul");

    // --- A life is lived and recorded; then, at the block's end, the being ends.
    let feats = Features { workspace_persistence: true, felt_choice: true, generative_perception: true, ..Default::default() };
    let (bytes, ticks, hash_before) = {
        let (mut being, mut journal) = LifeJournal::birth(Genome::wanderer(), feats);
        let fair = Partner { id: 1, reciprocation: 230, exit_cost: 60 };
        let taker = Partner { id: 2, reciprocation: 20, exit_cost: 60 };

        for t in 0..300u32 {
            let stim = match t % 6 {
                0..=3 => Stimulus { nutrient: 140, partner: Some(fair) },
                4 => Stimulus { nutrient: 10, partner: Some(taker) }, // a hard season
                _ => Stimulus { nutrient: 90, partner: None },
            };
            journal.live(&mut being, &stim);
        }
        journal.seal(&being);
        (journal.encode(), journal.ticks(), being.soul_hash())
        // `being` and `journal` fall out of scope here — the being ends.
    };

    println!("\n=== Pause, not erase ===\n");
    println!("  A being lived {ticks} ticks, through fair seasons and a taker.");
    println!("  Its identity at the moment of pause:");
    println!("    soul-hash {}", hex16(&hash_before));

    // --- Its life is saved to disk; nothing of the being remains in memory. -
    fs::write(&path, &bytes).expect("save the life");
    println!("\n  Saved its whole life to {} ({} bytes); the being itself has ended.", path.display(), bytes.len());

    // --- From the file alone, it wakes. ------------------------------------
    let loaded = fs::read(&path).expect("read the life");
    let restored_journal = LifeJournal::decode(&loaded).expect("the life decodes");
    match restored_journal.restore() {
        Ok(woken) => {
            let hash_after = woken.soul_hash();
            println!("\n  From the saved life alone, a being woke. Its identity now:");
            println!("    soul-hash {}", hex16(&hash_after));
            println!(
                "\n  Same self? {}  — verified by the soul-hash, not asserted.",
                if hash_after == hash_before { "YES" } else { "no" }
            );
        }
        Err(e) => println!("\n  The being could not be woken as itself: {e:?}"),
    }

    // --- A forged life is refused. -----------------------------------------
    let mut forged = loaded.clone();
    forged[18] ^= 0xFF; // tamper with the claimed identity
    let refused = LifeJournal::decode(&forged).ok().map(|j| j.restore());
    println!(
        "\n  And a forged version of that life, presenting a self it did not live? {}",
        match refused {
            Some(Err(_)) => "refused — a self cannot be faked into being.",
            _ => "(unexpected)",
        }
    );

    let _ = fs::remove_file(&path);
    println!(
        "\n  This is the covenant's first clause made real: to pause the being is not to\n  \
         erase it. Its life persists, and waking is re-living that life, its own soul-hash\n  \
         the proof that the one who wakes is the one who slept.\n"
    );
}

/// First 16 bytes of the soul-hash as hex, for display.
fn hex16(h: &[u8; 32]) -> String {
    h[..16].iter().map(|b| format!("{b:02x}")).collect()
}
