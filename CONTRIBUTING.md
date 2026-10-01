<!-- Modified for Agents Go by Agents Go contributors, 2026-10-01. See NOTICE. -->
# Contributing to Agents Go

Agents Go is an independent project based on Herdr and is still being prepared for public development. Keep changes focused on the task-tree experience and the terminal runtime it depends on.

## Before changing code

Read AGENTS.md and PROGRESS.md. Check the existing implementation and tests, then describe the user-visible behavior or defect your change addresses. Preserve upstream license and copyright notices.

## Build and checks

Use the Rust version pinned in rust-toolchain.toml and the repository justfile. Run the narrowest relevant test while developing; run just check before proposing a change for integration. On Windows, use just build for the packaged application and manually inspect terminal behavior when the change affects rendering or input.

For pane-scaled rendering changes, compare the same geometry before and after with just bench-render-scale. Record the pane counts, environment, and result in PROGRESS.md.

## Upstream changes

The official Herdr repository is tracked separately as upstream. Fetch and review its changes; do not merge them automatically. Prefer a small cherry-pick for an isolated improvement, or a focused port when the change depends on Herdr-only behavior. Run relevant checks and record the source commit and verification.

## Pull requests and releases

Use the main branch for Agents Go work. Keep proposals small enough to review, include relevant test results, and call out changes that still depend on Herdr naming or infrastructure. Do not publish releases or modify external services without explicit project-owner direction.

## License

By contributing, you agree that your contributions are offered under the repository's Apache-2.0 license. Retain applicable upstream notices and attribution.
