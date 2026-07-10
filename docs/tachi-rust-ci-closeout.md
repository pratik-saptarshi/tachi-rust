# Tachi-Rust CI Closeout Notes

**Status**: draft closeout evidence for RT-CI
**Purpose**: separate locally proven RT-CI changes from external verification
items that still require GitHub access

## Proven Locally

- Route policy manifest and route artifact contracts exist and are covered by
  workflow contract tests.
- Passive-docs narrowing, dependency-closure routing, and the emergency full-CI
  override are implemented in `rust-workspace.yml`.
- Protected refs (`main`, `release/*`, and tags) are forced to full mode.
- Shared Rust setup is centralized in `.github/actions/rust-setup/action.yml`.
- Heavy Rust-facing workflows emit elapsed runtime summaries.
- Phase 0 baseline inventory and local validation snapshot are recorded in
  `docs/tachi-rust-ci-baseline.md`.
- Warm local timing comparison exists for the same workflow test on
  `origin/main` (`real 0.58s`) and the current branch (`real 1.39s`), but it
  is not a substitute for live PR median evidence.

## Still Pending External Verification

- Live GitHub Actions timing evidence for pre-router vs post-router median PR
  durations.
- Branch-protection verification that the required-check migration is safe to
  finalize.
- Post-push monitoring of `main` after a publish step.

## Evidence Links

- [Baseline snapshot](./tachi-rust-ci-baseline.md)
- [Execution plan](./tachi-rust-ci-execution-plan.md)
- [Route policy](./tachi-rust-ci-route-policy.md)
- [Route artifact](./tachi-rust-ci-route-artifact.md)
