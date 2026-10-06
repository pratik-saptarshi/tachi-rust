# Tachi-Rust CI Route Artifact

**Status**: live RT-CI artifact schema
**Purpose**: define the observable route decision payload emitted by the
observe-only CI lane

## Payload Fields

- `mode`: `observe_only` for this artifact-producing workflow
- `route_mode`: the shared classifier's enforced or predicted mode:
  `passive_docs_only`, `dependency_closure`, or `full_pr_matrix`
- `route_reason`: the classifier's human-readable reason
- `changed_paths`: the list of changed repository paths considered by the
  router
- `selected_lanes`: the predicted lane set for the change shape
- `selected_packages`: the package matrix selected for dependency-closure
  changes; empty for passive-doc and full-matrix routes
- `repository_contracts_required`: whether workspace manifest, toolchain, or
  contract-policy inputs require the compact repository-wide contract job
- `escalation_reasons`: human-readable reasons that forced or preserved full
  mode
- `policy_version`: the policy version used to make the route decision

Direct execution on refs such as `main`, `release/*`, and tags always emits a
full-mode reason. Pull requests to `main` are classified by changed paths; a
protected base branch alone does not force full mode. The artifact and enforced
workflow use the same classifier.

The `rust-workspace.yml` route job uploads its own `route-decision` artifact
for every event. Alongside classifier fields, it records `run_id`,
`run_attempt`, `event`, PR number, source `head_sha`, `workflow_sha`, exact
`classifier_revision` and `path_producer_revision` blob IDs,
`runner_definition` (including the hosted image version), and
`force_full_requested`. The read-only matched-control collector joins those
fields with workflow timestamps and the Git tree SHA resolved from `head_sha`.
A valid control must be a successful
`workflow_dispatch` run with `force_full_requested: true` and
`route_mode: full_pr_matrix`.

## Stable Check

The observe-only workflow job `route-observe` remains the stable orchestrator check until specialist routing becomes enforcement.

## Example

```json
{
  "mode": "observe_only",
  "route_mode": "passive_docs_only",
  "route_reason": "docs-only passive paths observed",
  "changed_paths": ["docs/guide.md"],
  "selected_lanes": ["docs-pr-gate", "specialist-guards"],
  "selected_packages": [],
  "repository_contracts_required": false,
  "escalation_reasons": [],
  "policy_version": "2026-10-06"
}
```
