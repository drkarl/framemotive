# ADR-003: Analytic first operation; evidence-led solver selection

- **Status:** proposed for owner review
- **Date:** 2026-10-27

## Context

The first useful operation is a single static full-bleed crop with required ROIs. Its feasible-origin set is an intersection of intervals, so a general optimizer would add deployment and diagnostic complexity without improving the result. Later independent layers, non-overlap, discrete crop/pad modes, and time trajectories need more power.

## Alternatives

- Use [Cassowary](https://docs.rs/cassowary/latest/cassowary/) immediately for every geometry problem: useful for linear layout, but weak for disjunctive collisions and unneeded for M0.
- Use [`good_lp`](https://github.com/rust-or/good_lp) with MILP solver from day one: flexible for static layout, but solver packaging and deterministic ties would dominate the foundation.
- Use [Clarabel.rs](https://clarabel.org/stable/) or [`argmin`](https://argmin-rs.org/) now: suited to convex/smooth subproblems, not proof of arbitrary discrete layout feasibility.
- Use a custom general constraint engine: unjustified reinvention.

## Proposed decision

Implement M0 with auditable interval geometry and independent result validation. Describe the supported operation model precisely in `Infeasible`. Do not add a public solver plugin trait or native solver dependency yet. For future static multi-layer work, benchmark Cassowary and MILP/CP on the same synthetic fixtures. For temporal work, compare candidate-path methods and convex smoothing separately; interpolating keyframes is not a guarantee of visibility between samples. Keep any eventual backend behind an internal model/solve/validate boundary and preserve solver status distinctions.

## Consequences

- M0 stays portable, testable, and fast with O(number of ROIs) computation.
- It intentionally cannot claim to solve general layered adaptation or all possible compositions.
- A later optimizer may use mature C/C++ code over FFI if it demonstrably outperforms Rust-native choices and can be shipped/licensed safely.

## Revisit when

At least two validated problem classes require different algorithms, or an actual multi-layer fixture defeats analytic geometry. Record solver version, ordering, seeds, time limits, gap, and post-solve invariant checks in that decision.
