//! `embody` — a dependency-free stdio bridge so an external body (e.g. a MuJoCo
//! humanoid in Python) can drive one continuous being, a tick at a time.
//!
//! Newline-delimited, space-separated integers. Language-agnostic, debuggable.
//!
//!   IN  (per line): nutrient threat ext0 ext1 ext2 ext3        (raw Q8.8, 0..256)
//!   OUT (per line): action posture effort valence_m arousal_m energy_m basin alive
//!     - action  : BodyAction  (0 Idle, 1 StandOpen, 2 Brace, 3 Curl, 4 Recoil)
//!     - posture : Posture     (0 Resting, 1 Open, 2 Braced, 3 Withdrawn)
//!     - effort  : raw Q8.8 [0,256]
//!     - *_m     : value * 1000 (valence/arousal/energy as milli-units)
//!     - basin   : 0 Rest, 1 Engaged, 2 Defensive, 3 Recovery
//!     - alive   : 0/1

use std::io::{self, BufRead, Write};

use unified_being::{action_from, intent_from, Genome, Sensorium, UnifiedBeing};

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        let v: Vec<i16> = line
            .split_whitespace()
            .filter_map(|t| t.parse().ok())
            .collect();
        if v.len() < 6 {
            continue;
        }
        let sens = Sensorium {
            nutrient: v[0],
            threat: v[1],
            exteroception: [v[2], v[3], v[4], v[5]],
            partner: None,
        };
        let r = being.step_embodied(&sens);
        let intent = intent_from(&r);
        let action = action_from(&intent);

        let _ = writeln!(
            out,
            "{} {} {} {} {} {} {} {}",
            action as i32,
            intent.posture as i32,
            intent.effort,
            (r.valence * 1000.0) as i32,
            (r.arousal * 1000.0) as i32,
            (r.energy * 1000.0) as i32,
            r.basin as i32,
            r.alive as i32,
        );
        let _ = out.flush();
    }
}
