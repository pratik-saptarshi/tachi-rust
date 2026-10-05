# Phase 9 — Advisory verification commands

P0 count: zero. Remaining proposed P1: DA-1 controls false-zero. ARC-1 was downgraded to P2 during debate. Reviewers supplied runnable report-data probes and sed/awk source commands, outside this phase's narrow grep/cat/head/tail/wc allowlist. Those exact commands were not replayed here. Existing private-fixture reproductions remain evidence for later verification; their successful outcomes are not invented here.

One read-only source inspection adapted from DA-1's cited guard was run (below the five-command cap):

```sh
rtk proxy head -n 291 crates/tachi-core/src/report_data.rs | rtk proxy tail -n 7
```

Exit 0, raw output:
```rust
fn has_control_assessment(
    content: &str,
    data: &crate::compensating_controls::CompensatingControlsData,
) -> bool {
    if !data.findings.is_empty() || !data.controls.is_empty() || !data.coverage_matrix.is_empty() {
        return true;
    }
```

[CMD_CONFIRMED] The cited early-return guard exists and accepts nonempty control inventory independently of residual findings. This inspection does not establish input prevalence or P1 severity. No command contradicted a finding. Mutation/executable probes are outside Phase 9 and must retain their own evidence labels.
