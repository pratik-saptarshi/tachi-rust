# Tachi-Rust CI Route Policy

**Status**: live RT-CI manifest draft
**Purpose**: encode the current full-mode escalation rules before any route narrowing is enforced

## Full-Mode Escalations

- main, release refs, tags, lockfiles, workflow files, and unknown routes force full mode.
- active docs, shared surfaces, dependency-closure changes, and release/mainline
  contexts force full mode.
- docs-only passive paths may narrow only when the active contract surface is not touched.
- observe-only routing must publish an explanation before any narrowing is enforced.
- unknown, incomplete, or parse-failed route inputs must widen to full mode.

## Route Classes

- Passive docs: docs-only changes that do not touch active contract surfaces.
- Active docs: roadmap, standards, guide, BOM, publish-gate, and route-policy
  docs that must stay on full mode.
- Shared surfaces: `README.md`, `CHANGELOG.md`, `SECURITY.md`, `.aod/`, `.claude/`,
  `Makefile`, `Cargo.toml`, `Cargo.lock`, workflow files, adapters, and crate roots.
- Dependency closure: changed crate roots stay on full mode until the downstream
  closure lane is promoted in the execution phase.
- Release/mainline: `main`, release refs, and tag contexts always stay on full mode.

## Notes

- Route decisions are advisory until the observe-only proof is stable.
- Any safety-sensitive ambiguity should widen coverage, never narrow it.
- This manifest is the human-readable source for the RT-CI route-policy contract and its follow-on fixture tests.
