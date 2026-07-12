# Agentic-Oriented-Development-Kit - Common Commands

.PHONY: help init check update spec plan tasks analyze review-spec review-plan test test-route coverage-audit llvm-cov llvm-cov-nightly-branch workflow-gate codeql-maintenance-gate docs-version-gate docs-archive-version-gate scaffold-dependency-gate supply-chain-gate gitleaks-gate feature-combination-canary coverage-tool-proof release-gate fuzz-mutation-gate publish-gate rt-ci-latency-evidence

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

init: ## Initialize project (first-time setup)
	@./scripts/init.sh

check: ## Verify setup and prerequisites
	@./scripts/check.sh

update: ## Apply upstream template updates (AOD-kit → tachi); pass flags via ARGS='...'
	@./scripts/update.sh $(ARGS)

# Triad Workflow shortcuts
spec: ## Run /aod.spec
	@echo "Use /aod.spec in Claude Code"

plan: ## Run /aod.plan
	@echo "Use /aod.plan in Claude Code"

tasks: ## Run /aod.tasks
	@echo "Use /aod.tasks in Claude Code"

analyze: ## Run /aod.analyze
	@echo "Use /aod.analyze in Claude Code"

# Governance shortcuts
review-spec: ## Review spec.md with PM
	@echo "Use product-manager agent or /aod.spec for auto-review"

review-plan: ## Review plan.md with PM + Architect
	@echo "Use product-manager + architect agents or /aod.plan for auto-review"

# Rust test suite
test: ## Run the canonical local-full CI unit runner
	@./scripts/ci-local-runner.sh --mode local-full

test-route: ## Run the route-equivalent CI unit runner
	@./scripts/ci-local-runner.sh --mode local-route-equivalent

coverage-audit: ## Report the repository test surface by category
	@./scripts/coverage-audit.sh --root .

llvm-cov: ## Run cargo llvm-cov with the active toolchain's LLVM tools
	@./scripts/llvm-cov.sh

llvm-cov-nightly-branch: ## Run the governed nightly branch coverage gate (>=85%)
	@./scripts/llvm-cov-nightly-branch.sh

workflow-gate: ## Validate workflow action versions and checkout modernization
	@if rg "actions/checkout@v[0-6]|actions-rs/toolchain@|github/codeql-action/upload-sarif@v3|::set-output" .github/workflows; then \
	  echo "FAIL: stale GitHub Actions or CodeQL pins are still present in workflows"; \
	  exit 1; \
	else \
	  echo "workflow action gate passed"; \
	fi

codeql-maintenance-gate: ## Validate active CodeQL v4 references and release policy
	@./scripts/codeql-maintenance-check.sh

docs-version-gate: ## Validate docs and examples workflow-version hygiene
	@if rg "actions/checkout@v[0-6]|actions-rs/toolchain@|github/codeql-action/upload-sarif@v3|codeql/upload-sarif@v3|::set-output|Node 20" \
	  docs/testing/README.md \
	  docs/guides/DEVELOPER_GUIDE_TACHI.md \
	  docs/devops/README.md \
	  docs/devops/CI_CD_GUIDE.md \
	  docs/standards/PRECOMMIT_HOOKS.md \
	  docs/standards/GIT_WORKFLOW.md \
	  docs/architecture/00_Tech_Stack/README.md \
	  docs/architecture/01_system_design/README.md \
	  examples; then \
	  echo "FAIL: stale workflow-version references are still present in maintained docs or examples"; \
	  exit 1; \
	else \
	  echo "docs/version gate passed"; \
	fi

docs-archive-version-gate: ## Validate archived docs workflow-version hygiene
	@./scripts/docs-archive-version-gate.sh

scaffold-dependency-gate: ## Validate scaffold dependency floors against known Dependabot advisories
	@cargo test -p tachi-core --test scaffold_dependency_floors -- --nocapture

supply-chain-gate: ## Run dependency advisory, license, ban, and source policy checks
	@cargo audit
	@cargo deny check advisories bans licenses sources

gitleaks-gate: ## Run the local fail-closed secret scan used by the publish gate
	@set -eu; \
		report="$$(mktemp "$${TMPDIR:-/tmp}/tachi-gitleaks.XXXXXX")"; \
		staged_report="$$(mktemp "$${TMPDIR:-/tmp}/tachi-gitleaks-staged.XXXXXX")"; \
		trap 'rm -f "$$report" "$$staged_report"' EXIT; \
		set +e; \
		gitleaks detect --no-git --source . --config=.gitleaks.toml --report-format=sarif --report-path="$$report" --no-banner; \
		status=$$?; \
		gitleaks git --staged --config=.gitleaks.toml --report-format=sarif --report-path="$$staged_report" --no-banner; \
		staged_status=$$?; \
		set -e; \
		test -s "$$report"; \
		test -s "$$staged_report"; \
		jq -e '.version == "2.1.0" and (.runs | type == "array")' "$$report" >/dev/null; \
		jq -e '.version == "2.1.0" and (.runs | type == "array")' "$$staged_report" >/dev/null; \
		test "$$status" -eq 0; \
		test "$$staged_status" -eq 0

feature-combination-canary: ## Run cargo-hack feature-combination canary
	@cargo hack --version
	@cargo hack --version | grep -qx 'cargo-hack 0.6.45'
	@cargo hack check --workspace --each-feature --no-dev-deps
	@git diff --quiet -- Cargo.toml crates/*/Cargo.toml

coverage-tool-proof: ## Print cargo-llvm-cov proof and run coverage gate
	@cargo llvm-cov --version
	@cargo llvm-cov --version | grep -qx 'cargo-llvm-cov 0.8.7'
	@$(MAKE) llvm-cov

release-gate: ## Validate active desktop host release readiness
	@cargo test -p tachi-desktop --all-targets -- --nocapture

fuzz-mutation-gate: ## Validate documented fuzz/mutation lane and baseline report artifact
	@test -f docs/testing/fuzz-mutation-audit.md
	@test -f docs/reports/fuzz-mutation-baseline.md
	@grep -q "cargo fuzz" docs/testing/fuzz-mutation-audit.md
	@grep -q "cargo-mutants" docs/testing/fuzz-mutation-audit.md
	@grep -qi "follow-up Beads" docs/reports/fuzz-mutation-baseline.md

publish-gate: ## Run end-to-end publish-readiness gates locally
	@$(MAKE) check
	@$(MAKE) workflow-gate
	@$(MAKE) codeql-maintenance-gate
	@$(MAKE) docs-version-gate
	@$(MAKE) docs-archive-version-gate
	@$(MAKE) scaffold-dependency-gate
	@$(MAKE) supply-chain-gate
	@$(MAKE) gitleaks-gate
	@$(MAKE) llvm-cov-nightly-branch
	@$(MAKE) release-gate
	@$(MAKE) fuzz-mutation-gate
	@$(MAKE) test
	@cargo clippy --all-targets -- -D warnings
	@$(MAKE) coverage-audit
	@$(MAKE) llvm-cov

rt-ci-latency-evidence: ## Run queue-vs-run median evidence collection for workflow gate and route-observe lanes
	@./scripts/rt-ci-latency-evidence.sh "rust-workspace.yml,ci-route-observe.yml" "main" 40
