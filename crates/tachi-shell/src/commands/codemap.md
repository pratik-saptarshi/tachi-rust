# crates/tachi-shell/src/commands/

## Responsibility

Contains the bounded process-runtime implementation behind shell control-plane
commands. It separates process spawning/lifecycle policy from stream capture and
final `CommandOutput` construction.

## Design Patterns

- `ScriptExecutor` is a strategy port; `SystemScriptExecutor` is the production
  process adapter and injectable executors can replace it.
- `ScriptOutputSink` is a result-finalization port; `SystemScriptOutputSink`
  delegates to the shared runtime helper.
- Request structs carry explicit execution dependencies and policy values instead
  of relying on hidden mutable state.
- Environment-configurable timeout/output caps have safe defaults; polling allows
  cancellation and process-group termination during execution.

## Data & Control Flow

`run_script_command_with_progress_using` validates the requested script and emits
startup progress, then delegates to `ScriptExecutor::run`. The system executor
spawns the process, concurrently captures capped stdout/stderr, polls for exit,
cancellation, or timeout, terminates the process group when necessary, and passes
the wait/capture outcome to `ScriptOutputSink`. `finalize_script_output` maps the
outcome to normalized status and emits the terminal progress event.

## Integration Points

- Parent `commands.rs` supplies `CommandOutput` and calls this runtime for
  install/init/update/bootstrap scripts.
- `crate::progress` supplies cancellation tokens, reporters, and event emission.
- Uses OS process APIs; Unix builds terminate the whole process group to avoid
  orphan descendants, while non-Unix builds terminate the child directly.
