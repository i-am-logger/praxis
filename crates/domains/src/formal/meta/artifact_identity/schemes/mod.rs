//! Identity extractors — one per leaf of the taxonomy.
//!
//! Each file in this module defines the verification logic for one
//! `IdentityConcept` leaf. The initial PR implements two real extractors
//! (`raw_hash` and `xml_element_attribute`) and stubs the remaining 20;
//! stubs return `VerificationResult::Unverifiable { reason: "not yet
//! implemented" }` so the axiom `VerificationFailClosed` still holds.
//!
//! Adding a real implementation for any stubbed scheme is a focused PR that
//! touches exactly one file here.

pub mod raw_hash;
pub mod xml_element_attribute;

// Stubs — grouped by family for reviewability. Each stub is a thin module
// that returns Unverifiable; real implementations land as follow-up PRs.

// CryptographicSignature family (issue #70 tracks Sigstore)
pub mod ed25519_raw;
pub mod minisign;
pub mod openpgp;
pub mod sigstore_attestation;
pub mod ssh_signature;
pub mod x509_signature;

// ContentHash family (RawHash is real above)
pub mod bittorrent_info_hash;
pub mod git_object_sha;
pub mod ipfs_cid;
pub mod nix_store_path;

// PersistentIdentifier family
pub mod ark;
pub mod doi;
pub mod handle;
pub mod purl;

// SelfDescribingMetadata family (XmlElementAttribute is real above)
pub mod dct_identifier;
pub mod owl_version_info;
pub mod owl_version_iri;
pub mod xml_schema_version;
