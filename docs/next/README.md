<!-- Modified for Agents Go by Agents Go contributors, 2026-10-01. See NOTICE. -->
# Agents Go

A terminal workspace for AI coding agents, with a visual project and conversation tree.

Agents Go is a modified version of [Herdr](https://github.com/herdrdev/herdr), adding a project and terminal conversation tree, visible agent activity, recent-project recovery, and Git branch labels. The CLI and executable keep the name `herdr`; this repository is not an official Herdr release.

Automatic update checks, self-updates, and automatic remote-server downloads are disabled. This repository currently provides source code; inherited Herdr installers and release documentation describe the upstream product.

[中文说明](README.zh-CN.md)

## Build and develop

The repository pins its Rust toolchain in rust-toolchain.toml. Install Rust, Zig 0.16, Python, Bun, and just as required by the platform build.

On Windows:

    just build

Start the packaged Windows build in PowerShell:

    & .\target\release\herdr.exe --session agents-go

For a debug build, run `cargo build --locked` and start `& .\target\debug\herdr.exe --session agents-go`.

Windows is the manually verified platform for this fork. Other platform build paths are inherited from Herdr and have not been manually verified for these modifications.

Run tests and checks:

    just test
    just check

The Windows build recipe packages the required ConPTY runtime. A successful build does not replace manual verification of terminal rendering and interaction.

## Known check limitations

The project owner manually verified directory changes, archiving after closing all conversations, reopening recent projects, and retaining history after restart. The full automated suite has not completed on the development machine: its filesystem does not support the EFS test fixture, and two project-reopen tests that create real terminals did not finish. These limitations are recorded rather than claimed as passing checks.

## Upstream relationship

The source is derived from herdrdev/herdr and retains its Apache-2.0 license. The project aims to preserve useful upstream terminal and agent-runtime improvements while developing its own task-tree experience. Upstream changes are reviewed and integrated selectively; they are not synchronized automatically.

## License

Apache-2.0. See [LICENSE](LICENSE), [NOTICE](NOTICE), and the retained third-party license and notice files. Modified source files carry change notices; JSON files use adjacent `.license` notices.
