# RT-CI Matched-Control Readout — 2026-10-06

## Result

The cohort contains 10 identity-matched PR/control pairs for each route shape
across 20 distinct code trees. The accepted measure is end-to-end workflow
latency, `updatedAt - createdAt`, which includes workflow and downstream job
queue delays as well as execution. Per-job queue and execution work remain
separate diagnostics so the latency result is not mistaken for a cost
attribution. **`passive-docs` passes; `dependency-closure` misses. RT-0sd.2
remains open.**

| Route shape | Pairs | Routed end-to-end latency median | Full-control end-to-end latency median | Ratio | Gate |
|---|---:|---:|---:|---:|---|
| `passive-docs` | 10 | 16.5 s | 122 s | 13.5% | Pass |
| `dependency-closure` | 10 | 257.5 s | 133 s | 193.6% | Miss; <=65% required |

Workflow queue medians are reported separately: both show 0 ms at GitHub's
one-second timestamp precision. Those values cover only workflow creation to
workflow start; job scheduling after the workflow starts is included in the
latency gate. For example, the PR #80 control workflow started at 10:48:36Z
while measured jobs started between 10:49:26Z and 10:52:36Z. This establishes
contributor wait time, but not whether queueing or job runtime caused it; the
collector still lacks per-job timestamps.

The reclassified, replayable collector snapshot is
[`rt-ci-matched-control-final-2026-10-06.json`](rt-ci-matched-control-final-2026-10-06.json)
(SHA-256 `4578b2c1f07eb36ab21f7a6483155f38def202188a7656e1057e8658f2476033`).
Its schema version 3 summary evaluates the end-to-end wall-clock latency gate;
it marks execution-work attribution `not_evaluated`. The snapshot is derived
from the version-1 collector output.
The preserved source collector output has SHA-256
`2a98c3cf359fc94f06a9b4460832cf29cdf266297fbc7e72f3a1283ca22a3443`.
It contains 87 candidate workflow runs, dispositions, all 20 pair records,
route reasons and paths, PR/head and execution SHAs, Git tree SHAs, run IDs and
timestamps, workflow/classifier/path-producer revisions, and runner inventory.

Revalidation confirmed 10 pairs per shape; 20 distinct PR Git trees; identical
PR/control Git tree and execution SHA within every pair; successful first
attempts; and matching workflow-content revision and runner definition within
each pair. Dependency-closure inputs selected `tachi-cli`, `tachi-desktop`,
`tachi-mcp`, and `tachi-shell`, with repository contracts required.

## Pair run IDs

| Shape | PR | Routed run | Full-control run |
|---|---:|---:|---:|
| passive-docs | 69 | 37443944381 | 37444206069 |
| passive-docs | 70 | 37444765268 | 37444927318 |
| passive-docs | 71 | 37445471174 | 37445913303 |
| passive-docs | 72 | 37446729513 | 37447282161 |
| passive-docs | 73 | 37447870314 | 37448374073 |
| passive-docs | 75 | 37448454558 | 37449467333 |
| passive-docs | 76 | 37449424351 | 37450002700 |
| passive-docs | 77 | 37449641296 | 37450345736 |
| passive-docs | 78 | 37450291960 | 37450959398 |
| passive-docs | 79 | 37450444320 | 37450963767 |
| dependency-closure | 80 | 37451973428 | 37452217221 |
| dependency-closure | 81 | 37452012993 | 37452618935 |
| dependency-closure | 82 | 37452018994 | 37452818263 |
| dependency-closure | 83 | 37452020377 | 37452813870 |
| dependency-closure | 84 | 37452120935 | 37452814466 |
| dependency-closure | 85 | 37452273841 | 37453562958 |
| dependency-closure | 86 | 37452369300 | 37453560096 |
| dependency-closure | 87 | 37453052683 | 37453673119 |
| dependency-closure | 88 | 37453102310 | 37453887594 |
| dependency-closure | 89 | 37453146019 | 37453901594 |

All sample PRs are measurement-only and closed without merging. An earlier
PR #80 full-control attempt (`37451705845`) failed because the stale-base
workflow checkout had no merge base after main advanced; it is excluded. PR #80
was refreshed with an ordinary merge commit and its successful pair above uses
the refreshed tree. On PR #87, a Prisma job initially failed to bind port 55432
during concurrent workflow activity; its retry passed, and the successful
matched runs above are the only pair counted.

## Decision and next work

The acceptance requires ten valid pairs per shape and routed median
end-to-end workflow latency (`completed_at - created_at`) no greater than 65%
of the matched full-control median. This includes queue delay and is the
contributor-facing latency gate. The separate diagnostic metrics are workflow
queue (`started_at - created_at`), per-job scheduling wait (sum of job
`started_at - created_at`), and aggregate execution work (sum of job
`completed_at - started_at` for the route, selected package/shell, and
repository-contract jobs). The current cohort passes for `passive-docs` and
misses for `dependency-closure` at 193.6%; per-job diagnostics are still
required before attributing the cause or selecting an optimization. Do not
close `RT-0sd.2` or `RT-0vf.5`, or mark Phase 4 complete, until the missing
diagnostics and latency remediation are complete. Keep the 65% threshold.
