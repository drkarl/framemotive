# ADR-001: Rust core, with a small Rust CLI later

- **Status:** proposed for owner review
- **Date:** 2026-10-27

## Context

The project needs validated geometry, structured outcomes, portable distribution, synthetic testing, and eventually optimization and editor adapters. The owner strongly prefers Rust. The first operation is analytic and does not require a scientific Python stack.

## Alternatives

1. **Rust core and Rust CLI.** Strong type/invariant modeling, single native library and CLI toolchain, no Python runtime. [Rust geometry](https://docs.rs/kurbo/latest/kurbo/), [linear modeling](https://github.com/rust-or/good_lp), [convex optimization](https://clarabel.org/stable/), and [nonlinear methods](https://argmin-rs.org/) exist. Some advanced discrete methods may still need C/C++ FFI.
2. **Python core.** Faster experiments and access to [SciPy optimization](https://docs.scipy.org/doc/scipy/reference/optimize.html), [OR-Tools Python](https://developers.google.com/optimization/cp/cp_solver), and [OTIO Python](https://opentimelineio.readthedocs.io/en/latest/). More runtime/package variability for embedding as a core library and contrary to the owner preference.
3. **Rust/Python hybrid from day one.** Adds binding, ABI, wheel, CI, and versioning complexity before any Python consumer exists.

## Proposed decision

Use a Rust library crate as the primary implementation. The application-ready foundation should offer a library API and a runnable Rust example. Add a thin Rust CLI only when an explicit JSON input/output contract is reviewed; the CLI belongs to the early release roadmap, not the M0 acceptance gate. Optional Python bindings may be evaluated after a genuine Python integration need. Do not reject a mature C/C++ solver later merely for being native; assess FFI cost, licences, platform builds, and whether the solver solves a demonstrated class of problems.

## Consequences

- M0 can be built with the Rust standard library and few or no runtime dependencies; this simplifies cross-platform CI and ownership review.
- Rich mathematical prototyping may be slower than Python; use independently generated public fixtures or a throwaway solver comparison, then keep the production core in Rust unless evidence changes the decision.
- Backend integration should not be exposed through the stable public API before there is a proven need for multiple backends.
- Library licensing is undecided; proprietary distribution remains possible. Dependency licences must permit the selected distribution mode.

## Revisit when

Representative multi-layer or temporal fixtures cannot be solved acceptably with Rust-native crates or a maintainable Rust FFI to a mature solver; or a Python-heavy consumer justifies bindings.
