def median($values):
  ($values | sort) as $sorted
  | ($sorted | length) as $count
  | if $count == 0 then null
    elif ($count % 2) == 1 then $sorted[($count / 2 | floor)]
    else (($sorted[($count / 2 | floor) - 1] + $sorted[($count / 2 | floor)]) / 2)
    end;

.candidates as $candidates
| [ $candidates[] | select(.eligible == true and .conclusion == "success" and .run_attempt == 1 and .event == "pull_request" and (.route_mode == "passive_docs_only" or .route_mode == "dependency_closure") and (.tree_sha | type == "string" and length > 0) and (.workflow_file_revision | type == "string" and length > 0) and (.classifier_revision | type == "string" and length > 0) and (.path_producer_revision | type == "string" and length > 0) and (.runner_inventory | type == "object" and length > 0)) ] as $routed
| [ $candidates[] | select(.eligible == true and .conclusion == "success" and .run_attempt == 1 and .event == "workflow_dispatch" and .route_mode == "full_pr_matrix" and .force_full_requested == true and (.tree_sha | type == "string" and length > 0) and (.workflow_file_revision | type == "string" and length > 0) and (.classifier_revision | type == "string" and length > 0) and (.path_producer_revision | type == "string" and length > 0) and (.runner_inventory | type == "object" and length > 0)) ] as $controls
| [
    $routed[] as $pr
    | $controls[] as $control
    | select($pr.tree_sha == $control.tree_sha)
    | select($pr.workflow_file_revision == $control.workflow_file_revision)
    | select($pr.classifier_revision == $control.classifier_revision)
    | select($pr.path_producer_revision == $control.path_producer_revision)
    | select([ $pr.runner_inventory | to_entries[] | . as $entry | $control.runner_inventory[$entry.key] == $entry.value ] | all)
    | {
        route_shape: (if $pr.route_mode == "passive_docs_only" then "passive-docs" else "dependency-closure" end),
        tree_sha: $pr.tree_sha,
        pr: $pr,
        control: $control
      }
  ]
| sort_by(.route_shape, .pr.created_at, .control.created_at)
| group_by(.route_shape + ":" + .tree_sha)
| map(.[0])
| . as $pairs
| ([ $pairs[] | .pr.run_id, .control.run_id ] | unique) as $paired_run_ids
| ["passive-docs", "dependency-closure"] as $shapes
| {
    schema_version: 1,
    acceptance: {
      distinct_pairs_per_shape: 10,
      optimized_execution_median_max_ratio: 0.65
    },
    candidate_runs: $candidates,
    candidate_dispositions: [
      $candidates[] as $candidate
      | if $candidate.eligible != true then $candidate + {disposition:"excluded",reason:($candidate.rejection_reason // "candidate is ineligible")}
        elif ($paired_run_ids | index($candidate.run_id)) != null then $candidate + {disposition:"paired",reason:"matched on executed code tree, workflow file content, route revisions, and measured-job runner definitions"}
        else $candidate + {disposition:"unmatched",reason:"no compatible opposite-event run or this tree was already counted"}
        end
    ],
    pairs: $pairs,
    summary: [
      $shapes[] as $shape
      | [ $pairs[] | select(.route_shape == $shape) ] as $shape_pairs
      | (median([ $shape_pairs[].pr.queue_duration_ms ])) as $pr_queue
      | (median([ $shape_pairs[].control.queue_duration_ms ])) as $control_queue
      | (median([ $shape_pairs[].pr.execution_duration_ms ])) as $pr_execution
      | (median([ $shape_pairs[].control.execution_duration_ms ])) as $control_execution
      | (if $control_execution == null or $control_execution == 0 then null else ($pr_execution / $control_execution) end) as $ratio
      | {
          route_shape: $shape,
          valid_pairs: ($shape_pairs | length),
          routed_queue_median_ms: $pr_queue,
          full_control_queue_median_ms: $control_queue,
          routed_execution_median_ms: $pr_execution,
          full_control_execution_median_ms: $control_execution,
          optimized_to_full_execution_ratio: $ratio,
          status: (if ($shape_pairs | length) < 10 then "insufficient_pairs" elif $ratio <= 0.65 then "pass" else "threshold_missed" end)
        }
    ]
  }
