# Phase 3 independent review — Security Auditor

Score: **7.5/10**. Agreement intensity: 30%. Strategy: adversarial simulation. Mode: Precise. Recommendation: **request targeted corrections** for two P2 regressions. No P0/P1 vulnerability established.

Reviewed pinned main `8df554e884b1e5dd24146111a965597eff5f4779` against `dd3b293d81d358d1ae27424be83b720693539112` in `.worktrees/overseer-main-20261004`. Read context, Phase 2 trace, codemap, relevant shared diff sections, actual source/consumers/tests, and Overseer Phase 3 instructions. Did not read other reviewers. Graph discovery returned stale source positions; pinned files are authoritative. Memory informed cleanup-contract discovery only; findings were rechecked against current source and base.

## Findings

## SEC-1 [P2] [EXISTING_DEFECT] Cleanup deletes the sole backing image when the retained sibling is a symlink

- **Location:** `crates/tachi-core/src/assets.rs:43` and `:51`; equality helper `:61-64`. CLI invocation: `crates/tachi-cli/src/bin/report-data.rs:24-26`.
- **Trigger:** A PNG image is stored as `threat-risk-funnel.jpg`, and correctly named `threat-risk-funnel.png` is a relative symbolic link to that JPG. Invoke report-data with `--cleanup-mislabeled-images`.
- **Consequence:** Detection selects the PNG symlink because its bytes match its extension. Both `image_format` and `fs::read` follow it, so byte comparison succeeds. Cleanup removes the JPG regular file, leaving the retained PNG dangling. The only image bytes disappear; the generated image binding now references a nonexistent image. This is permanent local data loss.
- **Guard search:** Inspected `detect_images`, `choose_image`, `image_format`, `files_are_identical`, and CLI order; searched assets/report/CLI tests for symlink checks. There is no `symlink_metadata` or retained-file-independence guard. Catalog regeneration has separate symlink rejection, but this CLI path does not call it. Equality, opt-in flag, fixed stems, and nonfatal deletion errors exist; none prevent this successful harmful unlink. Hard links do not reproduce this defect because the retained link preserves bytes.
- **Introduction:** Cleanup is new in `35be8778c3aacea179224676d42c45a6f50a33b7` (blame verified). Base `dd3b293d` has detection but no cleanup deletion; the alias was usable.
- **Evidence:** A standalone Rust harness directly includes pinned `assets.rs`, creates private temporary files, calls detection and cleanup, and exits successfully with:

  ```text
  before: jpg=true png=true binding=Some("./threat-risk-funnel.png")
  after: jpg=false png=false dangling_symlink=true
  ```

  Harness: `/tmp/tachi-sec-review.VKfdxm/repro.rs`; fixture: `/tmp/tachi-sec-review.VKfdxm/fixture`. The fixture contains a PNG signature plus sentinel bytes because production only checks the signature; deletion is independent of image decoding and applies equally to a complete valid PNG.
- **Confidence:** High; actual pinned helper reproduced.
- **Fix/test:** Skip cleanup unless candidate and retained counterpart are ordinary non-symlink files, or otherwise prove the retained path survives unlink. Add Unix regression cases for PNG/JPG and JPEG/PNG symlink aliases, asserting retained bytes and selected bindings stay readable. Preserve ordinary regular-file duplicate cleanup.
- **Read-only `verification_command`:**

  ```sh
  rtk proxy sh -c 'test ! -e /tmp/tachi-sec-review.VKfdxm/fixture/threat-risk-funnel.jpg && test -L /tmp/tachi-sec-review.VKfdxm/fixture/threat-risk-funnel.png && test ! -e /tmp/tachi-sec-review.VKfdxm/fixture/threat-risk-funnel.png && readlink /tmp/tachi-sec-review.VKfdxm/fixture/threat-risk-funnel.png'
  ```

  This checks the retained reproduction fixture without deleting anything. The harness itself mutates only private fixtures and is not the read-only verification command.

## SEC-2 [P2] [EXISTING_DEFECT] Attack-tree IDs select images outside the report directory

- **Location:** `crates/tachi-core/src/report_data.rs:232-244`. Input acceptance: `crates/tachi-core/src/attack_trees.rs:111` and `:124-130`; consumer: `templates/tachi/security-report/attack-path.typ:69-74`.
- **Trigger:** A tree artifact contains `# Attack Tree: ../../private/private -- Unrelated image`, high risk metadata, and a Mermaid block. A neighboring directory contains `private/private-attack-tree.svg`. No matching raw finding is necessary: supplied high severity and this ID are accepted. Run normal report-data on `report/`.
- **Consequence:** The new lookup concatenates unvalidated ID and image suffix; `is_file` resolves parent components. The generator emits `has-image=true` and an image path outside the selected report. The template then passes it to `image()` instead of displaying Mermaid fallback. Lower-trust Markdown can thus select a known neighboring image, contaminating a report or disclosing that image when the PDF is shared.
- **Scope limits:** Local generation, not an exposed HTTP service. Attacker must influence report Markdown, know an existing image with the expected suffix/extension, and have that image within the compiler's broader project root. No arbitrary text-file read, network exfiltration, shell execution, or RCE established. [Typst path documentation](https://www.typst.app/docs/reference/foundations/path/) confirms the project-root restriction. The escape here is from the selected report into an adjacent directory inside that root. The Rust generator itself probes existence before compilation.
- **Guard search:** Read tree parser, image closure, relative-path helper, consumer, and tests. The alternative `ID: title` heading branch has an alphanumeric/hyphen check, but `Attack Tree:` and metadata `Finding ID` branches do not. No ID grammar, canonical containment check, or matched-finding requirement guards lookup. `relative_path` preserves parent components. No Typst `eval` is required.
- **Introduction:** Lookup and wiring added in `c933cd4bb6b7a99a1e07ce56f940dbccab0d6cc4` (blame verified). Base `dd3b293d` did not project tree IDs into file lookup or emit tree-image data in report-data. Permissive parsing predates the window; its new filesystem composition introduces this regression.
- **Evidence:** Built actual pinned CLI offline with `CARGO_TARGET_DIR=/private/tmp/tachi-overseer-target cargo build --offline --locked -p tachi-cli --bin report-data`, exit 0. Private fixture `/tmp/tachi-sec-path.7Qm3Cu` generated:

  ```text
  #let has-attack-trees = true
  #let attack-trees = (("id": "../../private/private", ... "has-image": true,"image-path": "../../../report/attack-trees/../../private/private-attack-tree.svg", ...),)
  report-data.typ generated
  ```

  Selected SVG is outside `report/`. PDF compilation was not executed; the consumer is directly traced to `image(img-path, ...)`.
- **Confidence:** High for actual path-selection escape; medium for disclosure because compilation and sharing depend on caller workflow/root.
- **Fix/test:** Validate IDs against intended finding-ID grammar before filename use. Canonicalize candidate and image directory and require containment, including symlink handling. Invalid IDs should use safe text fallback or return a contextual error. Cover parent-relative and absolute IDs, symlinked images, valid case variations, and a neighboring report image that must never be selected.
- **Read-only `verification_command`:**

  ```sh
  rtk proxy sh -c '/private/tmp/tachi-overseer-target/debug/report-data --target-dir /tmp/tachi-sec-path.7Qm3Cu/report --template-dir /tmp/tachi-sec-path.7Qm3Cu/templates/tachi/security-report | rg "^#let (has-attack-trees|attack-trees)"'
  ```

  Fixture has no top-level image stems, so detection cannot invoke its legacy correction-copy behavior; no output path or cleanup flag is supplied.

## Positive findings and rejected escalation candidates

- Typst serialization escapes backslashes and quotes, dictionary keys are quoted, binding names are static, and inspected consumers render strings/raw Mermaid rather than evaluating Markdown as code. No demonstrated source injection.
- Cleanup uses fixed stems and byte equality; SEC-1 is retained-copy dependence, not arbitrary outside-root deletion.
- Catalog regeneration rejects input symlinks, stages renders, verifies PDF envelopes/provenance, rechecks inputs, and attempts rollback. No concrete external-attacker exploit established. Direct publication writes are not crash-atomic; that is a reliability limitation, not a demonstrated security escalation.
- Next.js proxy preserves old matcher, `getUser()` verification, and cookie logic. RLS migration has own-ID SELECT/INSERT/UPDATE policies. Tests cover anonymous read/insert denial, own read/insert, cross-user update, and forged insertion. Policy shape matches [Supabase guidance](https://supabase.com/docs/guides/database/postgres/row-level-security). No auth-bypass regression established.
- New `lib/prisma.ts` does not propagate user claims, but no scaffold application callers were found. Future callers need authorization; an unused privileged client is not an existing cross-tenant endpoint.
- Checked report-data now propagates primary attribution failures through CLI/MCP. Phase 2 optional-artifact/tier candidates were not independently elevated to hostile-service vulnerabilities here.

## Coverage and limits

Focused on file/input/output compositions, cleanup/catalog publication, Typst sinks, and auth/RLS. Also inspected permission checker and MCP diff. Not an exhaustive audit of all 48-hour files. No Python or product edits. No live database, browser session, hostile SVG decoder, production deployment, full workspace suite, or PDF compile exercised. Offline pinned CLI build and private-fixture probes completed; other evidence is static. Phase 3 ends here.
