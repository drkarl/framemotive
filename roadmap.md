# Incremental roadmap and proposed issue plan

**Status:** M0 architecture decisions are accepted in ADR-001 through ADR-004. Project repository status is recorded in the [README](README.md). Roadmap work is tracked in [GitHub issues #2–#18](https://github.com/drkarl/framemotive/issues); no release has been made. Implementation is tracked separately. Later milestones are hypotheses to be refined by fixtures and integration experiments. [ADR-005](decisions/ADR-005-initial-scope-static-cropping.md) (proposed) narrows the initial plan to M0 and M1, constraint-aware static cropping; M2, M3, M4 and the early editor spike are deferred possibilities, kept below as a record. A detailed M1 issue breakdown follows after the architecture update is reviewed.

## Application-ready foundation (M0)

**User-visible capability:** Given a static full-bleed source visual, target canvas, required source-space regions, and an optional focus point, calculate the maximal target-aspect crop that preserves every required region, or explain why that supported crop model cannot satisfy them. Return crop and affine transform as typed data. This beats a center crop when a caller-supplied subject sits off center and avoids false success when multiple regions cannot fit.

**Suggested source shape after approval:**

```
README.md
Cargo.toml
src/lib.rs              # public API and documented outcomes
src/geometry.rs         # validated canvas, point and rectangle types
src/crop.rs             # one analytic adaptation operation
src/diagnostics.rs      # codes, statuses and evidence
examples/static_crop.rs # synthetic runnable example
tests/crop_cases.rs     # invariants and failure cases
docs/                  # these reviewed decisions and roadmap
```

This is a proposed structure, not a command to create files now. Split modules only where tests and invariants benefit; avoid a workspace or plugin framework for one operation. M0 stabilizes the typed library semantics and example; a thin Rust CLI and versioned JSON contract follow in M1 after D-01 is reviewed.

**Acceptance examples (to become synthetic fixtures):**

1. Source `1920×1080`, target `1080×1920`, required ROI `[1200, 1400]×[300, 800]`, no focus: center crop hides part of the ROI. The crop is `607.5×1080`; the hard x-interval is `[792.5, 1200]`; the union center is x = 1300, so the desired origin is `1300 − 303.75 = 996.25`, inside the interval. Result: crop x `996.25..1603.75`, y `0`, `Solved`, subject centered rather than flush to the edge.
2. Same canvases, ROI spanning source x coordinates `200..1600`: the required width exceeds any full-bleed portrait crop; result is `Infeasible` with actual versus available width and a fit/pad diagnostic (`s_fit = 0.5625`, no horizontal bars, vertical bars totaling `656.25 + 656.25 = 1312.5` target pixels); no `max_upscale` or T is set, so the unvalidated fit/pad alternative satisfies the hard constraints.
3. Same canvases, ROI `[1850, 1900]×[300, 800]`, margin `100` extending past the right source edge: by default `Solved` with crop x `1312.5..1920` and a reported right-edge shortfall (achieved `20`, requested `100`, shortfall `80`). In `hard` margin mode: `Infeasible`, naming region ID, margin and the right edge. A negative or nonfinite margin is `InvalidInput`.
4. Nonfinite coordinates, zero canvas dimension, duplicate region IDs, a wholly out-of-bounds region, or a region whose clipped portion is below the minimum extent: typed `InvalidInput` diagnostics. A region partly outside the source is clipped with a `region_clipped` diagnostic carrying the original bounds.
5. Empty ROI list with a preferred point: deterministic preferred-center crop; repeated runs yield identical geometry.
6. Source `1280×720`, target `1080×1920`, `max_upscale` `2.0`: the maximal crop is `405×720` and `s = 1080/405 = 2.666…` exceeds the limit, so `Infeasible` with measured scale and limit (and a fit/pad diagnostic: `s_fit = 0.84375 ≤ 2.0`, vertical bars totaling `1312.5` target pixels; no T, so the alternative satisfies the hard constraints).
7. Source `1920×1080`, target `1080×1080` (crop `1080×1080`, `s = 1`), required ROI `[1500, 1700]×[300, 700]`, focus `(1000, 540)`, safe rectangle `T = [100, 980]×[100, 980]`: the hard x-interval from source and ROI containment is `[620, 840]`; T adds `x ≤ 1500 − 100 = 1400` and `x ≥ 1700 − 980 = 720`, giving `[720, 840]`. The focus-derived origin `1000 − 540 = 460` clamps to `720`; crop x `720..1800`, y `0`; the ROI maps to `[780, 980]×[300, 700]`, inside T. Without T the origin would be `620`.
8. Source `1000×1000`, target `500×1000`, required ROI `[300, 700]×[200, 800]`, margin `100`, focus `(0, 500)`. x: `cw = 500`, hard interval `[200, 300]`, `dL = dR = 300`; `lo = 200 + m'`, `hi = 300 − m'`, so `m* = 50` and `M = [250, 250]`: crop x `250..750`, achieved margin `50` left and right, shortfall `50` each (requested `100`); the focus origin `0 − 250` clamps into `M`. y: crop height `1000 = H`, so y `0`; top/bottom achieved margin `200`, no shortfall. `Solved`. In `hard` margin mode: `Infeasible` (`m* = 50 < 100`).
9. Source `1000×1000`, target `1000×1000`, `max_upscale` `0.5`, any ROI: `s = 1 > 0.5`, so `Infeasible`; `s_fit = 1 > 0.5`, so the fit alternative is reported as violating `max_upscale`.

**M0 completion bar:** Tests compare source containment, aspect, all required ROI containment, affine mapping, status and diagnostic codes using independent assertions. README clearly says one static layer only. A normal `cargo test` and example run pass on supported platforms. Canonical output contract (ADR-002): golden fixtures record exact `f64` bit patterns (`to_bits` hex) and per-push CI on Linux x86_64 compares them; Linux aarch64 (local `cross`), macOS and Windows (manual workflow) must also pass before release, so M0 decisions are bitwise identical on all four. No rendering or editor integration is claimed.

## Milestones after M0

| Milestone | Verifiable result | Gating evidence |
| --- | --- | --- |
| Early editor spike — **deferred** | Apply one M0 crop in an editor, save, reopen, inspect and modify its editable operation. | Persisted crop/transform geometry and unsupported fields recorded; result informs the later adapter contract. |
| M1 — Constraint-aware static cropping | Single source layer only: several targets per request, required and preferred (tiered) regions, measured soft results, infeasibility evidence with binding regions and needed relaxation, versioned JSON report and CLI, opt-in integer snapping and cover-position mapping. | Static fixtures demonstrate tier and tie behavior, useful infeasibility evidence, revalidated snapping and no silent hard-constraint violation. |
| M2 — Layered layout — **deferred** | Add independently movable graphics/captions, per-layer target safe areas, layer bounds, collision avoidance and separation with hard/soft placement semantics; choose a solver from representative fixtures. | Two- and three-layer cases, measured violations and conflict IDs, infeasible versus indeterminate status, native dependency/licence review and measured solver behavior. |
| M3 — Time-varying composition — **deferred** | Validated caller-supplied subject tracks on a rational timebase, cut-aware smoothing, keyframe output and between-sample validation. | Moving-region fixtures include gaps, cuts and uncertain observations; velocity/visibility measurements, temporal error bounds and unverified spans are explicit. |
| M4 — Editable interchange — **deferred** | Versioned decision document and a maintained editor adapter extending the early spike. | Save/reopen/modify fixtures, unsupported-operation reporting, and separate structural and rendered fidelity results. |
| M5 — Hardening and release | Property/fuzz tests, benchmarks, platform CI, docs/examples, packaging and compatibility policy. | Representative public cases, explicit performance budgets and dependency licence review. |

M2, M3, M4 and the early editor spike are deferred possibilities (ADR-005): not scheduled, and none gates M1 or M5. Their rows and issues are kept unchanged as design history; reopening one needs a new decision. M5 hardening can follow M1 directly. The early editor spike was designed to follow a stable M0 decision and precede a commitment to M2; it is a feasibility experiment, not a production adapter or an M0 completion requirement.

## Issue plan

Each issue is deliberately bounded. Dependencies refer to the IDs below; linked GitHub issues contain acceptance criteria. Filing an issue does not authorize implementation.

| ID / dependency | Candidate issue and scope | Acceptance / test expectation |
| --- | --- | --- |
| [A-01](https://github.com/drkarl/framemotive/issues/2) / none | Record the accepted M0 contract in ADR-001–004, including numeric tolerance, margin and API naming. No code. | M0 decisions are explicit; future choices remain assigned to later issues. |
| [F-01](https://github.com/drkarl/framemotive/issues/3) / A-01 | Add a Cargo library skeleton and update the existing README to reflect implemented capabilities. | Source tree and docs independently understandable; no empty feature claims. |
| [F-02](https://github.com/drkarl/framemotive/issues/4) / F-01 | Add validated `Canvas`, `Rect`, `Region` and typed diagnostics. | Tests for finite/positive/bounds/ID validation and boundary convention. |
| [F-03](https://github.com/drkarl/framemotive/issues/5) / F-02 | Parent (epic) of F-03a, F-03b and F-03c; closes when they do. Implement maximal static crop interval algorithm and structured decision. | Synthetic off-center, impossible-wide, square, and empty-ROI fixtures; independent invariant checker. |
| [F-03a](https://github.com/drkarl/framemotive/issues/21) / F-02 | Compute the core maximal crop and affine mapping: union U, hard interval, default focus, clamp, scale, `Solved` vs `Infeasible` with extent evidence, empty-region behavior. | Examples 1, 2 and 5 as fixtures. |
| [F-03b](https://github.com/drkarl/framemotive/issues/22) / F-03a | Soft and hard margin with achieved-margin maximization (m*), per-region per-edge achieved margin and shortfall; hard mode `Infeasible` iff m* < m. | Examples 3 and 8 as fixtures. |
| [F-03c](https://github.com/drkarl/framemotive/issues/23) / F-03a | Safe rectangle T, `max_upscale` and the fit/pad alternative reported as unvalidated with per-constraint checks. | Examples 6, 7 and 9 as fixtures. |
| [Q-00](https://github.com/drkarl/framemotive/issues/24) / F-01 | Per-push GitHub Actions CI on Linux x86_64: fmt, clippy `-D warnings`, test, cargo-deny; plus a manual `workflow_dispatch` workflow for macOS and Windows and a documented local `cross` aarch64 command. | All checks pass on Linux x86_64 per push; the manual macOS/Windows workflow runs green before release. |
| [Q-01a](https://github.com/drkarl/framemotive/issues/25) / F-03a | Independent geometry validator separate from the solver, plus proptest property tests; extended as F-03b/F-03c land. | Invariants hold over generated finite inputs; failing cases keep seed/input. |
| [F-05](https://github.com/drkarl/framemotive/issues/26) / F-03b,F-03c,Q-00 | Canonical output (-0.0 normalized, field and diagnostic order) and golden `to_bits` fixtures, IEEE basic operations only. | Fixtures pass per push on Linux x86_64; aarch64 (local `cross`), macOS and Windows (manual workflow) before release. |
| [F-04](https://github.com/drkarl/framemotive/issues/6) / F-03 | Add runnable synthetic example and accurate README status. | Example executes using real library API; docs distinguish implemented and planned. Golden `to_bits` fixtures pass per push on Linux x86_64. M0 complete; the release additionally needs aarch64, macOS and Windows checks. |
| [E-01](https://github.com/drkarl/framemotive/issues/7) / F-04 | **Deferred.** Prototype one editor save/reopen/modify translation of an M0 crop. | Persisted editable geometry and unsupported or approximated fields measured before M2 design is committed. |
| [N-01](https://github.com/drkarl/framemotive/issues/8) / F-04 | **M1.** Define and implement hard/soft crop semantics for one static source layer. | Priority, measured violation, tie and infeasibility fixtures; no independently movable overlay or collision promise. |
| [S-01](https://github.com/drkarl/framemotive/issues/27) / F-04 | **M1.** Batch target decisions from one request. | Independent outcomes per target, stable order, request not mutated. |
| [S-02](https://github.com/drkarl/framemotive/issues/28) / F-04 | **Deferred.** Target-space anchors for required regions. | Anchor replaces derived focus; never violates hard constraints or margin. |
| [S-03](https://github.com/drkarl/framemotive/issues/29) / F-04 | **Deferred.** Crop zoom range between a minimum and the maximal crop; design semantics first. | Selection order and infeasibility semantics documented; fixtures. |
| [S-04](https://github.com/drkarl/framemotive/issues/30) / N-01 | **M1.** Priority-based dropping of required regions (sub-issue of N-01). | Largest priority-ordered fitting subset reported with region IDs. |
| [S-05](https://github.com/drkarl/framemotive/issues/31) / F-04 | **Deferred.** Static per-shot crop from multiple region samples; T-00 later supersedes the input form. | Crop contains all sample boxes. |
| [D-01](https://github.com/drkarl/framemotive/issues/9) / F-04 | **M1.** Define versioned JSON CLI contract for the supported static operation in M1. | Round-trip schema tests, bad-input diagnostics, stable output ordering. |
| [N-02](https://github.com/drkarl/framemotive/issues/10) / N-01,E-01 | **Deferred.** Benchmark Rust Cassowary, `good_lp` backends and, if needed, CP-SAT on representative multi-layer cases. | Same fixture set, status/latency/tie behavior and licence/build comparison; backend recommendation. |
| [N-03](https://github.com/drkarl/framemotive/issues/11) / N-02 | **Deferred.** Implement bounded multi-layer placement and post-solve validation. | Safe-area and overlap fixture tests; hard failures never reported as solved. |
| [T-00](https://github.com/drkarl/framemotive/issues/12) / F-04 | **Deferred.** Specify caller-supplied subject-track metadata and validation. | Track IDs, source space, timebase, gaps, cuts, confidence and observation provenance tested in fixtures. |
| [T-01](https://github.com/drkarl/framemotive/issues/13) / T-00 | **Deferred.** Specify trajectories and cut-aware smoothing constraints. | Movement fixtures and an explicit between-sample validation strategy. |
| [T-02](https://github.com/drkarl/framemotive/issues/14) / T-01 | **Deferred.** Implement temporal crop decisions and editable keyframes. | Cut-aware motion, visibility, between-sample checks and unverified spans tested on moving fixtures. |
| [E-02](https://github.com/drkarl/framemotive/issues/15) / E-01,D-01 | **Deferred.** Define a versioned geometric decision document. | Supported operations round-trip with stable IDs and explicit adapter fidelity semantics. |
| [E-03](https://github.com/drkarl/framemotive/issues/16) / E-02 | **Deferred.** Build and verify the first maintained editor adapter. | Save/reopen/modify tests and supported/approximated/unsupported/unverified reporting. |
| [Q-01](https://github.com/drkarl/framemotive/issues/17) / F-04 | Add property/fuzz and benchmark harness for geometry. | Invariants hold across generated finite inputs; baseline environment and timing recorded. |
| [Q-02](https://github.com/drkarl/framemotive/issues/18) / Q-01 | Define platform CI, packaging and release criteria. | Platform checks, dependency review, performance budgets and compatibility policy are documented. |

**Next implementable candidates:** the M0 issues in order: F-02, F-03a, F-03b, F-03c, Q-01a, F-05, F-04, each as a separate task (F-01 and Q-00 are done: the crate skeleton and CI exist). **M1, after review of ADR-005:** S-01, N-01 with S-04, D-01, plus new issues for infeasibility evidence, integer snapping and cover-position mapping ([architecture §14](architecture.md#14-initial-usable-implementation-slice)). **Deferred, not scheduled:** E-01 to E-03, N-02, N-03, T-00 to T-02, S-02, S-03, S-05. **Hardening:** Q-01, Q-02. Filing or reading an issue does not authorize implementation.

## Accepted M0 decisions

1. Rust is the primary core. M0 uses a typed library and example with no optimizer or CLI; the versioned CLI follows in M1.
2. M0 handles one full-bleed visual, maximal source crop, required ROIs, optional uniform margin (soft by default) and preferred focus, optional `max_upscale` and target safe rectangle. Padding and multi-layer placement remain outside M0.
3. Canvas dimensions are `1..=65_536` per axis. ADR-002 defines post-computation tolerance and a minimum required-region extent. Margin is a soft preference by default: a margin extending past the source yields `Solved` with reported shortfall, and is model-scoped `Infeasible` only in `hard` margin mode. Partly out-of-frame regions are clipped with a diagnostic.
4. The M0 public types and three outcomes are recorded in ADR-003; `Unsupported` and `Indeterminate` remain future outcome names.

The target editor and supported automation surface for the early save/reopen spike are chosen in E-01. Multi-layer solver selection and temporal input remain later decisions.

## Proposed initial-scope decision

[ADR-005](decisions/ADR-005-initial-scope-static-cropping.md) (proposed) makes M0 + M1 the initial plan, adds batch targets, preferred-region tiers, infeasibility evidence, integer snapping and cover mapping to M1, and lists the exact ADR-002, ADR-003 and ADR-004 statements it supersedes.

## Source index

Primary public material informing this design: [MediaPipe AutoFlip](https://github.com/google-ai-edge/mediapipe/blob/master/docs/solutions/autoflip.md), [Premiere Auto Reframe](https://helpx.adobe.com/premiere/desktop/add-video-effects/commonly-used-effects/add-auto-reframe-effect-to-a-sequence.html), [OpenTimelineIO](https://opentimelineio.readthedocs.io/en/latest/), [FilmCraft public architecture](https://github.com/storytold/filmcraft/blob/main/docs/architecture.md), [Rust `cassowary`](https://docs.rs/cassowary/latest/cassowary/), [`good_lp`](https://github.com/rust-or/good_lp), [Clarabel](https://clarabel.org/stable/), [`argmin`](https://argmin-rs.org/), [OR-Tools CP-SAT](https://developers.google.com/optimization/cp/cp_solver), [`kurbo`](https://docs.rs/kurbo/latest/kurbo/), [`nalgebra`](https://www.nalgebra.rs/docs/), [`parry2d`](https://docs.rs/parry2d/latest/parry2d/), and [`splines`](https://docs.rs/splines/latest/splines/). Their documented capabilities have not been executed in this project; pin versions and repeat the dependency review when implementation begins.
