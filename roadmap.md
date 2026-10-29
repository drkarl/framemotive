# Incremental roadmap and proposed issue plan

**Status:** proposal for review. The GitHub repository has an `origin` remote, and its initial architecture commit is on `main`; no issues have been filed, and there is no implementation or release yet. The architecture and first milestone require owner approval before coding or publication. Later milestones are hypotheses to be refined by fixtures and integration experiments, not preauthorized work.

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

This is a proposed structure, not a command to create files now. Split modules only where tests and invariants benefit; avoid a workspace or plugin framework for one operation. A Rust CLI and JSON request/response are desirable early follow-ups, but M0 should first stabilize the typed library semantics and example. If an executable is needed early, a thin CLI can be included after a small serialization ADR is reviewed.

**Acceptance examples (to become synthetic fixtures):**

1. Source `1920×1080`, target `1080×1920`, required ROI `[1200, 1400]×[300, 800]`: center crop hides part of the ROI; the library shifts the approximately `607.5×1080` source crop to include it and reports `Solved`.
2. Same canvases, ROI spanning source x coordinates `200..1600`: the required width exceeds any full-bleed portrait crop; result is `Infeasible` with actual versus available width.
3. ROI near an edge plus a margin extending outside the source: explicit `Infeasible` or `InvalidInput` according to the approved margin contract, never a clipped success.
4. Nonfinite coordinates, zero canvas dimension, duplicate region IDs and out-of-bounds region: typed `InvalidInput` diagnostics.
5. Empty ROI list with a preferred point: deterministic preferred-center crop; repeated runs on a pinned build yield identical geometry.

**M0 completion bar:** Tests compare source containment, aspect, all required ROI containment, affine mapping, status and diagnostic codes using independent assertions. README clearly says one static layer only. A normal `cargo test` and example run pass on supported platforms. No rendering or editor integration is claimed. Public release and external sharing require separate approval.

## Milestones after M0

| Milestone | Verifiable result | Gating evidence |
| --- | --- | --- |
| M1 — Reliable static API | Explicit hard/soft static visibility and placement constraints, safe rectangles, collision validation, coherent result schema and simple CLI. | Cases where center/analytic crop is inadequate; stable diagnostics and no silent hard-constraint violation. |
| M2 — Layered layout | Independent graphics/captions with bounds and separation; solver choice based on synthetic benchmark; infeasible/indeterminate distinguished. | Two- and three-layer fixtures, native dependency/licence review, measured solver behavior. |
| M3 — Time-varying composition | Rational time, trajectories, cut-aware smoothing, keyframe output and between-sample validation. | Moving-region fixture set, velocity/visibility measurements, clear temporal error bounds. |
| M4 — Editable interchange | Versioned decision document and one editor adapter; save/reopen/modify proof with unsupported-operation reporting. | Editor command/API contract and round-trip fixture; structural and rendered fidelity evaluated separately. |
| M5 — Hardening and release | Property/fuzz tests, benchmarks, platform CI, docs/examples, packaging and compatibility policy. | Real-world anonymized/public cases and explicit performance budgets; ownership/licence clearance. |

Milestones may split or reorder. For example, a minimal adapter spike can precede M2 if editability is the strongest value uncertainty, while a large temporal solver should not precede a working static validation core.

## Proposed issue candidates, not yet filed

Each proposed issue is deliberately bounded. References are local to this package; no issues have been filed.

| ID / dependency | Candidate issue and scope | Acceptance / test expectation |
| --- | --- | --- |
| A-01 / none | Approve ADR-001–004 and numeric tolerance, margin and API naming. No code. | Owner records decisions and unresolved points; future changes use ADR updates. |
| A-02 / A-01 | Check personal ownership, licence intent, and dependency/distribution boundaries. No legal-certainty claim. | Applicable agreements reviewed; repository visibility and submission gate documented. |
| F-01 / A-01,A-02 | Add an honest README and Cargo library skeleton to this local repository. No application-specific material. | Source tree and docs independently understandable; no empty feature claims. |
| F-02 / F-01 | Add validated `Canvas`, `Rect`, `Region` and typed diagnostics. | Tests for finite/positive/bounds/ID validation and boundary convention. |
| F-03 / F-02 | Implement maximal static crop interval algorithm and structured decision. | Synthetic off-center, impossible-wide, square, and empty-ROI fixtures; independent invariant checker. |
| F-04 / F-03 | Add runnable synthetic example and accurate README status. | Example executes using real library API; docs distinguish implemented and planned. M0 complete. |
| N-01 / F-04 | Design soft-priority semantics and static constraint schema. No solver selected yet. | ADR amendment with fixtures that show why M0 is insufficient. |
| N-02 / N-01 | Benchmark Rust Cassowary, `good_lp` backends and, if needed, CP-SAT on representative multi-layer cases. | Same fixture set, status/latency/tie behavior and licence/build comparison; backend recommendation. |
| N-03 / N-02 | Implement bounded multi-layer placement and post-solve validation. | Safe-area and overlap fixture tests; hard failures never reported as solved. |
| T-01 / N-03 | Specify rational-time samples, cuts and trajectory constraints. | Movement fixtures and an explicit between-sample validation strategy. |
| E-01 / F-04 | Prototype one public editor save/reopen translation of static crop decisions. | Editability and output geometry measured; unsupported operations reported. May run before N-03. |
| D-01 / F-04 | Define versioned JSON CLI contract for the supported operation. | Round-trip schema tests, bad-input diagnostics, stable output ordering. Could be pulled into M0 only after review. |
| Q-01 / F-04 | Add property/fuzz and benchmark harness for geometry. | Invariants hold across generated finite inputs; baseline environment and timing recorded. |

**Implementation-ready after architecture approval:** F-01 through F-04, subject to the ownership gate. **Needs design or an experiment:** N-01 onward, T-01, E-01, D-01, and Q-01 details. Do not file issues until the architecture and this breakdown have been reviewed, and do not implement the later roadmap merely because it is listed.

## Decisions requested at the Phase A review

1. Approve Rust as the primary core and the absence of an optimizer in M0; determine whether a CLI is needed immediately for the application-ready foundation.
2. Approve the M0 operation class: one full-bleed visual, maximal source crop, required ROIs, optional margin and preferred focus; no padding or multi-layer placement yet.
3. Decide whether a margin outside source bounds is `InvalidInput` or valid-but-`Infeasible`, and choose canvas-size limits and numeric tolerance before code.
4. Confirm the typed outcome vocabulary and that “infeasible” always names its supported model.
5. Review applicable IP/ownership agreements before the first implementation commit, external sharing, or publication.

## Source index

Primary public material informing this design: [MediaPipe AutoFlip](https://github.com/google-ai-edge/mediapipe/blob/master/docs/solutions/autoflip.md), [Premiere Auto Reframe](https://helpx.adobe.com/premiere/desktop/add-video-effects/commonly-used-effects/add-auto-reframe-effect-to-a-sequence.html), [OpenTimelineIO](https://opentimelineio.readthedocs.io/en/latest/), [FilmCraft public architecture](https://github.com/storytold/filmcraft/blob/main/docs/architecture.md), [Rust `cassowary`](https://docs.rs/cassowary/latest/cassowary/), [`good_lp`](https://github.com/rust-or/good_lp), [Clarabel](https://clarabel.org/stable/), [`argmin`](https://argmin-rs.org/), [OR-Tools CP-SAT](https://developers.google.com/optimization/cp/cp_solver), [`kurbo`](https://docs.rs/kurbo/latest/kurbo/), [`nalgebra`](https://www.nalgebra.rs/docs/), [`parry2d`](https://docs.rs/parry2d/latest/parry2d/), and [`splines`](https://docs.rs/splines/latest/splines/). Their documented capabilities have not been executed in this project; pin versions and repeat the dependency review when implementation begins.
