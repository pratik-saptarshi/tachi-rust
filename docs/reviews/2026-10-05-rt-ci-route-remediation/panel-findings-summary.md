# Adversarial Panel Findings Summary — RT-CI Route Remediation

Review baseline: `10339cc8f586fdf0050c01bd2d4889f7709906e6`  
Review date: 2026-10-05  
Status: Source-attributed summary; original panel report unavailable in the
workspace at integration time.

## Provenance and limits

This archive captures the panel findings and dispositions from the
decision-complete remediation plan supplied for integration. It is not the
verbatim panel output. The original review report could not be located in the
repository or temporary directories during this work. Finding text, category,
and acceptance details below are consequently based on that supplied summary;
the missing original should be attached if exact reviewer citations or votes
are needed. The 37-candidate historical audit is bounded to its retained
sample and is not evidence that the timing objective passed.

The separate cohort audit is available in open PR #56 at
[`docs/reports/rt-ci-route-cohort-audit-2026-10-05.md`](../../reports/rt-ci-route-cohort-audit-2026-10-05.md).
It reconstructs 37 historical candidates as full-matrix routes, leaving zero
eligible samples for either target route and producing no timing medians. This
supports the bounded historical note only; it is not the missing panel report.

## Findings

| ID | Priority | Finding | Disposition |
|---|---|---|---|
| P2-F01 | P2 | Rename classification loses one path endpoint; route input must include both old and new paths. | Required Phase 1 remediation. |
| P2-F02 | P2 | Shell reverse-dependency coverage omits downstream `tachi-mcp` tests. | Required Phase 2 remediation. |
| P2-F03 | P2 | Repository-wide crate-manifest contracts owned by `tachi-core` are skipped for routes such as an MCP-manifest-only change. | Required Phase 2 remediation. |
| P2-F04 | P2 | Test-owned `docs/testing/tdd-evidence.json` is classified as passive documentation, skipping its contract test. | Required Phase 1 remediation. |
| P3-F05 | P3 | Timing verifier scope for narrowed route runs is unclear; it currently verifies the full matrix's eight artifacts. | Nonblocking advisory; retain full-matrix behavior and define route-derived expectations before using it for narrowed evidence. |
| P3-F06 | P3 | Route-audit replay inputs are not retained in a compact manifest. | Nonblocking evidence improvement; preserve candidate/run/tree/path/reason/classifier inputs and keep conclusions sample-bounded. |

## Bounded historical observation

The retained audit reported zero eligible historical candidates for the target
route shapes. This is bounded evidence about that sample only. It neither
proves nor disproves the matched timing target; valid matched controls remain
required under the updated `RT-0sd.2` acceptance.

## Traceability

The remediation roadmap at
[`docs/roadmap/remediation-roadmap-2026-10-05.md`](../../roadmap/remediation-roadmap-2026-10-05.md)
contains refined acceptance criteria, RED/GREEN/regression test plans, phase
gates, PR boundaries, and Beads links for each finding.
