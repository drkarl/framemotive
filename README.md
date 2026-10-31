# FrameMotive

A Rust library for constraint-aware static cropping: given a source image or shot, the regions that must stay visible, and one or more target sizes, it computes the largest valid crop for each target, or explains with measurements why no such crop exists.

> **Status: design stage.** No crop operation is implemented yet and the crate exposes no public API. The examples below are **proposed** and do not run. See [Current maturity](#current-maturity).

## The problem

Publishing pipelines turn one visual into many renditions: 9:16 and 1:1 social variants of a 16:9 frame, 4:5 product images, banners, thumbnails, signage. Common crop tools choose a window from one focal point or a saliency estimate and always return a crop. When a product, logo, face or caption must remain visible, a clipped rendition is discovered only when someone looks at it. Callers cannot tell a best-effort crop from an impossible request, and end up hand-writing crop arithmetic for each case.

FrameMotive treats the regions a caller marks as constraints, not hints. It never silently cuts a required region: it either returns a crop that is independently rechecked against every hard constraint, or reports `Infeasible` with the regions that conflict, the gap in pixels and the relaxation that would be needed.

## Who it is for

- **Image and media pipelines** (e-commerce, CMS and asset-management systems, campaign tools) generating several aspect ratios from annotated sources.
- **CI and QA checks** that must fail a build when a rendition would lose a logo or legal text.
- **Video and motion tools** that want a static per-shot crop with explicit evidence instead of an opaque reframe.
- **Layout code** that positions an image with CSS-style `cover` behavior and wants the visible window chosen from constraints.

Typical workflow: a detector, an editor or a person supplies rectangles; FrameMotive returns per-target crops and diagnostics; an existing image library or renderer applies them.

## Example (proposed API)

```rust
// Proposed API; not implemented.
use framemotive::{crop_batch, Canvas, CropBatch, CropOutcome, Margin, Rect, Region, Requirement, Target};

let batch = CropBatch::new(Canvas::new(1920, 1080)?)
    .regions([
        Region::new("product", Rect::new(1200.0, 300.0, 200.0, 500.0)?)?,      // required
        Region::new("logo", Rect::new(1700.0, 60.0, 160.0, 80.0)?)?
            .with_requirement(Requirement::Preferred { tier: 1 }),             // keep if possible
    ])
    .margin(Margin::soft(24.0))
    .target(Target::new("story", Canvas::new(1080, 1920)?).max_upscale(2.0))
    .target(Target::new("square", Canvas::new(1080, 1080)?));

for result in crop_batch(&batch).results()? {
    match &result.outcome {
        CropOutcome::Solved(decision) => println!("{}: crop {:?}", result.target_id, decision.crop()),
        CropOutcome::Infeasible(report) => println!("{}: impossible: {report}", result.target_id),
        CropOutcome::InvalidInput(diagnostics) => println!("{}: invalid: {diagnostics:?}", result.target_id),
    }
}
```

Proposed CLI for pipelines and CI:

```text
framemotive crop renditions.json --snap outward     # exit 0 solved, 1 infeasible, 2 invalid input
```

For the first specified case (source `1920×1080`, target `1080×1920`, product box `[1200,1400)×[300,800)`), a center crop keeps about 32% of the product's width; the specified result is the crop `x = 996.25..1603.75`, which keeps all of it with the product centered. This value comes from the specification ([roadmap example 1](roadmap.md#application-ready-foundation-m0)), not from running code.

## Planned capabilities

Initial scope (milestones M0 and M1):

- Largest crop at each target's aspect ratio that keeps every **required** region inside the source crop.
- **Several targets** from one request, each evaluated independently.
- **Hard and soft constraints:** required regions, target safe rectangle and upscale limit are hard; margin is soft by default (or hard); **preferred** regions in priority tiers are kept when possible; an optional focus point decides among equally good crops.
- **Explicit outcomes:** `Solved`, `InvalidInput` or `Infeasible`, never a silent compromise.
- **Measured evidence:** margin shortfall per region and edge, dropped preferred regions with visible fraction, conflicting regions and gap for infeasible targets, needed relaxation, and an unvalidated fit/pad alternative.
- **Deterministic** decisions: bit-identical output for the same input on every checked platform.
- **Integration helpers:** opt-in integer crop rectangles for raster libraries (libvips, sharp, the `image` crate), revalidated after rounding, and a cover-position mapping for CSS `object-fit: cover`.
- A versioned JSON request/report and a small CLI with CI-friendly exit codes.

Non-goals: decoding or rendering media, detecting subjects or saliency, making creative judgments, editing timelines, or writing editor project files. Layered layout of movable captions and graphics, moving-subject reframing over time, and editor adapters are recorded as **deferred possibilities**, not planned work ([ADR-005](decisions/ADR-005-initial-scope-static-cropping.md)).

## Current maturity

| Area | State |
| --- | --- |
| Requirements, architecture, decision records | Written. M0 contract accepted (ADR-001 to ADR-004); initial-scope update accepted (ADR-005). |
| Rust crate | Empty library crate with crate-level documentation only; no public API. |
| Quality gate and CI | Working: rustfmt, clippy `-D warnings`, tests, rustdoc, `cargo deny`. |
| Crop computation, JSON, CLI | Not implemented. |
| Releases, licence | No release; no licence selected; not published. |

## Technical overview

- **Pure geometry.** Inputs are canvas sizes and rectangles in continuous pixel-edge coordinates (top-left origin, half-open rectangles); outputs are typed decisions: source crop, uniform scale, translation and constraint results. No pixels are touched.
- **Analytic algorithm.** For a full-bleed, axis-aligned crop at the target aspect, feasible origins form an interval per axis. The crate intersects intervals for hard constraints, then selects lexicographically: preferred tiers, then the largest achievable margin, then the position nearest the focus. Each target costs O(n log n) in the number of regions; no optimizer or native dependency is involved.
- **Independent recheck.** A separate validator checks every returned decision before it is reported as solved.
- **Rust quality bar.** MSRV Rust 1.94 (edition 2024), `unsafe_code` forbidden, documented public items, clippy pedantic with warnings denied, `cargo deny`, no panics on untrusted input. The default build has no dependencies; JSON and the CLI are optional features.

Read the [architecture](architecture.md) for the full design, then the [requirements](requirements.md), the [decision records](decisions/) and the [roadmap](roadmap.md).

## Development

The crate is pinned to Rust 1.94.1 by `rust-toolchain.toml` (edition 2024, no dependencies). It currently contains crate-level documentation only, so the commands below verify the build and tooling, not any feature.

```sh
cargo install cargo-deny --version 0.19.0 --locked   # once
./scripts/check.sh
```

`scripts/check.sh` is the single quality gate (rustfmt, clippy with `-D warnings`, tests, rustdoc with warnings denied, and `cargo deny` for advisories, licences, bans and sources). CI runs the same script.

### Platform checks

Per-push CI runs on Linux x86_64 only. macOS and Windows are checked by the manually triggered `release-platforms` workflow (tests and clippy only), which must pass before a release. Linux aarch64 is checked locally with `cross` (QEMU in a container): optional during development, required before a release. It needs podman.

```sh
cargo install cross --locked
CROSS_CONTAINER_ENGINE=podman RUSTC_WRAPPER= cross test --target aarch64-unknown-linux-gnu
```

The empty `RUSTC_WRAPPER` disables a host-configured wrapper such as sccache inside the container.
