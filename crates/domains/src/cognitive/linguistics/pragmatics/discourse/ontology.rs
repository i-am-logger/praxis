// Discourse Coherence — RST + SDRT.
//
// How text segments relate to each other to form coherent discourse.
// RST (Mann & Thompson 1988) provides the nucleus/satellite structure
// and the taxonomy of rhetorical relations. SDRT (Asher & Lascarides
// 2003) adds dynamic semantics and inference rules for computing
// discourse relations. Hobbs (1979) contributes the coherence
// relation classification.
//
// This ontology governs text generation: the NLG pipeline must
// produce coherent discourse by selecting appropriate rhetorical
// relations between segments. It also governs interpretation:
// understanding a text means recovering its discourse structure.
//
// Source: Mann & Thompson (1988); Asher & Lascarides (2003);
//         Hobbs (1979); Taboada & Mann (2006)

use pr4xis::category::Entity;
use pr4xis::define_ontology;
use pr4xis::ontology::{Axiom, Ontology, Quality};

/// Concepts in the Discourse Coherence ontology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Entity)]
pub enum DiscourseConcept {
    // === RST structural concepts (Mann & Thompson 1988) ===
    /// A minimal unit of text that participates in discourse structure.
    TextSpan,
    /// The more central part of a nucleus-satellite pair.
    Nucleus,
    /// The supporting part — depends on the nucleus for interpretation.
    Satellite,
    /// A rhetorical relation connecting two or more spans.
    RhetoricalRelation,

    // === SDRT structural concepts (Asher & Lascarides 2003) ===
    /// An elementary discourse unit — the atomic building block.
    DiscourseSegment,
    /// A complex discourse structure built from segments + relations.
    DiscourseStructure,

    // === Rhetorical relation taxonomy (Mann & Thompson; Hobbs) ===
    // Subject-matter relations (informational)
    /// Adding detail to the nucleus (most common RST relation).
    Elaboration,
    /// Temporal or logical sequence between events.
    Sequence,
    /// Causal connection: nucleus causes satellite or vice versa.
    Cause,
    /// A condition under which the nucleus holds.
    Condition,
    /// The goal or intention behind an action described in the nucleus.
    Purpose,
    /// Setting the context for understanding the nucleus.
    Background,

    // Presentational relations (affect reader's attitude)
    /// Making the nucleus more believable via evidence or reasoning.
    Justify,
    /// Restating the same content in different words.
    Restatement,
    /// Acknowledging an apparent incompatibility then overriding it.
    Concession,
    /// Highlighting difference between two spans.
    Contrast,

    // SDRT-specific relations (Asher & Lascarides 2003)
    /// Temporal progression of events.
    Narration,
    /// Causal explanation of a previously mentioned event.
    Explanation,
    /// Similar content presented in parallel structure.
    Parallel,
    /// Continuing a topic without adding new rhetorical structure.
    Continuation,
}

define_ontology! {
    /// Discourse Coherence — rhetorical structure of text.
    pub DiscourseOntology for DiscourseCategory {
        concepts: DiscourseConcept,
        relation: DiscourseRelation,

        being: AbstractObject,
        source: "Mann & Thompson (1988); Asher & Lascarides (2003); Hobbs (1979)",

        is_a: DiscourseTaxonomy [
            // Structural: Nucleus and Satellite are TextSpans
            (Nucleus, TextSpan),
            (Satellite, TextSpan),
            (DiscourseSegment, TextSpan),

            // Subject-matter relations are RhetoricalRelations
            (Elaboration, RhetoricalRelation),
            (Sequence, RhetoricalRelation),
            (Cause, RhetoricalRelation),
            (Condition, RhetoricalRelation),
            (Purpose, RhetoricalRelation),
            (Background, RhetoricalRelation),

            // Presentational relations are RhetoricalRelations
            (Justify, RhetoricalRelation),
            (Restatement, RhetoricalRelation),
            (Concession, RhetoricalRelation),
            (Contrast, RhetoricalRelation),

            // SDRT relations are RhetoricalRelations
            (Narration, RhetoricalRelation),
            (Explanation, RhetoricalRelation),
            (Parallel, RhetoricalRelation),
            (Continuation, RhetoricalRelation),
        ],

        has_a: DiscourseMereology [
            // A discourse structure is composed of segments
            (DiscourseStructure, DiscourseSegment),
            (DiscourseStructure, RhetoricalRelation),
        ],

        opposes: DiscourseOpposition [
            // Nucleus vs Satellite: central vs subordinate
            (Nucleus, Satellite),
            // Cause vs Explanation: forward vs backward causation
            (Cause, Explanation),
            // Contrast vs Parallel: difference vs similarity
            (Contrast, Parallel),
        ],
    }
}

/// Whether a concept is a structural element vs. a rhetorical relation.
#[derive(Debug, Clone)]
pub struct IsStructuralElement;

impl Quality for IsStructuralElement {
    type Individual = DiscourseConcept;
    type Value = bool;

    fn get(&self, individual: &DiscourseConcept) -> Option<bool> {
        Some(matches!(
            individual,
            DiscourseConcept::TextSpan
                | DiscourseConcept::Nucleus
                | DiscourseConcept::Satellite
                | DiscourseConcept::DiscourseSegment
                | DiscourseConcept::DiscourseStructure
        ))
    }
}

/// Every rhetorical relation is-a RhetoricalRelation (taxonomy completeness).
#[derive(Debug)]
pub struct AllRelationsClassified;

impl Axiom for AllRelationsClassified {
    fn description(&self) -> &str {
        "every specific relation is-a RhetoricalRelation (Mann & Thompson 1988 taxonomy)"
    }
    fn holds(&self) -> bool {
        use pr4xis::ontology::reasoning::taxonomy::TaxonomyDef;
        let rels = DiscourseTaxonomy::relations();
        let relation_types = [
            DiscourseConcept::Elaboration,
            DiscourseConcept::Sequence,
            DiscourseConcept::Cause,
            DiscourseConcept::Condition,
            DiscourseConcept::Purpose,
            DiscourseConcept::Background,
            DiscourseConcept::Justify,
            DiscourseConcept::Restatement,
            DiscourseConcept::Concession,
            DiscourseConcept::Contrast,
            DiscourseConcept::Narration,
            DiscourseConcept::Explanation,
            DiscourseConcept::Parallel,
            DiscourseConcept::Continuation,
        ];
        relation_types.iter().all(|rt| {
            rels.iter().any(|(child, parent)| {
                child == rt && *parent == DiscourseConcept::RhetoricalRelation
            })
        })
    }
}

/// Nucleus and Satellite are in opposition (Mann & Thompson 1988 §2.1).
#[derive(Debug)]
pub struct NucleusSatelliteOpposed;

impl Axiom for NucleusSatelliteOpposed {
    fn description(&self) -> &str {
        "Nucleus and Satellite are opposed: central vs subordinate (RST §2.1)"
    }
    fn holds(&self) -> bool {
        use pr4xis::ontology::reasoning::opposition::OppositionDef;
        let pairs = DiscourseOpposition::pairs();
        pairs
            .iter()
            .any(|(a, b)| *a == DiscourseConcept::Nucleus && *b == DiscourseConcept::Satellite)
    }
}

impl Ontology for DiscourseOntology {
    type Cat = DiscourseCategory;
    type Qual = IsStructuralElement;

    fn structural_axioms() -> Vec<Box<dyn Axiom>> {
        DiscourseOntology::generated_structural_axioms()
    }

    fn domain_axioms() -> Vec<Box<dyn Axiom>> {
        vec![
            Box::new(AllRelationsClassified),
            Box::new(NucleusSatelliteOpposed),
        ]
    }
}
