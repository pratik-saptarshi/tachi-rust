# Codex Command Allowlist Cleanup and Pruning Roadmap — 2026-10-06

## Purpose and bounded evidence

Historic session context reports 24 Codex command allowlist rules, of which 18
refer to a binary that no longer exists. Prefix allowlists can accumulate stale
entries without failing visibly. The rule source files and their paths were not
provided in this request, so the count and affected entries are user-reported
until Phase 0 locates the authoritative files and verifies each mapping.

The cleanup will remove confirmed dead entries, require a specific reason for
every surviving rule, and add recurring pruning so executable lifecycle changes
cannot leave allowlists stale indefinitely.

## Beads hierarchy

Epic **RT-fwt** tracks this work. Its children are:

| Beads | Phase | Scope | Depends on |
|---|---|---|---|
| **RT-fwt.1** | 0 — Source discovery and inventory | Locate authoritative rules from historic session context; map all 24 reported entries to commands, binaries, owners, purposes, and evidence. Confirm or correct the reported 18 dead entries. | — |
| **RT-fwt.2** | 1 — Cleanup and narrow matching | Remove entries proven obsolete; reduce surviving rules to exact command mappings with owner, purpose, last-verified date, and expiry. | RT-fwt.1 |
| **RT-fwt.3** | 2 — Recurring pruning gate | Add automated validation and a quarterly review, plus an immediate review when an executable is renamed, removed, or retired. | RT-fwt.2 |

The generated issue IDs and acceptance criteria are synchronized in
`.beads/issues.jsonl`. Each issue records its RED, GREEN, and regression checks.

## Phases, PR boundaries, and exit criteria

### Phase 0 — Find and verify the rule sources

Trace historic session references to the authoritative Codex rule files and
record the exact path, entry, command prefix, target executable, owner, purpose,
last verification, and current executable resolution for every reported rule.
Treat the 18 dead entries as hypotheses until each target is independently
checked. Record ambiguous or undocumented rules for owner disposition.

**PR boundary:** inventory and source references only. Keep any rule changes
out of this PR. Exit when the inventory accounts for all 24 reported entries
or explicitly explains a corrected count and every unresolved mapping has an
owner.

**Validation:** replay the inventory against the current executable/tool
catalog and compare the rule count, source paths, and obsolete-entry list.

### Phase 1 — Remove stale rules and constrain survivors

Delete only rules verified to target retired or unavailable commands. Keep a
rule only when its command remains supported and its owner confirms its need.
Replace broad prefix matching with exact executable/subcommand patterns.
Require an owner, purpose, last-verified date, and expiry of no more than 90
days for each retained entry.

**PR boundary:** allowlist edits and focused validation. Exit when no verified
dead rule remains and every survivor is justified, exact, owned, and current.

**Validation:** negative checks reject a removed binary, unknown executable,
unbounded prefix, missing metadata, and expired entry. Positive checks prove
each surviving allowlisted command still resolves and behaves as intended.

### Phase 2 — Add scheduled and lifecycle-triggered pruning

Add a machine-checkable gate that checks command resolution, match scope,
ownership, rationale, review date, and expiry. Run it in CI and on a quarterly
schedule. Require the same pruning review whenever a referenced binary or
command is renamed, removed, or retired. Emit a diffable report of additions,
removals, and owner decisions.

**PR boundary:** pruning validator, fixtures, schedule, and operator instructions.
Exit when an overdue or stale rule fails the gate and a valid exact rule passes.

**Validation:** test invalid binary, prefix wildcard, missing owner/purpose,
expired entry, due quarterly review, and lifecycle-triggered review. Confirm a
valid rule passes and the report captures the rule-set change.

### Phase 3 — Integrated closeout

Reconcile the final rule inventory, source files, automation, tests, Beads
export, and this roadmap. Preserve the before/after counts and the proof for
each removed or retained entry. Do not claim the reported 18 deletions unless
Phase 0 verifies that exact count.

**PR boundary:** closeout documentation after the cleanup and pruning gate have
merged. Close **RT-fwt** only after all child acceptance criteria pass and the
scheduled pruning owner/cadence is active.

## Delivery controls

- Use an isolated branch/worktree for each milestone and keep the main checkout
  and `.beads.gate.lock` untouched.
- Make separate Conventional Commit slices for inventory, cleanup, pruning,
  and closeout. Open a separate PR for each major milestone.
- Enable auto-merge only after required checks are terminal and successful,
  review threads are resolved, and GitHub reports the PR mergeable. If auto-merge
  is rejected as unstable, wait for a stable state and use the normal protected
  merge path; never bypass repository protection.
- Resolve review comments on the branch attached to the affected PR.

## Current state

Planning and tracker setup only. **RT-fwt**, **RT-fwt.1**, **RT-fwt.2**, and
**RT-fwt.3** are open. No Codex allowlist source has yet been inspected, no rule
has been changed, and no cleanup or pruning validation has been executed.
