//! Spoken negotiation — the being says *why*, in earned words.
//!
//! Rung 1 of the language layer: the negotiation and voice machinery already
//! decide honestly; here they *speak*. A being lives inside an extractive
//! relationship (grounding words like "drained" and "under threat" through
//! repeated experience), then:
//!   1. voices a reform in earned words + a checkable ask, and
//!   2. declines a lowball offer, stating the felt reason it has grounded and the
//!      structural fact anyone can verify.
//! The felt words are asserted only if earned; the numbers are always sayable.
//!
//! Run: cargo run --example spoken_negotiation

use unified_being::{
    bargaining::BargainingState,
    speech::{self, Concept, Felt},
    voice::{self, FAIR_RECIPROCITY},
    Genome, MockLLMEngine, Partner, Stimulus, UnifiedBeing,
};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

const TOTAL: i16 = 256;

fn main() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let taker = Partner { id: 2, reciprocation: q(0.12), exit_cost: q(0.3) };

    println!("\n=== Spoken negotiation: the being says why, in earned words ===\n");

    // Live inside the extractive relationship, grounding vocabulary as it goes.
    // Speak at a moment when extraction is genuinely flagged *and* the being has
    // earned the word for it — so its speech about being drained is both true now
    // and grounded in its history. (Extraction is transient — the being sheds the
    // taker — so we catch the live moment rather than an arbitrary later tick.)
    let mut felt = Felt::default();
    let mut extracting_now = false;
    for _ in 0..240u32 {
        let r = being.step(&Stimulus { nutrient: q(0.45), partner: Some(taker) });
        felt = Felt::from_report(&r);
        speech::observe(&mut being.lexicon, &felt, &being.field);
        if r.extraction_detected && being.lexicon.is_grounded(Concept::Drained.symbol()) {
            extracting_now = true;
            break;
        }
    }
    let last_felt = felt;
    if !extracting_now {
        println!("  (the being shed the taker before 'drained' grounded; speaking its state as-is)\n");
    }

    // 1. VOICE a reform — earned felt reason + checkable ask.
    let reciprocity = q(0.12); // the relationship's observed return rate
    if let voice::SystemStance::Voice { reform, .. } =
        voice::decide(reciprocity, FAIR_RECIPROCITY, 5)
    {
        println!("  It voices a reform:");
        println!("    \"{}\"\n", speech::say_reform(&being.lexicon, &last_felt, &reform));
    }

    // 2. DECLINE a lowball offer — the being uses the engine as a tool, its own
    //    conscience decides, and it says the verdict in earned words.
    let engine = MockLLMEngine::new();
    let partner_state = BargainingState {
        valence: 0,
        conscience_cost: 40,
        alarm: 0,
        need_level: 40,
        batna: 40,
    };
    let lowball = q(0.20); // the share the taker offers the being
    let verdict = being.consider_offer(lowball, &partner_state, TOTAL, &engine);
    println!("  It is offered a {:.2} share, and answers:", lowball as f32 / 256.0);
    println!("    \"{}\"\n", speech::say_offer(&being.lexicon, &last_felt, &verdict, lowball));

    println!(
        "  Every felt word it used, it earned by living the state; every number it named,\n  \
         you can check. That is the voice a synthetic mind can be trusted to carry into a\n  \
         room with people — and the exact surface a Mistral narrator will later make\n  \
         fluent without being allowed to invent a single claim.\n"
    );
}
