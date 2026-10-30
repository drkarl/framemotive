# FrameMotive — Phase A design package

**Status:** M0 architecture decisions accepted 29 October 2026. The design documents are on GitHub `main`. This repository contains design documents and an empty Rust library crate (`framemotive`). No geometry, crop, or layout operation is implemented yet, and the crate exposes no public API.

## Purpose

Build a standalone library that converts a source composition and caller-supplied layout constraints into **structured, editable adaptation decisions** for a target canvas. The library operates on geometry and metadata; callers supply media and regions of interest. It does not infer subjects, render pixels, edit a timeline, or decide what is creatively important.

Potential users include video editors, campaign production tools, motion-graphics systems, slide and digital-signage software, and media automation pipelines. The first useful result should be a reproducible static crop decision with a clear infeasibility report. Multi-layer placement and temporal motion are planned, not claimed.

## Read order

1. [Requirements and acceptance measures](requirements.md)
2. [Architecture, domain model, and technical risks](architecture.md)
3. [Decision records](decisions/)
4. [Milestones and proposed issue breakdown](roadmap.md)

## M0 decision status

ADR-001 through ADR-004 record the accepted M0 operation, coordinate and numeric contract, Rust packaging, outcomes, and editable-decision boundary. The first implementation milestone is specified in [the roadmap](roadmap.md#application-ready-foundation-m0).

## Development

The crate is pinned to Rust 1.94.1 by `rust-toolchain.toml` (edition 2024, no dependencies). It currently contains crate-level documentation only, so the commands below verify the build and tooling, not any feature.

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo doc --no-deps
```

## Design provenance

This package is specified from generic geometry, publicly documented tool capabilities, and synthetic examples. It does not depend on any particular editor's project format.

## Design status

| Topic | Current design status | To validate later |
| --- | --- | --- |
| Core language | Rust library; no hybrid core initially | Packaging ergonomics and bindings demand |
| First operation | One static full-bleed visual layer; required region visibility; maximal source crop at target aspect; preferred focus (default: center of required regions) | Actual client usefulness and expected fallback behavior |
| Coordinates | Continuous pixel-edge geometry, top-left origin, half-open rectangles | Fractional-pixel precision across editors |
| Solver | Analytic geometry for M0; no general solver dependency | Static multi-layer constraint experiments |
| Output | Typed crop and affine placement decision plus diagnostics | JSON schema and editor adapters |
| Later | Layer layout, safe zones, time, trajectories, interoperability | Specific algorithms, versions, benchmarks |

Later milestones remain proposals. There is no fabricated example output.
