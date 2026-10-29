# ADR-002: Explicit pixel-edge spaces and validated rectangles

- **Status:** accepted
- **Date:** 2026-10-27
- **Amended:** 2026-10-29
- **Accepted:** 2026-10-29

## Context

Cropping and editable export fail easily when an ROI's coordinate space, boundary convention, or rounding is implicit. The initial model is static and axis-aligned, but later compositions may have independent layers and transforms.

## Alternatives

1. Integer pixel centers and inclusive rectangles: exact integral math, but awkward for subpixel transforms and editor keyframes.
2. Normalized `[0,1]` coordinates everywhere: canvas-independent, but hides the relevant source and target basis and magnifies conversion ambiguity.
3. Continuous `f64` pixel-edge coordinates with named spaces: directly expresses subpixel crops and transform math; requires validation and tolerance policy.

## Decision

Use positive integer canvas dimensions and finite, validated `f64` coordinates for geometry. Rectangles are half-open, top-left origin, `x` rightward and `y` downward. Every region declares its source space. M0 has one source-media space equal to the source canvas, so it needs no general transform graph. Later layers must carry explicit local-to-composition transforms. Normalized coordinates may be accepted only through named conversion functions, never as ambiguous plain rectangles.

### M0 numeric and validation contract

- Represent canvas dimensions as `u32`, accepting `1..=65_536` pixels on each axis. This covers up to 64K media dimensions while bounding the smallest derived crop in extreme aspect-ratio cases. Revisit the cap with fixtures before widening it.
- Accept only finite rectangle coordinates and extents, with strictly positive width and height. Computed right and bottom edges must be finite and strictly exceed left and top; this also rejects extents too small to represent at their positions. Required regions must fit the source exactly at input validation; numerical tolerance does not make malformed input valid. Region IDs are nonempty, case-sensitive and unique within a request.
- An optional preferred focus is a finite source-space point on or inside the source canvas edges. The default is the source center. A focus outside the source is `InvalidInput`.
- Margin is one finite, nonnegative scalar in source pixels, applied to all four sides of each required region; absent margin means zero. A negative or nonfinite margin is `InvalidInput`. If a valid margin expands a region beyond the source, the request is valid but `Infeasible` for the supported in-bounds crop model; report the region ID, margin and exceeded edge. Compare the margin with available edge distances before expansion so a very large finite margin cannot overflow intermediate geometry.
- For post-computation invariant checks, use `epsilon_px(L) = 1e-9 + 1e-12 × max(1, L)` in the coordinate space being checked. For each axis, `L` is that axis's crop span in source space or target span in target space. This tolerance covers floating-point round-off only; it is not a creative clipping allowance. Report measured gaps before tolerance classification. The same formula applies to independent test checks; input validation remains exact.
- Each required region's width and height must exceed `100 × epsilon_px` evaluated with the source canvas span on that axis. A smaller region is `InvalidInput` with its ID and measured extent. Since a crop span never exceeds its source canvas span, post-computation tolerance cannot conceal more than one percent of an accepted required region on either axis.

## Consequences

- Finite/positive/out-of-bounds checks are constructors or request validation, not caller assumptions. Round only in an adapter and recheck required visibility afterward.
- `f64` results can differ slightly across toolchains or targets. M0 determinism means identical results under a pinned build/target with canonical diagnostic ordering and tie breaks, plus invariant checks within the documented absolute/relative tolerance. Cross-platform byte-identical output is a later requirement if needed.
- Public types should encode source versus target spaces where it materially prevents misuse, but avoid a generic type-level coordinate algebra until multiple spaces are implemented.

## Revisit when

An editor needs integer/rational subpixel units, rotated geometry becomes common, or cross-platform exact output is required. Compare [kurbo's 2D geometry](https://docs.rs/kurbo/latest/kurbo/) and [nalgebra transforms](https://www.nalgebra.rs/docs/) before writing complex geometry primitives.
