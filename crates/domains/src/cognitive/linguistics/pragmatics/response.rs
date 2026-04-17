use pr4xis::category::Entity;
use pr4xis::define_ontology;

// Response Generation ontology — the right adjoint of parsing.
//
// Parsing: Text → Syntax → Semantics (left adjoint, F)
// Generation: Semantics → Syntax → Text (right adjoint, G)
// Together: Parse ⊣ Generate (adjunction)
//
// The unit η: Id → G∘F means: parse then generate ≈ paraphrase.
// The counit ε: F∘G → Id means: generate then parse ≈ comprehension check.
// G∘F ≠ Id: paraphrase loses information (many texts → one meaning).
// F∘G ≠ Id: a meaning can be expressed many ways.
//
// This ontology defines the concepts for generating responses from
// ontological states. It composes:
// - Metacognition (what happened: gap, repair, clarification)
// - Epistemics (what the system knows: KK, KU, UK, UU)
// - SelfModel (what the system IS: components, capabilities)
// - Speech acts (what kind of response: assertion, question)
//
// The response is NOT a hardcoded string. It is composed from
// ontological concepts that are then realized through the Language.
//
// References:
// - Reiter & Dale, "Building Natural Language Generation Systems" (2000)
// - White, "Efficient Realization of Coordinate Structures in CCG" (2006)
// - Lambek & Scott, "Introduction to Higher Order Categorical Logic" (1986)

/// Concepts in response generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Entity)]
pub enum ResponseConcept {
    /// The communicative intent — what the system wants to express.
    /// Determined by metacognition (gap → clarify, repair → suggest).
    Intent,
    /// The epistemic frame — the system's knowledge state about this exchange.
    /// From epistemics: KnownKnown, KnownUnknown, UnknownKnown, UnknownUnknown.
    EpistemicFrame,
    /// The content — the actual information to convey.
    /// From self-model, knowledge base, or query results.
    Content,
    /// The speech act type — assertion, question, directive, etc.
    /// From speech_act.rs.
    SpeechActType,
    /// The surface form — the linguistic realization.
    /// The right adjoint maps intent → text through the Language.
    SurfaceForm,
    /// The context — what was said before, what the user asked.
    /// From dialogue state.
    Context,
}

define_ontology! {
    /// Response Generation — the right adjoint of parsing (Reiter & Dale 2000).
    pub ResponseOntology for ResponseCategory {
        concepts: ResponseConcept,
        relation: ResponseRelation,
        kind: ResponseRelationKind,
        kinds: [
            /// Metacognition determines Intent.
            Determines,
            /// Epistemics frames the Content.
            Frames,
            /// Intent selects SpeechActType.
            Selects,
            /// Content realizes as SurfaceForm (the generation step).
            Realizes,
            /// Context constrains Intent.
            Constrains,
            /// SpeechActType shapes SurfaceForm.
            Shapes,
        ],
        edges: [
            // Metacognition → Intent (gap/repair/clarification determines what to say)
            (Context, Intent, Constrains),
            // Epistemics frames Content (KK→assert, KU→ask, UK→suggest, UU→admit)
            (EpistemicFrame, Content, Frames),
            // Intent selects SpeechActType (clarification→question, assertion→statement)
            (Intent, SpeechActType, Selects),
            // Content realizes as SurfaceForm (the generation functor)
            (Content, SurfaceForm, Realizes),
            // SpeechActType shapes SurfaceForm (question → interrogative form)
            (SpeechActType, SurfaceForm, Shapes),
        ],
        composed: [
            (Context, SpeechActType),
            (Context, SurfaceForm),
            (Intent, SurfaceForm),
            (EpistemicFrame, SurfaceForm),
        ],

        being: Process,
        source: "Reiter & Dale (2000); Lambek & Scott (1986)",
    }
}

// =========================================================================
// Functor: Epistemics → ResponseFrame
// =========================================================================
//
// Maps each epistemic state to its response framing:
// - KnownKnown → assert with confidence
// - KnownUnknown → acknowledge gap, ask for clarification
// - UnknownKnown → partial understanding, suggest interpretation
// - UnknownUnknown → admit complete lack of understanding

/// The response frame for each epistemic state.
/// These are NOT hardcoded strings — they are semantic frames
/// that the Language realizes into text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseFrame {
    /// System knows and can answer confidently.
    AssertKnowledge,
    /// System knows what it doesn't know — acknowledge gap.
    AcknowledgeGap,
    /// System has partial knowledge — suggest interpretation.
    SuggestInterpretation,
    /// System has no relevant knowledge — admit and guide.
    AdmitLimitation,
}

impl ResponseFrame {
    /// Map from epistemic state to response frame.
    pub fn from_epistemic(
        state: &crate::cognitive::cognition::epistemics::EpistemicConcept,
    ) -> Self {
        use crate::cognitive::cognition::epistemics::EpistemicConcept;
        match state {
            EpistemicConcept::KnownKnown => Self::AssertKnowledge,
            EpistemicConcept::KnownUnknown => Self::AcknowledgeGap,
            EpistemicConcept::UnknownKnown => Self::SuggestInterpretation,
            EpistemicConcept::UnknownUnknown => Self::AdmitLimitation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pr4xis::category::Category;
    use pr4xis::category::Entity;

    #[test]
    fn category_identity_law() {
        for obj in ResponseConcept::variants() {
            let id = ResponseCategory::identity(&obj);
            assert_eq!(id.from, obj);
            assert_eq!(id.to, obj);
        }
    }

    #[test]
    fn category_composition_with_identity() {
        for m in &ResponseCategory::morphisms() {
            let left = ResponseCategory::compose(&ResponseCategory::identity(&m.from), m).unwrap();
            assert_eq!(left.from, m.from);
            assert_eq!(left.to, m.to);
        }
    }

    #[test]
    fn has_six_concepts() {
        assert_eq!(ResponseConcept::variants().len(), 6);
    }

    #[test]
    fn content_realizes_as_surface_form() {
        assert!(
            ResponseCategory::morphisms()
                .iter()
                .any(|m| m.from == ResponseConcept::Content
                    && m.to == ResponseConcept::SurfaceForm
                    && m.kind == ResponseRelationKind::Realizes)
        );
    }

    #[test]
    fn context_reaches_surface_form() {
        // Full path: Context → Intent → SpeechActType → SurfaceForm
        assert!(
            ResponseCategory::morphisms()
                .iter()
                .any(|m| m.from == ResponseConcept::Context
                    && m.to == ResponseConcept::SurfaceForm
                    && m.kind == ResponseRelationKind::Composed)
        );
    }

    #[test]
    fn epistemic_frames_map_correctly() {
        use crate::cognitive::cognition::epistemics::EpistemicConcept;
        assert_eq!(
            ResponseFrame::from_epistemic(&EpistemicConcept::KnownKnown),
            ResponseFrame::AssertKnowledge
        );
        assert_eq!(
            ResponseFrame::from_epistemic(&EpistemicConcept::KnownUnknown),
            ResponseFrame::AcknowledgeGap
        );
        assert_eq!(
            ResponseFrame::from_epistemic(&EpistemicConcept::UnknownKnown),
            ResponseFrame::SuggestInterpretation
        );
        assert_eq!(
            ResponseFrame::from_epistemic(&EpistemicConcept::UnknownUnknown),
            ResponseFrame::AdmitLimitation
        );
    }
}
