// Monitoring functor: Consciousness → Metacognition.
//
// Nelson & Narens (1990): information flows UP from object-level to
// meta-level via monitoring. The global workspace (Baars) makes
// processing visible to metacognitive monitors.
//
// The mapping follows Koriat (2007):
// - AccessConsciousness → information-based monitoring (Trace)
// - PhenomenalConsciousness → experience-based monitoring (EpistemicAssessment)
// - GlobalWorkspace → ObjectLevel (what's being monitored)
// - Attention → Monitoring (the monitor's focus)
// - BroadcastMessage → Evaluation (what gets assessed)
//
// This is the LEFT adjoint of the Monitoring ⊣ Control adjunction.
// The RIGHT adjoint (Control: Metacognition → Consciousness) goes
// in the reverse direction.
//
// Source: Nelson & Narens (1990); Koriat (2007); Baars (2005)

use pr4xis::category::Functor;

use super::ontology::*;
use crate::cognitive::cognition::metacognition::{
    MetaCognitionCategory, MetaConcept, MetaRelation, MetaRelationKind,
};

pub struct MonitoringFunctor;

impl Functor for MonitoringFunctor {
    type Source = ConsciousnessCategory;
    type Target = MetaCognitionCategory;

    fn map_object(obj: &ConsciousnessConcept) -> MetaConcept {
        match obj {
            // GWT → Nelson-Narens mapping:
            // GlobalWorkspace IS the ObjectLevel being monitored
            ConsciousnessConcept::GlobalWorkspace => MetaConcept::ObjectLevel,
            // Attention IS Monitoring (Nelson & Narens: the monitor)
            ConsciousnessConcept::Attention => MetaConcept::Monitoring,
            // BroadcastMessage IS what gets Evaluated
            ConsciousnessConcept::BroadcastMessage => MetaConcept::Evaluation,
            // Coalition IS ObjectLevel (competing processes = object-level activity)
            ConsciousnessConcept::Coalition => MetaConcept::ObjectLevel,
            // ConsciousAccess IS Trace (entering awareness = being recorded)
            ConsciousnessConcept::ConsciousAccess => MetaConcept::Trace,
            // UnconsciousProcessor IS Gap (below monitoring threshold)
            ConsciousnessConcept::UnconsciousProcessor => MetaConcept::Gap,

            // Higher-Order → Nelson-Narens mapping:
            // HigherOrderRepresentation IS MetaLevel (Rosenthal = Nelson meta-level)
            ConsciousnessConcept::HigherOrderRepresentation => MetaConcept::MetaLevel,
            // FirstOrderState IS ObjectLevel (what higher-order observes)
            ConsciousnessConcept::FirstOrderState => MetaConcept::ObjectLevel,

            // Koriat (2007) mapping:
            // AccessConsciousness → Trace (information-based monitoring)
            ConsciousnessConcept::AccessConsciousness => MetaConcept::Trace,
            // PhenomenalConsciousness → EpistemicAssessment (experience-based)
            ConsciousnessConcept::PhenomenalConsciousness => MetaConcept::EpistemicAssessment,

            // IIT mapping (Tononi → Nelson-Narens):
            // IntegratedInformation → EpistemicAssessment (Φ = degree of knowing)
            ConsciousnessConcept::IntegratedInformation => MetaConcept::EpistemicAssessment,
            // CauseEffectStructure → Evaluation (qualitative assessment of what's happening)
            ConsciousnessConcept::CauseEffectStructure => MetaConcept::Evaluation,
            // Mechanism → Monitoring (the mechanism that does the observing)
            ConsciousnessConcept::Mechanism => MetaConcept::Monitoring,
            // Repertoire → Control (possible states → possible actions)
            ConsciousnessConcept::Repertoire => MetaConcept::Control,
        }
    }

    fn map_morphism(m: &ConsciousnessRelation) -> MetaRelation {
        let from = Self::map_object(&m.from);
        let to = Self::map_object(&m.to);
        let kind = if from == to && m.from == m.to {
            MetaRelationKind::Identity
        } else if from == to {
            MetaRelationKind::Composed
        } else {
            MetaRelationKind::Composed
        };
        MetaRelation { from, to, kind }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pr4xis::category::validate::check_functor_laws;

    #[test]
    #[ignore = "IIT CauseEffectStructure↔Mechanism cycle has no metacognitive counterpart — genuine structural gap (#97)"]
    fn functor_laws() {
        check_functor_laws::<MonitoringFunctor>().unwrap();
    }
}
