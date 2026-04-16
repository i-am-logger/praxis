// Response → Discourse functor.
//
// The response frame determines the rhetorical structure.
// An assertion uses Elaboration, a question uses Background,
// a gap acknowledgment uses Concession.
//
// Source: Reiter & Dale (2000); Mann & Thompson RST (1988)

use pr4xis::category::Functor;

use super::discourse::ontology::{DiscourseCategory, DiscourseConcept, DiscourseRelation};
use super::response::{ResponseCategory, ResponseConcept, ResponseRelation};

pub struct ResponseToDiscourse;

impl Functor for ResponseToDiscourse {
    type Source = ResponseCategory;
    type Target = DiscourseCategory;

    fn map_object(obj: &ResponseConcept) -> DiscourseConcept {
        match obj {
            ResponseConcept::Intent => DiscourseConcept::RhetoricalRelation,
            ResponseConcept::EpistemicFrame => DiscourseConcept::Background,
            ResponseConcept::Content => DiscourseConcept::Nucleus,
            ResponseConcept::SpeechActType => DiscourseConcept::DiscourseStructure,
            ResponseConcept::SurfaceForm => DiscourseConcept::TextSpan,
            ResponseConcept::Context => DiscourseConcept::DiscourseSegment,
        }
    }

    fn map_morphism(m: &ResponseRelation) -> DiscourseRelation {
        DiscourseRelation {
            from: Self::map_object(&m.from),
            to: Self::map_object(&m.to),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pr4xis::category::validate::check_functor_laws;

    #[test]
    fn functor_laws() {
        check_functor_laws::<ResponseToDiscourse>().unwrap();
    }
}
