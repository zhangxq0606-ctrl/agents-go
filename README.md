# Agents Go

A terminal workspace for AI coding agents, with a visual project and conversation tree.

Agents Go is an independent project based on [Herdr](https://github.com/herdrdev/herdr). The current development build adds a task tree for projects and terminal conversations, visible agent activity, recent-project recovery, and Git branch labels. It is under active development; the CLI package name and parts of the inherited release setup still use Herdr identifiers.

Automatic version checks and self-updates are disabled until Agents Go publishes its own release assets.

## Build and develop

The repository pins its Rust toolchain in rust-toolchain.toml. Install Rust, Zig 0.16, Python, Bun, and just as required by the platform build.

On Windows:

    just build

Run the current development session:

    target/debug/herdr.exe --session sidebar-ui

Run tests and checks:

    just test
    just check

The Windows build recipe packages the required ConPTY runtime. A successful build does not replace manual verification of terminal rendering and interaction.

## Upstream relationship

The source is derived from herdrdev/herdr and retains its Apache-2.0 license. The project aims to preserve useful upstream terminal and agent-runtime improvements while developing its own task-tree experience. Upstream changes are reviewed and integrated selectively; they are not synchronized automatically.

## License

Apache-2.0. See LICENSE and preserve the upstream copyright and attribution notices.
