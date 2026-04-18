use super::category::Category;
use crate::ontology::meta::{Citation, ModulePath, OntologyName};

/// Lemon-style lexical metadata for a functor — its identity, citation,
/// and module path. Matches `OntologyMeta` / `AxiomMeta` shape so the
/// lexicon treats ontologies, axioms, and functors uniformly (issue #148:
/// "every structural entity announces itself lexically").
///
/// Construct via the `functor!` macro, which emits these values from
/// compile-time constants. Functors outside that macro fall back to the
/// `Functor` trait's default `meta()` derived from `type_name`.
#[derive(Debug, Clone)]
pub struct FunctorMeta {
    pub name: OntologyName,
    pub citation: Citation,
    pub module_path: ModulePath,
}

/// A functor is a structure-preserving map between two categories.
///
/// Given categories C and D, a functor F: C → D maps:
/// - Objects of C to objects of D
/// - Morphisms of C to morphisms of D
///
/// # Laws
///
/// A valid functor must satisfy:
/// - **Identity preservation**: F(id_A) == id_{F(A)}
/// - **Composition preservation**: F(g∘f) == F(g)∘F(f)
///
/// Use [`crate::validate`] to verify these laws.
///
/// Every functor announces itself via `meta()` — its name, citation,
/// and module path, carried in the same Lemon-style wrappers as
/// ontologies and axioms.
pub trait Functor {
    type Source: Category;
    type Target: Category;

    /// Map an object from the source category to the target category.
    fn map_object(obj: &<Self::Source as Category>::Object) -> <Self::Target as Category>::Object;

    /// Map a morphism from the source category to the target category.
    fn map_morphism(
        m: &<Self::Source as Category>::Morphism,
    ) -> <Self::Target as Category>::Morphism;

    /// Structured metadata — name, citation, module path.
    ///
    /// Default implementation derives a sensible identity from
    /// `std::any::type_name` so every existing functor keeps compiling
    /// without migration. Functors declared via the `functor!` macro
    /// override this with the literature citation captured at the
    /// declaration site.
    fn meta() -> FunctorMeta {
        FunctorMeta {
            name: OntologyName::new(std::any::type_name::<Self>().to_string()),
            citation: Citation::EMPTY,
            module_path: ModulePath::new(module_path!().to_string()),
        }
    }
}
