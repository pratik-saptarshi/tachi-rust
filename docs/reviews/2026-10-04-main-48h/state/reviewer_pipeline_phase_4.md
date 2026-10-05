# Pipeline Reviewer — Phase 4 Private Reflection

Only my own Phase 3 and the cited current source/documentation were reread. No other reviewer output was accessed.

## Confidence Ratings

- **PIPE-1: High** for the new target-specific test compilation defect; **Medium** for its project-level importance. The test module has only `cfg(test)`, imports `std::os::unix::fs::PermissionsExt`, and unconditionally compiles the shell fixture helper. This is deterministic on a non-Unix target, but I have not established a required Windows CLI test lane or that the entire base workspace was Windows-test-clean. Keep [P2], with impact confined to the new CLI test target.
- **PIPE-2: Medium** overall, reduced from the Phase 3 numerical confidence. The implementation mismatch is a strong source-level inference: root inputs are hashed, a compiled-in builder executes, and only root stability is checked. However, the documented regeneration procedure explicitly uses `cargo run --locked` in the checkout and then runs the PDF compatibility test. That procedure prevents the ordinary stale executable trigger and should reveal a mismatch. The finding concerns the arbitrary-root/direct-binary interface and strength of the provenance claim, not the prescribed maintainer path or existing committed artifacts.

## Most/Least Defensible

Most defensible: PIPE-1 is a localized code defect with a conventional fix matching existing CLI platform guards. Its existence does not depend on interpreting the manifest contract.

Least defensible: PIPE-2 as a blocking acceptance issue. All native tools can be invoked from stale binaries; a requirement to authenticate the executable against a mutable source checkout is stronger than ordinary build reproducibility. The special feature here is that this command writes a source fingerprint that appears to certify association with generated PDFs. The docs at `docs/feature-roadmap-2026-10-04.md:95` support that interpretation, but do not explicitly promise refusal of binaries built elsewhere. I retain it as a conditional [P2] provenance weakness suitable for debate, not a release blocker.

## Changes

- No factual retraction for PIPE-1; clarify no baseline-wide Windows readiness claim.
- PIPE-2 confidence lowered to Medium and recommendation narrowed: first make the precondition explicit and decide whether arbitrary-root regeneration is intended across source revisions. If arbitrary roots are supported as the CLI suggests, enforce source/executable matching. Otherwise document and reject mismatched roots rather than immediately mandate a complex build-provenance subsystem.
- Phase 3 received formatting-only changes: added `## Findings` and bracketed `[P2]` labels. Its original assessment remains available as the independent record.
- No new findings or execution claims. No product files changed.

## Remaining Uncertainty

No Windows target or Typst binary is available in this review environment. PIPE-1 has not been cross-compiled. PIPE-2 has not been demonstrated with two builds and a rendered PDF; a targeted reproduction would distinguish an observable stale-output defect from a contractual improvement request. Existing input drift, renderer pinning, staged publication and ordinary-error rollback remain meaningful protections and must not be described as absent.
