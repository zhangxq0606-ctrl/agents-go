<!-- Modified for Agents Go by Agents Go contributors, 2026-10-01. See NOTICE. -->
# Agents Go

Agents Go is a Herdr-based terminal workspace for running AI coding agents. This repository is being prepared as an independent project; some package names, update endpoints, and upstream release automation may still refer to Herdr.

## Project guidance

These instructions apply to Agents Go development. Keep changes focused, preserve upstream license notices, and distinguish verified behavior from planned work.

### Architecture

- Keep application state separate from runtime resources such as PTYs. Make workspace and pane behavior testable without launching real terminals where practical.
- Keep rendering pure: layout computation may update presentation state; drawing reads state and does not mutate it.
- Keep OS-specific behavior in src/platform/<os>.rs; shared interfaces and testable contracts belong in src/platform/mod.rs.
- Keep agent detection based on terminal evidence. Do not use scrollable user viewport content as the source of agent status.
- Keep runtime/session facts in server state and expose them through the public JSON API or event path when practical. Keep colors, selection, layout, and other presentation state in the client.
- Treat published endpoint codecs and method shapes as compatibility contracts. Add optional capabilities or new versioned methods instead of changing existing wire meanings.

### Performance

Rendering, layout, PTY parsing, detection, resizing, and client fanout scale with panes, workspaces, or clients. Before widening these paths, identify the work frequency and cardinality. In pane-scaled loops, use narrow state accessors, avoid filesystem or process inspection, avoid unnecessary allocation, and keep terminal locks short. When a change widens a hot path, compare fixed-geometry runs with 1 and at least 15 populated panes using just bench-render-scale.

### Changes and verification

- Read the relevant implementation and tests before changing behavior. Make the smallest change that fixes the observed problem.
- Add regression coverage for behavior changes and important failure paths. Do not weaken a test to make a change pass.
- Prefer repository just recipes. just test runs the Rust and maintenance suites; just check runs the repository checks. On Windows, just lint and just build use the repository's PowerShell packaging scripts.
- Report checks that could not run and why. A successful compile does not replace manual UI verification.
- Update PROGRESS.md when behavior, structure, or project direction changes. Keep exploratory notes in .local/prd/; do not maintain a second progress log.
- Keep user-facing docs aligned with implemented behavior. Do not edit upstream release snapshots or claim planned work as available.

### Safety and Git

- Do not delete or replace files outside the approved task scope. Before deleting files, list the exact paths and confirm they are not referenced by source, build scripts, tests, or project documentation.
- Do not commit, push, publish, or deploy unless the user explicitly asks.
- Use main as the default branch for this new repository. Preserve existing history unless the user requests otherwise.
- Keep personal configuration, credentials, build outputs, and temporary files out of Git. .tmp/ is a retained local workspace container; do not remove it.
- Never include secrets in source, test fixtures, logs, or documentation.

### Project facts

- The application is written in Rust and uses justfile recipes for build and validation.
- This codebase derives from herdrdev/herdr and retains its Apache-2.0 license. Preserve upstream notices and review attribution obligations when redistributing.
- Current development checkout and its remaining upstream-specific files are tracked in PROGRESS.md. Confirm the code and build configuration before documenting a feature as released or supported.

### Upstream maintenance

- The Agents Go repository is origin and the official Herdr repository is upstream. Agents Go uses main; upstream currently uses master. Never push to upstream.
- Fetch upstream for comparison; do not merge or rebase it automatically.
- Review upstream performance changes by reading the code diff and affected tests. For render or pane-scaled changes, record a comparable before/after result with just bench-render-scale.
- Integrate isolated changes with cherry-pick when their dependencies are clear. For changes coupled to Herdr-only product or release behavior, port the smallest useful implementation and its tests instead.
- After integration, rerun the relevant tests and checks, record the upstream commit and result in PROGRESS.md, and retain required attribution and license notices.
- Never push Agents Go changes to the upstream remote.
