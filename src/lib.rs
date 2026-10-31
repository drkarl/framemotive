//! FrameMotive will compute constraint-aware static crop decisions from
//! caller-supplied geometry.
//!
//! Given a source canvas, the regions that must stay visible and one or more
//! target canvases, the crate will produce a verified crop decision per target,
//! or an explicit infeasibility report with measured evidence. It works on
//! geometry and metadata only: it does not analyse media, render pixels, or
//! edit timelines.
//!
//! # Status
//!
//! No operation is implemented yet, and this crate exposes no public API. The
//! planned work is described in the repository's `roadmap.md`, and the accepted
//! design in `architecture.md` and `decisions/`.
