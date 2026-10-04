# Rust-Native Upstream Feature Roadmap — 2026-10-04

Delivery status: implementation evidence below is historical. [Milestone delivery](roadmap/2026-10-04-upstream-milestone-delivery.md), epic `RT-3zm`, now governs phased PRs, review remediation and protected auto-merge. Closure of implementation epic `RT-5vk` did not establish merged delivery; all phase gates must be verified independently.

Review window: May 4–October 4, 2026. Upstream anchor: `63438d78cfedc7abf4ae6e2f6b3301161246299d`; fork anchor: `cb567d3235eee0d45081b099a3c6df7c98f12eb7`. Prioritize value versus effort and preserve the Rust architecture. Implementation lives on `feat/rust-native-upstream-2026-10-04` in an isolated worktree, separate from PR #26 and the original dirty checkout.

## Mandatory constraints

No feature may introduce Python, including indirect application, build, code-generation, test, baseline-regeneration, documentation-tooling or CI-action requirements. Parsing, validation, transformations and regression tests belong in Rust. Existing Typst, GitHub CLI and minimal shell orchestration are allowed; domain logic must not migrate to shell. Audit new normal, build, development and wrapper dependencies; any Python execution chain blocks acceptance. Historical references and archived examples are not runtime dependencies and are outside cleanup scope. Port contracts, not upstream Python or pytest implementations.

Crosswalk parity, safe image cleanup, taxonomy expansions, asset-tag output and citation monitoring are already adopted. The [previous roadmap](feature-roadmap-2026-10-03-v1.md) is historical, not new open work.

## Ranked implementation and acceptance

### RT-5vk.2 — Complete OWASP 2026 semantic cutover (P1 / medium)

Source: [upstream #363](https://github.com/davidmatousek/tachi/pull/363). Reconcile active instructions, adapters, fixtures and generated outputs against the disposition ledger and current catalog. Output integrity emits LLM10, not LLM05; misinformation emits LLM07, not LLM09. Preserve explicit historical references and the finding schema; do not globally replace tokens.

Acceptance: active category meaning and IDs agree. Tests: all changed mappings, deliberately stale instructions, catalog resolution, historical exceptions, existing SARIF and report regressions. Dependencies: planning/audit.

### RT-5vk.3 — Hosted permissions consistency check (P1 / very small)

Source: [upstream #347](https://github.com/davidmatousek/tachi/pull/347). Reuse non-Python JSON validation and permissions-document checks in a distinct hosted check on relevant pull requests and main pushes. Use read-only permissions and current fork Action pins; do not change branch protection.

Acceptance: malformed settings, undocumented permissions and missing required sections fail actionably; valid configuration passes. Tests: temporary malformed/valid fixtures, actionlint, trigger/permission contracts. Dependencies: planning/audit.

### RT-5vk.4 — Truthful MAESTRO evaluation states (P1 / medium)

Source: [upstream #318](https://github.com/davidmatousek/tachi/pull/318). Add a shared Rust enum and additive coverage-state fields: `findings` (positive count), `clean` (explicit evaluation with zero findings), `not_applicable` (explicit non-applicability), `not_evaluated` (missing or ambiguous evidence). Keep all seven canonical layers. Renderers consume the shared classification and must not infer applicability from component mappings. Legacy inputs default to `not_evaluated`.

Acceptance: Markdown evidence, Typst data and infographic JSON agree per layer; missing evidence never becomes clean. Tests: punctuation/whitespace, missing rows, ambiguous zero counts, positive findings, forced output disagreement with layer/output diagnostics. Update representative examples. Dependencies: planning/audit.

### RT-5vk.5 — Catalog drift and baseline regeneration (P2 / medium)

Source: [upstream #344](https://github.com/davidmatousek/tachi/pull/344). Add Rust `catalog-drift --check` using the renderer framework registry/loading semantics. Compare ordered raw and in-scope `(id, out_of_scope)` fingerprints with a committed manifest. Add `--regenerate-baselines`, invoking Rust reporting and pinned Typst directly; stage all six existing PDFs and publish the set and manifest only after all renders succeed. Record hashes and renderer provenance.

Acceptance: membership, ordering, ID and scope changes require regeneration; citation-only edits pass. Missing/malformed inputs fail explicitly. CI checking needs no Python, Typst or network. Tests: record/framework changes, scope flips, malformed input, stale manifests, failed regeneration, PDF compatibility in the pinned environment. Dependencies: RT-5vk.2 and RT-5vk.4.

### RT-5vk.6 — Output-integrity preservation (P2 / small–medium)

Sources: [upstream #353](https://github.com/davidmatousek/tachi/pull/353), [failed verification defect #356](https://github.com/davidmatousek/tachi/issues/356). Add a deterministic multi-tenant vector-filter fixture through Rust parsing, report and SARIF paths. Identify findings with `partialFingerprints["findingId/v1"]`, not shared SARIF rule IDs. Preserve OI identities, CWE-943, current OWASP citations, source paths and assets. Fix only reproduced failures.

Acceptance: mixed OI/LLM findings retain identity and attribution. Tests: mixed families, absent OI, lost citations, path variation, repeat stability. These prove transformations, not successful live-agent evaluation; upstream failed verification remains failed. Dependencies: RT-5vk.2.

### RT-5vk.7 — Adopter secret scanning (P3 / small)

Source: [upstream #347](https://github.com/davidmatousek/tachi/pull/347). Add adopter configuration and maintenance templates retaining default scanner rules and current fork pins.

Acceptance: adopters can extend rules and reproduce validation. Tests: synthetic credentials in temporary directories outside excluded fixtures, permitted placeholders, inherited rules, malformed configuration. Use existing non-Python tools. Dependencies: planning/audit.

## Tracking and delivery

Create one new Beads epic, six feature children, separate implementation and validation tasks under each, plus shared planning/Rust-only audit and delivery verification tasks. Each issue carries acceptance criteria, test plan, dependency links, upstream links and this roadmap reference; generated Beads IDs replace planning identifiers below. Never reuse closed issues.

Planning/audit records comparison anchors, adoption status, dependency review, criteria and test plans before feature edits. Delivery synchronizes evidence and tracker export, audits final dependency/workflow changes for Python, and confirms required CI. Run focused Rust tests, formatting, Clippy, workspace, documentation and hosted checks. Prove new commands/workflows execute without Python. Unavailable or failed checks remain explicitly unverified; do not claim delivery until gates pass.

## Execution evidence

- 2026-10-04: refreshed `origin/main` through `rtk`; matches fork anchor. Created isolated worktree; original dirty checkout preserved. No feature code changed before this roadmap was saved.
- Beads epic: `RT-5vk`; planning/audit: `RT-5vk.1`; delivery: `RT-5vk.8`.

| Feature | Beads feature | Implementation | Validation | Depends on |
|---|---|---|---|---|
| OWASP 2026 | RT-5vk.2 | RT-5vk.2.1 | RT-5vk.2.2 | RT-5vk.1 |
| Permissions CI | RT-5vk.3 | RT-5vk.3.1 | RT-5vk.3.2 | RT-5vk.1 |
| MAESTRO states | RT-5vk.4 | RT-5vk.4.1 | RT-5vk.4.2 | RT-5vk.1 |
| Catalog drift | RT-5vk.5 | RT-5vk.5.1 | RT-5vk.5.2 | RT-5vk.2, RT-5vk.4 |
| Output integrity | RT-5vk.6 | RT-5vk.6.1 | RT-5vk.6.2 | RT-5vk.2 |
| Adopter scanning | RT-5vk.7 | RT-5vk.7.1 | RT-5vk.7.2 | RT-5vk.1 |

## Contextual OWASP dispositions

Active output-integrity instructions use LLM10; vector-filter findings use LLM09 as primary and LLM10 as related. Misinformation uses LLM07, hidden context exposure LLM08, supply-chain compromise LLM04, poisoning LLM05 and resource consumption LLM06. Adapter framework labels now identify 2026. Explicit attack-chain provenance citing the 2025 source remains historical.

The six baseline inputs are current rendering fixtures. In `maestro-reference`, training/fine-tuning poisoning (LLM-2/LLM-5) uses LLM05; model extraction (LLM-3) uses LLM06 per the current pattern catalog, while membership inference (LLM-6) uses LLM02 for information disclosure. In `mermaid-agentic-app`, indirect injection (LLM-2) uses LLM01, poisoned retrieval content (T-2/LLM-3) uses LLM09, configuration/system-prompt extraction (LLM-4) uses LLM08, and inference resource exhaustion (D-1) uses LLM06. Removed an orphan attribution for nonexistent LLM-5. Active agent and adapter examples have parsed-YAML reference/attribution tests. These are contextual corrections, not blanket ID substitutions.

## Native validation commands

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo test -p tachi-core --test adopter_secret_scanning -- --ignored
cargo test -p tachi-cli --test upstream_contract_commands
cargo run --locked -p tachi-cli --bin permissions-check -- .
bash .aod/scripts/bash/claude-permissions-ac2-crosscheck.sh
cargo run --locked -p tachi-cli --bin catalog-drift -- --check
actionlint .github/workflows/*.yml
```

Baseline regeneration requires the native `typst 0.15.1 (9dfd3a08)` executable; no Python, Node or shell renderer wrapper is used. Set `--typst` to that executable and run `cargo run --locked -p tachi-cli --bin catalog-drift -- --regenerate-baselines --typst /absolute/path/to/typst`. Then put the same executable on PATH and run `cargo test -p tachi-core --test backward_compatibility`. Both paths use epoch 1700000000 and embedded fonts only. The offline check validates the committed catalog fingerprints and all six PDF hashes without Typst or network access.

The Rust report data builder now supplies the complete template binding contract, preserving available risk-score and compensating-control tiers. Optional report sections not populated by this builder remain explicitly unavailable; regeneration does not claim parity with every historical upstream report section. The PDF compatibility test compares the committed native baselines with this Rust path.

Phase 2 corrects three review findings on PR #42: available brand assets are detected and staged for native rendering; remediation recommendations retain their finding IDs, severity, status and SLA; component distribution comes from the selected control/risk/raw tier. Dedicated Rust regressions cover present/absent branding, tier precedence and actual `maestro-reference` recommendations. The regenerated six-PDF set includes these available inputs instead of silently suppressing them.

## Delivery evidence — 2026-10-04

- Review: [fork PR #42](https://github.com/pratik-saptarshi/tachi-rust/pull/42), isolated branch `feat/rust-native-upstream-2026-10-04`. The original checkout was preserved.
- Local terminal results: `cargo test --workspace --all-targets`, formatting, Clippy with warnings denied, both documentation version gates, actionlint across all workflows, permissions Rust/AC-2 checks, and native Gitleaks scan passed. The ignored adopter-scanning test was explicitly executed and passed.
- Both CLI checks passed with `PATH=/nonexistent`; baseline regeneration passed in the same environment using the absolute pinned Typst executable. All six PDF comparisons passed byte-for-byte with embedded fonts. The canonical local CI manifest and hosted shell-smoke slice both include the OI preservation regression; their contract tests pass.
- Dependency audit: no external package added or upgraded. `serde_yaml_ng 0.10.0` moved from an existing development dependency into core runtime dependencies; its normal/build tree consists of Rust crates, including `unsafe-libyaml`. CLI adds the existing local core crate. New workflows reuse checkout v7 and the existing Rust setup/cache action; native Gitleaks stays at 8.30.1. No new Python code, interpreter, wrapper, build/codegen step or action execution chain was introduced.
- All feature, audit and delivery tasks, including epic `RT-5vk`, are closed. On implementation commit `7f322b6`, all 17 required checks succeeded and every hosted job completed. Evidence: [workspace](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37199463483), [CodeQL](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37199462686), [permissions](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37199463522), [catalog](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37199463507), [Gitleaks/adopter](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37199463523). The final evidence/export commit is checked again before delivery is reported. No branch-protection setting was changed; review/merge policy remains applicable.
