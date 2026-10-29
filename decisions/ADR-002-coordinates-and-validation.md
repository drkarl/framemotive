# ADR-002: Explicit pixel-edge spaces and validated rectangles

- **Status:** proposed for owner review
- **Date:** 2026-10-27

## Context

Cropping and editable export fail easily when an ROI's coordinate space, boundary convention, or rounding is implicit. The initial model is static and axis-aligned, but later compositions may have independent layers and transforms.

## Alternatives

1. Integer pixel centers and inclusive rectangles: exact integral math, but awkward for subpixel transforms and editor keyframes.
2. Normalized `[0,1]` coordinates everywhere: canvas-independent, but hides the relevant source and target basis and magnifies conversion ambiguity.
3. Continuous `f64` pixel-edge coordinates with named spaces: directly expresses subpixel crops and transform math; requires validation and tolerance policy.

## Proposed decision

Use positive integer canvas dimensions and finite, validated `f64` coordinates for geometry. Rectangles are half-open, top-left origin, `x` rightward and `y` downward. Every region declares its source space. M0 has one source-media space equal to the source canvas, so it needs no general transform graph. Later layers must carry explicit local-to-composition transforms. Normalized coordinates may be accepted only through named conversion functions, never as ambiguous plain rectangles.

## Consequences

- Finite/positive/out-of-bounds checks are constructors or request validation, not caller assumptions. Round only in an adapter and recheck required visibility afterward.
- `f64` results can differ slightly across toolchains or targets. M0 determinism means identical results under a pinned build/target with ordered inputs and tie breaks, plus invariant checks within a documented absolute/relative tolerance. Cross-platform byte-identical output is a later requirement if needed.
- Public types should encode source versus target spaces where it materially prevents misuse, but avoid a generic type-level coordinate algebra until multiple spaces are implemented.

## Revisit when

An editor needs integer/rational subpixel units, rotated geometry becomes common, or cross-platform exact output is required. Compare [kurbo's 2D geometry](https://docs.rs/kurbo/latest/kurbo/) and [nalgebra transforms](https://www.nalgebra.rs/docs/) before writing complex geometry primitives.
