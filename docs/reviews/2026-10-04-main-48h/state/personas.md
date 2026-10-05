# Persona Profiles Registry

All agents use the same inherited available model. The skill's requested Opus model and Agent-specific plugin types are unavailable in this runtime; this is an explicitly adapted Overseer run. Shared model biases remain a limitation. Three child slots require two isolated batches for six independent reviewers; no reviewer sees another's Phase 3 output before debate.

| Slug | Persona | Expertise / lens | Reasoning | Agreement | Phases |
|---|---|---|---|---|---|
| correctness | Correctness Hawk | Rust parsing, data integrity, edge cases | Systematic enumeration | 30% | 3, 4, 5, 7 |
| architecture | Architecture Critic | Cross-boundary contracts and compatibility | Backward reasoning | 50% | 3, 4, 5, 7 |
| security | Security Auditor | Filesystem safety, auth and isolation | Adversarial simulation | 30% | 3, 4, 5, 7 |
| devils_advocate | Devil's Advocate | Challenge tests, assumptions and happy paths | Analogical reasoning | 20% | 3, 4, 5, 7 |
| code_quality | Code Quality Auditor | Exact source/config and taxonomy semantics | Systematic enumeration | 40% | 3, 4, 5, 7 |
| pipeline | Pipeline Reviewer | CI, dependencies, failure behavior and operational reliability | Checklist verification | 30% | 3, 4, 5, 7 |

Support roles: data-flow tracer (Phase 2, boundary certificates); completeness auditor (Phase 8, overlooked cases); citation verifier (Phase 10, factual evidence); severity verifier (Phase 11, current defect impact and safeguards); tier advisor and focused verification specialists when needed (12–13); neutral judge (14); independent judge-output verifier (14.5); HTML rendering specialist (15.3). Support agents do not vote as panelists.

Classification: code/implementation review with supporting prose, hence Precise code/config findings and Exhaustive prose analysis. Code Quality Auditor is mandatory for code; Pipeline Reviewer is the signal specialist for CI/infra changes. Security and architecture cover the SQL/auth and reliability signals without exceeding six reviewers. Standard trace, one run, default Correctness/Completeness/Quality/Edge Cases criteria.
