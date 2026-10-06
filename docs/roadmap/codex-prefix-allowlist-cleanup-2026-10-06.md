# Codex Prefix Allowlist Cleanup Plan — 2026-10-06

Status: **Plan prepared; the global Codex rules file has not been changed.**

## Purpose

Clean up historical command-prefix permissions without broadening access, then
add a recurring check so stale rules are removed when tools or repositories
move. Keep this maintenance separate from application runtime changes.

## Confirmed baseline

Read-only inspection found 24 rules in
`/Users/neo/.codex/rules/default.rules`. Eighteen refer to
`/usr/local/bin/lean-ctx`, and that executable path currently does not exist.
The other six rules cover deletion of `node_modules`, `tmp`, and three named
snapshot files; changing to `/tmp`; installing requirements into a local Python
virtual environment; and changing to an old browser add-on checkout. Review all
six individually rather than assuming they are still useful or automatically
stale.

Historic rule text also names archived or temporary `stealth-lightbeacon`
checkouts. Before removal, use the corresponding Codex session history to map
each rule to its original task, intended command, and any current replacement.
No allowlist entry should be retained solely because it appears in an old
session.

## Phased cleanup

| Phase | Work | Exit criteria |
|---|---|---|
| **1 — Snapshot and inventory** | Save a dated backup of `default.rules`. Produce a table of all 24 patterns with rule number, executable, repository/path target, destructive scope, and current purpose. | Every entry has an explicit disposition candidate; baseline count and file checksum are recorded. |
| **2 — Historic-use review** | Search only relevant Codex sessions for the old `lean-ctx` invocations and project paths. Check whether the executable or exact project still exists, whether an equivalent supported command is used now, and whether the rule has a current owner. | Each of the 18 `lean-ctx` entries is confirmed obsolete or linked to a concrete migration need. Each of the six other entries has a keep, narrow, or remove decision. |
| **3 — Prune and narrow** | Remove confirmed dead entries. For any still-needed permission, replace it with the narrowest current executable and exact command prefix. Review recursive deletion and install permissions separately; do not replace them with broad wildcards. | No rule points to a missing executable or archived checkout without an active owner. No replacement permission is broader than the rule it replaces. |
| **4 — Validate and record** | Validate the rules file with the installed Codex rules parser or a non-executing Codex dry run. Exercise representative matching and non-matching command strings in a safe test harness. Review the final diff and record before/after counts. | Codex loads the file; retained rules match only intended prefixes; rejected and unrelated commands remain outside the allowlist; backup supports rollback. |

## Recurring pruning step

Add this step to Codex configuration maintenance **monthly and whenever an
allowed executable is installed, removed, renamed, or moved**:

1. Enumerate every `prefix_rule` and record its executable, purpose, and last
   relevant session evidence.
2. Verify the executable and any repository/path literals still exist.
3. Remove entries with missing executables, archived repositories, expired
   tasks, or no current owner; narrow any rule whose command scope grew.
4. Validate the rules file without executing destructive allowlisted commands.
5. Record the date, rule count before and after, removed and retained entries,
   and the validation result.

Do not silently keep stale entries when a check cannot establish current need.
Retain a dated backup before each cleanup so a mistaken prune is reversible.

## Completion checklist

- [ ] The 24 current entries are inventoried against relevant session history.
- [ ] All 18 missing-`lean-ctx` entries are removed or tied to an explicit,
  current migration requirement.
- [ ] All six other entries are individually reviewed for path freshness and
  least privilege.
- [ ] Codex accepts the revised file, match tests pass, and no missing tool or
  retired project paths remain.
- [ ] The recurring pruning step is added to the Codex maintenance checklist
  and has a named owner and monthly cadence.
- [ ] Before/after counts, backup location, diff, and validation are recorded.

No rule edits are included in this plan; cleanup should be a separately reviewed
change to the global Codex configuration.
