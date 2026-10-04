# Upstream Feature Adoption Roadmap

**Review date:** 2026-10-03
**Version:** 1
**Fork baseline:** `ddd19ef852e99f805eed23c0062a1b932ba7f462` (`origin/main`)
**Upstream baseline:** `63438d78cfedc7abf4ae6e2f6b3301161246299d` (`davidmatousek/tachi` `main`; last upstream commit in the review window, August 13, 2026). Both refs were verified through `rtk` on 2026-10-03.

## Scope and ranking

Review upstream `main` from May 3 through October 3, 2026. The prior comparison found that PR #37 had already brought the major framework updates, including OWASP LLM 2026, NIST AI 600-1, asset-tag output, and citation-link monitoring. The remaining high-value gaps, ranked by user value, gap severity, fit with the Rust architecture, and effort, are:

1. **Complete taxonomy crosswalk parity — high priority.** The review snapshot reported 582 fork edges and 645 upstream edges. Add the 63 missing edges (57 primary and 6 related), validating endpoints against the matching taxonomy catalogs and preserving upstream citations and ordering. Update documented edge totals. This adopts the remaining mapping work from [upstream PR #323](https://github.com/davidmatousek/tachi/pull/323) and [PR #328](https://github.com/davidmatousek/tachi/pull/328).
2. **Opt-in cleanup for mislabeled image duplicates — medium priority.** Extend the Rust `report-data` CLI with `--cleanup-mislabeled-images`. Delete a mislabeled image only when the flag is supplied and a correctly named, byte-identical counterpart is verified. Cover pre-existing pairs and siblings created during image correction. Cleanup errors must not fail report generation; default behavior preserves files. This ports [upstream PR #351](https://github.com/davidmatousek/tachi/pull/351).

Already adopted or present in the review snapshot: LLM 2026 and NIST catalogs, asset-tag output, citation URL monitor, output-integrity refinements, MAESTRO coverage work, and ordered framework registry. Defer upstream adopter-outreach materials and AOD-specific governance documentation because they add no Rust product capability.

## Interfaces and compatibility

- Crosswalk parity is a data and documentation update; it requires no public API or schema change.
- Image cleanup adds only a CLI flag. Keep it opt-in and out of desktop and MCP interfaces in this increment.
- Preserve the Rust-native runtime and existing schema compatibility.

## Acceptance checks

- Crosswalk: assert the 63 expected edges are present, every taxonomy endpoint resolves, citations are retained, and resulting totals match the updated catalog.
- Image cleanup: test default no-delete behavior, deletion of identical counterparts, preservation of non-identical files and pre-existing sibling collisions, both correction paths, and successful report generation when deletion fails.
- Run relevant Rust tests, formatting, Clippy, taxonomy integrity checks, and workflow checks for changed surfaces.

## Implementation baseline reconciliation

The original active checkout was an older divergent branch (`684167384dcb3c7bb44e9a1a287a5808b4d26630`) with 526 primary entries and changed taxonomy catalogs. To preserve its unrelated dirty changes and apply this roadmap to the stated baseline, implementation is isolated in `.worktrees/feature-adoption-2026-10-03` at fork `main` (`ddd19ef8`). Direct comparison against upstream `main` (`63438d78`) confirms exactly 63 upstream-only edges: 57 primary and 6 related, with no fork-only edges. The implementation takes the edge records in upstream order and retains each upstream citation.
The exact baseline comparison also found six upstream ATLAS endpoint records required by the new edges but absent from the fork catalog. The implementation adds those six catalog rows from upstream's F-A1.3 expansion and updates the catalog count so all 63 edges resolve. This is a data-only prerequisite discovered during endpoint validation.
