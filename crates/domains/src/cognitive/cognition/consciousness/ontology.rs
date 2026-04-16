// Consciousness — IIT + GWT + Higher-Order Theories.
//
// Three complementary theories unified categorically.
// IIT (Tononi): consciousness = integrated information (Φ).
// GWT (Baars): consciousness = global broadcasting.
// Higher-Order (Rosenthal): consciousness = representation of representation.
//
// Source: Tononi (2004, 2012); Baars (1988, 2005); Rosenthal (2005); Block (1995)

use pr4xis::category::Entity;
use pr4xis::define_ontology;
use pr4xis::ontology::{Axiom, Ontology, Quality};

/// Concepts in the Consciousness ontology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Entity)]
pub enum ConsciousnessConcept {
    /// Integrated information (Φ) — the quantity of consciousness (IIT).
    IntegratedInformation,
    /// A cause-effect structure — qualitative character of experience (IIT).
    CauseEffectStructure,
    /// A mechanism — elements in a definite state (IIT).
    Mechanism,
    /// The repertoire of possible states (IIT).
    Repertoire,
    /// The global workspace — shared broadcast medium (GWT).
    GlobalWorkspace,
    /// A coalition of processors competing for access (GWT).
    Coalition,
    /// The message broadcast when a coalition wins (GWT).
    BroadcastMessage,
    /// A processor operating outside awareness (GWT).
    UnconsciousProcessor,
    /// The act of entering the global workspace (GWT).
    ConsciousAccess,
    /// The spotlight selecting what enters the workspace (GWT).
    Attention,
    /// A first-order state — representation of the world (Higher-Order).
    FirstOrderState,
    /// A higher-order representation OF a first-order state (Higher-Order).
    HigherOrderRepresentation,
    /// Access consciousness — available for reasoning/report (Block).
    AccessConsciousness,
    /// Phenomenal consciousness — subjective experience (Block).
    PhenomenalConsciousness,
}

define_ontology! {
    /// Consciousness — IIT + GWT + Higher-Order.
    pub ConsciousnessOntology for ConsciousnessCategory {
        concepts: ConsciousnessConcept,
        relation: ConsciousnessRelation,
        kind: ConsciousnessRelationKind,
        kinds: [
            /// IIT: mechanism generates integrated information.
            Generates,
            /// GWT: attention selects for conscious access.
            Selects,
            /// GWT: coalition broadcasts message.
            Broadcasts,
            /// Higher-order: represents a lower-order state.
            Represents,
            /// IIT: structure has components.
            HasComponent,
            /// Block: conscious access enables reporting.
            Enables,
        ],
        edges: [
            // IIT: Mechanism generates IntegratedInformation
            (Mechanism, IntegratedInformation, Generates),
            // IIT: IntegratedInformation produces CauseEffectStructure
            (IntegratedInformation, CauseEffectStructure, Generates),
            // IIT: CauseEffectStructure has Mechanism and Repertoire
            (CauseEffectStructure, Mechanism, HasComponent),
            (CauseEffectStructure, Repertoire, HasComponent),
            // GWT: Attention selects ConsciousAccess
            (Attention, ConsciousAccess, Selects),
            // GWT: GlobalWorkspace has Coalition and BroadcastMessage
            (GlobalWorkspace, Coalition, HasComponent),
            (GlobalWorkspace, BroadcastMessage, HasComponent),
            // GWT: Coalition broadcasts BroadcastMessage
            (Coalition, BroadcastMessage, Broadcasts),
            // Higher-order: HigherOrderRepresentation represents FirstOrderState
            (HigherOrderRepresentation, FirstOrderState, Represents),
            // Higher-order → access: representation enables access consciousness
            (HigherOrderRepresentation, AccessConsciousness, Enables),
            // Block: ConsciousAccess enables AccessConsciousness
            (ConsciousAccess, AccessConsciousness, Enables),
            // IIT → phenomenal: CauseEffectStructure IS phenomenal consciousness
            (CauseEffectStructure, PhenomenalConsciousness, Generates),
        ],
        composed: [
            // GlobalWorkspace reaches UnconsciousProcessor through Coalition
            (GlobalWorkspace, UnconsciousProcessor),
            // Attention reaches BroadcastMessage through ConsciousAccess
            (Attention, BroadcastMessage),
            (Attention, AccessConsciousness),
            // Mechanism reaches CauseEffectStructure through IntegratedInformation
            (Mechanism, CauseEffectStructure),
            (Mechanism, Repertoire),
        ],

        being: MentalObject,
        source: "Tononi (2004, 2012); Baars (1988, 2005); Rosenthal (2005); Block (1995)",
    }
}

/// Which theory a concept originates from.
#[derive(Debug, Clone)]
pub struct TheoryOrigin;

impl Quality for TheoryOrigin {
    type Individual = ConsciousnessConcept;
    type Value = &'static str;

    fn get(&self, individual: &ConsciousnessConcept) -> Option<&'static str> {
        Some(match individual {
            ConsciousnessConcept::IntegratedInformation
            | ConsciousnessConcept::CauseEffectStructure
            | ConsciousnessConcept::Mechanism
            | ConsciousnessConcept::Repertoire => "IIT",
            ConsciousnessConcept::GlobalWorkspace
            | ConsciousnessConcept::Coalition
            | ConsciousnessConcept::BroadcastMessage
            | ConsciousnessConcept::UnconsciousProcessor
            | ConsciousnessConcept::ConsciousAccess
            | ConsciousnessConcept::Attention => "GWT",
            ConsciousnessConcept::FirstOrderState
            | ConsciousnessConcept::HigherOrderRepresentation => "Higher-Order",
            ConsciousnessConcept::AccessConsciousness
            | ConsciousnessConcept::PhenomenalConsciousness => "Block",
        })
    }
}

/// Attention causes ConsciousAccess (GWT core).
#[derive(Debug)]
pub struct AttentionCausesAccess;

impl Axiom for AttentionCausesAccess {
    fn description(&self) -> &str {
        "Attention selects ConsciousAccess (Baars 1988)"
    }
    fn holds(&self) -> bool {
        use pr4xis::category::Category;
        ConsciousnessCategory::morphisms().iter().any(|r| {
            r.from == ConsciousnessConcept::Attention
                && r.to == ConsciousnessConcept::ConsciousAccess
                && r.kind == ConsciousnessRelationKind::Selects
        })
    }
}

/// Integration produces cause-effect structure (IIT core).
#[derive(Debug)]
pub struct IntegrationProducesStructure;

impl Axiom for IntegrationProducesStructure {
    fn description(&self) -> &str {
        "IntegratedInformation generates CauseEffectStructure (Tononi 2012)"
    }
    fn holds(&self) -> bool {
        use pr4xis::category::Category;
        ConsciousnessCategory::morphisms().iter().any(|r| {
            r.from == ConsciousnessConcept::IntegratedInformation
                && r.to == ConsciousnessConcept::CauseEffectStructure
                && r.kind == ConsciousnessRelationKind::Generates
        })
    }
}

impl Ontology for ConsciousnessOntology {
    type Cat = ConsciousnessCategory;
    type Qual = TheoryOrigin;

    fn structural_axioms() -> Vec<Box<dyn Axiom>> {
        ConsciousnessOntology::generated_structural_axioms()
    }

    fn domain_axioms() -> Vec<Box<dyn Axiom>> {
        vec![
            Box::new(AttentionCausesAccess),
            Box::new(IntegrationProducesStructure),
        ]
    }
}
