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
- GitHub Actions artifact uploads: `actions/upload-artifact@v5`; current
  checkout and CodeQL major lines remain v7 and v4.
- Typst setup action: `typst-community/setup-typst@v5.1.0`.
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
