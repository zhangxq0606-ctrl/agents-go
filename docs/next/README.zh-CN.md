# Agents Go

一个面向 AI 编程智能体的终端工作台，用可视化任务树组织项目与终端对话。

Agents Go 基于 [Herdr](https://github.com/herdrdev/herdr) 独立开发。当前开发版增加了项目与终端对话任务树、智能体运行状态、最近项目恢复和 Git 分支显示。项目仍在开发中；命令行包名及部分继承的发布配置暂时保留 Herdr 标识。

在 Agents Go 发布自己的安装包前，后台版本检查和自更新保持关闭。

## 构建与开发

Rust 工具链版本由 rust-toolchain.toml 固定。按平台准备 Rust、Zig 0.16、Python、Bun 和 just 等构建工具。

编译本地调试版：

    cargo build

启动侧栏测试会话：

    target/debug/herdr.exe --session sidebar-ui

Windows 分发构建：

    just build

运行测试与检查：

    just test
    just check

Windows 构建配方会打包所需的 ConPTY 运行组件。编译成功后仍需手动检查终端中的显示和交互。

## 上游关系

本项目源自 herdrdev/herdr，并保留 Apache-2.0 许可证。项目会继续关注 Herdr 的终端、智能体运行时及性能改进，并经审查后选择性集成；不会自动同步上游改动。

## 许可证

项目采用 Apache-2.0。请查阅 LICENSE，并保留上游版权与归属声明。
