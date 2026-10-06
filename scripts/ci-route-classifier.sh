#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 5 ]; then
  echo "usage: ci-route-classifier.sh <event> <ref> <changed-paths-file> <force-full-input> <force-full-variable>" >&2
  exit 2
fi

event="$1"
ref="$2"
changed_paths_file="$3"
force_full_input="$4"
force_full_var="$5"

full_packages_json='["tachi-core","tachi-mcp","tachi-cli","tachi-shell","tachi-desktop"]'
mode="full_pr_matrix"
reason="non-pull_request events stay full mode"
packages_json="$full_packages_json"
changed_paths_json='[]'

active_contract_pattern='^(README\.md|CHANGELOG\.md|SECURITY\.md|docs/testing/tdd-evidence\.json|docs/(roadmap/|standards/|guides/|bill-of-materials\.html\.md|publish-readiness-checklist\.html\.md|platform-compatibility\.md|tachi-rust-ci-|ci-improvement-plan\.html)|\.github/workflows/|Cargo\.toml$|Cargo.lock$|Makefile$|\.aod/|\.claude/|adapters/)'
repository_contract_pattern='^(Cargo\.toml|Cargo\.lock|rust-toolchain\.toml|deny\.toml|Makefile|\.cargo/.*|\.github/(ci-test-units\.json|actions/.*/action\.yml|workflows/.*)|crates/[^/]+/Cargo\.toml|crates/tachi-core/tests/workflow_ci_gates\.rs|scripts/ci-route-(classifier|changed-paths)\.sh)$'
repository_contracts_required=false

is_protected_ref() {
  case "$1" in
    main|release/*|refs/heads/main|refs/heads/release/*|refs/tags/*)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

if [ -f "$changed_paths_file" ]; then
  changed_paths_json="$(jq -R -s -c 'split("\n") | map(select(length > 0))' "$changed_paths_file")"
fi
if [ "$event" != "pull_request" ] || [ ! -s "$changed_paths_file" ] || grep -Eq "$repository_contract_pattern" "$changed_paths_file" || grep -vq '^docs/' "$changed_paths_file"; then
  repository_contracts_required=true
fi

if [ "$force_full_input" = "true" ]; then
  reason="emergency full-ci override (workflow_dispatch input)"
elif [ "$force_full_var" = "true" ]; then
  reason="emergency full-ci override (repo variable)"
elif is_protected_ref "$ref"; then
  reason="protected ref stays full mode"
elif [ "$event" = "pull_request" ]; then
  if [ ! -f "$changed_paths_file" ] || [ ! -s "$changed_paths_file" ]; then
    reason="empty or unavailable diff stays full mode"
  elif grep -Eq "$active_contract_pattern" "$changed_paths_file"; then
    reason="active docs or shared surface touched"
  elif grep -vq '^docs/' "$changed_paths_file"; then
    touched_crates=""
    selected_packages=""
    unknown_non_doc_paths=false

    closure_for() {
      case "$1" in
        tachi-core)
          printf '%s\n' tachi-core tachi-mcp tachi-cli tachi-shell tachi-desktop
          ;;
        tachi-shell)
          printf '%s\n' tachi-shell tachi-cli tachi-mcp tachi-desktop
          ;;
        tachi-mcp)
          printf '%s\n' tachi-mcp
          ;;
        tachi-cli)
          printf '%s\n' tachi-cli
          ;;
        tachi-desktop)
          printf '%s\n' tachi-desktop
          ;;
      esac
    }

    while IFS= read -r changed_path; do
      case "$changed_path" in
        crates/tachi-core/*) touched_crate="tachi-core" ;;
        crates/tachi-shell/*) touched_crate="tachi-shell" ;;
        crates/tachi-mcp/*) touched_crate="tachi-mcp" ;;
        crates/tachi-cli/*) touched_crate="tachi-cli" ;;
        crates/tachi-desktop/*) touched_crate="tachi-desktop" ;;
        docs/*) touched_crate="" ;;
        *)
          touched_crate=""
          unknown_non_doc_paths=true
          ;;
      esac
      if [ -n "$touched_crate" ]; then
        case " $touched_crates " in
          *" $touched_crate "*) ;;
          *) touched_crates="${touched_crates:+$touched_crates }$touched_crate" ;;
        esac
      fi
    done < "$changed_paths_file"

    if [ "$unknown_non_doc_paths" = "true" ]; then
      reason="unknown non-docs paths stay full mode"
    elif [ -n "$touched_crates" ]; then
      mode="dependency_closure"
      reason="crate-local changes routed through dependency closure"
      for crate in $touched_crates; do
        while IFS= read -r package; do
          if [ -n "$package" ]; then
            case " $selected_packages " in
              *" $package "*) ;;
              *) selected_packages="${selected_packages:+$selected_packages }$package" ;;
            esac
          fi
        done < <(closure_for "$crate")
      done
      packages_json="$(printf '%s\n' "$selected_packages" | tr ' ' '\n' | sort -u | jq -R -s -c 'split("\n") | map(select(length > 0))')"
    else
      reason="unknown non-docs paths stay full mode"
    fi
  else
    mode="passive_docs_only"
    reason="docs-only passive paths observed"
  fi
elif is_protected_ref "$ref"; then
  reason="protected ref stays full mode"
fi

jq -n \
  --arg mode "$mode" \
  --arg reason "$reason" \
  --argjson packages "$packages_json" \
  --argjson repository_contracts_required "$repository_contracts_required" \
  --argjson changed_paths "$changed_paths_json" \
  '{mode: $mode, reason: $reason, packages: $packages, repository_contracts_required: $repository_contracts_required, changed_paths: $changed_paths, policy_version: "2026-10-06"}'
