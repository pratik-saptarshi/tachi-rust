# ADR-046: Async Runtime Adoption Boundary

**Status**: Accepted
**Date**: 2026-07-05
**Feature**: RT-TC-006
**Cross-references**: Rust Toolchain Upgrade roadmap, RT-TC-001, RT-TC-004

## Context

The Rust toolchain modernization track pins compiler, Cargo, lint, coverage,
and supply-chain behavior for the active workspace. The same review also noted
that `smol-rs` publishes useful async runtime primitives, including `smol`,
`async-channel`, `blocking`, `async-io`, `polling`, and `async-task`.

Those crates may be relevant to a future MCP transport or desktop-host async IO
feature, but adopting them during the compiler and CI hardening track would
expand the blast radius from reproducible builds into runtime behavior.

The current track already adopted selected `taiki-e` CI tooling through
`cargo-hack` and `cargo-llvm-cov` canaries. That tooling decision is separate
from any runtime dependency decision.

## Decision

The Rust toolchain modernization track does not adopt `smol-rs` runtime crates
or replace the current async/runtime model.

Any future `smol-rs` evaluation must be opened as a separate MCP or desktop
async-runtime feature with its own ADR before dependencies are added.

## Required Future Evidence

A future async-runtime ADR must include:

1. A concrete feature or failure mode that requires runtime change.
2. Benchmark evidence for the affected path.
3. Cancellation and shutdown regression tests.
4. Compatibility evidence for MCP and desktop command dispatch behavior.
5. A dependency diff showing the proposed runtime crates and transitive impact.
6. A rollback plan that restores the previous runtime behavior.

## Boundaries

- Toolchain pinning, supply-chain gates, semantic CI tests, feature canaries,
  and coverage canaries must not add `smol-rs` runtime dependencies.
- `taiki-e` CI-tool adoption remains a CI confidence decision, not precedent
  for runtime-crate adoption.
- Manual/scheduled canaries may measure runtime-sensitive behavior, but they do
  not authorize runtime dependency changes.
- The retired `src-tauri` adapter does not authorize runtime dependency
  changes; any future desktop async work still needs a separate issue and ADR.

## Alternatives Considered

### Adopt `smol-rs` during the toolchain upgrade

**Pros**:
- Could simplify a future async implementation if the runtime change proves
  useful.
- Would evaluate runtime crates while CI is already being hardened.

**Cons**:
- Mixes compiler reproducibility with runtime behavior changes.
- Requires benchmark, cancellation, shutdown, and compatibility evidence that
  the toolchain track does not otherwise need.
- Risks masking runtime regressions as compiler or dependency-update issues.

**Why Not Chosen**: The toolchain track needs a small, auditable blast radius.
Runtime replacement is a separate architectural decision.

### Ban future `smol-rs` adoption

**Pros**:
- Avoids dependency churn.
- Keeps runtime governance simple.

**Cons**:
- Over-constrains future MCP or desktop features.
- Prevents evidence-based evaluation if a concrete async IO need appears.

**Why Not Chosen**: The correct decision is deferral with evidence gates, not a
permanent ban.

## Consequences

### Positive

- Compiler and CI hardening stay reproducible and narrowly scoped.
- Future runtime work has explicit benchmarks and shutdown/cancellation tests.
- Dependency review can separate CI tools from runtime crates.

### Negative

- Future async-runtime work will require a new ADR before implementation.
- Potential `smol-rs` benefits are not realized in the current track.

### Mitigation

- Keep this ADR linked from the toolchain roadmap and publish checklist.
- Require dependency-diff review before any async-runtime feature lands.
- Keep MCP and desktop transport tests as the acceptance surface for any future
  runtime experiment.

## Validation

- `rg "smol|async-runtime|ADR-046" docs/architecture/02_ADRs docs/roadmap docs/bill-of-materials.html.md docs/publish-readiness-checklist.html.md codemap.md`
- `cargo metadata --locked --format-version 1`

## References

- `docs/roadmap/2026-07-05-rust-toolchain-upgrade-roadmap.html.md`
- `docs/roadmap/2026-07-05-rust-toolchain-upgrade-issue-cards.md`
