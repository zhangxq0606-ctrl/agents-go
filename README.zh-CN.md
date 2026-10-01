<!-- Modified for Agents Go by Agents Go contributors, 2026-10-01. See NOTICE. -->
# Agents Go

一个面向 AI 编程智能体的终端工作台，用可视化任务树组织项目与终端对话。

Agents Go 是 [Herdr](https://github.com/herdrdev/herdr) 的改造版，增加了项目与终端对话任务树、智能体运行状态、最近项目恢复和 Git 分支显示。命令行与可执行文件仍叫 `herdr`，本仓库不是 Herdr 官方版本。

后台更新检查、自更新和远程服务器自动下载目前关闭。本仓库提供源码；继承的 Herdr 安装脚本与发布文档介绍的是上游产品。

## 构建与开发

Rust 工具链版本由 rust-toolchain.toml 固定。按平台准备 Rust、Zig 0.16、Python、Bun 和 just 等构建工具。

Windows 打包构建：

    just build

在 PowerShell 中启动：

    & .\target\release\herdr.exe --session agents-go

如需调试版：

    cargo build --locked
    & .\target\debug\herdr.exe --session agents-go

本改造版目前在 Windows 上进行过手动验证。其他平台的构建路径沿用 Herdr，尚未手动验证这些改动。

运行测试与检查：

    just test
    just check

Windows 构建配方会打包所需的 ConPTY 运行组件。编译成功后仍需手动检查终端中的显示和交互。

## 已知检查限制

项目维护者已手动验证切换目录、关闭全部对话后归档、点击最近项目重开，以及重启后的历史记录保留。开发机上的完整自动测试尚未全部完成：文件系统不支持 EFS 测试，两项创建真实终端的项目重开测试未能结束。这些限制保留记录，不视为测试通过。

## 上游关系

本项目源自 herdrdev/herdr，并保留 Apache-2.0 许可证。项目会继续关注 Herdr 的终端、智能体运行时及性能改进，并经审查后选择性集成；不会自动同步上游改动。

## 许可证

项目采用 Apache-2.0。请查阅 [LICENSE](LICENSE)、[NOTICE](NOTICE) 及保留的第三方许可证与归属文件。修改的源文件已注明改动；JSON 文件的声明放在相邻的 `.license` 文件中。
