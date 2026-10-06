def expected_name($key):
  if $key == "route" then "route decision and stable orchestrator check"
  elif $key == "repository-contracts" then "repository-wide contract tests"
  elif ($key | startswith("cargo-test-")) then
    "cargo test -p " + ($key[11:])
  elif ($key | startswith("shell-tests-")) then
    "cargo test -p tachi-shell (" + ($key[12:]) + ")"
  else error("unknown measured job key: " + $key)
  end;

def timestamp($value):
  if ($value | type) == "string" and ($value | length) > 0 then
    try ($value | fromdateiso8601) catch null
  else null end;

($runner_inventory | to_entries) as $measured
| [
    $measured[] as $entry
    | expected_name($entry.key) as $name
    | [ $run_jobs[] | select(.name == $name) ] as $matches
    | (if ($matches | length) != 1 then
        error("expected exactly one GitHub job named " + $name)
      else $matches[0] end) as $job
    | timestamp($job.created_at) as $created
    | timestamp($job.started_at) as $started
    | timestamp($job.completed_at) as $completed
    | select($job.conclusion == "success")
    | if $created == null or $started == null or $completed == null
      or $started < $created or $completed < $started then
        error("job timestamps are missing, invalid, or out of order for " + $name)
      else
        {
          key: $entry.key,
          name: $name,
          conclusion: $job.conclusion,
          created_at: $job.created_at,
          started_at: $job.started_at,
          completed_at: $job.completed_at,
          scheduling_ms: (($started - $created) * 1000),
          execution_ms: (($completed - $started) * 1000)
        }
      end
  ] as $timings
| if ($timings | length) != ($measured | length) then
    error("one or more measured jobs did not succeed")
  else
    {
      job_timings: (reduce $timings[] as $timing ({}; .[$timing.key] = ($timing | del(.key)))),
      per_job_scheduling_ms: ([ $timings[].scheduling_ms ] | add // 0),
      aggregate_execution_work_ms: ([ $timings[].execution_ms ] | add // 0)
    }
  end
