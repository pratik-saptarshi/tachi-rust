# Phase 8 — Independent Completeness Audit

## Coverage

Read context, personas, Phase 2, both Phase 6 summaries, all six Phase 7 finals, and relevant Correctness/Pipeline Phase 3 findings. Compared pinned main `8df554e884b1e5dd24146111a965597eff5f4779` with base `dd3b293d81d358d1ae27424be83b720693539112`. Reviewed the full seven-commit interval inventory before considering temporal claims; no new temporal/event-frequency claim is made. This audit searches for omissions, not panel score quality.

Additional focus: MAESTRO duplicate evidence/state reduction, actual curl process behavior, permissions malformed-input guards, workflow triggers, and scaffold runtime wiring. Graph search was first for MAESTRO discovery; pinned source overrides stale graph. Current runtime probes used the supplied pinned report-data binary and a freshly successful offline build of taxonomy-link-monitor into `/private/tmp/tachi-overseer-target`. No product edits or Python execution occurred.

## New Findings

### AUD-1 — P2 / DEFECT / High confidence: duplicate MAESTRO layer rows can discard positive evidence and report clean

**Source:** `crates/tachi-core/src/infographic.rs:273-289`, with state propagation at `:292-306` and attempted per-finding repair at `:443-474`.

**Trigger:** A summary-only threats.md contains the canonical `#### Risk by MAESTRO Layer` table with rows `L1 | 2 | High` and then `L1 | 0 | Clean`. Duplicate layer IDs are inconsistent evidence, but must not silently erase known positive evidence. A report containing layer summary evidence without detailed finding rows is supported by existing zero-finding/report-state tests.

**Observed:** The pinned report-data command succeeds and emits exactly `(layer-id: "L1", layer-name: "Foundation Model", finding-count: 0, coverage-state: "clean", coverage-label: "Evaluated — no findings")`, with `most-exposed-layer = ""`. Positive High evidence in the same input is lost. This is distinct from the panel optional risk/controls tier replacement findings.

**Base:** Base parser appended every parsed summary row to a Vec. Both rows survived and most-exposed selection retained the positive count. The new map insertion introduces last-row-wins evidence loss; no old clean-state contract is assumed.

**Guard check:** Inspected complete parser, classifier, extraction reconciliation and MAESTRO state tests. `classify_evaluation` correctly prioritizes a positive count within one row, but only runs before duplicate reduction. `extract_maestro_data` repairs from per-finding rows only when those rows exist; the reproducer has none. `verify_states` compares already-reduced output and cannot recover discarded source evidence. No duplicate-input rejection exists.

**Fix/regression:** Reject conflicting duplicate canonical layer IDs or merge conservatively so positive findings dominate clean/not-applicable states. Test both row orders, canonical label aliases, and summary-only inputs; counts must not be summed blindly across duplicates.

### AUD-2 — P2 / DEFECT / High confidence: exhausted HTTP redirects are reported as healthy citations

**Source:** `crates/tachi-cli/src/bin/taxonomy-link-monitor.rs:157-163`; curl error capture `:148-154`; HEAD-to-GET fallback `:113-119`.

**Trigger:** A citation endpoint responds with an endless HTTP 302 redirect to itself. Real curl follows redirects, terminates with exit 47, and still writes final HTTP code 302. Both HEAD and GET fail this way.

**Observed:** Using a private Node loopback server and real installed curl, freshly built pinned monitor returned success with summary `Checked 1 URLs: 1 healthy, 0 needs review, 0 broken, 0 transient` and JSON containing `http_status: 302`, `status: "healthy"`, and `error: "curl: (47) Maximum (50) redirects followed"`. The failed HEAD did invoke the fallback, but the final failure is ignored by the earlier 200..=399 classifier arm. This is an informational-monitor false negative, not CI bypass or evidence that any current external citation is broken.

**Base:** Entire binary is absent at base. Its new health classification introduces this defect.

**Guard check:** Reviewed complete curl invocation, fallback, classifier, summary and tests. Tests deliberately expect `classify(204, true) == "healthy"`, but no real redirect-loop probe challenges that policy. The error remains in JSON, which limits concealment, yet summary and status are wrong. Existing timeout, range request and retry guards do not make failed redirects healthy.

**Fix/regression:** Prioritize transport/process failures over a residual successful/redirect HTTP code, at least for redirect exhaustion; emit transient or needs-review rather than healthy. Add real local HTTP redirect-loop coverage and a classifier regression for 302 plus failure.

## Rejected Candidates

- Permissions allow/ask/deny category collisions: checker promises existing AC-2 membership consistency; combined set behavior matches that scope. JSON syntax, missing/non-array sections, nonstring and blank rules are rejected. No new category-policy guarantee established.
- Next.js redirect discards refreshed cookies: same redirect construction exists in base middleware; excluded as preexisting. Current getUser validation remains present.
- Prisma adapter is not tenant-context-bound: no newly exposed Prisma application endpoint established. Existing policy/verification checks and unused-client limitations prevent elevating a hypothetical cross-tenant issue.
- Curl invalid-link findings fail to gate CI: explicitly informational by workflow and command contract, so successful command exit itself is not a defect.
- MAESTRO component mapping as evidence: implementation explicitly avoids treating mappings alone as completed analysis. New finding concerns conflicting positive summary rows only.

## Limitations

Focused supplemental audit, not exhaustive scaffold/browser/SQL execution or whole-workspace validation. No external web request, live taxonomy health claim, PDF compilation, Windows build or production-frequency estimate. The first loopback attempt was blocked by sandbox socket policy; a narrow approved escalation then completed the actual-curl reproduction. The initial report-data invocation used an incorrect flag and was corrected before obtaining the quoted successful output. Binary build success is not a suite-pass claim. All agents share the same underlying model; panel independence is not model diversity. Memory registry was consulted only for graph/workspace orientation (MEMORY.md lines 58 and 67), not current behavioral evidence.
