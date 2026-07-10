# Tachi-Rust CI Route Artifact

**Status**: live RT-CI artifact schema draft
**Purpose**: define the observable route decision payload emitted by the
observe-only CI lane

## Payload Fields

- `mode`: the route mode used for the PR, usually `observe_only` until route
  enforcement is promoted
- `changed_paths`: the list of changed repository paths considered by the
  router
- `selected_lanes`: the predicted lane set for the change shape
- `escalation_reasons`: human-readable reasons that forced or preserved full
  mode
- `policy_version`: the policy version used to make the route decision

Protected refs such as `main`, `release/*`, and tags always emit a full-mode
reason instead of a narrowing mode.

## Stable Check

The observe-only workflow job `route-observe` remains the stable orchestrator check until specialist routing becomes enforcement.

## Example

```json
{
  "mode": "observe_only",
  "changed_paths": ["docs/guide.md"],
  "selected_lanes": ["docs-pr-gate", "specialist-guards"],
  "escalation_reasons": ["docs-only passive paths observed"],
  "policy_version": "2026-07-09"
}
```
