# ADR-003: Analytic first operation; evidence-led solver selection

- **Status:** accepted
- **Date:** 2026-10-27
- **Amended:** 2026-10-30
- **Accepted:** 2026-10-29

## Context

The first useful operation is a single static full-bleed crop with required ROIs. Its feasible-origin set is an intersection of intervals, so a general optimizer would add deployment and diagnostic complexity without improving the result. Later independent layers, non-overlap, discrete crop/pad modes, and time trajectories need more power.

## Alternatives

- Use [Cassowary](https://docs.rs/cassowary/latest/cassowary/) immediately for every geometry problem: useful for linear layout, but weak for disjunctive collisions and unneeded for M0.
- Use [`good_lp`](https://github.com/rust-or/good_lp) with MILP solver from day one: flexible for static layout, but solver packaging and deterministic ties would dominate the foundation.
- Use [Clarabel.rs](https://clarabel.org/stable/) or [`argmin`](https://argmin-rs.org/) now: suited to convex/smooth subproblems, not proof of arbitrary discrete layout feasibility.
- Use a custom general constraint engine: unjustified reinvention.

## Decision

Implement M0 with auditable interval geometry and independent result validation. Describe the supported operation model precisely in `Infeasible`. Do not add a public solver plugin trait or native solver dependency yet. For future static multi-layer work, benchmark Cassowary and MILP/CP on the same synthetic fixtures. For temporal work, compare candidate-path methods and convex smoothing separately; interpolating keyframes is not a guarantee of visibility between samples. Keep any eventual backend behind an internal model/solve/validate boundary and preserve solver status distinctions.

The M0 request is one source canvas, one target canvas, required source-space rectangles, one optional uniform margin (soft by default; optional `hard` mode), one optional preferred focus, an optional finite `max_upscale > 0`, and an optional target-space safe rectangle T (finite, inside the target canvas, positive extent; every unexpanded required region must map inside T). The decision is the largest in-bounds, axis-aligned source crop at the target aspect, with uniform scaling to fill the target. Feasible crop origins form an interval per axis (hard interval: source containment, raw-union containment, T bounds). Within it, select the interval `M(m*)` for the largest achievable margin `m* ≤ m` (equal on both sides up to source-edge caps; see architecture.md), then choose the origin whose crop center is closest to the preferred focus; focus never reduces achieved margin. With no preferred focus, use the center of the clipped required-region union if any regions exist, otherwise the source center. Clamp each axis independently. Sort diagnostic region IDs by their UTF-8 bytes so input order does not change the report.

Use `Canvas`, `Rect`, `Region`, `CropRequest`, `CropDecision`, `Diagnostic` and tagged `CropOutcome` as the public M0 vocabulary. M0 `CropOutcome` has three variants: `Solved` carries a checked decision; `InvalidInput` identifies malformed fields or regions; `Infeasible` identifies an empty hard interval (including the safe rectangle), a `hard`-mode margin with `m* < m`, or scale above `max_upscale`, and names this supported crop model; it also carries a whole-source fit/pad diagnostic as an unvalidated alternative (checked against `max_upscale` and T), evidence only. `Unsupported` and `Indeterminate` are reserved outcome names for later request and solver models. A later bounded solver must never turn a timeout or failed search into `Infeasible`.

## Consequences

- M0 stays portable, testable, and fast with O(number of ROIs) computation.
- It intentionally cannot claim to solve general layered adaptation or all possible compositions.
- A later optimizer may use mature C/C++ code over FFI if it demonstrably outperforms Rust-native choices and can be shipped/licensed safely.

## Revisit when

At least two validated problem classes require different algorithms, or an actual multi-layer fixture defeats analytic geometry. Record solver version, ordering, seeds, time limits, gap, and post-solve invariant checks in that decision.
