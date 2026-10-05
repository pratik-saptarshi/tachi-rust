# Phase 6 — Round 1 summary

## Resolved this round
- Risk-table silent replacement is reproduced (COR-1/ARC-1); Architecture lowers P1 to P2 based on malformed-input trigger and absent ordinary-workflow exposure evidence.
- Group COR-2 and DA-1 as incomplete controls assessment selection, preserving both distinct triggers: unconsumed populated flat residual table, and control metadata without residual completion evidence.
- Deduplicate COR-3/ARC-2 (invalid attack image bypasses text fallback). PDF compilation remains unperformed; generated bad binding is reproduced.
- SEC-1 cleanup symlink and SEC-2 image path traversal remain separate bounded P2 issues, with local-input/project-root limitations.
- CQ-2 documented relationship default is independently reproduced by Security and Code Quality: nested omitted relationship errors, flat form succeeds.
- PIPE-1 Unix-only import remains a P2 test-target compilation defect, no Windows execution claim.
- CQ-1 historical taxonomy, DA-2 companion integrity and PIPE-2 stale executable provenance are downgraded to advisories/P3 by their original reviewers after checking narrower contracts. No claim that committed PDFs are currently corrupt.

## Still in dispute
One material priority dispute remains: DA proposes P1 for successful zero-finding output from partial controls; the other five prefer P2 given malformed/incomplete input and unmeasured normal-workflow prevalence. Mechanism is agreed. The relevant gate (`crates/tachi-core/src/report_data.rs:289-291`) is:

```rust
    if !data.findings.is_empty() || !data.controls.is_empty() || !data.coverage_matrix.is_empty() {
        return true;
    }
```

The competing contract evidence is the canonical severity-grouped residual schema (`templates/tachi/output-schemas/compensating-controls.md:75-101`) and deliberate completed-empty tests; these do not legitimize silent truncation but constrain impact claims. Round 2 asks reviewers only to distinguish known impact from exposure assumptions and cite any project priority policy. Minimum justified severity is the eventual judge's responsibility.

## New discoveries
No new defect classes. CQ-2 gained independent runtime verification. All reviewers accepted that generated baseline equality does not independently establish semantic report correctness.

## Sycophancy audit
Four reviewers lowered a position (ARC-1, CQ-1, DA-2, PIPE-2) with explicit source-contract or trigger evidence. Unsupported majority-following changes: 0/4; no alert. Agreement is not verification, and correlated-model warning remains required.

## Next round
Round 2 required for the substantive P1/P2 disagreement. No need to reopen resolved technical mechanisms or invent new defects. If no new evidence emerges, preserve the disagreement and send it to severity verification/judge; omit Round 3 rather than repeat votes.
