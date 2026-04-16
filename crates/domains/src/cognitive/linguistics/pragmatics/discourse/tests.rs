use super::ontology::*;
use pr4xis::category::Category;
use pr4xis::category::entity::Entity;
use pr4xis::category::validate::check_category_laws;
use pr4xis::ontology::{Axiom, Ontology};

#[test]
fn category_laws() {
    check_category_laws::<DiscourseCategory>().unwrap();
}

#[test]
fn ontology_validates() {
    DiscourseOntology::validate().unwrap();
}

#[test]
fn twenty_concepts() {
    assert_eq!(DiscourseConcept::variants().len(), 20);
}

#[test]
fn all_relations_classified() {
    assert!(AllRelationsClassified.holds());
}

#[test]
fn nucleus_satellite_opposed() {
    assert!(NucleusSatelliteOpposed.holds());
}

#[test]
fn fourteen_rhetorical_relations() {
    use pr4xis::ontology::reasoning::taxonomy::TaxonomyDef;
    let rels = DiscourseTaxonomy::relations();
    let count = rels
        .iter()
        .filter(|(_, parent)| *parent == DiscourseConcept::RhetoricalRelation)
        .count();
    assert_eq!(count, 14, "should have 14 rhetorical relations");
}

#[test]
fn discourse_structure_has_parts() {
    use pr4xis::ontology::reasoning::mereology::MereologyDef;
    let parts = DiscourseMereology::relations();
    assert!(
        parts.iter().any(
            |(whole, part)| *whole == DiscourseConcept::DiscourseStructure
                && *part == DiscourseConcept::DiscourseSegment
        ),
        "DiscourseStructure should have DiscourseSegment as part"
    );
}

mod prop {
    use super::*;
    use proptest::prelude::*;

    fn arb_discourse() -> impl Strategy<Value = DiscourseConcept> {
        (0..20usize)
            .prop_map(|i| DiscourseConcept::variants()[i % DiscourseConcept::variants().len()])
    }

    proptest! {
        #[test]
        fn prop_identity_idempotent(c in arb_discourse()) {
            let id = DiscourseCategory::identity(&c);
            prop_assert_eq!(DiscourseCategory::compose(&id, &id), Some(id));
        }
    }
}
