# RT-CI Matched-Control Readout — 2026-10-06

## Result

The collector found 10 valid matched pairs for each optimized route shape across
20 distinct code trees. `passive-docs` passes the timing threshold;
`dependency-closure` does not. **RT-0sd.2 remains open.**

| Route shape | Pairs | Optimized execution median | Full-control execution median | Ratio | Gate |
|---|---:|---:|---:|---:|---|
| `passive-docs` | 10 | 16.5 s | 122 s | 13.5% | Pass |
| `dependency-closure` | 10 | 257.5 s | 133 s | 193.6% | Miss; required <=65% |

Queue medians are reported separately: both show 0 ms. GitHub's run timestamps
in this collection have one-second precision, so those zeros are timestamp
resolution values and should not be interpreted as a precise queue-time result.

The raw, replayable collector output is
[`rt-ci-matched-control-final-2026-10-06.json`](rt-ci-matched-control-final-2026-10-06.json)
(SHA-256 `2a98c3cf359fc94f06a9b4460832cf29cdf266297fbc7e72f3a1283ca22a3443`).
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

The acceptance requires both route shapes to have 10 valid pairs and each
optimized execution median to be at most 65% of its full-control median.
`dependency-closure` misses at 193.6%, so do not close `RT-0sd.2` or `RT-0vf.5`
and do not mark Phase 4 complete. Record the timing result as a threshold miss
and define follow-up work to reduce dependency-closure execution cost before
collecting a new cohort. Keep the 65% threshold unchanged.
