#!/usr/bin/env bash
set -euo pipefail

case "${1:-}" in
    approval)
        printf 'approved\n'
        ;;
    timeout|cancel)
        child_pid=0
        cleanup() {
            if [ "$child_pid" -gt 0 ]; then
                kill "$child_pid" 2>/dev/null || true
                wait "$child_pid" 2>/dev/null || true
            fi
            exit 143
        }
        trap cleanup INT TERM
        sleep 60 &
        child_pid=$!
        if [ -n "${AGENTIC_FAKE_TOOL_CHILD_PID_FILE:-}" ]; then
            printf '%s\n' "$child_pid" >> "$AGENTIC_FAKE_TOOL_CHILD_PID_FILE"
        fi
        wait "$child_pid"
        ;;
    circuit_breaker)
        printf 'synthetic tool failure\n' >&2
        exit 75
        ;;
    *)
        printf 'fake-tool: unsupported case\n' >&2
        exit 2
        ;;
esac
