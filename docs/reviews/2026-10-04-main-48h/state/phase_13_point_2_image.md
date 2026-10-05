# Phase 13 point 2 — Empty diagram image PDF consequence

**Tier: Deep. VR verdict: [VR_CONFIRMED] for COR-3/ARC-2's attack-tree binding, fallback bypass, and actual PDF compilation failure. Priority: retain P2. Confidence: high for the exercised path.** The formerly inferred decoder consequence is now executed: report-data succeeds with a zero-byte PNG, but the unchanged canonical main template fails to compile. Removing only that PNG from the private copied fixture restores successful PDF generation through the available Mermaid-source fallback.

## Scope and provenance

Read Phase 12a/12b, Phase 10, and COR-3/ARC-2 prior evidence. Repository atlas was read before source inspection. Graph discovery was attempted first; searches returned `project not found or not indexed`, including the available-project name, so exact existing source citations were inspected directly. This probe did not rely on graph contents. The preferred lean-ctx executable is absent; execution used `rtk proxy`. No Python, original-fixture mutation, product edits, network publication, or broad test campaign.

- Pinned worktree HEAD: `8df554e884b1e5dd24146111a965597eff5f4779`.
- Typst: `/private/tmp/tachi-typst-0.15.1/typst-x86_64-apple-darwin/typst`; observed `typst 0.15.1 (9dfd3a08)`; SHA-256 `a6fb5786e71f6f95d8323b326353770734c1e4624e7c108783ea578723916b46`.
- Report CLI: `/private/tmp/tachi-overseer-target/debug/report-data`; SHA-256 `b6a7b534d0cede8ce9d01a4bdbbfe3b695ae706fcc547ff114fb01998f806318`. Used supplied pinned executable; this phase did not independently rebuild or attest executable/source correspondence.
- Original retained fixture: `/private/tmp/overseer-correctness-DpxDFG/empty-tree-image`.
- Private evidence tree: `/private/tmp/overseer-phase13-image-Sp9rCi`. Original zero-byte PNG remains in place; its SHA-256 is `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.

## Executed commands

Executed from `/Volumes/dev/Git-SCM/tachi-rust/.worktrees/overseer-main-20261004`; the script below is the exact probe command with formatting expanded. `--root` includes both copied templates and copied input, so the failure is not a root-access restriction. All templates were copied unchanged; `cmp` also confirmed the copied main and attack-path templates equal their source files.

```sh
rtk proxy /private/tmp/tachi-typst-0.15.1/typst-x86_64-apple-darwin/typst --version
rtk proxy sh -c '
probe_dir=$(mktemp -d /private/tmp/overseer-phase13-image-XXXXXX)
printf "%s\n" "$probe_dir" > /private/tmp/overseer-phase13-image-path
cp -R /private/tmp/overseer-correctness-DpxDFG/empty-tree-image "$probe_dir/fixture"
cp -R templates/tachi/security-report "$probe_dir/template"
/private/tmp/tachi-overseer-target/debug/report-data --target-dir "$probe_dir/fixture" --template-dir "$probe_dir/template" --output "$probe_dir/template/report-data.typ" > "$probe_dir/bad-generate.stdout" 2> "$probe_dir/bad-generate.stderr"
printf "bad_generate_exit=%s\n" "$?"
cp "$probe_dir/template/report-data.typ" "$probe_dir/bad-report-data.typ"
/private/tmp/tachi-typst-0.15.1/typst-x86_64-apple-darwin/typst compile --root "$probe_dir" "$probe_dir/template/main.typ" "$probe_dir/bad.pdf" > "$probe_dir/bad-compile.stdout" 2> "$probe_dir/bad-compile.stderr"
printf "bad_compile_exit=%s\n" "$?"
mv "$probe_dir/fixture/attack-trees/S-1-attack-tree.png" "$probe_dir/zero-byte.png"
/private/tmp/tachi-overseer-target/debug/report-data --target-dir "$probe_dir/fixture" --template-dir "$probe_dir/template" --output "$probe_dir/template/report-data.typ" > "$probe_dir/control-generate.stdout" 2> "$probe_dir/control-generate.stderr"
printf "control_generate_exit=%s\n" "$?"
cp "$probe_dir/template/report-data.typ" "$probe_dir/control-report-data.typ"
/private/tmp/tachi-typst-0.15.1/typst-x86_64-apple-darwin/typst compile --root "$probe_dir" "$probe_dir/template/main.typ" "$probe_dir/control.pdf" > "$probe_dir/control-compile.stdout" 2> "$probe_dir/control-compile.stderr"
printf "control_compile_exit=%s\n" "$?"
printf "probe_dir=%s\n" "$probe_dir"
cat "$probe_dir/bad-compile.stderr" "$probe_dir/control-compile.stderr"
ls -l "$probe_dir"/*.pdf
'
```

## Raw result and diagnostic

```text
bad_generate_exit=0
bad_compile_exit=1
control_generate_exit=0
control_compile_exit=0
probe_dir=/private/tmp/overseer-phase13-image-Sp9rCi
error: failed to decode image (unexpected end of file)
   ┌─ ../../../../../../private/tmp/overseer-phase13-image-Sp9rCi/template/attack-path.typ:74:6
   │
74 │       image(img-path, width: 100%, fit: "contain"),
   │       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

`control-compile.stderr` is zero bytes. `control.pdf` is 95,852 bytes; `file` identifies `PDF document, version 1.7, 12 pages`. No `bad.pdf` was created in the fresh private tree. The fixture's source is `graph TD` followed by `A --> B`. Both generated files retain `has-attack-trees = true` and the same S-1/Gateway/High entry and Mermaid source; line 58 of `bad-report-data.typ` has `"has-image": true,"image-path": "../fixture/attack-trees/S-1-attack-tree.png"`, whereas line 58 of `control-report-data.typ` has `"has-image": false,"image-path": ""`. The selected bad PNG is verified zero bytes.

## Causal citations and priority

- `crates/tachi-core/src/report_data.rs:227-239`: PNG/JPG/SVG candidate selection stops at the first `is_file()` result, without validating nonempty or decodable content.
- `crates/tachi-core/src/report_data.rs:243-247`: that presence becomes `has-image` and the relative image path, while Mermaid source remains available.
- `templates/tachi/security-report/attack-path.typ:69-81`: the selected image reaches `image()` at line 74; the alternative branch renders raw Mermaid source at line 79. This is source-text fallback, not Mermaid-to-vector diagram rendering.
- `crates/tachi-core/src/report_data.rs:255-259` shares image selection with attack chains. `templates/tachi/security-report/attack-chain.typ:83-92` likewise invokes `image()` but has **no Mermaid fallback**. Do not describe chains as bypassing a Mermaid fallback.

P2 remains appropriate: an optional incomplete diagram artifact deterministically blocks the entire exercised report PDF even though usable source fallback exists and report-data returns success. The narrow malformed-artifact trigger is established; routine exposure or broader outage urgency is not. Actual compilation strengthens the consequence evidence but does not itself imply P1. A targeted fix should reject unusable candidates, continue to valid alternate formats, and retain fallback; focused regression coverage should include this exact empty-PNG case.

## Limits

Only the attack-tree path was executed. The chain's analogous invalid-image risk is source-traced, not separately runtime-verified. No later-valid JPG/SVG masking probe, arbitrary corrupt nonempty image probe, or neighboring-file disclosure execution was performed. Successful control compilation isolates this failure from missing fonts, packages, assets, and compiler-root setup for this fixture; it does not certify all reports or visual quality. Existing templates and the original fixture were not modified. This is focused integration evidence, not release readiness or a full report test campaign. Persona independence is not model diversity.
