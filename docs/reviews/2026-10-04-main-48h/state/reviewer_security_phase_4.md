# Phase 4 private reflection — Security Auditor

Reviewed only my Phase 3 artifact, relevant source, and the Phase 4 protocol. No other reviewer files read. Pinned source remains `8df554e884b1e5dd24146111a965597eff5f4779`.

## Confidence Ratings

- **SEC-1: High; retain P2 EXISTING_DEFECT.** The direct pinned-source reproduction demonstrates deletion of the only backing file, with the selected image path then dangling. Re-reading assets.rs:43-51 confirms byte equality is insufficient to prove the retained pathname survives unlink. Existing input symlink rejection is exclusive to catalog regeneration, which is not the report-data cleanup path. The flag is opt-in and local; those conditions limit exposure but do not authorize loss of the only image. A malformed signature-only fixture does not undermine the unlink mechanism, though a full-image fixture would make the regression test stronger.
- **SEC-2: High for wrong filesystem selection, Medium for confidentiality impact; retain P2 EXISTING_DEFECT with bounded wording.** Pinned CLI output proves parent traversal selects a neighboring SVG and emits has-image=true. report_data.rs:232-244 has only is_file; the parser has no applicable ID validation for the demonstrated heading branch. However, disclosure requires a later compile and sharing step, and the neighboring image must be inside the configured Typst root. The application is a trusted local generator; no remote request boundary has been established. The core actionable defect is accepting a path as a finding identifier and binding an unrelated image. Do not promote this into arbitrary-file read or P1 without stronger evidence.

## Most/Least Defensible

**Most defensible: SEC-1.** Concrete data loss is already demonstrated by actual pinned code. There is no separate compiler, service deployment, race, or malicious concurrent writer assumption. Symlinked aliases are unusual but valid filesystem inputs, and preservation of a retained copy is the purpose of the equality guard.

**Least defensible: SEC-2's downstream disclosure characterization.** Wrong path selection is solid. Whether the target report directory is an intended confidentiality boundary is less explicit than the cleanup preservation contract. Typst root confinement protects paths outside its root; if an adopter deliberately regards all report Markdown and every image under that root as equally trusted, the impact narrows to malformed-input report contamination. Keep both that condition and the suffix restriction prominent.

## Changes

No severity or score changes: 7.5/10, two P2 findings, no demonstrated P0/P1. Added literal `## Findings` above SEC-1 in Phase 3 solely to repair the requested schema. No other Phase 3 content changed. No new issue added and no repeated runtime checks needed: source reinspection supports the previous terminal reproductions.

For SEC-1, the simplest robust fix is to reject symlink counterparts during cleanup. For SEC-2, ID grammar validation and canonical image-directory containment should be separate protections; validating only slashes leaves symlink aliases, while containment alone still accepts nonsensical finding IDs. These are targeted fixes rather than grounds for claiming a complete sandbox.

## Remaining Uncertainty

No full Typst compile of the private-image fixture, database runtime/RLS replay, browser session-refresh test, or production consumer was available. The public primary-source Typst documentation establishes the outer project-root boundary, but not a guarantee that every adopter chooses the same root. Exact metadata and stale/malformed finding handling deserve correctness review; I have not generalized them into new security vulnerabilities. Existing Prisma server client is unused by inspected scaffold application code, so RLS bypass through hypothetical future callers remains excluded. Await debate before revising conclusions from other reviewers' evidence.
