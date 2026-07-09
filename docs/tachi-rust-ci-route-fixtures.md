# Tachi-Rust CI Route Fixtures

**Status**: live RT-CI fixture manifest draft
**Purpose**: define the common change-set matrix used to prove route decisions

## Fixture Matrix

| Fixture | Expected route | Notes |
|---|---|---|
| docs-only | observe_only | Passive docs can narrow only when the contract surface stays untouched. |
| Rust crate | full | Active Rust crate work keeps the current validation breadth. |
| UI | full | Desktop and UI-facing changes keep the broader contract set until route proofs are stable. |
| workflow | full | Workflow edits always widen to full mode. |
| lockfile | full | Lockfile drift always widens to full mode. |
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
