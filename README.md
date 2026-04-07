# Praxis

Prove your domain is correct. Define rules as ontology, enforce them as axioms, verify with property-based testing. From chess to quantum mechanics — if the rules can be stated, praxis proves they hold.

## Features

- **Rule enforcement** — preconditions validate every state transition before it happens
- **Rich diagnostics** — violations carry full context (which rule, why, what state, what was attempted)
- **Undo/redo** — back() and forward() through state history
- **Trace recording** — every action logged with human-readable dump
- **Logical composition** — AND, OR, NOT, IMPLIES, thresholds for building complex rules
- **Property-based testing** — invariants verified across thousands of random inputs via proptest
- **Category theory foundation** — identity, associativity, closure laws proven for domain models
- **Ontology-driven physics** — Newton's laws, energy conservation, Heisenberg uncertainty as enforceable axioms
- **PGN parser** — replay authentic chess games from standard notation files

## Proofs

| What | How | Crate |
|---|---|---|
| Königsberg bridges are impossible | Exhaustive search of all paths from all starting nodes | praxis-puzzles |
| Chess rules are complete | 5 famous games (1851–1858) replayed from PGN to checkmate | praxis-chess |
| Rubik's cube preserves colors | Every random move sequence maintains 9 of each color | praxis-rubik |
| Monty Hall: switching wins 2/3 | Property test: switching always wins when initial choice is wrong | praxis-puzzles |
| F = ma | Δv = (F/m)⋅Δt verified for all random mass/force/time | praxis-physics |
| Energy conservation | KE + PE = constant after drop/rise, verified for all inputs | praxis-physics |
| V = IR (Ohm's law) | Enforced as precondition on every circuit change | praxis-physics |
| v < c (speed limit) | Engine blocks any velocity ≥ speed of light | praxis-physics |
| ΔxΔp ≥ ℏ/2 (Heisenberg) | Enforced: measuring position more precisely increases momentum uncertainty | praxis-physics |
| a² + b² = c² | Pythagorean theorem enforced as precondition on every triangle transformation | praxis-math |
| Roots satisfy ax²+bx+c=0 | Verified for all random coefficients via proptest | praxis-math |
| Goldbach conjecture | Every even n > 2 decomposed into two primes (verified to 1000) | praxis-math |
| Cassini's identity | F(n-1)F(n+1) - F(n)² = (-1)^n verified for n=1..25 | praxis-math |
| De Morgan's laws | A∖(B∪C) = (A∖B)∩(A∖C) verified for random sets | praxis-math |
| Category laws hold | Identity, associativity, closure verified exhaustively + proptest | rust-category |
| NOT NOT x = x | Double negation elimination proven for all integers | rust-ontology |
| A→B ≡ ¬A∨B | Implication equivalence verified by proptest | rust-ontology |
| N-Queens: no attacks | Every placement verified: no shared column or diagonal | praxis-puzzles |
| Sudoku: no duplicates | Row, column, and box uniqueness enforced and tested | praxis-puzzles |
| sin²+cos² = 1 | Pythagorean identity verified for all angles | praxis-calculator |

## Crates

| Crate | Purpose |
|---|---|
| `rust-category` | Category theory: Entity, Relationship, Category, Functor |
| `rust-ontology` | Ontology: Ontology, Quality, Axiom, logical composition |
| `rust-praxis` | Engine: Situation, Action, Precondition, Trace, back/forward |

### Domain Crates

| Crate | Domain | Key enforcement |
|---|---|---|
| `praxis-chess` | Chess | Check, checkmate, castling, en passant, promotion, pins, PGN replay |
| `praxis-calculator` | Scientific calculator | Domain constraints, exact rationals, expression simplification |
| `praxis-rubik` | Rubik's cube | Group theory, 18 moves, color invariant |
| `praxis-elevator` | Elevator dispatch | Capacity, direction commitment, no starvation |
| `praxis-traffic` | Traffic signals | Intersection conflict prevention, timing |
| `praxis-tetris` | Tetris | Collision, rotation, wall kicks, line clears |
| `praxis-simon` | Simon Says | Sequence memory, state machine |
| `praxis-http` | HTTP | Method semantics, connection state machine, retries |
| `praxis-music` | Music theory | Scales, chords, intervals, MIDI range |
| `praxis-colors` | Color theory | WCAG contrast, mixing modes, blending |
| `praxis-legal` | Legal cases | Case lifecycle, motions, rulings, rich state enums |
| `praxis-puzzles` | Logic puzzles | 11 puzzles (see below) |
| `praxis-math` | Mathematics | Pythagorean theorem, quadratic formula, Fibonacci, primes, set theory |
| `praxis-physics` | Physics | Mechanics (F=ma), energy conservation, Ohm's law, relativity (E=mc²), quantum (Heisenberg) |

### Puzzles

River crossing, Tower of Hanoi, water jugs, missionaries & cannibals, Monty Hall, Byzantine generals, prisoner's dilemma, N-queens, knight's tour, Sudoku, bridges of Königsberg.

## Documentation

- [Architecture](docs/architecture.md) — three-layer design, dependency flow, engine lifecycle, design decisions
- [Concepts](docs/concepts.md) — ontology vs praxis, situations, actions, preconditions, logical composition, rich enums, property-based testing
- [Domain Crates](docs/domain-crates.md) — enforcement details for all domain crates

## Testing

687 tests with property-based testing ([proptest](https://github.com/proptest-rs/proptest)).

```bash
cargo test --workspace
```

## License

CC BY-NC-SA 4.0 — see [LICENSE](LICENSE).
