# Tachi-Rust CI Route Fixtures

**Status**: live RT-CI route classifier fixtures
**Purpose**: define the common change-set matrix used to prove route decisions

## Fixture Matrix

| Fixture | Expected route | Notes |
|---|---|---|
| docs-only | passive_docs_only | Passive docs can narrow only when the contract surface stays untouched. |
| active-docs | full_pr_matrix | Roadmap, standards, guide, BOM, and publish-gate docs keep full validation. |
| Rust crate | dependency_closure | Crate-local changes select the complete downstream package closure. |
| dependency-closure | dependency_closure | The route artifact records the selected package set for the crate-local diff. |
| UI | full | Desktop and UI-facing changes keep the broader contract set until route proofs are stable. |
| shared-surface | full_pr_matrix | README, changelog, security, workflow, and root-manifest changes stay full. |
| workflow | full_pr_matrix | Workflow edits always widen to full mode. |
| lockfile | full_pr_matrix | Lockfile drift always widens to full mode. |
| release-mainline | full_pr_matrix | Direct main, release, and tag refs stay full; PR base alone does not escalate. |
| aod | full_pr_matrix | Shared agent-workflow surfaces under `.aod/` remain full-mode inputs. |
| mixed | full_pr_matrix | Mixed change sets default to the safest route. |
| unknown-file | full_pr_matrix | Unknown paths never narrow coverage. |
| scheduled/security/canary | full_pr_matrix | Scheduled and security/coverage lane contexts stay full coverage. |

## Stable JSON Shape

The route output is treated as stable JSON with the following fields:

```json
{
  "mode": "passive_docs_only",
  "reason": "docs-only passive paths observed",
  "packages": ["tachi-core", "tachi-mcp", "tachi-cli", "tachi-shell", "tachi-desktop"],
  "changed_paths": ["docs/reference/cli-usage.md"],
  "policy_version": "2026-10-05"
}
```

## Notes

- The matrix must stay easy to extend when new workspaces or required checks are added.
- The fixture set intentionally includes full-mode escalation, passive-docs
  narrowing, and dependency-closure selection.
- The protected base branch of a pull request does not by itself force full
  mode; active-contract and shared-surface changes still do.
- Direct pushes to protected refs, active-doc, shared-surface, `.aod`, unknown,
  and release/mainline paths remain full-mode proofs.
