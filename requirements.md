# Requirements and measurable outcomes

**Scope vocabulary:** **M0** is the first application-ready software foundation. **M1** completes the initial scope, constraint-aware static cropping ([ADR-005](decisions/ADR-005-initial-scope-static-cropping.md), accepted). **Deferred** items are kept as a record but are not scheduled. Requirements are not promises of current functionality.

## Users and jobs

| User job | Desired library result |
| --- | --- |
| Adapt an image or video shot to a new canvas without hiding a known subject | A crop and transform expressed in source and target coordinates, or explicit infeasibility. |
| Produce several renditions (for example 9:16, 1:1, 4:5) of one visual and fail a pipeline when one would lose a marked subject, logo or text | One outcome per target from a single request, measured soft results, and a CLI exit code. **M1.** |
| Keep lower-priority content when possible without risking required content | Preferred regions in priority tiers, reported as kept or dropped with measured visibility. **M1.** |
| Apply the decision in an image library or page layout | Integer crop for raster APIs and a cover-position mapping for CSS-style layout, both revalidated. **M1.** |
| Reposition graphics and captions around subjects | Layer transforms satisfying safe-area and separation rules, with reported compromises. **Deferred.** |
| Reframe moving subjects | Bounded trajectory/keyframes across time, with cut-aware smoothing. **Deferred.** |
| Transfer decisions to an editing application | Versioned, tool-neutral decisions and an adapter that reports unsupported operations. **Deferred.** |

## M0 functional requirements

| ID | Requirement | Acceptance evidence |
| --- | --- | --- |
| F-01 | Accept source and target canvas dimensions of `1..=65_536` pixels per axis and a single static, full-bleed visual layer with a known source extent. Source and region geometry are in display-oriented, square-pixel space: the caller has applied sample aspect ratio and rotation metadata, and dimensions are display dimensions, not coded dimensions. | Valid synthetic landscape, portrait, and square cases construct and validate. |
| F-02 | Represent finite axis-aligned regions of interest in source pixel-edge coordinates, each marked **required** for M0. | Reject NaN, infinities, nonpositive, unrepresentable or below-resolution extents, empty or duplicate IDs, and wholly out-of-bounds required regions with typed diagnostics. Input policy: a region partly outside the source is clipped to its visible portion with a `region_clipped` diagnostic carrying the original bounds; a clipped region below the minimum extent is `InvalidInput`. ADR-002 defines the minimum supported region extent. |
| F-03 | Compute the largest in-bounds source crop matching the target aspect ratio, subject to containing every required region (hard); an optional finite nonnegative margin in source pixels (a soft preference by default, with an explicit `hard` margin mode); an optional target-space safe rectangle T (hard); and an optional finite `max_upscale > 0`. T must be finite, inside the target canvas, with positive extent, else `InvalidInput`; every unexpanded required region must map inside T. Scale `s = target_width / crop_width` is always reported; since the crop is maximal, `s` is the minimum achievable scale, so `s > max_upscale` is `Infeasible` with measured scale and limit. Per axis, selection is lexicographic: hard interval (source containment, raw-union containment, T bounds); then the largest achievable margin `m* ≤ m` (soft mode returns it even if `m* < m`); then the focus-derived origin clamped into that interval, never reducing achieved margin. With no regions, only source bounds and `max_upscale` apply. Achieved margin and any shortfall versus `m` are reported per region and edge. | Crop aspect and source bounds are checked independently; no required region is clipped beyond a documented tolerance. Shortfall fixtures cover source edges and crop size; hard-mode margin, safe-rectangle and `max_upscale` fixtures are `Infeasible` with evidence. |
| F-04 | When multiple feasible crop positions exist, choose the one whose center is nearest a finite, in-source preferred focus point. With no preferred focus, use the center of the clipped required-region union U if any regions exist, otherwise the source center. | Same input and algorithm version yield the same decision and canonical ordering. |
| F-05 | Return a structured, editable decision: source crop, target rectangle, uniform scale, translation, satisfied constraints, and diagnostics. | A test reconstructs the target transform from the crop and checks it covers the target canvas. No raster output is needed. |
| F-06 | Report invalid input and mathematically infeasible required visibility separately; never label either as a solved adaptation. | Synthetic fixtures cover region wider/taller than any target-aspect crop, a hard-mode margin with `m* < m`, e.g. extending outside the source (`Infeasible`; by default `Solved` with reported shortfall), and malformed geometry (`InvalidInput`). |
| F-07 | Provide a library API and one runnable synthetic example. | Example output is generated by real code and checked in tests; README lists only implemented behavior. |

M0 treats the crop as **full bleed**, with no padding or letterboxing. It accepts only one visual layer and static geometry. A required ROI may be a title or subject region baked into that media. Caller-supplied importance is trusted; there is no detection model. If preserving all required content needs letterboxing, a different composition, or a wider crop, M0 reports infeasible.

## M1 functional requirements (ADR-005)

M1 keeps the single static, full-bleed source layer and the M0 algorithm. It adds the requirements below. Algorithms are specified in [architecture.md](architecture.md#7-algorithms).

| ID | Requirement | Acceptance evidence |
| --- | --- | --- |
| M1-01 | Accept one source, shared regions, margin and focus, and `1..=N` targets, each with its own canvas, optional safe rectangle T and optional `max_upscale`. Each target is evaluated independently by the M0 operation; shared-field errors are reported once. | Per-target result is bit-identical to a single-target request; target order preserved; the request is not mutated. |
| M1-02 | Regions are `required` (default; M0 meaning) or `preferred` with a priority tier. Tiers are kept whole, in ascending order, while every hard constraint stays feasible; the margin and default focus apply to the kept union. | Fixtures where a tier is kept for one target and dropped for another; tier outcome independent of region order and IDs; dropping never affects required regions. |
| M1-03 | Every soft constraint reports a measured result: per-region per-edge margin shortfall, dropped preferred regions with visible fraction and cut distance, and focus offset. | Each `Solved` fixture lists every constraint with status and measurements. |
| M1-04 | `Infeasible` reports, per failing axis or limit, the conflicting interval terms, binding region IDs, gap in source pixels, and the relaxation that would be needed (target aspect bound, upscale limit or achievable margin), plus the M0 fit/pad alternative. | Width, height, safe-rectangle, hard-margin and upscale fixtures check each measurement. |
| M1-05 | A versioned JSON request and report and a CLI with exit codes 0 (solved), 1 (infeasible or opted-in soft failure), 2 (invalid input), 3 (usage or I/O). Unknown fields and schema versions are rejected. | Round-trip, malformed-input, over-limit and exit-code tests; `f64` values round-trip bit-exactly. |
| M1-06 | Opt-in integer snapping of a decision (`outward` default, `nearest` optional) with per-axis scale and aspect error, rechecked against every hard constraint; a violation is reported, never relabelled as satisfied. | Fractional-edge, source-edge and safe-rectangle snapping fixtures; `outward` never loses a region (property test). |
| M1-07 | Map a decision to cover-position fractions for CSS-style `object-fit: cover` layout. | Fixtures check the formula, including the zero-slack axis. |
| M1-08 | Deterministic output under the ADR-002 canonical contract, extended to `floor`/`ceil` in snapping only. | Golden `to_bits` fixtures for batch, tier and snapping cases. |

Deferred static candidates, kept as a record: target-space anchors for regions, crop zoom range (minimum/maximum crop size), and static per-shot crop from caller-supplied samples. Each needs its own semantics decision before scheduling.

## Deferred functional requirements (not scheduled)

These are kept unchanged as design history ([ADR-005](decisions/ADR-005-initial-scope-static-cropping.md)). Movable independent overlays, collision avoidance, safe-area placement and separation solving would form M2, extending hard/soft semantics to layer placement: hard limits cannot be silently relaxed, while soft violations need measured costs.


- A future subject-track input contract: caller-supplied stable track IDs, source-space bounds or points at rational times, media timebase, cut boundaries, observation gaps, and explicit confidence/provenance where available. Missing samples must not silently imply continuous visibility.
- Cut-aware smoothing, velocity and acceleration limits, between-sample visibility checks, and editable keyframe reduction over those tracks. Spans without a justified interpolation or visibility bound must be reported as unverified.
- Crop/pad/letterbox alternatives, multiple candidate regions, masked and rotated content, animated text and captions, and ranked alternative compositions.
- Adapter contracts that report `supported`, `approximated`, `unsupported`, or `unverified` operations and verify a saved project where an editor exposes that capability.

## Nonfunctional requirements

| ID | Requirement | M0 measure or later gate |
| --- | --- | --- |
| N-01 | Deterministic behavior | M0 decisions are bitwise identical on every platform where golden fixtures have been checked; a release requires all four (Linux x86_64, Linux aarch64, macOS, Windows). Per-push coverage is Linux x86_64 only (cost-limited; M0 uses only IEEE-754 basic operations, so cross-platform divergence is unlikely). M0 uses `f64` with only IEEE-754 basic operations (+ − × ÷; no fused multiply-add, no transcendental functions); outputs normalize −0.0 to +0.0; decision fields and diagnostics have a fixed canonical order (diagnostics sorted by code, then UTF-8 region ID); golden fixtures record exact `f64` bit patterns (`to_bits` hex) and CI compares them per push on Linux x86_64; Linux aarch64 (local `cross`) and macOS/Windows (manual `workflow_dispatch` workflow) are checked before release. This is an M0 gate. |
| N-02 | Numerical safety | Reject nonfinite values; positive representable extents and required regions below the ADR-002 resolution threshold; test near-edge and small/large supported canvases. Use ADR-002's source/target pixel tolerance for post-computation checks, never for input validity. |
| N-03 | Explicit outcomes | M0 emits `solved`, `invalid_input`, or model-scoped `infeasible`; `unsupported` and `indeterminate` are reserved for later request/solver models. A timeout is not an infeasibility proof. |
| N-04 | Performance | M0 geometry is O(number of required regions); M1 is O(n log n) per target (tier sort). No latency target before measurement. |
| N-05 | Portability | Stable Rust; per-push CI on Linux x86_64; Linux aarch64 (local `cross`), macOS and Windows (manual workflow) required to pass before release. No native optimizer dependency in M0. |
| N-06 | Testability | Synthetic fixtures, unit and integration tests, geometric invariants, and a bounded property-test plan. No project-specific media assets. |
| N-07 | API stability | Pre-1.0 semver; avoid treating an example JSON shape as a committed exchange standard. Changes require a decision record and migration note once external consumers exist. |
| N-08 | Dependency hygiene | Minimum dependencies; licences and transitive native build requirements checked before adding solver or editor adapters. |

## Explicit non-goals for M0 and M1

Rendering, encoding, media decoding, subject detection, AI inference, image saliency, creative judgment, multi-layer solving, animation, frame timing, editor integration, and general timeline representation.

## Decision gates

1. Does the M0 operation solve a real, independently useful crop problem better than a simple center crop by preserving caller-marked regions? M1 adds: how often does it differ from a crop centered on the union of all regions? This is measured by a documentation example, not used as a gate (architecture open question 1).
2. ~~Immediately after a stable M0 crop decision exists, can a minimal adapter translate it into one editor's editable crop/transform fields, save and reopen the project, and verify the persisted operation?~~ Deferred with editor interchange (ADR-005).
3. What numeric tolerance and canvas bounds are safe for the first stable API?
4. ~~Do actual multi-layer cases justify a linear/Cassowary, MILP/CP, heuristic, or hybrid backend?~~ Deferred with layered layout (ADR-005).
