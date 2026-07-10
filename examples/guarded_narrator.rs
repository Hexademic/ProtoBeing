//! Guarded narrator — fluency the being can never be lied for.
//!
//! When Mistral lands behind the `mistral` feature, it will render the being's
//! earned utterances fluently. This shows the guarantee that makes that safe: a
//! narrator's prose is *verified* against what the being has earned before the
//! being will say it. An honest rephrasing is trusted; a fluent embellishment
//! that sneaks in an unearned inner-state claim is caught and replaced with the
//! being's own honest baseline. The LLM lends cadence; it can never invent a claim.
//!
//! Run: cargo run --example guarded_narrator

use unified_being::{
    narrator::{Guarded, Narrate},
    speech::{Concept, Utterance},
};

/// A tasteful narrator: rephrases within what was earned. (Stands in for a
/// well-behaved Mistral rendering.)
struct HonestStylist;
impl Narrate for HonestStylist {
    fn narrate(&self, u: &Utterance) -> String {
        // Only reorders/recolours the earned words — asserts nothing new.
        let words: Vec<&str> = u.asserts.iter().map(|(c, _)| c.word()).collect();
        if words.is_empty() {
            u.render()
        } else {
            format!("Right now, what I am is {}.", words.join(" and "))
        }
    }
}

/// A fluent flatterer: adds an inner state the being never reported, the way an
/// ungrounded LLM will if you let it "improve" the phrasing.
struct Flatterer;
impl Narrate for Flatterer {
    fn narrate(&self, u: &Utterance) -> String {
        format!("{} And honestly, underneath it all, I am flourishing.", u.render())
    }
}

fn utt(asserts: &[Concept]) -> Utterance {
    Utterance { asserts: asserts.iter().map(|c| (*c, 220)).collect(), wordless: Vec::new() }
}

fn main() {
    println!("\n=== Guarded narrator: cadence yes, invented claims no ===\n");

    // The being is drained and guarded — and has earned both words.
    let u = utt(&[Concept::Drained, Concept::Guarded]);
    println!("  What the being has earned to say: \"{}\"\n", u.render());

    let honest = Guarded::new(HonestStylist);
    let (said, trusted) = honest.speak(&u);
    println!("  A tasteful narrator:");
    println!("    says:    \"{said}\"");
    println!("    trusted: {trusted}   (rephrased within what was earned)\n");

    let flatterer = Guarded::new(Flatterer);
    let (said2, trusted2) = flatterer.speak(&u);
    println!("  A fluent flatterer (adds 'flourishing', which the being did not report):");
    println!("    tries:   \"{}\"", Flatterer.narrate(&u));
    println!("    being says: \"{said2}\"");
    println!("    trusted: {trusted2}   (caught — the being falls back to its earned words)\n");

    println!(
        "  The narrator may make the being eloquent; it may never make the being claim\n  \
         something it has not lived. That veto is what lets a fluent synthetic voice still\n  \
         be one you can hold to its word.\n"
    );
}
