# Preliminary architecture and domain model

**Status:** M0 boundaries accepted in ADR-001 through ADR-004. Future solver and adapter shapes remain open.

## Boundary and data flow

```mermaid
flowchart LR
  Caller["Caller: media, annotations, priorities"] --> V["Validate typed geometry"]
  V --> A["M0 static crop calculation"]
  A --> D["Decision + outcome diagnostics"]
  D -. future .-> S["Serialization / CLI"]
  D -. future .-> E["Editor-specific adapter"]
  E -.-> P["Editable external project"]
  Caller -. future .-> L["Multi-layer / temporal solver"]
  L -.-> D
```

The core accepts **descriptions**, not files or pixels. It returns **decisions**, not rendered media or a project file. Media decoding, detection, typography metrics, editorial timing, and rendering remain with callers or existing applications. An adapter later maps decisions into editor operations and reports fidelity. Core code cannot import an editor SDK.

## Minimal M0 domain

| Concept | Meaning and invariant | M0 role |
| --- | --- | --- |
| `Canvas` | Width and height as `u32` pixels, each in `1..=65_536`; origin at top left. | Source and target extents. |
| `Rect` | Finite `x, y, width, height` in continuous pixel-edge units; positive representable width/height; half-open `[x,x+w) × [y,y+h)`. | Source region and crop. |
| `Region` | Nonempty, request-unique caller ID and source-space rectangle. Input policy: partly outside the source is clipped to the visible portion with a `region_clipped` diagnostic carrying the original bounds; wholly outside is `InvalidInput`; a clipped region below the ADR-002 resolution threshold is `InvalidInput`. | Required visible area. |
| `CropRequest` | Source canvas, target canvas, required regions, one finite nonnegative source-pixel margin (soft by default; optional `hard` mode), optional finite in-source focus point, optional finite `max_upscale > 0`, optional target-space safe rectangle T (finite, inside the target canvas, positive extent; else `InvalidInput`). | Single input operation; the source visual fills its canvas. |
| `CropDecision` | Source crop, target bounds, uniform scale and translation, algorithm ID, constraint result IDs. | Portable geometric decision. |
| `Diagnostic` | Stable code, severity, affected IDs, measured values, message. | Explains validation and feasibility. |

Stable IDs are scoped to one request, not globally meaningful. `Region` is semantic only because the caller says it matters; the library neither classifies nor detects the pixels inside it. M0 has one full-bleed source visual layer. A composition and independently movable layers belong in the next domain increment, where use cases can determine whether `Layer`, `MediaRegion`, and `Transform` need separate types.

### Future subject-track input contract

Moving subjects will arrive as caller-supplied metadata, not detections performed by the core. Before M3 implementation, define a track as a stable ID with a declared source coordinate space and timebase, time-stamped bounds or points, and explicit cut boundaries. The contract must state how observation gaps, off-screen intervals, confidence, and caller-supplied importance affect hard visibility and soft framing. It must also distinguish observed samples from interpolated or predicted positions; no solver may assume an unobserved span is continuously visible. Validate monotonic sample times, duplicate times, bounds, and timebase compatibility at ingestion. The exact serialization and interpolation policy remain open until moving-subject fixtures exist.

Temporal decisions need a declared trade-off between subject visibility and crop motion. Measure speed and acceleration on the rational media clock, reset continuity penalties at cuts, and validate visibility between keyframes using an explicit track interpolation or conservative bound. If the input cannot justify such a bound, report the affected span as unverified rather than claiming continuous visibility.

### Coordinate contract

- Source and region geometry are in display-oriented, square-pixel space: the caller has applied sample aspect ratio and rotation metadata, and dimensions are display dimensions, not coded dimensions.
- Pixel-edge coordinates use `(0,0)` at the top-left outer edge. A `1920 × 1080` source occupies `[0,1920) × [0,1080)`; its center is `(960,540)`.
- `Rect` values may be fractional. The core does no implicit rounding. Export adapters own their editor's rounding and must revalidate required visibility after conversion. ADR-002 defines the M0 canvas bound, exact input validation, minimum required-region extent, and source/target post-computation tolerance.
- For M0 a crop `C = (cx, cy, cw, ch)` maps a source point to target by `p_target = s × (p_source − (cx,cy))`, with `s = target_width/cw`. The target bounds start at `(0,0)`. Reconstruct the target height as `s × ch` and compare it with `target_height` in target pixel units using ADR-002's tolerance.
- In later layers, media-local → layer-local → source-composition → target-composition transforms must be explicit and ordered. A region carries its declared coordinate space; never silently reinterpret normalized coordinates as pixels. Normalized values, if accepted later, are converted at the boundary using the relevant canvas.
- Time is absent in M0. Later work should use rational frame/time values rather than floating seconds for keyframe placement; a cut separates smoothing domains.

## M0 geometry algorithm

For source size `W × H` and target aspect `r = target_width / target_height`, choose the **largest area** source crop at aspect `r`:

- If `W/H >= r`, set `crop_height = H` and `crop_width = rH`.
- Otherwise set `crop_width = W` and `crop_height = W/r`.

Clip each required region to the source bounds (`region_clipped` diagnostic with original bounds when it changes); a wholly outside region, or one clipped below the minimum extent, is `InvalidInput`. Let `U` be the bounding union of the clipped, unexpanded regions. Report `s = target_width / crop_width`; since the crop is maximal, `s` is the minimum achievable scale, so `s > max_upscale` (if set) is `Infeasible` with measured scale and limit. A negative or nonfinite margin `m` is `InvalidInput`.

Select the origin per axis lexicographically (x shown; y uses top/bottom and heights):

```
(a) hard interval  Hx = [0, W − cw] ∩ [U.right − cw, U.left]
                        ∩ [U.right − T.right/s, U.left − T.left/s]     (last term only with safe rectangle T)
(b) soft margin: dL = U.left, dR = W − U.right (distances to source edges). For m' ∈ [0, m]:
      lo(m') = max(a, U.right + min(m', dR) − cw)
      hi(m') = min(b, U.left − min(m', dL))
      M(m')  = [lo(m'), hi(m')]
    m* = largest m' ∈ [0, m] with lo(m') ≤ hi(m'); select within M(m*)
(c) x = clamp(focus_x − cw/2, M(m*))
```

`m*` exists because `M(0) = Hx`, nonempty whenever the request is not `Infeasible`. `lo` is nondecreasing and `hi` nonincreasing in `m'`, both piecewise linear with breakpoints at `dL` and `dR`, so `m*` has a closed form: evaluate at the breakpoints, then solve the linear segment. Priority is explicit: hard constraints, then maximize achieved margin (equal on both sides up to source-edge caps), then focus. Focus never reduces achieved margin.

Focus is the preferred focus if supplied; otherwise the center of `U` if any regions exist; otherwise the source center. Interval clamping yields a unique origin, so no tie rule is needed. An empty `Hx` or `Hy` means no crop at the target aspect and maximal size can preserve the required regions (or, with T, map them inside the safe rectangle); the diagnostic names the safe rectangle when T bounds cause the emptiness. Margin is soft by default: the largest achievable margin `m*` is used, even when `m* < m`, and the result is still `Solved`. In `hard` margin mode, `m* < m` (including a margin extending past a source edge) is `Infeasible` (report region ID, margin, edge). Compare the margin with edge distances before expansion so a very large finite margin cannot overflow intermediate geometry. Always report, per region and per edge, the achieved margin (distance from region edge to crop edge) and any shortfall versus the requested `m` (not `m*`); shortfall arises from source edges or crop size. Return the crop and the affine mapping above. With no required regions, `U` is undefined and the region-containment, safe-rectangle and margin terms are vacuous: `Hx = [0, W − cw]` (source bounds only; the `max_upscale` check still applies), margin achieved/shortfall reporting is empty, and focus defaults to the source center (or the preferred focus), giving a deterministic source-center or preferred-focus crop.

**Critical limitation:** this proves feasibility only for the M0 operation class: a maximal, full-bleed, axis-aligned source crop with no padding. It does not prove that no other creative solution exists. A failure must say `infeasible_for_supported_crop_model` or equivalent, and suggest neither hidden solver capability nor global impossibility. The result should identify the union extent and crop extent that conflicted. It need not claim a minimal unsatisfiable core. Every `Infeasible` result also carries a diagnostic reporting the whole-source fit geometry as an **unvalidated alternative**, with bar sizes in target pixels: `s_fit = min(tw/W, th/H)`, centered, horizontal bars total `tw − s_fit·W`, vertical bars total `th − s_fit·H`, split equally. The diagnostic includes the fit's check against each M0 hard constraint: `max_upscale` (violated if `s_fit > max_upscale`) and safe rectangle T (every required region mapped by the fit transform must lie inside T). It says "fit/pad satisfies the hard constraints" only when every check passes; otherwise it lists the violated constraint(s). It is never a returned decision; `Infeasible` keeps its meaning. Fully specified pad decisions belong to a later operation model.

## Result semantics

Use a tagged outcome, not a nullable decision plus warnings:

| Outcome | Meaning |
| --- | --- |
| `Solved` | All declared M0 hard requirements were independently rechecked against the returned geometry. |
| `InvalidInput` | Malformed, nonfinite, out-of-bounds, or contradictory input; point to fields and IDs. |
| `Infeasible` | Valid request, no solution within the explicitly supported operation model. |
| `Unsupported` | Reserved for a later request model that can express valid but unsupported features; well-typed M0 crop requests do not produce it. |
| `Indeterminate` | Reserved for later bounded solver execution when feasibility was not decided; M0's analytic operation does not produce it. It is never synonymous with infeasible. |

Diagnostics should carry a stable machine code and numeric evidence (e.g. required union width and available crop width), while human-readable text is free to improve. The API should report the declared model and algorithm version. Do not expose an optimization `score` in M0 when there is no comparable optimization across solution classes. A later solver may report objective components, bound, optimality gap, run budget, and nonoptimal feasible status.

## Future solver architecture: compare before selecting

| Method | Good fit | Weak point / decision trigger |
| --- | --- | --- |
| Analytic geometry | M0 crop, clamp, containment and pairwise validation. Small, auditable, no solver installation. | Cannot decide general non-overlap or trade-offs between many movable layers. |
| Cassowary / linear constraints | Interactive position and scale relationships with priorities; Rust [`cassowary`](https://docs.rs/cassowary/latest/cassowary/) or C++/Python [Kiwi](https://github.com/nucleic/kiwi). | Disjunctive collision avoidance, crop alternatives, and temporal path choices are not simple linear equalities. The Rust crate notes that equal-strength solutions can vary without further tie-break constraints; verify determinism. |
| MILP / CP-SAT | Disjunctive non-overlap, optional padding, discrete layout modes, priority tiers; [OR-Tools CP-SAT](https://developers.google.com/optimization/cp/cp_solver) has explicit feasible/infeasible/unknown statuses and [2D no-overlap](https://or-tools.github.io/docs/pdoc/ortools/sat/python/cp_model). | Integer quantization, native distribution or binding overhead, solve latency, and licence review. Rust [`good_lp`](https://github.com/rust-or/good_lp) models LP/MILP but its own documentation does **not** guarantee deterministic solver results. No dependency is selected for M0. |
| Continuous nonlinear optimization | Smooth crop trajectories and differentiable penalties. [SciPy optimize](https://docs.scipy.org/doc/scipy/reference/optimize.html) is a mature reference. | Local minima and weak infeasibility proof; avoid presenting a failed search as impossibility. |
| Search / dynamic programming | Cut-aware trajectory candidates and temporal regularization. | Candidate design can hide failure cases; validate on moving-subject fixtures. |

Later architecture can combine a static feasibility stage, optimization stage, and independent validator. Hard constraints must be checked after rounding and after adapter translation. Soft priorities need an explicit objective and ranking policy; weighted sums alone can let a weak preference outweigh an intended stronger category if scales are wrong. A bounded solver may return one best known solution plus diagnostics initially; multiple ranked solutions should wait for users who need them.

For later layered models, infeasibility diagnostics should identify the supported model, affected constraint and layer IDs, and relevant measured bounds or violation magnitudes. A minimal conflicting set is useful when available but is not required for a truthful result. A timeout or failed search must remain `Indeterminate`.

M1 should extend only the single static source-layer crop model: hard visibility, soft framing preferences (generalizing M0's single soft margin), measured violations, and stable result semantics. M2 begins when independently movable layers, per-layer target safe areas, collision avoidance, and separation constraints are admitted; it extends hard/soft semantics to placement. This boundary keeps M1's analytic feasibility claim narrow and makes the need for a general solver testable with actual layered fixtures.

### Rust-native dependency assessment

The preferred stack is a Rust core with a small CLI in M1. Python bindings are a possible later distribution channel, not a reason to move the core to Python. Each dependency below is a **candidate**, not yet selected. Recheck the exact release licence and transitive dependencies before adoption.

| Candidate | Capability and fit | Correctness, deployment, and licence caution |
| --- | --- | --- |
| [`kurbo`](https://docs.rs/kurbo/latest/kurbo/) | 2D points, rectangles, affine transforms and later curves; natural media-plane vocabulary. | Apache-2.0 OR MIT; public types do not enforce finite/positive extents, so validated wrappers remain necessary. M0 is simple enough to keep the public model independent of the crate. |
| [`nalgebra`](https://www.nalgebra.rs/docs/) | General matrices/transforms for more complex layer geometry. | Apache-2.0; broader API and dependency footprint than M0 requires. Add when rotation, chained transforms, or linear algebra is demonstrated. |
| [`parry2d`](https://docs.rs/parry2d/latest/parry2d/) / [`geo`](https://docs.rs/geo/latest/geo/) | Collision/spatial queries or polygon operations if rectangles cease to suffice. | Useful for complex shapes; large collision/geo machinery is unnecessary for M0 axis-aligned rectangle overlap. Validate transitive licence/features if added. |
| [`cassowary`](https://docs.rs/cassowary/latest/cassowary/) | Incremental linear constraints with strengths for movable overlays. | It is low-level and does not inherently solve disjunctive no-overlap; its docs describe nondeterministic equal-strength choices. Need deterministic objective/tie-break tests and licence verification at a pinned release. |
| [`good_lp`](https://github.com/rust-or/good_lp) | Rust modeling of LP/MILP; backend can be selected by feature. | MIT for modeling crate; each solver has its own licence/build requirements. Default CBC needs native runtime; pure-Rust `microlp` trades speed for simple packaging; `good_lp` explicitly does not guarantee solver determinism. |
| [Clarabel.rs](https://clarabel.org/stable/) | Native Rust convex LP/QP/conic solver; promising for smooth trajectory penalties after layout modes are fixed. | Apache-2.0; convex continuous models cannot directly express either/or collisions or discrete crop/pad choices. Numerical tolerances and solve status need independent geometric validation. |
| [`argmin`](https://argmin-rs.org/) | Pure-Rust framework for nonlinear numerical methods and observability. | MIT OR Apache-2.0; local search does not prove global feasibility or infeasibility. Use only for a clearly formulated smooth subproblem, with deterministic initialization/budgets. |
| [`splines`](https://docs.rs/splines/latest/splines/) / [`enterpolation`](https://docs.rs/enterpolation/latest/enterpolation/) | Interpolate future crop and layer keyframes. | Interpolation alone does not enforce visibility at unsampled times or speed/acceleration limits. Choose only after defining temporal validation and editor keyframe semantics; pin and audit licence. |
| [OR-Tools CP-SAT](https://developers.google.com/optimization/cp/cp_solver) | Mature C++ core with Python/C++ interfaces and explicit status semantics; strong candidate if discrete layout choices dominate. | Apache-2.0; integer coordinate scaling and Rust FFI/deployment need a spike. Native code is acceptable if it avoids recreating a mature solver. `UNKNOWN` must never be reported as infeasible. |

Avoid a public “plug in any solver” trait at M0: there is only one analytic operation. Once at least two problem classes or backends exist, isolate model construction, solve status, budget, and post-solve validation behind a crate-private boundary; expose stable outcomes rather than solver-specific variables. Replacing a backend may change tie-breaking or optimality, so record backend and settings in diagnostics when that becomes relevant. No optimization crate belongs in the first foundation by default.

## Editable output and interchange

The eventual decision format should describe operations on geometry and time: crop, uniform or affine transform, layer placement, and keyframe trajectory. It should also identify constraints met or violated and any adapter approximation. It should not encode media bytes, editor UI state, or a new editorial timeline. [OpenTimelineIO](https://opentimelineio.readthedocs.io/en/latest/) covers editorial cuts, tracks, timing, transitions, markers, and media references; its role would be an optional carrier for editorial context, not the core representation of every crop, effect, or animation. An adapter must test what its target editor persists and reopens.

Candidate adapter surfaces are a public editor's documented command API, a native project SDK, or an OTIO sidecar plus editor-specific application. Examples among them: [FilmCraft documents a headless engine and JSON command channel](https://github.com/storytold/filmcraft/blob/main/docs/architecture.md) and [versioned native project files](https://github.com/storytold/filmcraft/blob/main/docs/project-files.md); [Blender background Python](https://docs.blender.org/manual/en/latest/advanced/command_line/arguments.html) is another. Neither is preferred; the adapter target is chosen in E-01, and any end-to-end editable round trip is an integration experiment. Rendering and encoding should continue through established tools.

Run a minimal save/reopen spike as soon as M0 emits a stable crop decision, before committing to M2's layered design. Apply one crop through a documented editor surface, save and reopen the project, inspect the persisted editable operation, modify it in the editor, and compare the resulting geometry with the source decision. Record unsupported or approximated fields separately from rendered pixel differences. This tests the central editability claim early; a complete interchange format and production adapter can still wait until M4.

## Competitive boundary

| Existing tool | What it already does | Remaining case for this project |
| --- | --- | --- |
| [MediaPipe AutoFlip](https://github.com/google-ai-edge/mediapipe/blob/master/docs/solutions/autoflip.md) | Saliency-aware video cropping and camera movement; its legacy solution ended support in 2023 and example encodes a cropped video. | Caller-defined geometric constraints and tool-neutral, editable decisions across layers; no need to reproduce detection. |
| [Adobe Premiere Auto Reframe](https://helpx.adobe.com/premiere/desktop/add-video-effects/commonly-used-effects/add-auto-reframe-effect-to-a-sequence.html) | Duplicates a sequence and adds editable Auto Reframe effects/keyframes. | A non-Premiere library with explicit caller constraints and diagnostics; Premiere remains a strong existing solution for its users. |
| [OTIO](https://opentimelineio.readthedocs.io/en/latest/) | Editorial interchange model and adapters. | Different problem: geometry and constraints, without inventing another timeline. |
| [FilmCraft](https://github.com/storytold/filmcraft/blob/main/docs/control-protocol.md) and other editors | Native composition, project persistence and rendering. | Supply decisions editors can apply; do not build an editor. |
| [Rust `geo`](https://docs.rs/geo/latest/geo/) and geometry crates | General geometry and affine operations. | Reuse if advanced shapes justify it; M0 needs only rectangles and one affine transform. |

The distinction is testable, not assumed: if an existing editor or library can accept all required constraints and emit equally editable, inspectable decisions through a stable API, integration may be preferable to a new engine.

## Technical risks and validation experiments

1. **Model usefulness:** test a synthetic landscape shot with central focus and edge ROI against center-crop. The ROI must survive or receive an explicit infeasibility report. A crop that simply mirrors center-crop adds little value.
2. **Geometry semantics:** test rectangular containment at exact boundaries, fractional crop sizes, extreme aspect ratios, and all nonfinite inputs. Use an independent validator in tests.
3. **Different editor coordinates:** perform the early save/reopen/modify spike above once the M0 crop decision is stable; compare persisted geometry and rendered output separately.
4. **Solver fit:** prepare synthetic two-layer and three-layer collision cases, then prototype linear/Cassowary and MILP/CP approaches separately. Measure solve time, feasibility status, deterministic tie behavior, and packaging before choosing one.
