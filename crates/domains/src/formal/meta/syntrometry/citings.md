# Syntrometry ontology — bibliography

## Primary source — Heim's own work

**Heim, B. (~1980).** *Syntrometrische Maximentelezentrik*. Unpublished manuscript; partial summaries surfaced posthumously.

Burkhard Heim (1925–2001), a German theoretical physicist, developed two parallel works: the unified field theory commonly called "Heim theory," and the underlying philosophical/logical foundation published only informally as *Syntrometric Maxim-Telecentricy*. This module encodes the latter.

## Primary source — modernised reformulation

**"A Modernized Syntrometric Logic: Foundations and Applications" (2025).** heim-theory.com. <https://heim-theory.com/wp-content/uploads/2025/11/A-Modernized-Syntrometric-Logic-Foundations-and-Applications.pdf>

The paper reconstructs Heim's system in contemporary category-theoretic dress:

- **§1** — distinction primitives: `Predicate`, `Predikatrix`, `Dialektik`, `Koordination`, `Aspekt`
- **§2** — syntrometric structures: `Syntrix`, `Synkolator`, `Korporator`, `Metroplex`
- **§2.2** — "The Syntrix as the Category of Leveled Structures (C_SL)" — the exact statement the lineage functor verifies
- **§3** — mereological extensions (CEM / `Part`)

Every Phase 1 concept in `ontology.rs` has a specific section reference in the paper. Phase 2's telecenter / maxime / transzendenzstufe concepts are covered in the paper's later sections and are deferred pending the C1/C2 consciousness + BDI planning work to bind them to.

## Intellectual lineage (cited, not directly extracted)

**Spencer-Brown, G. (1969).** *Laws of Form*. Allen & Unwin.

The distinction calculus that underlies Heim's `Predicate` primitive. Encoded separately in pr4xis at `crates/domains/src/cognitive/cognition/distinction.rs`.

**Mac Lane, S. (1971).** *Categories for the Working Mathematician*. Springer.

The category-theoretic substrate that both pr4xis (`crates/pr4xis/src/category/*.rs`) and the modernised Heim paper build on. Ch. II §1 for functors, Ch. II §2 for opposite categories. Endofunctor-as-functor-specialised-to-self (used in the `SynkolatorIsKorporator` axiom) is from Ch. II §1.

**Conant, R. & Ashby, W. R. (1970).** *"Every Good Regulator of a System Must Be a Model of That System."* Int. J. Systems Science.

The cybernetic equivalent of Heim's telecenter/maxime architecture that Phase 2 will bind to.

## Related ontologies in this workspace

- `crates/domains/src/cognitive/cognition/distinction.rs` — Spencer-Brown Laws of Form. Phase 2 will add a functor `Distinction → Syntrometry::Predicate`.
- `crates/pr4xis/src/category/` — the pr4xis-core substrate the lineage functor targets. The minimal `Pr4xisSubstrate` ontology in this module is a first-class mirror of these trait definitions for functor purposes.
- `crates/domains/src/cognitive/cognition/consciousness/` — C1/C2 consciousness ontology. Phase 2 will bind transzendenzstufen to the C1/C2 split.
- `crates/domains/src/formal/information/dialogue/pragmatics/planning/` (if/when built from #117) — BDI planning. Phase 2 will bind maximes.

## Project-internal references

- Issue [#62](https://github.com/i-am-logger/pr4xis/issues/62) — the operationalisation of this lineage claim.
- Issue [#51](https://github.com/i-am-logger/pr4xis/issues/51) (closed) — the first-pass research verdict that identified the lineage; superseded by #62.
- Memory `project_heim_transport.md` — the architectural mapping `Korporator=compose API, Metroplex=system-of-systems, Syntrokline Brücken=DDS transport`. Phase 2 will add functors from Syntrometry to those existing pr4xis ontologies.
- `docs/research/kinded-functor-failures.md` — the #98 research note; Phase 1's choice of a dense target ontology is informed by the dense-source-vs-kinded-target constraint documented there.
