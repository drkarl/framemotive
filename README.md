# FrameMotive — Phase A design package

**Status:** proposed architecture for review, 27 October 2026. This local Git repository contains design documents and no implementation. No ownership, licence, or publication approval is implied.

## Purpose

Build a standalone library that converts a source composition and caller-supplied layout constraints into **structured, editable adaptation decisions** for a target canvas. The library operates on geometry and metadata; callers supply media and regions of interest. It does not infer subjects, render pixels, edit a timeline, or decide what is creatively important.

Potential users include video editors, campaign production tools, motion-graphics systems, slide and digital-signage software, and media automation pipelines. The first useful result should be a reproducible static crop decision with a clear infeasibility report. Multi-layer placement and temporal motion are planned, not claimed.

## Read order

1. [Requirements and acceptance measures](requirements.md)
2. [Architecture, domain model, and technical risks](architecture.md)
3. [Decision records](decisions/)
4. [Milestones and proposed issue breakdown](roadmap.md)

## Immediate review gate

Confirm the bounded first operation, coordinate and numerical conventions, Rust packaging, and error semantics before implementing code. The first implementation milestone is specified in [the roadmap](roadmap.md#application-ready-foundation-m0). Later architecture remains provisional and must be tested against real use cases.

## Independence and ownership

This package is specified from generic geometry, publicly documented tool capabilities, and synthetic examples. It contains no private product schemas, source code, application architecture, internal algorithms, customer data, or company-specific workflow. A future project can be useful without integration with any particular editor.

Technical independence does **not** establish personal legal ownership. Before making a personal repository accessible externally or sharing its contents with a third party, review applicable employment, startup, invention-assignment, contributor, and platform terms. A private repository and private licence do not by themselves control copies retained by external services. Publication and external sharing require a separate decision.

## Design status

| Topic | Phase A proposal | To validate later |
| --- | --- | --- |
| Core language | Rust library; no hybrid core initially | Packaging ergonomics and bindings demand |
| First operation | One static full-bleed visual layer; required region visibility; maximal source crop at target aspect; preferred center | Actual client usefulness and expected fallback behavior |
| Coordinates | Continuous pixel-edge geometry, top-left origin, half-open rectangles | Fractional-pixel precision across editors |
| Solver | Analytic geometry for M0; no general solver dependency | Static multi-layer constraint experiments |
| Output | Typed crop and affine placement decision plus diagnostics | JSON schema and editor adapters |
| Later | Layer layout, safe zones, time, trajectories, interoperability | Specific algorithms, versions, benchmarks |

This is a design proposal for discussion. There is no implementation claim or fabricated example output.
