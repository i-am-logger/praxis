//! The registry — concrete `RegistryEntry` instances, one per managed
//! dataset. This is the ontology's instance layer. Adding a new dataset =
//! adding a new entry here; no other code changes.

use super::ontology::{ContentType, RegistryEntry};
use crate::formal::meta::artifact_identity::ontology::{
    ClaimData, CompositeIdentity, IdentityClaim, IdentityConcept,
};

/// The placeholder SHA-256 for English WordNet 2025. Will be replaced with
/// the real hash once the `data-v1` GitHub Release is created and the
/// `english-wordnet-2025.xml.gz` asset is uploaded. Until then, the
/// `RawHash` claim will fail verification by design — `VerificationFailClosed`
/// keeps pr4xis from using the file until the real hash lands.
const WORDNET_2025_SHA256_PLACEHOLDER: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

/// Managed external data sources. Order is stable for reporting.
pub const DATA_SOURCES: &[RegistryEntry] = &[RegistryEntry {
    name: "wordnet",
    description: "English WordNet 2025 — Global WordNet Association WN-LMF 1.3",
    remote_location: "https://github.com/i-am-logger/pr4xis/releases/download/data-v1/english-wordnet-2025.xml.gz",
    local_path: "crates/domains/data/wordnet/english-wordnet-2025.xml",
    content_type: ContentType::XmlLmf,
    gzipped: true,
    identity: CompositeIdentity(Vec::new()),
}];

/// Look up a `RegistryEntry` by name. Linear scan because the registry is
/// small; switch to a map if it grows past ~100 entries.
pub fn by_name(name: &str) -> Option<&'static RegistryEntry> {
    DATA_SOURCES.iter().find(|e| e.name == name)
}

/// Build the composite identity for a registry entry at runtime. Since
/// `Vec<IdentityClaim>` is not a const type, the `DATA_SOURCES` entries above
/// declare `CompositeIdentity(Vec::new())` as a placeholder and this function
/// constructs the real identity at runtime. Every function `resolve_identity`
/// returns must match what the registry "should" be declaring statically.
///
/// This is the only place identity claims live in Rust source. Each entry's
/// identity is reconstructed here on every call; the registry entries above
/// hold all the other metadata.
pub fn resolve_identity(name: &str) -> Option<CompositeIdentity> {
    match name {
        "wordnet" => Some(CompositeIdentity(vec![
            // SelfDescribingMetadata: the upstream XML declares <Lexicon version="2025">
            IdentityClaim {
                concept: IdentityConcept::XmlElementAttribute,
                data: ClaimData::XmlAttribute {
                    element: "Lexicon",
                    attribute: "version",
                    expected: "2025".into(),
                },
            },
            // ContentHash: belt-and-braces cryptographic integrity
            IdentityClaim {
                concept: IdentityConcept::RawHash,
                data: ClaimData::Sha256(WORDNET_2025_SHA256_PLACEHOLDER.into()),
            },
        ])),
        _ => None,
    }
}

/// Every registry entry's resolved identity (not the const placeholder).
/// Used by the ontology axioms to verify identity claims at test time.
pub fn resolved_identities() -> Vec<(&'static str, CompositeIdentity)> {
    DATA_SOURCES
        .iter()
        .filter_map(|e| resolve_identity(e.name).map(|id| (e.name, id)))
        .collect()
}
