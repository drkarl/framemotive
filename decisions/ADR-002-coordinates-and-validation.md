# ADR-002: Explicit pixel-edge spaces and validated rectangles

- **Status:** accepted
- **Date:** 2026-10-27
- **Amended:** 2026-10-30
- **Accepted:** 2026-10-29

## Context

Cropping and editable export fail easily when an ROI's coordinate space, boundary convention, or rounding is implicit. The initial model is static and axis-aligned, but later compositions may have independent layers and transforms.

## Alternatives

1. Integer pixel centers and inclusive rectangles: exact integral math, but awkward for subpixel transforms and editor keyframes.
2. Normalized `[0,1]` coordinates everywhere: canvas-independent, but hides the relevant source and target basis and magnifies conversion ambiguity.
3. Continuous `f64` pixel-edge coordinates with named spaces: directly expresses subpixel crops and transform math; requires validation and tolerance policy.

## Decision

Use positive integer canvas dimensions and finite, validated `f64` coordinates for geometry. Rectangles are half-open, top-left origin, `x` rightward and `y` downward. Every region declares its source space. M0 has one source-media space equal to the source canvas, so it needs no general transform graph. Source and region geometry are in display-oriented, square-pixel space: the caller has applied sample aspect ratio and rotation metadata, and dimensions are display dimensions, not coded dimensions. Later layers must carry explicit local-to-composition transforms. Normalized coordinates may be accepted only through named conversion functions, never as ambiguous plain rectangles.

### M0 numeric and validation contract

- Represent canvas dimensions as `u32`, accepting `1..=65_536` pixels on each axis. This covers up to 64K media dimensions while bounding the smallest derived crop in extreme aspect-ratio cases. Revisit the cap with fixtures before widening it.
- Accept only finite rectangle coordinates and extents, with strictly positive width and height. Computed right and bottom edges must be finite and strictly exceed left and top; this also rejects extents too small to represent at their positions. A required region partly outside the source is clipped to its visible portion with a diagnostic (`region_clipped`) carrying the original bounds; a region wholly outside the source is `InvalidInput`. Numerical tolerance does not make malformed input valid. Region IDs are nonempty, case-sensitive and unique within a request.
- An optional preferred focus is a finite source-space point on or inside the source canvas edges. When no focus is supplied, the default is the center of the required-region union U (after clipping) if any regions exist, otherwise the source center. A focus outside the source is `InvalidInput`.
- Margin is one finite, nonnegative scalar in source pixels, applied to all four sides of each required region; absent margin means zero. A negative or nonfinite margin is `InvalidInput`. Margin is a soft preference by default, with an explicit `hard` margin mode; required-region visibility stays hard. Selection is lexicographic per axis: the hard interval (source containment and raw-union containment) first, then the largest achievable margin `m* ≤ m` (the hard interval narrowed to keep U expanded by `m*` inside the crop, with per-edge caps at source edges; see architecture.md), then the focus-derived origin clamped into that interval. Priority: hard constraints, then achieved margin, then focus. Achieved margin and any shortfall are reported per region and edge. In hard mode, `m* < m` (including a margin extending past a source edge) is `Infeasible` (report region ID, margin and edge). Compare the margin with available edge distances before expansion so a very large finite margin cannot overflow intermediate geometry.
- For post-computation invariant checks, use `epsilon_px(L) = 1e-9 + 1e-12 × max(1, L)` in the coordinate space being checked. For each axis, `L` is that axis's crop span in source space or target span in target space. This tolerance covers floating-point round-off only; it is not a creative clipping allowance. Report measured gaps before tolerance classification. The same formula applies to independent test checks; input validation remains exact.
- After clipping, each required region's width and height must exceed `100 × epsilon_px` evaluated with the source canvas span on that axis. A smaller region is `InvalidInput` with its ID and measured extent. Since a crop span never exceeds its source canvas span, post-computation tolerance cannot conceal more than one percent of an accepted required region on either axis.

## Consequences

- Finite/positive/out-of-bounds checks are constructors or request validation, not caller assumptions. Round only in an adapter and recheck required visibility afterward.
- Canonical output contract (an M0 gate): keep `f64`; M0 uses only IEEE-754 basic operations (+ − × ÷, no fused multiply-add, no transcendental functions); outputs normalize −0.0 to +0.0; decision fields and diagnostics have a fixed canonical order (diagnostics sorted by code, then UTF-8 region ID); golden fixtures record exact `f64` bit patterns (`to_bits` hex); GitHub Actions compares them per push on Linux x86_64 only (private repo, limited Actions minutes; M0 uses only IEEE-754 basic operations, so divergence is unlikely). Linux aarch64 runs the fixtures locally with `cross` (optional during development, required before a release); macOS and Windows run in a manually triggered `workflow_dispatch` workflow that must pass before any release. M0 decisions are bitwise identical on every platform where fixtures have been checked; a release requires all four. Invariant checks still use the documented tolerance.
- Public types should encode source versus target spaces where it materially prevents misuse, but avoid a generic type-level coordinate algebra until multiple spaces are implemented.

## Revisit when

An editor needs integer/rational subpixel units, rotated geometry becomes common, or the canonical `f64` contract cannot hold across targets (consider exact rational arithmetic). Compare [kurbo's 2D geometry](https://docs.rs/kurbo/latest/kurbo/) and [nalgebra transforms](https://www.nalgebra.rs/docs/) before writing complex geometry primitives.
