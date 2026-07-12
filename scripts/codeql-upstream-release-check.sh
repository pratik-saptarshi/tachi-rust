#!/usr/bin/env bash
set -euo pipefail

policy="docs/security/codeql-maintenance.md"
api_url="https://api.github.com/repos/github/codeql-action/releases?per_page=30"

test -s "$policy"
command -v curl >/dev/null
command -v jq >/dev/null

expected_tag="$(rg -o 'CodeQL Action `v[0-9]+\.[0-9]+\.[0-9]+`' "$policy" | sed -E 's/.*`([^`]*)`.*/\1/' | head -n 1)"
test -n "$expected_tag"
case "$expected_tag" in
    v4.*) ;;
    *) echo "FAIL: policy is not on the supported CodeQL v4 release line: $expected_tag" >&2; exit 1 ;;
esac

releases="$(curl --fail --silent --show-error --location --max-time 20 \
    -H 'Accept: application/vnd.github+json' \
    -H 'X-GitHub-Api-Version: 2022-11-28' \
    "$api_url")"
latest_tag="$(jq -r '[.[] | select(.prerelease == false) | select(.draft == false) | select(.tag_name | startswith("v4."))] | .[0].tag_name // empty' <<<"$releases")"
test -n "$latest_tag"

echo "CodeQL upstream release check: policy=$expected_tag latest=$latest_tag"
if [[ "$expected_tag" != "$latest_tag" ]]; then
    echo "FAIL: CodeQL v4 policy is stale; review the upstream release before updating the pinned mapping" >&2
    exit 1
fi

echo "CodeQL upstream release check passed: $expected_tag is the latest non-prerelease v4 tag"
