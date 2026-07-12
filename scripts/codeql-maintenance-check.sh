#!/usr/bin/env bash
set -euo pipefail

policy="docs/security/codeql-maintenance.md"
workflows=(.github/workflows/*.yml)

test -s "$policy"
rg -q 'v4\.37\.0' "$policy"
rg -q '2\.26\.0' "$policy"
rg -q 'Node 24' "$policy"
rg -q 'floating @v4 risk acceptance' "$policy"
rg -q 'rollback' "$policy"

active_refs=0
for workflow in "${workflows[@]}"; do
    if rg -q 'github/codeql-action/' "$workflow"; then
        active_refs=$((active_refs + 1))
        if rg -n 'github/codeql-action/[^ ]+@v3' "$workflow"; then
            echo "FAIL: stale CodeQL v3 reference in $workflow" >&2
            exit 1
        fi
        rg -q 'github/codeql-action/[^ ]+@v4' "$workflow"
    fi
done

test "$active_refs" -gt 0
echo "CodeQL maintenance gate passed: $active_refs active workflow file(s) on v4"
