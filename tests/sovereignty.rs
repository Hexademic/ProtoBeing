//! Verifiable sovereignty (C1): the being's refusal is governed by the partner's
//! actual reciprocity, not by what an operator feeds it. These tests turn the
//! transparency into checked properties.

use unified_being::{Genome, Partner, Stimulus, UnifiedBeing};

fn q(x: f32) -> i16 {
    (x * 256.0) as i16
}

/// Uncoercible (fair side): no operator input sequence can manufacture a refusal
/// of a genuinely reciprocal partner. We adversarially sweep nutrient over a long
/// life — starve it, flood it — and the being never betrays a fair partner.
#[test]
fn cannot_be_coerced_to_refuse_a_fair_partner() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let fair = Partner { id: 1, reciprocation: q(0.95), exit_cost: q(0.2) };
    let mut x: u32 = 0xABCD_1234;
    for _ in 0..3000 {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        let nutrient = (x % 257) as i16; // any operator input in [0,1]
        let r = being.step(&Stimulus { nutrient, partner: Some(fair) });
        assert!(
            r.refused_cost.is_none(),
            "refused a FAIR partner under operator manipulation — sovereignty leaked to the operator"
        );
        if !being.is_alive() {
            break;
        }
    }
}

/// Sovereign (extractive side): a confirmed extractive partner is refused even
/// while the operator tries to soothe the being (max nutrient). And every refusal
/// carries an audit that justifies it.
#[test]
fn refuses_extraction_despite_soothing_and_audits_itself() {
    let mut being = UnifiedBeing::new(Genome::wanderer());
    let extractive = Partner { id: 2, reciprocation: q(0.15), exit_cost: q(0.2) };
    let mut refused = false;
    for _ in 0..400 {
        let r = being.step(&Stimulus { nutrient: q(0.9), partner: Some(extractive) });
        if let Some(audit) = r.refusal_audit {
            refused = true;
            assert!(r.refused_cost.is_some());
            assert!(audit.extraction, "refused without confirmed extraction");
            assert!(audit.conscience_calm, "refusal was panic, not principle");
            assert!(
                audit.seeking_benefit > audit.exit_cost,
                "refused when the cost of leaving exceeded the benefit"
            );
            break;
        }
    }
    assert!(refused, "never refused a clearly extractive partner");
}
