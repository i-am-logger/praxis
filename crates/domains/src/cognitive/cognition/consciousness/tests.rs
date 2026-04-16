use super::ontology::*;
use pr4xis::category::Category;
use pr4xis::category::entity::Entity;
use pr4xis::category::validate::check_category_laws;
use pr4xis::ontology::{Axiom, Ontology, Quality};

#[test]
fn category_laws() {
    check_category_laws::<ConsciousnessCategory>().unwrap();
}

#[test]
fn ontology_validates() {
    ConsciousnessOntology::validate().unwrap();
}

#[test]
fn fourteen_concepts() {
    assert_eq!(ConsciousnessConcept::variants().len(), 14);
}

#[test]
fn attention_causes_access() {
    assert!(AttentionCausesAccess.holds());
}

#[test]
fn integration_produces_structure() {
    assert!(IntegrationProducesStructure.holds());
}

#[test]
fn all_concepts_have_theory_origin() {
    for c in ConsciousnessConcept::variants() {
        assert!(TheoryOrigin.get(&c).is_some(), "{:?} missing origin", c);
    }
}

#[test]
fn global_workspace_has_coalition_and_broadcast() {
    let m = ConsciousnessCategory::morphisms();
    assert!(
        m.iter()
            .any(|r| r.from == ConsciousnessConcept::GlobalWorkspace
                && r.to == ConsciousnessConcept::Coalition
                && r.kind == ConsciousnessRelationKind::HasComponent)
    );
    assert!(
        m.iter()
            .any(|r| r.from == ConsciousnessConcept::GlobalWorkspace
                && r.to == ConsciousnessConcept::BroadcastMessage
                && r.kind == ConsciousnessRelationKind::HasComponent)
    );
}
