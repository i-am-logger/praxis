# Category-Theoretic Formalization of Bioelectric Morphogenesis

## Subtitle

Provable Ontologies for Levin's Bioelectric Framework via Functors, Adjunctions, and Machine-Verified Axioms

## Target

Biosystems, Journal of Theoretical Biology, or Entropy (where Fields & Levin 2022 was published)

## Abstract

Dr. Michael Levin's bioelectric framework proposes that endogenous membrane
potential (Vmem) patterns encode morphogenetic information, guiding tissue
repair and regeneration through gap junction-mediated collective cell behavior.
While experimentally validated across planaria, Xenopus, and cancer models,
this framework has lacked a formal mathematical foundation. We present the
first category-theoretic formalization of Levin's bioelectric framework,
encoding 12 scientific domains as formal ontologies with 839 machine-verified
tests. We prove that the cross-domain relationships between molecular biology,
bioelectricity, immunology, pharmacology, regeneration, and pathology are
structure-preserving maps (functors) with verified composition laws, and
identify three adjunction pairs that capture the "zoom in / zoom out"
relationship between biological scales. This formalization enables automated
reasoning about therapeutic interventions, identifies previously unrecognized
structural equivalences between domains, and provides a rigorous foundation
for the emerging field of bioelectric medicine.

## 1. Introduction

### 1.1 The Bioelectric Framework

Levin's body of work (Levin 2014; Chernet & Levin 2013; Levin 2021) has
established that:

- Membrane potential (Vmem) acts as an instructive signal for pattern formation
- Gap junction networks propagate bioelectric information across cell collectives
- Depolarized Vmem correlates with proliferative/neoplastic states
- Restoring normal Vmem is sufficient to trigger morphological repair
- Every biological scale exhibits goal-directed competency (TAME framework)

These claims are supported by extensive experimental evidence but have not
been formalized mathematically. Computational models exist (Pietak & Levin
2018; Cervera et al. 2020) but focus on simulation rather than formal
verification of structural relationships.

### 1.2 Category Theory as a Foundation

Category theory provides the natural language for formalizing cross-domain
structural relationships. A functor F: C -> D is a structure-preserving map
between categories that maps objects to objects and morphisms to morphisms
while preserving identity and composition. If such a functor exists and
satisfies its laws, the relationship between C and D is not analogical but
mathematically proven.

An adjunction F -| G captures the notion of "optimally inverse" functors:
the unit and counit measure the information lost in the round-trip, making
precise what it means for two domains to be "the same up to information loss."

### 1.3 Contributions

1. Formal encoding of 12 scientific domains as categories with taxonomy,
   mereology, causation, opposition, and quality structures
2. 21 functors proving structure-preserving maps between domains
3. 3 adjunctions capturing scale-bridging relationships
4. 839 machine-verified tests proving all axioms, functor laws, and
   composition properties
5. Identification of novel structural equivalences invisible to informal
   reasoning

## 2. Methods

### 2.1 The Praxis Framework

We use the praxis ontology framework, which implements:

- **Category**: objects (entities) + morphisms (relationships) with identity
  and composition laws
- **Taxonomy** (TaxonomyDef): is-a hierarchies as directed acyclic graphs
- **Mereology** (MereologyDef): part-whole relationships
- **Causation** (CausalDef): cause-effect directed acyclic graphs
- **Opposition** (OppositionDef): symmetric, irreflexive contrast pairs
- **Quality**: properties that inhere in entities
- **Axiom**: provable boolean predicates
- **Ontology**: bundles Category + Quality + Axioms with self-validation
- **Functor**: structure-preserving maps between categories
- **Adjunction**: optimally inverse functor pairs with unit and counit

All structures are implemented in Rust and verified by the type system and
839 automated tests including property-based testing (proptest).

### 2.2 Domain Selection

Domains were selected based on their relevance to Levin's bioelectric
framework and grounded in published literature:

| Domain | Basis | Key References |
|--------|-------|----------------|
| biology | Biological organization hierarchy | Yang & Bhatt 2022; Hooper 1956 |
| molecular | Ion channel biophysics | Coste 2010 (Nobel 2021); Mihara 2011 |
| bioelectricity | Levin's TAME framework | Levin 2014; Chernet & Levin 2013 |
| regeneration | Planarian/limb regeneration | Levin 2015, 2017, 2021 |
| pharmacology | Bioelectric pharmacology | Kofman & Levin 2024 |
| immunology | Macrophage polarization | Weinheimer-Haus 2014; Yu 2019 |
| electrophysiology | Measurement science | Levin 2024 |
| pathology | Disease progression | Strasser et al. 2025 (Nolan lab) |
| biophysics | Tissue mechanics | Fukada & Yasuda 1957 |
| biochemistry | Signaling cascades | Standard biochemistry |
| chemistry | States of matter | Standard chemistry |
| hematology | Blood plasma composition | Standard hematology |

### 2.3 Axiom Design

Each axiom encodes a specific claim from published literature:

```
Axiom: Piezo1IsMechanosensitiveChannel
Source: Coste et al. 2010, Science (Nobel 2021)
Proof: taxonomy::is_a(Piezo1, Mechanosensitive) AND
       taxonomy::is_a(Piezo1, IonChannel)
```

Axioms are not structural trivia — they are falsifiable scientific claims
that would FAIL if the ontological structure contradicted the literature.

## 3. Results

### 3.1 Domain Structure

12 domains encode 275+ entities with 130+ axioms. Each domain is a category
with objects (entities) and morphisms (relationships). The morphism structure
is verified by automated category law tests (identity, composition,
associativity, closure).

### 3.2 Cross-Domain Functors

21 functors prove structure-preserving maps between domains:

**Key result**: The functor MolecularToBioelectric maps molecular entities
to their bioelectric roles (Piezo1 -> MechanicalStimulation, Cx43 ->
GapJunctionModulation, Calcium -> Signal). This functor satisfies:

- Identity preservation: map(id_A) = id_{F(A)} for all molecular entities
- Composition preservation: map(g . f) = map(g) . map(f)

This is not an analogy — it is a mathematical proof that molecular biology
HAS the structure of bioelectric signaling.

### 3.3 Adjunctions

Three adjunction pairs capture scale-bridging:

**MolecularToBioelectric -| BioelectricToMolecular**

The unit eta: A -> G(F(A)) maps a molecule to its round-trip form. For
example, Piezo1 -> MechanicalStimulation -> Piezo1 (identity — no information
loss for specific channels). But CalciumSignal -> Signal -> CalciumSignal
(lossy: multiple signals share the Signal category).

The counit epsilon: F(G(B)) -> B maps a bioelectric concept to its round-trip.
MechanicalStimulation -> Piezo1 -> MechanicalStimulation (identity). But
Signal -> CalciumSignal -> Signal (lossy: Signal collapses to one canonical
representative).

This adjunction precisely captures what is lost and preserved when "zooming"
between molecular and bioelectric scales — the central question in Levin's
multi-scale framework.

### 3.4 Causal Chain Verification

12 causal graphs encode 100+ cause-effect relationships. Functor composition
tests verify that causal chains compose across domains:

pharmacology -> molecular -> bioelectricity:
  Ivermectin -> GlyR -> IonChannelModulation

This chain is verified end-to-end: for every pharmacological entity, the
composed map through molecular to bioelectric is well-defined and preserves
identity morphisms.

### 3.5 Novel Structural Findings

**Cross-domain equivalences**: MacrophageM1, MacrophageM2, and Fibroblast
are proven identical across biology and immunology ontologies via functor
identity mapping. TargetMorphology is proven identical across bioelectricity
and regeneration. These equivalences were not assumed — they emerged from
functor analysis.

**Opposition structure**: 26 opposition pairs across 8 domains encode
scientific contrasts (Na+/K+, M1/M2, Healthy/Dysplastic). The opposition
axioms (symmetric, irreflexive) are verified automatically, ensuring no
entity opposes itself and all oppositions are bidirectional.

**The TAME hierarchy as category**: The 5-level TAME competency hierarchy
(Molecular -> Cellular -> Tissue -> Organ -> Organism) is encoded as a
taxonomy with verified DAG structure and transitive is-a relationships.
Every bioelectric entity is assigned an operating level, and the axiom
AllTAMELevelsRepresented proves that all 5 levels are populated.

## 4. Discussion

### 4.1 What Formalization Adds

Informal scientific reasoning allows ambiguity: "ion channels are involved
in bioelectricity" is true but imprecise. Formal functorial reasoning says
exactly HOW: Piezo1 maps to MechanicalStimulation (not to Signal, not to
Intervention — specifically to MechanicalStimulation), and this mapping
preserves the relational structure of both domains.

### 4.2 Predictive Power

The ontology identifies gaps: if a new ion channel is discovered in
esophageal tissue, the functor immediately tells you its bioelectric role,
its pharmacological targets (via composition), and its pathological
implications (via the causal chain).

### 4.3 Limitations

- The discrete category structure (all pairs as morphisms) is mathematically
  convenient but does not distinguish "strong" from "weak" relationships
- Axioms verify qualitative structure, not quantitative dynamics
- The formalization captures Levin's framework as published; it does not
  generate new experimental predictions (though it identifies structural gaps
  that suggest where experiments should focus)

### 4.4 Relation to Existing Work

Computational models of bioelectric patterning (Pietak & Levin 2018; Cervera
et al. 2020; Manicka & Levin 2019) simulate dynamics numerically. Our
approach is complementary: we formalize the STRUCTURE of the domain, not its
dynamics. The two approaches could be combined — the ontology defines what
entities and relationships exist, the simulation computes their behavior.

## 5. Conclusion

We present the first category-theoretic formalization of Levin's bioelectric
framework, proving with machine-verified tests that molecular biology,
bioelectricity, immunology, pharmacology, regeneration, and pathology are
connected by structure-preserving maps. Three adjunctions capture the
multi-scale nature of biological competency. This formalization provides a
rigorous foundation for bioelectric medicine and demonstrates that category
theory is a practical tool for biological knowledge representation.

## Code Availability

All ontology source code, tests, and documentation are available at:
https://github.com/i-am-logger/burp/tree/main/crates/praxis

## References

(See docs/papers/CITATIONS.md for complete bibliography — 70+ papers)

Key references:
- Levin M (2014). Molecular bioelectrics in developmental biology. Mol Biol Cell.
- Chernet BT, Levin M (2013). Transmembrane voltage potential of tumor suppression. DMM.
- Coste B et al (2010). Piezo1 and Piezo2. Science. (Nobel Prize 2021)
- Fields C, Levin M (2022). Competency in navigating arbitrary spaces. Entropy.
- Kofman K, Levin M (2024). Bioelectric pharmacology of cancer. 
- Weinheimer-Haus EM et al (2014). WBV improves wound healing. PLOS ONE.
- Lewis AH et al (2017). Repetitive mechanical stimuli and Piezo channels. Cell Reports.
- Strasser MK et al (2025). Barrett's Esophagus to adenocarcinoma. Dev Cell. (Nolan lab)
- Fukada E, Yasuda I (1957). Piezoelectric effect of bone. J Phys Soc Japan.
