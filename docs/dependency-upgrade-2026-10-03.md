# Dependency and Runtime Upgrade (2026-10-03)

This records the stable versions selected for the repository upgrade. Direct
Cargo and npm dependencies are exact-pinned in their manifests; Cargo.lock and
the three npm lockfiles capture the resolved transitive dependency trees.

## Rust workspace

- Toolchain and declared compiler floor: Rust 1.99.0 / 1.99.
- Direct crates: serde 1.0.229, serde_json 1.0.151, sha2 0.11.0,
  thiserror 2.0.21, proptest 1.11.0, pretty_assertions 1.4.1, regex 1.13.1,
  rstest 0.27.0, and serde_yaml_ng 0.10.0.
- `serde_yaml_ng` replaces deprecated `serde_yaml`; its package is aliased to
  the existing `serde_yaml` crate name to preserve call sites.
- SHA-2 0.11 uses a new digest output type, so the fixture, desktop artifact,
  and MCP contract hash paths now encode digest bytes explicitly as lowercase
  hexadecimal.

## Frontend scaffolds

- Both Vite scaffolds use React 19.3.0, TanStack Query 5.104.1, TypeScript
  7.0.2, Vite 8.3.2, plugin-react 6.1.1, Tailwind 4.3.3, Vitest 5.0.3, and
  Biome 2.5.15, with current stable testing and React type packages.
- The Next.js scaffold uses Next 16.3.8, React 19.3.0, Supabase JS 2.117.2,
  Supabase SSR 0.12.7, Prisma 7.10.0, Zod 4.6.5, and current stable testing,
  browser automation, and type packages.
- Prisma 7 now uses `prisma.config.ts`, the `prisma-client` generator, and the
  PostgreSQL driver adapter. `deepmerge-ts` 8.0.2 and `mysql2` 3.24.5 are
  locked through npm overrides to clear advisories reported for Prisma's
  transitive dependency ranges.
- Biome 2 config migration, Tailwind CSS parsing, generated-output ignores,
  TypeScript 7 path aliases, and Vite environment typings are included. The
  Vite API wrapper now normalizes request headers with the Web `Headers` API.
- The Next.js `middleware.ts` convention is migrated to `proxy.ts`. The
  scaffold's no-test Vitest scripts use `--passWithNoTests` so they succeed
  clearly until test files are added.

## Runtime and CI pins

- PostgreSQL scaffold image: `postgres:18-alpine`.
- Python preflight runtime: Python 3.14 using `actions/setup-python@v7`.
- GitHub Actions artifact uploads: `actions/upload-artifact@v7`; current
  checkout and CodeQL major lines remain v7 and v4.
- Typst setup action: `typst-community/setup-typst@v5.3.0`.
- Pinned Cargo helpers: cargo-hack 0.6.45, cargo-llvm-cov 0.9.1,
  cargo-audit 0.22.2, cargo-deny 0.20.2, clippy-sarif 0.8.0, and
  sarif-fmt 0.8.0.
- The `act-smoke-run` trusted workflow hash was refreshed after the workflow's
  artifact action update.

## Deferred upgrades

None. Prisma 8 was a prerelease at selection time, so the latest stable Prisma
7 release was selected. A live Prisma migration could not be run without a
PostgreSQL server; schema validation, client generation, and initial migration
SQL generation succeeded.

## Closeout verification (2026-10-04)

On 2026-10-04, the exact direct Cargo pins and npm lockfile package pins were
checked against the Cargo registry and npm registry metadata through `rtk`.
All listed crates matched their latest stable release at the time of the
check. `npm outdated --json` was empty for both Vite scaffolds; the only Next.js
result was Prisma 8.0.0-rc.19, so Prisma 7.10.0 remains the latest stable
Prisma 7 pin. The Rust toolchain is 1.99.0 (stable, 2026-09-28); `rustup
check` reported a later nightly build, which is not the committed stable
channel.

GitHub release metadata confirmed `actions/upload-artifact` v7.0.1,
`actions/checkout` v7.0.1, `actions/setup-python` v7.0.0, and
`typst-community/setup-typst` v5.3.0. Cargo registry metadata confirmed the
pinned helper versions listed above. The Python 3.14 action channel and
`postgres:18-alpine` OCI manifest were checked; the latter resolved to a
multi-architecture image.
Sources: [upload-artifact releases](https://github.com/actions/upload-artifact/releases),
[checkout releases](https://github.com/actions/checkout/releases),
[setup-python releases](https://github.com/actions/setup-python/releases),
[Typst setup releases](https://github.com/typst-community/setup-typst/releases),
[Prisma releases](https://www.npmjs.com/package/prisma?activeTab=versions), and
[PostgreSQL image](https://hub.docker.com/_/postgres).

Local evidence on the upgrade worktree:

- `cargo fmt --all -- --check`, workspace Clippy with `--locked -D warnings`,
  `cargo test --workspace --locked`, `make workflow-gate`,
  `make scaffold-dependency-gate`, `make release-gate`, and
  `make supply-chain-gate` passed on the fresh branch.
- Both Vite scaffolds passed clean `npm ci`, Biome lint, the configured
  no-test Vitest command, and production build. The Next.js scaffold passed
  clean `npm ci`, Biome lint, no-test Vitest, Prisma client generation,
  migration SQL generation, and production build. No test files exist in any
  of the three scaffolds.
- Under nightly coverage, the one-second fixture timeout initially raced the
  fake shell's TERM trap (`fake cargo did not receive TERM`). Extending that
  test-only timeout to five seconds made all four runner-contract tests pass
  under nightly coverage and the full locked workspace suite pass. The targeted
  shell init suite passed 2/2 in 10.74 seconds; no init-test stall was
  reproduced.
- The full aggregate publish gate passes workflow, docs, scaffold,
  supply-chain, gitleaks, nightly coverage, and release-readiness checks. All
  workspace tests pass under nightly instrumentation; branch coverage is
  85.08174386920983%, above the configured 85% threshold. Coverage-focused
  offline taxonomy-link-monitor unit and CLI integration tests were added
  after the current-main baseline measured 83.88%.
- `make release-gate` passed. Prisma schema validation and generated SQL pass,
  but live PostgreSQL migration remains unverified because no local server has
  been established.
- The upload action is updated to v7 across workflows, and the route workflow
  trusted-content SHA-256 is refreshed to match. A stale workflow test
  expectation was also updated to v7. PR #39 is open and mergeable. Its 16
  required protected contexts passed at implementation commit
  `a3de8c7ed9445b05b32d202db50ab4ef858f9eb6`; a docs-only sync commit follows,
  so the required-check rollup must be refreshed for the final PR head. Live
  PostgreSQL migration remains the environment limitation. See [the active
  closeout plan](roadmap/2026-10-04-dependency-runtime-upgrade-closeout-v1.md).
