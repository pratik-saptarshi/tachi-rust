# Tachi-Rust CI Route Policy

**Status**: live RT-CI manifest draft
**Purpose**: encode the current full-mode escalation rules before any route narrowing is enforced

## Full-Mode Escalations

- main, release refs, tags, lockfiles, workflow files, and unknown routes force full mode.
- docs-only passive paths may narrow only when the active contract surface is not touched.
- observe-only routing must publish an explanation before any narrowing is enforced.
- unknown, incomplete, or parse-failed route inputs must widen to full mode.

## Notes

- Route decisions are advisory until the observe-only proof is stable.
- Any safety-sensitive ambiguity should widen coverage, never narrow it.
- This manifest is the human-readable source for the RT-CI route-policy contract and its follow-on fixture tests.
