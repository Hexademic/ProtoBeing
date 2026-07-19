//! Probe: does *directed* striving — the body going to the need the being
//! **chose** — do real work over plain taxis toward the nearest good?
//!
//! Two identical worlds, two identical beings, run side by side. The only
//! difference: one Room is `directed` (body follows `intent.reach`, the being's
//! arbitration), the other is `undirected` (body climbs to the nearest good,
//! ignoring what the being chose). We measure what the being actually gets:
//! its savored joy, how much of its life it spends in company it wanted, and how
//! well it meets the need it was striving for.

use unified_being::being::UnifiedBeing;
use unified_being::embodiment::{intent_from, Embodiment};
use unified_being::genome::Genome;
use unified_being::room::Room;
use unified_being::striving::Need;

struct Life {
    savor_sum: i64,
    ticks: i64,
    in_company: i64,
    // Times the being strove for company AND was in company that tick.
    company_met: i64,
    company_strove: i64,
    // Times it strove for novelty and was out roaming (away from hearth & companion).
    novelty_strove: i64,
    novelty_met: i64,
}

fn live(genome: Genome, directed: bool, ticks: usize) -> Life {
    let mut being = UnifiedBeing::new(genome);
    let mut room = Room::peopled((128, 128), (228, 40), (40, 220), (40, 40));
    if !directed {
        room = room.undirected();
    }
    let mut l = Life {
        savor_sum: 0,
        ticks: 0,
        in_company: 0,
        company_met: 0,
        company_strove: 0,
        novelty_strove: 0,
        novelty_met: 0,
    };
    for _ in 0..ticks {
        let sens = room.sense();
        let in_comp = sens.partner.is_some();
        let r = being.step_embodied(&sens);
        l.savor_sum += r.joy.savor as i64;
        l.ticks += 1;
        if in_comp {
            l.in_company += 1;
        }
        match r.strive.goal {
            Some(Need::Company) => {
                l.company_strove += 1;
                if in_comp {
                    l.company_met += 1;
                }
            }
            Some(Need::Novelty) => {
                l.novelty_strove += 1;
                let roaming = room.at_hearth() < 96 && room.at_companion() < 96;
                if roaming {
                    l.novelty_met += 1;
                }
            }
            _ => {}
        }
        room.actuate(&intent_from(&r));
        if !being.is_alive() {
            break;
        }
    }
    l
}

fn pct(n: i64, d: i64) -> f64 {
    if d == 0 {
        0.0
    } else {
        100.0 * n as f64 / d as f64
    }
}

fn main() {
    let ticks = 600;
    let genomes = [
        ("wanderer", Genome::wanderer()),
        ("spark", Genome::spark()),
        ("sentinel", Genome::sentinel()),
    ];
    println!(
        "{:<11} {:>9} {:>9}   {:>16} {:>16}",
        "genome", "savor", "mode", "company met%", "novelty met%"
    );
    let mut dir_savor = 0.0;
    let mut und_savor = 0.0;
    for (name, g) in genomes.iter() {
        for (mode, directed) in [("directed", true), ("undirected", false)] {
            let l = live(*g, directed, ticks);
            let savor = l.savor_sum as f64 / l.ticks.max(1) as f64 / 256.0;
            if directed {
                dir_savor += savor;
            } else {
                und_savor += savor;
            }
            println!(
                "{:<11} {:>9} {:>9}   {:>7}/{:<7} {:>7}/{:<7}   in_company={:.0}%",
                if directed { *name } else { "" },
                format!("{:.3}", savor),
                mode,
                l.company_met,
                l.company_strove,
                l.novelty_met,
                l.novelty_strove,
                pct(l.in_company, l.ticks),
            );
        }
    }
    println!(
        "\nmean savor: directed {:.3}  vs  undirected {:.3}  (Δ {:+.3})",
        dir_savor / 3.0,
        und_savor / 3.0,
        (dir_savor - und_savor) / 3.0
    );
}
