def median($values):
  ($values | sort) as $sorted
  | ($sorted | length) as $count
  | if $count == 0 then null
    elif ($count % 2) == 1 then $sorted[($count / 2 | floor)]
    else (($sorted[($count / 2 | floor) - 1] + $sorted[($count / 2 | floor)]) / 2)
    end;

.candidates as $candidates
| [ $candidates[] | select(.eligible == true and .conclusion == "success" and .run_attempt == 1 and .event == "pull_request" and (.control_pr_number // 0) == 0 and (.route_mode == "passive_docs_only" or .route_mode == "dependency_closure") and (.execution_sha | type == "string" and length > 0) and (.tree_sha | type == "string" and length > 0) and (.workflow_file_revision | type == "string" and length > 0) and (.classifier_revision | type == "string" and length > 0) and (.path_producer_revision | type == "string" and length > 0) and (.runner_inventory | type == "object" and length > 0)) ] as $routed
| [ $candidates[] | select(.eligible == true and .conclusion == "success" and .run_attempt == 1 and .event == "pull_request" and .route_mode == "full_pr_matrix" and .force_full_requested == true and (.execution_sha | type == "string" and length > 0) and (.control_tree_sha == .execution_sha) and (.control_pr_number | type == "number" and . > 0) and (.control_pr_head_sha | type == "string" and length > 0) and (.tree_sha | type == "string" and length > 0) and (.workflow_file_revision | type == "string" and length > 0) and (.classifier_revision | type == "string" and length > 0) and (.path_producer_revision | type == "string" and length > 0) and (.runner_inventory | type == "object" and length > 0)) ] as $controls
| [
    $routed[] as $pr
    | $controls[] as $control
    | select($pr.execution_sha == $control.execution_sha)
    | select($pr.pr_number == $control.control_pr_number)
    | select($pr.head_sha == $control.control_pr_head_sha)
    | select($pr.tree_sha == $control.tree_sha)
    | select($pr.workflow_file_revision == $control.workflow_file_revision)
    | select($pr.classifier_revision == $control.classifier_revision)
    | select($pr.path_producer_revision == $control.path_producer_revision)
    | select([ $pr.runner_inventory | to_entries[] | . as $entry | $control.runner_inventory[$entry.key] == $entry.value ] | all)
    | {
        route_shape: (if $pr.route_mode == "passive_docs_only" then "passive-docs" else "dependency-closure" end),
        tree_sha: $pr.tree_sha,
        execution_sha: $pr.execution_sha,
        control_pr_number: $control.control_pr_number,
        control_tree_sha: $control.control_tree_sha,
        control_pr_head_sha: $control.control_pr_head_sha,
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
    schema_version: 4,
    acceptance: {
      distinct_pairs_per_shape: 10,
      accepted_metric: "workflow updatedAt minus createdAt",
      optimized_to_full_end_to_end_latency_median_max_ratio: 0.65,
      diagnostic_metrics: ["workflow_queue_ms", "per_job_scheduling_ms", "aggregate_execution_work_ms"]
    },
    candidate_runs: $candidates,
    candidate_dispositions: [
      $candidates[] as $candidate
      | if $candidate.eligible != true then $candidate + {disposition:"excluded",reason:($candidate.rejection_reason // "candidate is ineligible")}
        elif ($paired_run_ids | index($candidate.run_id)) != null then $candidate + {disposition:"paired",reason:"matched on executed code tree, workflow file content, route revisions, successful measured jobs, and runner definitions"}
        else $candidate + {disposition:"unmatched",reason:"no compatible opposite-route run or this tree was already counted"}
        end
    ],
    pairs: $pairs,
    summary: [
      $shapes[] as $shape
      | [ $pairs[] | select(.route_shape == $shape) ] as $shape_pairs
      | (median([ $shape_pairs[].pr.workflow_queue_ms ])) as $pr_queue
      | (median([ $shape_pairs[].control.workflow_queue_ms ])) as $control_queue
      | (median([ $shape_pairs[].pr.end_to_end_latency_ms ])) as $pr_latency
      | (median([ $shape_pairs[].control.end_to_end_latency_ms ])) as $control_latency
      | (median([ $shape_pairs[].pr.per_job_scheduling_ms ])) as $pr_job_scheduling
      | (median([ $shape_pairs[].control.per_job_scheduling_ms ])) as $control_job_scheduling
      | (median([ $shape_pairs[].pr.aggregate_execution_work_ms ])) as $pr_execution_work
      | (median([ $shape_pairs[].control.aggregate_execution_work_ms ])) as $control_execution_work
      | (if $control_latency == null or $control_latency == 0 then null else ($pr_latency / $control_latency) end) as $ratio
      | {
          route_shape: $shape,
          valid_pairs: ($shape_pairs | length),
          routed_workflow_queue_median_ms: $pr_queue,
          full_control_workflow_queue_median_ms: $control_queue,
          routed_end_to_end_latency_median_ms: $pr_latency,
          full_control_end_to_end_latency_median_ms: $control_latency,
          optimized_to_full_end_to_end_latency_ratio: $ratio,
          routed_per_job_scheduling_median_ms: $pr_job_scheduling,
          full_control_per_job_scheduling_median_ms: $control_job_scheduling,
          routed_aggregate_execution_work_median_ms: $pr_execution_work,
          full_control_aggregate_execution_work_median_ms: $control_execution_work,
          status: (if ($shape_pairs | length) < 10 then "insufficient_pairs" elif $ratio <= 0.65 then "pass" else "threshold_missed" end)
        }
    ]
  }
