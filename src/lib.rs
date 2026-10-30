//! FrameMotive will compute editable crop and layout adaptation decisions from
//! caller-supplied geometry.
//!
//! Given a source composition and layout constraints, the crate will produce
//! structured decisions for a target canvas, together with diagnostics when a
//! constraint cannot be satisfied. It works on geometry and metadata only: it
//! does not analyse media, render pixels, or edit timelines.
//!
//! # Status
//!
//! No operation is implemented yet, and this crate exposes no public API. The
//! planned work is described in the repository's `roadmap.md`, and the accepted
//! design in `architecture.md` and `decisions/`.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
