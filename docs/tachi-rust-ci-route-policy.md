# Tachi-Rust CI Route Policy

**Status**: live RT-CI policy
**Purpose**: encode the current route classification and full-mode escalation rules

## Full-Mode Escalations

- Direct runs on `main`, release refs, and tags force full mode. A pull request
  targeting a protected base branch is classified from its changed paths; the
  base branch alone does not force full mode.
- Lockfiles, workflow files, and unknown routes force full mode.
- Scheduled release/security/canary lanes (including scheduled coverage/audit jobs) must
  remain full mode and may not be narrowed by route shape.
- Active docs and shared surfaces force full mode.
- docs-only passive paths may narrow only when the active contract surface is not touched.
- The observe-only artifact publishes the same classification used by the enforced Rust workflow.
- unknown, incomplete, or parse-failed route inputs must widen to full mode.

## Route Classes

- Passive docs: docs-only changes that do not touch active contract surfaces.
- Active docs: roadmap, standards, guide, BOM, publish-gate, and route-policy
  docs that must stay on full mode.
- Shared surfaces: `README.md`, `CHANGELOG.md`, `SECURITY.md`, `.aod/`, `.claude/`,
  `Makefile`, root `Cargo.toml`, `Cargo.lock`, workflow files, and adapters.
- Dependency closure: crate-local changes run the impacted crate and its
  documented downstream package closure.
- Release/mainline: direct execution on `main`, release refs, and tag contexts always stays on full mode.

## Notes

- The enforced Rust workflow and observe-only artifact use the same shared
  classifier so evidence reflects the route that would execute.
- Route decisions remain fail-closed for active contracts, shared surfaces,
  protected execution refs, and ambiguous inputs.
- Any safety-sensitive ambiguity should widen coverage, never narrow it.
- This manifest is the human-readable source for the RT-CI route-policy contract and its follow-on fixture tests.
