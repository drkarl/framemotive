# Incremental roadmap and proposed issue plan

**Status:** proposal for review. Project repository and registration status are recorded in the [README](README.md). Roadmap work is tracked in [GitHub issues #2–#18](https://github.com/drkarl/framemotive/issues); no release has been made. The architecture and first milestone require owner approval before coding or publication. Later milestones are hypotheses to be refined by fixtures and integration experiments, not preauthorized work.

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

1. Source `1920×1080`, target `1080×1920`, required ROI `[1200, 1400]×[300, 800]`: center crop hides part of the ROI; the library shifts the approximately `607.5×1080` source crop to include it and reports `Solved`.
2. Same canvases, ROI spanning source x coordinates `200..1600`: the required width exceeds any full-bleed portrait crop; result is `Infeasible` with actual versus available width.
3. ROI near an edge plus a valid margin extending outside the source: `Infeasible` for the supported in-bounds crop model, never a clipped success. A negative or nonfinite margin is `InvalidInput`.
4. Nonfinite coordinates, zero canvas dimension, duplicate region IDs and out-of-bounds region: typed `InvalidInput` diagnostics.
5. Empty ROI list with a preferred point: deterministic preferred-center crop; repeated runs on a pinned build yield identical geometry.

**M0 completion bar:** Tests compare source containment, aspect, all required ROI containment, affine mapping, status and diagnostic codes using independent assertions. README clearly says one static layer only. A normal `cargo test` and example run pass on supported platforms. No rendering or editor integration is claimed.

## Milestones after M0

| Milestone | Verifiable result | Gating evidence |
| --- | --- | --- |
| Early editor spike | Apply one M0 crop in an editor, save, reopen, inspect and modify its editable operation. | Persisted crop/transform geometry and unsupported fields recorded; result informs the later adapter contract. |
| M1 — Reliable static crop API | Single source layer only: hard visibility, soft framing preferences with measured violations, stable result schema and a small versioned CLI. | Static fixtures demonstrate priority and tie behavior, useful infeasibility evidence and no silent hard-constraint violation. |
| M2 — Layered layout | Add independently movable graphics/captions, target safe areas, layer bounds, collision avoidance and separation with hard/soft placement semantics; choose a solver from representative fixtures. | Two- and three-layer cases, measured violations and conflict IDs, infeasible versus indeterminate status, native dependency/licence review and measured solver behavior. |
| M3 — Time-varying composition | Validated caller-supplied subject tracks on a rational timebase, cut-aware smoothing, keyframe output and between-sample validation. | Moving-region fixtures include gaps, cuts and uncertain observations; velocity/visibility measurements, temporal error bounds and unverified spans are explicit. |
| M4 — Editable interchange | Versioned decision document and a maintained editor adapter extending the early spike. | Save/reopen/modify fixtures, unsupported-operation reporting, and separate structural and rendered fidelity results. |
| M5 — Hardening and release | Property/fuzz tests, benchmarks, platform CI, docs/examples, packaging and compatibility policy. | Representative public cases, explicit performance budgets and dependency licence review. |

The early editor spike follows a stable M0 decision and precedes a commitment to M2. It is a feasibility experiment, not a production adapter or an M0 completion requirement. Later milestones may split or reorder as fixtures provide evidence.

## Issue plan

Each issue is deliberately bounded. Dependencies refer to the IDs below; linked GitHub issues contain acceptance criteria. Filing an issue does not authorize implementation.

| ID / dependency | Candidate issue and scope | Acceptance / test expectation |
| --- | --- | --- |
| [A-01](https://github.com/drkarl/framemotive/issues/2) / none | Approve ADR-001–004 and numeric tolerance, margin and API naming. No code. | Owner records decisions and unresolved points; future changes use ADR updates. |
| [F-01](https://github.com/drkarl/framemotive/issues/3) / A-01 | Add a Cargo library skeleton and update the existing README to reflect implemented capabilities. | Source tree and docs independently understandable; no empty feature claims. |
| [F-02](https://github.com/drkarl/framemotive/issues/4) / F-01 | Add validated `Canvas`, `Rect`, `Region` and typed diagnostics. | Tests for finite/positive/bounds/ID validation and boundary convention. |
| [F-03](https://github.com/drkarl/framemotive/issues/5) / F-02 | Implement maximal static crop interval algorithm and structured decision. | Synthetic off-center, impossible-wide, square, and empty-ROI fixtures; independent invariant checker. |
| [F-04](https://github.com/drkarl/framemotive/issues/6) / F-03 | Add runnable synthetic example and accurate README status. | Example executes using real library API; docs distinguish implemented and planned. M0 complete. |
| [E-01](https://github.com/drkarl/framemotive/issues/7) / F-04 | Prototype one editor save/reopen/modify translation of an M0 crop. | Persisted editable geometry and unsupported or approximated fields measured before M2 design is committed. |
| [N-01](https://github.com/drkarl/framemotive/issues/8) / F-04 | Define and implement hard/soft crop semantics for one static source layer. | Priority, measured violation, tie and infeasibility fixtures; no independently movable overlay or collision promise. |
| [D-01](https://github.com/drkarl/framemotive/issues/9) / F-04 | Define versioned JSON CLI contract for the supported static operation in M1. | Round-trip schema tests, bad-input diagnostics, stable output ordering. |
| [N-02](https://github.com/drkarl/framemotive/issues/10) / N-01,E-01 | Benchmark Rust Cassowary, `good_lp` backends and, if needed, CP-SAT on representative multi-layer cases. | Same fixture set, status/latency/tie behavior and licence/build comparison; backend recommendation. |
| [N-03](https://github.com/drkarl/framemotive/issues/11) / N-02 | Implement bounded multi-layer placement and post-solve validation. | Safe-area and overlap fixture tests; hard failures never reported as solved. |
| [T-00](https://github.com/drkarl/framemotive/issues/12) / F-04 | Specify caller-supplied subject-track metadata and validation. | Track IDs, source space, timebase, gaps, cuts, confidence and observation provenance tested in fixtures. |
| [T-01](https://github.com/drkarl/framemotive/issues/13) / T-00,N-03 | Specify trajectories and cut-aware smoothing constraints. | Movement fixtures and an explicit between-sample validation strategy. |
| [T-02](https://github.com/drkarl/framemotive/issues/14) / T-01 | Implement temporal crop decisions and editable keyframes. | Cut-aware motion, visibility, between-sample checks and unverified spans tested on moving fixtures. |
| [E-02](https://github.com/drkarl/framemotive/issues/15) / E-01,D-01 | Define a versioned geometric decision document. | Supported operations round-trip with stable IDs and explicit adapter fidelity semantics. |
| [E-03](https://github.com/drkarl/framemotive/issues/16) / E-02 | Build and verify the first maintained editor adapter. | Save/reopen/modify tests and supported/approximated/unsupported/unverified reporting. |
| [Q-01](https://github.com/drkarl/framemotive/issues/17) / F-04 | Add property/fuzz and benchmark harness for geometry. | Invariants hold across generated finite inputs; baseline environment and timing recorded. |
| [Q-02](https://github.com/drkarl/framemotive/issues/18) / Q-01 | Define platform CI, packaging and release criteria. | Platform checks, dependency review, performance budgets and compatibility policy are documented. |

**Implementation-ready after architecture approval:** F-01 through F-04. **Needs design or an experiment:** E-01 onward, N-01 onward, D-01, T-00 onward, and Q-01 onward. The later roadmap remains provisional and requires its own decisions and evidence before implementation.

## Decisions requested at the Phase A review

These are proposed choices for owner review, not approved decisions.

1. Approve Rust as the primary core, no optimizer in M0, and a typed library plus example in M0 with the versioned CLI deferred to M1.
2. Approve the M0 operation class: one full-bleed visual, maximal source crop, required ROIs, optional margin and preferred focus; no padding or multi-layer placement yet.
3. Approve the proposed `1..=65_536` canvas range and ADR-002 post-computation tolerance, and classify a valid margin outside source bounds as model-scoped `Infeasible`.
4. Confirm the M0 public type and outcome vocabulary; `Unsupported` and `Indeterminate` remain reserved for future models.
5. Select an editor and its supported automation surface for the early save/reopen spike; decide whether the result changes M2 priorities.

## Source index

Primary public material informing this design: [MediaPipe AutoFlip](https://github.com/google-ai-edge/mediapipe/blob/master/docs/solutions/autoflip.md), [Premiere Auto Reframe](https://helpx.adobe.com/premiere/desktop/add-video-effects/commonly-used-effects/add-auto-reframe-effect-to-a-sequence.html), [OpenTimelineIO](https://opentimelineio.readthedocs.io/en/latest/), [FilmCraft public architecture](https://github.com/storytold/filmcraft/blob/main/docs/architecture.md), [Rust `cassowary`](https://docs.rs/cassowary/latest/cassowary/), [`good_lp`](https://github.com/rust-or/good_lp), [Clarabel](https://clarabel.org/stable/), [`argmin`](https://argmin-rs.org/), [OR-Tools CP-SAT](https://developers.google.com/optimization/cp/cp_solver), [`kurbo`](https://docs.rs/kurbo/latest/kurbo/), [`nalgebra`](https://www.nalgebra.rs/docs/), [`parry2d`](https://docs.rs/parry2d/latest/parry2d/), and [`splines`](https://docs.rs/splines/latest/splines/). Their documented capabilities have not been executed in this project; pin versions and repeat the dependency review when implementation begins.
