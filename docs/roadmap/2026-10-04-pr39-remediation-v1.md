# PR #39 Review and Closeout Remediation

- **Date:** 2026-10-04
- **GitHub issue:** [#40](https://github.com/pratik-saptarshi/tachi-rust/issues/40)
- **Beads epic:** `RT-bz6` (children `RT-bz6.1` through `RT-bz6.7`)
- **Branch:** `040-pr39-closeout-remediation`
- **Original PR:** [#39](https://github.com/pratik-saptarshi/tachi-rust/pull/39)
- **Current PR:** [#41](https://github.com/pratik-saptarshi/tachi-rust/pull/41)

## Findings and changes

1. **PostgreSQL 18 volume compatibility (P2).** Change the FastAPI/React
   Compose volume target to `/var/lib/postgresql`; document a PostgreSQL 16
   logical dump and PostgreSQL 18 restore path that preserves the old volume.
2. **Supabase environment template (P2).** Add placeholders for
   `NEXT_PUBLIC_SUPABASE_URL` and `NEXT_PUBLIC_SUPABASE_ANON_KEY`.
3. **User table exposure (P1).** Use Supabase Auth UUIDs as profile IDs, enable
   RLS, and allow authenticated users to select, insert, and update only their
   own profile. Anonymous and cross-user access stay denied.
4. **Live Prisma migration evidence.** Add a PostgreSQL 18 service workflow that
   applies migrations to a fresh database and verifies the RLS policy behavior.
5. **CodeQL ruleset bypass notice.** Remove the redundant push-time CodeQL
   ruleset rule. GitHub branch protection continues to require the `CodeQL`
   status check before PR merge. The active ruleset retains deletion,
   non-fast-forward, and code-quality protections.
6. **Branch/issue alignment.** Anchor this remediation to issue #40 and use its
   zero-padded number in the follow-up branch name. Existing `RT-bbi` history
   remains unchanged.
7. **Root atlas accuracy (PR #41 review).** Update the `Cargo.toml` and
   `rust-toolchain.toml` rows in `codemap.md` to match the Rust 1.99 MSRV and
   1.99.0 toolchain pins.
8. **Compose command path (PR #41 review).** Add the scaffold Compose file path
   to each migration command so the documented steps work from the repository
   root.

## Validation plan

- Run the Prisma scaffold lockfile install, Prisma generation, migration
  deployment/status, and RLS behavior checks in
  `.github/workflows/prisma-postgres.yml` against disposable PostgreSQL 18.
- Run Next.js scaffold lint, tests, and production build; validate the schema
  and workflow syntax with the repository gates.
- Run `actionlint .github/workflows/prisma-postgres.yml` and
  `git diff --check`.
- Verify each Docker Compose command in `POSTGRESQL_UPGRADE.md` names the
  scaffold Compose file and that the referenced file exists.
- Read back the active GitHub ruleset and main branch required contexts. Confirm
  CodeQL is still required and feature pushes no longer trigger a pending-scan
  bypass notice.
- Run the full required PR checks and wait for terminal CodeQL and PostgreSQL
  migration results before reporting merge readiness.

## Local verification (2026-10-04)

- `npm ci` passes from the committed lockfile, runs Prisma postinstall
  generation, and reports zero vulnerabilities (242 packages installed).
- Next.js scaffold `npm run lint`, `npm test`, and `npm run build` pass. Vitest
  reports no test files and exits successfully under the configured
  `--passWithNoTests` option.
- `npx prisma validate`, `npx prisma generate`, and
  `npx prisma migrate diff --from-empty --to-schema prisma/schema.prisma
  --script` pass. The generated SQL confirms the UUID profile key; custom RLS
  migration behavior still requires the live CI database.
- `actionlint .github/workflows/prisma-postgres.yml`, `make workflow-gate`,
  `make docs-version-gate`, Node syntax checks for both PostgreSQL scripts, and
  Compose YAML parsing pass.
- `docker info` cannot reach the Colima daemon (`permission denied`), so a live
  migration could not run locally. Hosted validation succeeded instead: run
  [37191445486](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37191445486)
  on implementation commit `01634d5` completed PostgreSQL 18 setup, Prisma
  migration deploy/status, and all RLS assertions successfully.
- PR #41's inline codemap correction is included in commit `d027adb`; the root
  atlas now matches `Cargo.toml` Rust 1.99 MSRV and `rust-toolchain.toml`
  1.99.0 channel. `make docs-version-gate` and whitespace checks passed.
- PR #41's Compose-path comment is tracked as `RT-bz6.8`; the migration guide
  now passes `-f stacks/fastapi-react/scaffold/docker-compose.yml` to each
  command run from the repository root.

## Tracker and delivery status

The Beads hierarchy is `RT-bz6` with eight children. Each child records
acceptance criteria and a test plan. All eight children and the epic are closed
after PR #41 merged at `cb567d3`. Hosted PostgreSQL migration and RLS validation
passed in PR CI and post-merge run `37193448630`; the Docker daemon remains
unavailable locally. PR #41 finished with 24 successful checks, zero failures,
and one skipped check. The regenerated `.beads/issues.jsonl` is the tracker
snapshot for this closeout.

The ruleset update was applied on 2026-10-04 to ruleset `17635989`. The
`code_scanning` rule was removed from push-time ruleset evaluation; the main
branch still requires the `CodeQL` status context. No force push or GitHub
bypass flag was used. The subsequent implementation push did not report a
bypass notice. CodeQL remains required by protected PR checks. PR #41 is merged;
final-head and post-merge checks have been verified.
