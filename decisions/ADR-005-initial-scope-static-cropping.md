# ADR-005: Initial scope is constraint-aware static cropping

- **Status:** accepted
- **Date:** 2026-10-31

## Context

ADR-001 through ADR-004 accepted the M0 contract: one static, full-bleed source visual, required regions, an optional margin, focus, `max_upscale` and target safe rectangle, solved by interval geometry with explicit `Solved`, `InvalidInput` and `Infeasible` outcomes. The roadmap then continued into a general layered-layout solver (M2), time-varying composition (M3) and editor interchange (M4), with an editor save/reopen experiment placed before M2.

The useful, distinctive part of the design is narrower and closer: callers that generate several renditions of one image or shot (social, e-commerce, CMS, signage, thumbnails) need to know that marked subjects, logos and text survive every target, and need an honest refusal with measured evidence when they cannot. Common tools offer one focal point or a saliency guess and always return *some* crop. The layered solver and temporal work are research-sized and are not needed to deliver that value.

## Decision

1. The initial development plan is **M0 plus M1**, and M1 is redefined as *constraint-aware static cropping*:
   - several target canvases per request (batch), each evaluated independently by the M0 algorithm;
   - hard and soft constraints on one static source layer: required regions (hard), **preferred** regions in priority tiers (soft, dropped whole tier by tier and reported), margin (soft or hard), safe rectangle and `max_upscale` (hard), focus (lowest priority);
   - measured evidence for every soft shortfall, dropped region and infeasibility, including the binding regions and the relaxation that would be needed;
   - a versioned JSON request/report and a small CLI with CI-friendly exit codes (D-01);
   - an explicit, opt-in integer-pixel snapping step for raster pipelines, revalidated against the hard constraints;
   - a mapping helper for CSS-style `cover` layout.
2. M2 (layered layout and solver selection), M3 (time-varying composition) and M4 (editor interchange and adapters) are **deferred possibilities**. Their designs, issues and acceptance criteria are kept unchanged as a record. No work on them is scheduled, and none of them gates M1 or a first release.
3. The established vocabulary is kept: `Canvas`, `Rect`, `Region`, `CropRequest`, `CropDecision`, `Diagnostic` and `CropOutcome` keep their ADR-003 meaning. M1 adds types rather than changing these: `Requirement` on `Region` (default `Required`), `Target`, `CropBatch`, `BatchOutcome`, `PixelCrop`.
4. Hardening (M5: property tests, platform release gate, packaging) stays in the plan and may follow M1 directly.

## Statements superseded

Only the statements below change. Every other part of ADR-001 to ADR-004 stays in force.

| Where | Accepted statement | Change |
| --- | --- | --- |
| ADR-003, Decision ¶1 | "For future static multi-layer work, benchmark Cassowary and MILP/CP on the same synthetic fixtures. For temporal work, compare candidate-path methods and convex smoothing separately." | Applies only if layered or temporal work is reopened by a later decision. No benchmark or comparison is planned. The rest of ¶1 (interval geometry, independent validation, no public solver trait, preserved status distinctions) is unchanged. |
| ADR-004, Decision ¶3 | "After M0 produces a stable crop decision, run a minimal editor adapter experiment before settling M2's layered design." | Deferred with M2 and M4. It is no longer scheduled after M0 and does not gate M1. The initial integration targets are raster image pipelines and CSS-style layout (item 1 above). |
| ADR-002, Consequences ¶1 and the coordinate contract ("Round only in an adapter"; "Export adapters own their editor's rounding") | Rounding belongs to adapters. | The crate may provide one explicit, opt-in integer snapping function (`PixelCrop`) because every common raster API takes integer crop rectangles and each caller would otherwise reimplement rounding and the required revalidation. `CropDecision` itself stays unrounded and the core still never rounds implicitly. The snapped result is rechecked against every hard constraint and reports any violation; it never relabels a violation as success. |
| ADR-002, canonical-output contract ("only IEEE-754 basic operations (+ − × ÷ …)") | Basic operations only. | Also allow `floor` and `ceil` (IEEE 754 `roundToIntegral` operations, which are exact) in the snapping module only. `sqrt` remains allowed as already noted in `clippy.toml`. The transcendental and fused-multiply-add bans are unchanged. |
| ADR-002, margin ("applied to all four sides of each required region") | Margin applies to required regions. | In M1 the same margin also applies to preferred regions that are kept. M0 has no preferred regions, so M0 behavior is unchanged. |

Issue S-04 ("priority-based dropping of required regions") is realized as preferred-region tiers: a region marked `Required` is never dropped, and a caller who wants a region dropped under pressure marks it `Preferred`. With one region per tier this yields S-04's "largest priority-ordered fitting subset".

ADR-001 (Rust core; thin Rust CLI in M1) needs no change. ADR-003's M0 request (one source, one target) is unchanged: a batch is expanded into ordinary per-target `CropRequest`s.

## Consequences

- The first release can be judged on one clear capability, cropping one static visual for several targets without silently losing marked content, instead of an unproven general layout engine.
- Open GitHub issues for deferred work (layered layout, temporal work, editor interchange) stay open and are labelled as deferred in the roadmap; none is closed by this decision.
- The editability claim narrows: decisions are typed, unrounded geometry that callers can inspect and adjust, plus mappings to raster and CSS parameters. Saving into editor projects is not claimed.
- Preferred-region tiers make some results depend on soft priorities, so their measured evidence and deterministic tie rules become part of the M1 contract.

## Revisit when

A user need for independently movable overlays or moving-subject reframing is demonstrated with concrete fixtures, or a consumer requires a maintained editor adapter. Reopening any deferred milestone needs its own decision record.
