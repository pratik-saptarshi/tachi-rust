# Tachi-Rust CI Route Fixtures

**Status**: live RT-CI fixture manifest draft
**Purpose**: define the common change-set matrix used to prove route decisions

## Fixture Matrix

| Fixture | Expected route | Notes |
|---|---|---|
| docs-only | observe_only | Passive docs can narrow only when the contract surface stays untouched. |
| active-docs | full | Roadmap, standards, guide, BOM, and publish-gate docs keep full validation. |
| Rust crate | full | Active Rust crate work keeps the current validation breadth. |
| dependency-closure | full | Crate-local changes stay full until downstream closure routing is enabled. |
| UI | full | Desktop and UI-facing changes keep the broader contract set until route proofs are stable. |
| shared-surface | full | README, changelog, security, workflow, and root-manifest changes stay full. |
| workflow | full | Workflow edits always widen to full mode. |
| lockfile | full | Lockfile drift always widens to full mode. |
| release-mainline | full | Main, release, and tag contexts always widen to full mode. |
| aod | full | Shared agent-workflow surfaces under `.aod/` remain full-mode inputs. |
| mixed | full | Mixed change sets default to the safest route. |
| unknown-file | full | Unknown paths never narrow coverage. |

## Stable JSON Shape

The route output is treated as stable JSON with the following fields:

```json
{
  "route": "full",
  "fallback reason": "unknown, incomplete, or parse-failed inputs widen to full mode"
}
```

```json
{
  "route": "observe_only",
  "fallback reason": "docs-only passive paths may narrow only when the active contract surface is not touched"
}
```

## Notes

- The matrix must stay easy to extend when new workspaces or required checks are added.
- The fixture set intentionally includes at least one full-mode escalation and one narrowed observe-only case.
- Active-doc, shared-surface, dependency-closure, `.aod`, and release/mainline
  paths remain full-mode proofs until the execution phase promotes them.
