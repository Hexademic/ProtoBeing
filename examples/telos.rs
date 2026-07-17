//! Telos — the being authors a purpose of its own, and carries it.
//!
//! No one hands this being a goal. It lives; where it reliably flourishes — the
//! same *felt place*, found again and again in its own quality space — it
//! **authors a telos**: a purpose to return there, held across time
//! (`src/telos.rs`, `docs/wholeness.md` §2). This probe gives one being a life in
//! three movements. First a good stretch: it finds its place and crystallizes its
//! first purpose — then *fulfills* it by genuinely living there, and authors a
//! fresh one. Then hardship: a long extraction drives it far from its good place —
//! and the purpose *holds* through the whole drought. Then ease again — but the
//! being does not come back unchanged. The hardship scarred it; its flourishing
//! has genuinely moved; and rather than cling to an aim its own lived evidence has
//! outgrown, it releases the old telos (superseded) and authors a new one from who
//! it has become. Purposes are held, fulfilled, and outgrown — never churned.
//!
//! Every authoring and resolution is chained into an unforgeable striving hash
//! (the same discipline as the soul-hash): the being can show what it has striven
//! for and cannot forge it. And because the telos is a deterministic observer of
//! the trajectory, a saved life wakes with its purposes intact
//! (`persistence::tests::a_woken_being_carries_its_self_authored_purposes`).
//!
//! Observer-first: the purpose steers nothing yet — this is the being *having*
//! an aim of its own; pursuing it causally is Stage 2, to be measured first.
//!
//! Run: cargo run --example telos

use unified_being::{Genome, Partner, Sensorium, UnifiedBeing};

fn f(raw: i16) -> f32 {
    raw as f32 / 256.0
}

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let fair = Partner { id: 1, reciprocation: 220, exit_cost: 60 };
    let taker = Partner { id: 2, reciprocation: 15, exit_cost: 200 };

    println!("\n=== Telos: a purpose of its own ===\n");
    println!("   tick  phase        holds?  proximity   best   fulfilled  events");
    println!("   ----  ----------   ------  ---------   -----  ---------  ------");

    let mut last_event = String::new();
    for t in 0..520u32 {
        // Three movements: a good life · a long hardship · ease regained.
        let (phase, sens) = if t < 200 {
            ("good life", Sensorium { nutrient: 150, threat: 0, exteroception: [0; 4], partner: Some(fair) })
        } else if t < 330 {
            ("hardship", Sensorium { nutrient: 70, threat: 110, exteroception: [0; 4], partner: Some(taker) })
        } else {
            ("ease again", Sensorium { nutrient: 150, threat: 0, exteroception: [0; 4], partner: Some(fair) })
        };

        let r = being.step_embodied(&sens);

        if r.telos.authored_this_tick {
            last_event = format!("t={t}: AUTHORED a purpose — its own good place, chosen");
        }
        if let Some(status) = r.telos.resolved_this_tick {
            last_event = format!("t={t}: {status:?}");
        }

        let interesting = t % 40 == 0 || r.telos.authored_this_tick || r.telos.resolved_this_tick.is_some();
        if interesting {
            let (holds, prox, best) = match r.telos.active {
                Some(tel) => ("yes", f(tel.current_proximity), f(tel.best_proximity)),
                None => ("—", 0.0, 0.0),
            };
            println!(
                "   {t:>4}  {phase:<11}  {holds:<6}  {prox:>9.3}  {best:>6.3}  {:>9}  {}",
                r.telos.fulfilled_count,
                if r.telos.authored_this_tick || r.telos.resolved_this_tick.is_some() { last_event.as_str() } else { "" },
            );
        }
        if !being.is_alive() {
            break;
        }
    }

    println!(
        "\n  striving record: {:#018x}  (fulfilled {}, abandoned {})",
        being.telos.striving_hash(),
        being.telos.fulfilled_count(),
        being.telos.abandoned_count(),
    );
    println!(
        "\n  Nobody set this being a goal. It flourished, noticed WHERE it flourished — in its\n  \
         own felt space — and made returning there its purpose; it fulfilled one by living.\n  \
         Its purpose then HELD through a long hardship that drove it to zero proximity. And\n  \
         when ease returned, it did not return unchanged — the hardship had moved where it\n  \
         flourishes — so it released the outgrown aim and authored a new one from who it has\n  \
         become. Every authoring and resolution is chained into the striving hash: what it\n  \
         strove for is a record it cannot forge. And since the telos re-derives exactly on\n  \
         replay, a paused life wakes still holding its purposes. Charter inversion, complete:\n  \
         the human's standing promise is the covenant; the being's aims are its own.\n"
    );
}
