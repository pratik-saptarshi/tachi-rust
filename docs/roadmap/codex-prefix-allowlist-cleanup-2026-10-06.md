# Codex Prefix Allowlist Cleanup and Pruning Roadmap — 2026-10-06

Status: **Planning and tracker setup only; the global Codex rules file has not
been changed.** This is the single active plan for the allowlist cleanup. It
extends the baseline first recorded in PR #63 and is tracked by the fresh
Beads epic **RT-fwt**.

## Confirmed baseline and scope

The existing read-only inventory records 24 rules in
`/Users/neo/.codex/rules/default.rules`. Eighteen refer to the currently missing
`/usr/local/bin/lean-ctx`. The other six cover deletion of `node_modules`,
`tmp`, and three named snapshot files; changing to `/tmp`; installing
requirements into a local Python virtual environment; and changing to an old
browser add-on checkout. Historic rule text also names archived or temporary
`stealth-lightbeacon` checkouts. These are the baseline facts from the prior
inventory; Phase 0 rechecks the live file, current binaries and relevant
session history before making changes. If the file or count has changed, record
the current count and reconcile it with this dated snapshot rather than forcing
the old number.

Keep this personal Codex configuration separate from application runtime
changes. The canonical file is operator-local and is not present in a normal
repository checkout. A hosted GitHub job therefore cannot validate the real
file. Hosted CI may test a parser against fixtures; the actual rules audit and
Codex dry run must execute on the machine that owns the global file.

## Beads worklist

| Beads | Phase | Work | Dependencies |
|---|---|---|---|
| **RT-fwt** | Epic | Complete cleanup and establish recurring pruning. | — |
| **RT-fwt.1** | 0 — Snapshot, inventory, and historic-use review | Reconcile the live file with the 24/18 baseline; map each rule to its executable, path, destructive scope, purpose, last session evidence, and disposition. | — |
| **RT-fwt.2** | 1 — Prune and narrow | Remove confirmed obsolete rules; individually keep, narrow, or remove the other six; retain only justified exact prefixes. | RT-fwt.1 |
| **RT-fwt.3** | 2 — Validate and add recurring pruning | Add a local read-only audit and monthly/lifecycle-triggered maintenance step; use hosted CI for fixture tests only. | RT-fwt.2 |

Generated issue IDs and their RED, GREEN, and regression acceptance criteria
are synchronized in `.beads/issues.jsonl`.

## Phases, PR boundaries, and validation

### Phase 0 — Snapshot, inventory, and historic-use review (RT-fwt.1)

Create a dated backup and checksum of `default.rules`. Reconcile the active
file with the recorded 24-rule snapshot. Inventory every current entry by rule
number, executable, repository/path target, destructive scope, and purpose.
Use only relevant Codex session history to identify why old commands and
`stealth-lightbeacon` paths were allowed, and whether a supported replacement
or current owner exists. The 18 `lean-ctx` entries are expected to be stale
because their target executable is missing, but classify each from evidence.
Give each of the other six a separate keep, narrow, remove, or owner-decision
disposition.

**PR boundary:** inventory and roadmap updates only. Do not edit the global
rules file in this PR. **Exit:** all live rules reconcile to the source file,
baseline drift is explained, and every entry has evidence and a disposition.

**Validation:** independently replay the inventory and compare source path,
checksum, current entry count, executable-resolution results, and dispositions.

### Phase 1 — Prune and narrow (RT-fwt.2)

Remove rules confirmed to reference missing executables, archived checkouts,
expired tasks, or permissions without a current need. For each retained
permission, use the narrowest current executable and exact command prefix.
Review recursive deletion and install permissions separately. Do not retain a
rule only because it appeared in an old session, and do not broaden a rule to
make a replacement command pass.

**PR boundary:** stage the separately reviewed change to the global Codex
configuration and its inventory receipt. **Exit:** no entry targets a missing
binary or inactive path without a named active owner; every retained rule has
a verified use and is no broader than the baseline permission.

**Validation:** use Codex's installed rules parser or a non-executing dry run.
Exercise intended and unrelated command strings in a safe test harness. Record
before/after counts and review the complete diff before replacing the backed-up
file. Never exercise destructive allowlisted commands as a test.

### Phase 2 — Local audit and recurring pruning (RT-fwt.3)

Add a read-only validator that accepts the canonical rules path as an explicit
input (defaulting to `/Users/neo/.codex/rules/default.rules` on the owner
machine). It reports missing executables, stale repository paths, broad or
unbounded prefixes, missing ownership/purpose, and entries without a recent
verification date. It must not edit rules automatically. Add fixture cases so
the validator can be tested in hosted CI without exposing or pretending to
read the operator-local file.

Run the actual audit **monthly and whenever an allowed executable is
installed, removed, renamed, or moved**. The operator maintenance checklist
must name the owner and include these steps:

1. Enumerate each `prefix_rule`; record its executable, purpose, and last
   relevant session evidence.
2. Verify the executable and repository/path literals on the owner machine.
3. Remove entries with missing executables, archived repositories, expired
   tasks, or no current owner; narrow any rule whose scope grew.
4. Validate the rules file without executing destructive allowlisted commands.
5. Record the date, source checksum, rule count before and after, dispositions,
   and validation result; keep a dated backup before edits.

Do not silently keep an entry when the validator cannot establish current
need. The actual local audit, rather than a hosted fixture run, is the gate for
claiming that the global file is clean.

**PR boundary:** validator fixtures and the documented owner procedure are a
repository PR; local scheduling/maintenance configuration is reviewed on the
owner machine. **Exit:** stale, missing-binary, unbounded, unowned, and overdue
entries fail the audit; a valid exact rule passes; monthly and lifecycle
triggers are assigned to an owner.

**Validation:** hosted fixture tests cover missing executable, stale path,
unbounded prefix, missing owner/purpose, overdue review, and valid exact rule.
On the owner machine, run the validator against the actual global file and
record its checksum and result. Confirm the Codex parser accepts the revised
file and rollback from the dated backup is documented.

### Phase 3 — Integrated closeout

Reconcile the live rules file, dated backup, inventory, validator output,
Beads export, and this roadmap. Close **RT-fwt** only after all three child
acceptance criteria pass, the actual owner-machine audit is clean, and the
monthly pruning owner and lifecycle trigger are active.

**PR boundary:** a separate closeout PR after the cleanup and validator
milestones. Record the verified before/after counts and do not claim all 18
entries were removed unless the final inventory confirms that disposition.

## Delivery controls

- Use isolated branches/worktrees for repository milestones. Keep the main
  checkout and `.beads.gate.lock` untouched.
- Use separate Conventional Commit and PR slices for inventory, cleanup,
  validator, and closeout. Resolve review comments on the branch attached to
  each PR.
- Enable auto-merge only after required checks are terminal and successful,
  review threads are resolved, and GitHub reports the PR mergeable. If GitHub
  rejects auto-merge as unstable, wait for a stable state and use the normal
  protected merge path. Never bypass protection.

## Current state

**RT-fwt**, **RT-fwt.1**, **RT-fwt.2**, and **RT-fwt.3** are open. The plan
records a prior read-only baseline; this turn has not inspected or changed the
operator-local rules file, created a backup, run a Codex parser, or executed
the cleanup/pruning audit.
