# CodeQL and SARIF maintenance policy

## Current release contract

The active workflows use `github/codeql-action/upload-sarif@v4`. The release
mapping reviewed for this repository is CodeQL Action `v4.37.0` with the
bundled CodeQL CLI `2.26.0` (released 2026-07-08). The upload action runs on
Node 24; active CodeQL jobs use GitHub-hosted Ubuntu runners, so this policy
does not authorize a macOS CodeQL migration.

The repository keeps the major tag floating at `@v4` rather than pinning an
immutable commit. This is an explicit **floating @v4 risk acceptance**:

- Owner: repository maintainers; review cadence: quarterly and on upstream
  security advisories.
- Compensating controls: workflow contract tests, actionlint, remote CodeQL,
  gitleaks, supply-chain checks, SARIF schema validation, and protected review
  of workflow diffs.
- Change rule: update all active CodeQL action references atomically, record
  the release-to-bundle mapping here, and retain a rollback commit/tag.
- A future immutable SHA pin must identify the exact upstream release and be
  changed atomically across every active CodeQL action use.

Historical v3 references in archived inventories are provenance only. They are
not active workflow configuration and must remain labeled historical.

## SARIF security contract

Every active SARIF upload must:

- use one supported CodeQL Action v4 line and a named producer/category;
- validate the SARIF version and run shape before upload;
- keep artifact paths repository-contained and bound result/message sizes;
- apply redaction to credentials, tokens, private URLs, and sensitive
  filesystem details;
- record checked-out-commit provenance where the producer supports it;
- upload with the minimum trusted-event permission boundary and `if: always()`;
- fail the job after upload when the scanner, converter, formatter, or SARIF
  validator fails.

Local emulators never receive `GITHUB_TOKEN` and never upload SARIF. Remote
CodeQL/SARIF execution remains the authoritative ingestion check.

## Verification and rollback

Run `make codeql-maintenance-gate`, the workflow contract tests, the SARIF
producer tests, and the remote CodeQL workflow after each action update. The
gate rejects active v3 references, missing category/provenance policy, and a
missing release mapping. If a release breaks, restore the last verified
action/bundle mapping in one conventional commit and rerun the full security
and workflow gates.

The scheduled/manual `CodeQL upstream maintenance` workflow runs
`make codeql-upstream-release-check`'s underlying script against the GitHub
release API every Monday. It is an advisory maintenance signal, not a PR or
publish gate: a newer non-prerelease v4 tag fails that scheduled run and
requires a reviewed mapping update plus a Beads note. The check uses only
`contents: read`, does not receive secrets, and never uploads SARIF or mutates
issues automatically.
