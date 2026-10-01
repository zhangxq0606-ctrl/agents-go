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

#### 本地验证与运行中会话（Windows）

- 始终使用仓库默认 `target/`，它保持增量、从秒级起步。切勿为避让文件占用而临时指定一个全新的 `CARGO_TARGET_DIR`：那会重编全部依赖，把验证从秒级拖成分钟级。
- 本机常在 `ag` 会话中运行 `target/debug/herdr.exe`。Windows 不允许覆盖运行中的可执行文件，因此任何需要重新链接主程序的命令（`just check`、`just test`、`cargo build`）会在 `target/debug/herdr.exe` 上报 os error 5。
- 会话运行期间的针对性验证：`cargo test --locked --bin herdr <过滤>`（构建独立测试二进制，不重链主 exe）与 `cargo clippy --all-targets --locked -- -D warnings`（只做检查、不链接）。
- 确需完整 `just check` 或 `just build` 时，先请船长关闭 `ag` 会话再执行；若跳过，明确说明跳过项与原因。
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

#### 上游改动汇报（中文约定）

向船长汇报上游差异时，逐个提交给出一张「一眼卡片」，不贴大段 diff、不堆术语：

- 编号：每张卡片带递增编号（①②③…），便于船长按编号决策。
- 首行：`<编号> <短SHA>  <一句话说清改了什么>`，用大白话，让船长不读代码就能懂。
- `价值`：对我们有没有用（一句话）。
- `难度`：省事 / 一般 / 麻烦，按下面定义判断并给出对应处理方式。
- `建议`：采纳 / 暂缓 / 不采纳，并说明理由。

难度分级：

- 省事：不碰我们改过的文件，拿来即用；直接集成后跑编译与相关测试。
- 一般：能自动合并，但落点在我们改过的文件；文本无冲突不等于语义正确，集成后必须编译并跑相关测试确认。
- 麻烦：有冲突，或依赖我们已删/已改的功能；先单独立项，手工移植并重写测试。

交互原则：

- 先自查可行性再汇报：能否干净集成、有无冲突，在隔离环境（如临时 worktree）自行验证，不把技术细节判断推给船长。
- 技术细节自主决策：是否拆分提交、改哪些文件、如何移植、快照要不要动等实现层面的取舍，由 agent 科学判断并直接执行，只在汇报里简述结论与理由，不作为问题抛给船长。只把「要不要做、范围多大、新增依赖、改 CI/公共契约」等真正需要拍板的事项交给船长。
- 价值与成本分开讲：「要不要」和「贵不贵」是两个独立信号，不要混成一段。
- 一次问清：该船长拍板的事项（集成范围、新增依赖、提交方式等）一次性列成选项，不来回挤牙膏。
- 红线显式标注：新增依赖、改 CI、改公共契约等单独点明，但仍附上推荐选项。
- 提交方式由 agent 科学决策后提交审批：船长不需要自行判断用 cherry-pick 还是归并，agent 在汇报卡片这一步就给出明确推荐（保留上游提交与作者，或按模块归并成一个 fork 提交）及理由，随范围、依赖等一并交由船长审批。
- 集成与否由船长决定：只 fetch 比较，不自动合并；集成后跑测试并将上游提交与结果记录到 PROGRESS.md。
