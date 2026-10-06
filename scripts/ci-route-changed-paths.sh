#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -ne 2 ]; then
  echo "usage: ci-route-changed-paths.sh <base-ref> <head-ref>" >&2
  exit 2
fi

base_ref="$1"
head_ref="$2"

git diff --name-status --find-renames -z "${base_ref}...${head_ref}" |
  while IFS= read -r -d '' status; do
    case "$status" in
      R*|C*)
        IFS= read -r -d '' old_path
        IFS= read -r -d '' new_path
        printf '%s\n%s\n' "$old_path" "$new_path"
        ;;
      *)
        IFS= read -r -d '' path
        printf '%s\n' "$path"
        ;;
    esac
  done
