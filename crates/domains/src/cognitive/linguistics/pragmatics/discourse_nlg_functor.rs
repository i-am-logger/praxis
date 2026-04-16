// Discourse → NLG functor.
//
// The rhetorical structure becomes the NLG document plan.
// RST structure maps to Reiter & Dale's pipeline stages.
//
// Source: Mann & Thompson (1988) → Reiter & Dale (2000)

use pr4xis::category::Functor;

use super::discourse::ontology::{DiscourseCategory, DiscourseConcept, DiscourseRelation};
use super::nlg::{NlgCategory, NlgConcept, NlgRelation, NlgRelationKind};

pub struct DiscourseToNlg;

impl Functor for DiscourseToNlg {
    type Source = DiscourseCategory;
    type Target = NlgCategory;

    fn map_object(obj: &DiscourseConcept) -> NlgConcept {
        match obj {
            DiscourseConcept::TextSpan => NlgConcept::SurfaceText,
            DiscourseConcept::Nucleus => NlgConcept::Message,
            DiscourseConcept::Satellite => NlgConcept::ReferringExpression,
            DiscourseConcept::RhetoricalRelation => NlgConcept::RhetoricalRelation,
            DiscourseConcept::DiscourseSegment => NlgConcept::ContentDetermination,
            DiscourseConcept::DiscourseStructure => NlgConcept::DocumentPlanning,
            DiscourseConcept::Elaboration => NlgConcept::RhetoricalRelation,
            DiscourseConcept::Sequence => NlgConcept::RhetoricalRelation,
            DiscourseConcept::Cause => NlgConcept::RhetoricalRelation,
            DiscourseConcept::Condition => NlgConcept::RhetoricalRelation,
            DiscourseConcept::Purpose => NlgConcept::RhetoricalRelation,
            DiscourseConcept::Background => NlgConcept::KnowledgeGathering,
            DiscourseConcept::Justify => NlgConcept::RhetoricalRelation,
            DiscourseConcept::Restatement => NlgConcept::ReferringExpression,
            DiscourseConcept::Concession => NlgConcept::RhetoricalRelation,
            DiscourseConcept::Contrast => NlgConcept::RhetoricalRelation,
            DiscourseConcept::Narration => NlgConcept::RhetoricalRelation,
            DiscourseConcept::Explanation => NlgConcept::RhetoricalRelation,
            DiscourseConcept::Parallel => NlgConcept::RhetoricalRelation,
            DiscourseConcept::Continuation => NlgConcept::Microplanning,
        }
    }

    fn map_morphism(m: &DiscourseRelation) -> NlgRelation {
        let from = Self::map_object(&m.from);
        let to = Self::map_object(&m.to);
        let kind = if from == to && m.from == m.to {
            NlgRelationKind::Identity
        } else {
            NlgRelationKind::Composed
        };
        NlgRelation { from, to, kind }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pr4xis::category::validate::check_functor_laws;

    #[test]
    #[ignore = "dense→kinded: discourse round-trips become non-identity self-loops in NLG (#98)"]
    fn functor_laws() {
        check_functor_laws::<DiscourseToNlg>().unwrap();
    }
}
