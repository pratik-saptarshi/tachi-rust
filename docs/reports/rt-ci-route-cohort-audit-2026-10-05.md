# RT-CI Route Cohort Eligibility Audit — 2026-10-05

This audit supplements the route-specific timing follow-up in
[`2026-10-05-rt-ci-timing-evidence-followup.md`](../roadmap/2026-10-05-rt-ci-timing-evidence-followup.md).
Its historical candidate count is bounded evidence and is not the timing
acceptance. The canonical acceptance now uses matched full controls as defined
in the follow-up and `RT-0sd.2`.

## Result

A historical scan paired successful `ci-route-observe.yml` and
`rust-workspace.yml` pull-request runs by the exact PR head SHA. It found 37
unique PR heads from the July 2026 route rollout through the classifier-fix
window on 2026-10-05. Each had both successful workflow runs and a downloadable
`route-decision` artifact containing changed paths. The historical artifact
route mode was reconstructed by passing those paths to the current shared
classifier with `event=pull_request` and `ref=refs/pull/<number>/merge`.

All 37 reconstructed as `full_pr_matrix`: 33 because active documentation or
a shared surface was touched, and 4 because unknown non-documentation paths
must stay full. Thus the audit yields **0 eligible historical passive-docs
samples and 0 eligible historical dependency-closure samples**. It yields no
route-specific timing medians. These full-matrix runs cannot stand in for
route-specific observations.

The newest post-correction PR sample available at this audit was PR #55. Its
successful route-observe artifact also reports `full_pr_matrix` because the
change touched active/shared documentation. It is not eligible for either
optimized route shape. At the audit point, the retained historical sample had
no narrowed-route candidates. This does not make the agreed acceptance
unachievable: future matched pairs use a successful routed PR run and a
successful forced-full `workflow_dispatch` control on each identical code
tree, for both `passive-docs` and `dependency-closure`.

## Reproduction and boundaries

The candidate window starts after route rollout commit
`52a0f8e4d603b5bef1f5d058d1db69f3ce56e702` (2026-07-09 20:07 CDT) and ends
before PR #54's classifier correction began (2026-10-05 06:08 UTC). A candidate
was retained only where the final PR head SHA had a successful pull-request
run in each workflow. Its route artifact had to be downloadable and include
`changed_paths`. Each candidate was classified with:

```sh
bash scripts/ci-route-classifier.sh \
  pull_request refs/pull/<PR>/merge <changed-paths-file> false false
```

The result table below links every paired run and records the classifier
reason, exact head SHA, and workspace timing timestamps. Reconstructed
classifications are evidence about route eligibility only; they do not make
historical route artifacts equivalent to route modes that were actually
executed.

| PR | Route-observe run | Workspace run | Head SHA | Workspace created | Workspace started | Workspace completed | Reconstructed route |
|---|---:|---:|---|---|---|---|---|
| [#15](https://github.com/pratik-saptarshi/tachi-rust/pull/15) | [29091065279](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29091065279) | [29091065263](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29091065263) | `fe78e7fd59159e600cafa84576a3e26fad917e8f` | 2026-07-10T11:58:17Z | 2026-07-10T11:58:17Z | 2026-07-10T11:59:50Z | `full_pr_matrix` — active docs or shared surface touched |
| [#16](https://github.com/pratik-saptarshi/tachi-rust/pull/16) | [29175876294](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29175876294) | [29175876300](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29175876300) | `dc22e2457f1601ae3d9e828fc0bd4537e50342d1` | 2026-07-12T01:51:30Z | 2026-07-12T01:51:30Z | 2026-07-12T01:53:05Z | `full_pr_matrix` — active docs or shared surface touched |
| [#17](https://github.com/pratik-saptarshi/tachi-rust/pull/17) | [29176622541](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29176622541) | [29176622576](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29176622576) | `7f5e649b229655f1bd936956168a6f00a0ed6445` | 2026-07-12T02:20:33Z | 2026-07-12T02:20:33Z | 2026-07-12T02:22:05Z | `full_pr_matrix` — active docs or shared surface touched |
| [#18](https://github.com/pratik-saptarshi/tachi-rust/pull/18) | [29177303920](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29177303920) | [29177303911](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29177303911) | `6720c3ffc799ce003c04c97bf6a5cf0fd14fc162` | 2026-07-12T02:48:33Z | 2026-07-12T02:48:33Z | 2026-07-12T02:49:53Z | `full_pr_matrix` — active docs or shared surface touched |
| [#19](https://github.com/pratik-saptarshi/tachi-rust/pull/19) | [29177694463](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29177694463) | [29177694446](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29177694446) | `8db5636fec6a9cf8af7f52488b78069625fb66bf` | 2026-07-12T03:04:09Z | 2026-07-12T03:04:09Z | 2026-07-12T03:05:28Z | `full_pr_matrix` — active docs or shared surface touched |
| [#20](https://github.com/pratik-saptarshi/tachi-rust/pull/20) | [29177859361](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29177859361) | [29177859336](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29177859336) | `ecd22d1238cfa53fe0789ac26ba4a4f98dd576c5` | 2026-07-12T03:10:48Z | 2026-07-12T03:10:48Z | 2026-07-12T03:12:08Z | `full_pr_matrix` — unknown non-docs paths stay full mode |
| [#21](https://github.com/pratik-saptarshi/tachi-rust/pull/21) | [29178255148](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29178255148) | [29178255153](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29178255153) | `0a96ca6246289555fd13c3f7eaeaf95a617c10b9` | 2026-07-12T03:27:01Z | 2026-07-12T03:27:01Z | 2026-07-12T03:28:41Z | `full_pr_matrix` — active docs or shared surface touched |
| [#22](https://github.com/pratik-saptarshi/tachi-rust/pull/22) | [29181987388](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29181987388) | [29181987410](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29181987410) | `6fb92229a09a79376148f856903e69bbdb7c2630` | 2026-07-12T06:00:08Z | 2026-07-12T06:00:08Z | 2026-07-12T06:01:39Z | `full_pr_matrix` — active docs or shared surface touched |
| [#23](https://github.com/pratik-saptarshi/tachi-rust/pull/23) | [29200679866](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29200679866) | [29200679869](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29200679869) | `c55be147a684498be102d65f393523f0f6e19fdd` | 2026-07-12T16:44:15Z | 2026-07-12T16:44:15Z | 2026-07-12T16:45:40Z | `full_pr_matrix` — active docs or shared surface touched |
| [#24](https://github.com/pratik-saptarshi/tachi-rust/pull/24) | [29203699746](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29203699746) | [29203699709](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29203699709) | `b4ac92e1cc8cff67ee13d5cafd386ee4dc0efb5f` | 2026-07-12T18:20:49Z | 2026-07-12T18:20:49Z | 2026-07-12T18:22:22Z | `full_pr_matrix` — active docs or shared surface touched |
| [#25](https://github.com/pratik-saptarshi/tachi-rust/pull/25) | [29204879860](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29204879860) | [29204879937](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29204879937) | `a9563bb18ae47cbcabc61c573c07cdf211c163ce` | 2026-07-12T18:58:08Z | 2026-07-12T18:58:08Z | 2026-07-12T18:59:25Z | `full_pr_matrix` — active docs or shared surface touched |
| [#26](https://github.com/pratik-saptarshi/tachi-rust/pull/26) | [37269704560](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37269704560) | [37269704637](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37269704637) | `562666a5e0e025050a55d757ec723285fbea0085` | 2026-10-05T05:52:31Z | 2026-10-05T05:52:31Z | 2026-10-05T05:54:30Z | `full_pr_matrix` — active docs or shared surface touched |
| [#27](https://github.com/pratik-saptarshi/tachi-rust/pull/27) | [29212714304](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29212714304) | [29212714337](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29212714337) | `44b2b43bb184afafb71260d0393184163d2e88f7` | 2026-07-12T23:04:04Z | 2026-07-12T23:04:04Z | 2026-07-12T23:05:30Z | `full_pr_matrix` — active docs or shared surface touched |
| [#28](https://github.com/pratik-saptarshi/tachi-rust/pull/28) | [29212960891](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29212960891) | [29212960887](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29212960887) | `64b40a0e0872f9600be68e204e60db1542dda97f` | 2026-07-12T23:11:51Z | 2026-07-12T23:11:51Z | 2026-07-12T23:13:12Z | `full_pr_matrix` — active docs or shared surface touched |
| [#29](https://github.com/pratik-saptarshi/tachi-rust/pull/29) | [29213378046](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29213378046) | [29213378065](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29213378065) | `b80c1ce5cd45b2cb65440f039bf8d7f33b1843b7` | 2026-07-12T23:25:42Z | 2026-07-12T23:25:42Z | 2026-07-12T23:27:03Z | `full_pr_matrix` — active docs or shared surface touched |
| [#30](https://github.com/pratik-saptarshi/tachi-rust/pull/30) | [29218723543](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29218723543) | [29218723545](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29218723545) | `c794c442ff29a5ed7915e92507047e4824c1fc41` | 2026-07-13T02:04:25Z | 2026-07-13T02:04:25Z | 2026-07-13T02:06:04Z | `full_pr_matrix` — active docs or shared surface touched |
| [#31](https://github.com/pratik-saptarshi/tachi-rust/pull/31) | [29219332636](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29219332636) | [29219332659](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29219332659) | `56829eb5288bb8f7cf36044edd98fddf0e93c62e` | 2026-07-13T02:21:08Z | 2026-07-13T02:21:08Z | 2026-07-13T02:22:41Z | `full_pr_matrix` — unknown non-docs paths stay full mode |
| [#32](https://github.com/pratik-saptarshi/tachi-rust/pull/32) | [29221122357](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29221122357) | [29221122329](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29221122329) | `c2a213e495f666c3239c934af1fe9d6642304a6d` | 2026-07-13T03:11:09Z | 2026-07-13T03:11:09Z | 2026-07-13T03:12:41Z | `full_pr_matrix` — active docs or shared surface touched |
| [#33](https://github.com/pratik-saptarshi/tachi-rust/pull/33) | [29255219181](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29255219181) | [29255219291](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29255219291) | `586916758e09bc5622e9a3a4745de589032ca3d3` | 2026-07-13T13:47:57Z | 2026-07-13T13:47:57Z | 2026-07-13T13:50:03Z | `full_pr_matrix` — active docs or shared surface touched |
| [#34](https://github.com/pratik-saptarshi/tachi-rust/pull/34) | [29256016483](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29256016483) | [29256017413](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/29256017413) | `684167384dcb3c7bb44e9a1a287a5808b4d26630` | 2026-07-13T13:59:18Z | 2026-07-13T13:59:18Z | 2026-07-13T14:00:52Z | `full_pr_matrix` — active docs or shared surface touched |
| [#35](https://github.com/pratik-saptarshi/tachi-rust/pull/35) | [34380620680](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/34380620680) | [34380620683](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/34380620683) | `8962474acd8aced8625f78b37e99b06ef5349538` | 2026-09-09T17:04:00Z | 2026-09-09T17:04:00Z | 2026-09-09T17:05:42Z | `full_pr_matrix` — unknown non-docs paths stay full mode |
| [#37](https://github.com/pratik-saptarshi/tachi-rust/pull/37) | [37159593403](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37159593403) | [37159593410](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37159593410) | `9c61be37900c4f0ca53a452bf7326347485b58e6` | 2026-10-03T22:46:56Z | 2026-10-03T22:46:56Z | 2026-10-03T22:48:47Z | `full_pr_matrix` — active docs or shared surface touched |
| [#38](https://github.com/pratik-saptarshi/tachi-rust/pull/38) | [37163626320](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37163626320) | [37163626149](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37163626149) | `d9cee7a9445d1dbe5f96ef594744bd67f01992e7` | 2026-10-04T00:01:39Z | 2026-10-04T00:01:39Z | 2026-10-04T00:03:21Z | `full_pr_matrix` — unknown non-docs paths stay full mode |
| [#39](https://github.com/pratik-saptarshi/tachi-rust/pull/39) | [37191130659](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37191130659) | [37191130624](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37191130624) | `5aa372fba6cbe3c3b766472fd1c40ce99e0089c6` | 2026-10-04T09:07:22Z | 2026-10-04T09:07:22Z | 2026-10-04T09:08:51Z | `full_pr_matrix` — active docs or shared surface touched |
| [#41](https://github.com/pratik-saptarshi/tachi-rust/pull/41) | [37193038961](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37193038961) | [37193039061](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37193039061) | `7012b7ad70d4a2039858214ee8458926cd1f5b87` | 2026-10-04T09:42:58Z | 2026-10-04T09:42:58Z | 2026-10-04T09:44:54Z | `full_pr_matrix` — active docs or shared surface touched |
| [#42](https://github.com/pratik-saptarshi/tachi-rust/pull/42) | [37209777846](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37209777846) | [37209777860](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37209777860) | `c68969dbf3796e3ab2f73f024050afb8a90cd00b` | 2026-10-04T14:34:48Z | 2026-10-04T14:34:48Z | 2026-10-04T14:36:29Z | `full_pr_matrix` — active docs or shared surface touched |
| [#43](https://github.com/pratik-saptarshi/tachi-rust/pull/43) | [37205853783](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37205853783) | [37205853788](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37205853788) | `28ad38cace3d3e047ac2a33251b846e748510e23` | 2026-10-04T13:30:22Z | 2026-10-04T13:30:22Z | 2026-10-04T13:31:52Z | `full_pr_matrix` — active docs or shared surface touched |
| [#44](https://github.com/pratik-saptarshi/tachi-rust/pull/44) | [37207334428](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37207334428) | [37207334418](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37207334418) | `d202416a998cccd08f97d02a9e17fab10e9932d7` | 2026-10-04T13:55:04Z | 2026-10-04T13:55:04Z | 2026-10-04T13:56:49Z | `full_pr_matrix` — active docs or shared surface touched |
| [#45](https://github.com/pratik-saptarshi/tachi-rust/pull/45) | [37250506669](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37250506669) | [37250506899](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37250506899) | `46a5e979535be4cbf4b11847375b5a7fa5436c07` | 2026-10-05T01:12:19Z | 2026-10-05T01:12:19Z | 2026-10-05T01:13:56Z | `full_pr_matrix` — active docs or shared surface touched |
| [#46](https://github.com/pratik-saptarshi/tachi-rust/pull/46) | [37252413657](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37252413657) | [37252413648](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37252413648) | `7d30864cbf769c6f991b12b3d437512e9b456401` | 2026-10-05T01:41:29Z | 2026-10-05T01:41:29Z | 2026-10-05T01:43:05Z | `full_pr_matrix` — active docs or shared surface touched |
| [#47](https://github.com/pratik-saptarshi/tachi-rust/pull/47) | [37255726655](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37255726655) | [37255726650](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37255726650) | `c4fa63780594336ee32e44053fa39e3a92d0aa97` | 2026-10-05T02:31:14Z | 2026-10-05T02:33:42Z | 2026-10-05T02:35:03Z | `full_pr_matrix` — active docs or shared surface touched |
| [#48](https://github.com/pratik-saptarshi/tachi-rust/pull/48) | [37261759812](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37261759812) | [37261759764](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37261759764) | `67072440978c8b65798f1ca0071994d5ce249409` | 2026-10-05T04:01:43Z | 2026-10-05T04:01:43Z | 2026-10-05T04:03:47Z | `full_pr_matrix` — active docs or shared surface touched |
| [#49](https://github.com/pratik-saptarshi/tachi-rust/pull/49) | [37263992058](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37263992058) | [37263991960](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37263991960) | `ca73b269c6a64f35c39ebf396fb783471e91d88a` | 2026-10-05T04:32:47Z | 2026-10-05T04:32:47Z | 2026-10-05T04:34:39Z | `full_pr_matrix` — active docs or shared surface touched |
| [#50](https://github.com/pratik-saptarshi/tachi-rust/pull/50) | [37264920975](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37264920975) | [37264920951](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37264920951) | `f3a3df1465eec8a6322bc36d3e41db4c67df01c7` | 2026-10-05T04:45:44Z | 2026-10-05T04:45:44Z | 2026-10-05T04:47:30Z | `full_pr_matrix` — active docs or shared surface touched |
| [#51](https://github.com/pratik-saptarshi/tachi-rust/pull/51) | [37266535428](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37266535428) | [37266535553](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37266535553) | `661ceee761c1bc4bb666fa5d7737ba77d5428e9d` | 2026-10-05T05:09:40Z | 2026-10-05T05:09:40Z | 2026-10-05T05:11:28Z | `full_pr_matrix` — active docs or shared surface touched |
| [#52](https://github.com/pratik-saptarshi/tachi-rust/pull/52) | [37266905855](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37266905855) | [37266905905](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37266905905) | `fb2226bc68eb06610474f12bf7ce5376b179c807` | 2026-10-05T05:14:41Z | 2026-10-05T05:14:41Z | 2026-10-05T05:16:37Z | `full_pr_matrix` — active docs or shared surface touched |
| [#53](https://github.com/pratik-saptarshi/tachi-rust/pull/53) | [37267212809](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37267212809) | [37267212753](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37267212753) | `e9a19dac8f5d00628f4b2412d98462e07b1ef999` | 2026-10-05T05:18:46Z | 2026-10-05T05:18:46Z | 2026-10-05T05:20:56Z | `full_pr_matrix` — active docs or shared surface touched |

The separately inspected newest successful post-correction route artifact is
[PR #55 run 37274320021](https://github.com/pratik-saptarshi/tachi-rust/actions/runs/37274320021).
Its head is `0e248208eb1434472bc9259a406b934403f1a434`, its route is
`full_pr_matrix`, and its reason is `active docs or shared surface touched`.

## Required next evidence

Keep Beads `RT-0sd.2` open. For each of `passive-docs` and
`dependency-closure`, collect ten distinct code trees with a successful routed
PR run and a successful forced-full `workflow_dispatch` control on the exact
same tree SHA and comparable workflow/runner definition. Retain run IDs, PR and
head, event, attempt, route decision, timestamps, queue duration, and execution
duration. Report queue and execution medians separately. Close only if the
optimized execution median is at most 65% of its matched full-control median
for both route shapes; fewer than ten pairs or a missed threshold keeps
`RT-0sd.2` open.
