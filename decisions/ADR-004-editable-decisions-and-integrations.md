# ADR-004: Tool-neutral geometric decisions, not a new timeline

- **Status:** proposed for owner review
- **Date:** 2026-10-27

## Context

Users need results they can adjust later. A rendered output cannot express how a crop or layer placement was chosen. Editors differ in project schemas and supported effects. [OpenTimelineIO](https://opentimelineio.readthedocs.io/en/latest/) models editorial timing and interchange, but it is not a universal representation for every spatial transform or effect.

## Alternatives

1. Emit only cropped media: easy consumption, loses editability.
2. Adopt one editor's project format as the core model: quick first integration, vendor coupling and version churn.
3. Define a complete new multimedia interchange standard: much larger scope and overlaps mature timeline work.
4. Emit narrow, versioned geometric edit decisions and translate them through adapters.

## Proposed decision

The core result is a typed geometric decision with explicit source/target spaces and diagnostics. M0 requires no serialized public standard, editor project writer, or renderer. Once a consumer exists, define a small versioned interchange document for supported operations and retain an explicit `unsupported/approximated` report for adapters. An adapter is responsible for applying and re-reading operations in its target editor; the core remains unaware of that editor. Reuse OTIO where editorial context is needed and existing media engines for decoding, rendering, and encoding.

## Consequences

- Editability is a capability to **verify** through save/reopen and operation inspection, not a claim from having JSON.
- An adapter may preserve structural operations but produce different pixels due to color, resampling, font, or compositing differences. Fidelity reporting should distinguish structure from rendered equivalence.
- An initial CLI can serialize only the supported M0 request and result, with a schema version; premature general timeline objects are excluded.

## Revisit when

A target editor directly accepts a common public operation model covering the needed transforms, or two adapters expose genuinely shared interchange semantics that should be standardized.
