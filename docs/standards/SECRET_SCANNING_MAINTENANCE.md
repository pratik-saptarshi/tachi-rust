# Adopter secret scanning maintenance

Use the existing pinned Gitleaks **8.30.1** executable directly. Copy
`.gitleaks-adopter.toml.example` to `.gitleaks-adopter.toml`, keep its `[extend]`
path pointing to `.gitleaks.toml`, and append organization-specific rules with
unique IDs. The parent inherits Gitleaks defaults through `useDefault = true`.
This workflow needs neither Python nor a hook framework.

From the repository root:

```sh
gitleaks version
gitleaks dir . --config .gitleaks-adopter.toml --redact --no-banner
gitleaks git --staged --config .gitleaks-adopter.toml --redact --no-banner
cargo test --locked -p tachi-core --test adopter_secret_scanning -- --ignored
```

For a scanner update, record the old/new version, official release URL and
verified binary checksum in the change description. Review release notes and
configuration inheritance, update the existing workflow and hook pins together,
and rerun the Rust regression above with the candidate binary. Run the complete
repository scan and inspect redacted findings before publication. Do not extend
fixture exclusions to application directories to silence failures.

The regression creates synthetic credentials only in a private temporary
directory outside `tests/fixtures`, checks the default GitHub token detector,
checks a narrow placeholder, and verifies malformed configuration fails.
It also checks an adopter rule while retaining the inherited default rule.

Update evidence template:

| Evidence | Value |
|---|---|
| Old / proposed version | |
| Official release and checksum | |
| Inheritance / rule changes reviewed | |
| Rust regression and full scan exit codes | |
| Reviewed false positives and exact allowlist rationale | |
| Dependency audit (no Python execution chain) | |
