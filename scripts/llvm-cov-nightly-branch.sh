#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TOOLCHAIN="${RUST_TOOLCHAIN_NIGHTLY:-nightly}"
RUSTC_VERSION="$(rustup run "$TOOLCHAIN" rustc -Vv)"
grep -q '^release: 1\.99\.0-nightly$' <<<"$RUSTC_VERSION" || {
  echo "nightly branch gate requires Rust 1.99.0-nightly" >&2
  exit 1
}
HOST_TRIPLE="$(sed -n 's/^host: //p' <<<"$RUSTC_VERSION")"
SYSROOT="$(rustup run "$TOOLCHAIN" rustc --print sysroot)"
LLVM_BIN="$SYSROOT/lib/rustlib/$HOST_TRIPLE/bin"
REPORT="$(mktemp "${TMPDIR:-/tmp}/tachi-nightly-coverage.XXXXXX.json")"
trap 'rm -f "$REPORT"' EXIT

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target/llvm-cov-nightly}"
export RUSTC="${RUSTC:-$(rustup which --toolchain "$TOOLCHAIN" rustc)}"
export RUSTDOC="${RUSTDOC:-$(rustup which --toolchain "$TOOLCHAIN" rustdoc)}"
export LLVM_COV="${LLVM_COV:-$LLVM_BIN/llvm-cov}"
export LLVM_PROFDATA="${LLVM_PROFDATA:-$LLVM_BIN/llvm-profdata}"

cd "$ROOT"
rustup run "$TOOLCHAIN" cargo llvm-cov --workspace --branch --json --summary-only \
  --output-path "$REPORT" "$@"

jq -e '.data[0].totals.branches.percent >= 85' "$REPORT" >/dev/null
jq -r '"nightly branch coverage: " + (.data[0].totals.branches.percent | tostring) + "%"' "$REPORT"
