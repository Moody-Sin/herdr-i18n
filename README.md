# herdr-i18n

> 为智能体编码而生的终端复用器运行时（全量简体中文本地化分支）。

<p align="center">
  <img src="assets/logo.png" alt="herdr logo" width="100" />
</p>

<p align="center">
  <a href="https://herdr.dev">官方主页</a> · <a href="#安装">安装</a> · <a href="https://herdr.dev/zh-cn/docs/quick-start/">快速开始</a> · <a href="https://herdr.dev/zh-cn/docs/">文档</a>
</p>

<p align="center">
  简体中文 · <a href="README.zh-CN.md">快速镜像</a> · <a href="https://github.com/herdrdev/herdr">上游英文原版</a>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-666666?labelColor=333333" alt="Apache 2.0 许可证" /></a>
  <a href="https://github.com/herdrdev/herdr/releases"><img src="https://img.shields.io/github/downloads/herdrdev/herdr/total?labelColor=333333&color=666666" alt="GitHub 发行版总下载量" /></a>
  <a href="https://github.com/herdrdev/herdr/stargazers"><img src="https://img.shields.io/github/stars/herdrdev/herdr?labelColor=333333&color=666666&logo=github" alt="GitHub Star 数" /></a>
  <a href="https://github.com/herdrdev/herdr/releases/latest"><img src="https://img.shields.io/github/v/release/herdrdev/herdr?label=release&labelColor=333333&color=666666" alt="最新稳定版本" /></a>
</p>

---

## 目录

- [背景与定位](#背景与定位)
- [与上游的关系](#与上游的关系)
- [核心特性](#核心特性)
- [安装](#安装)
- [快速开始](#快速开始)
- [文档](#文档)
- [本地化机制](#本地化机制)
- [开发指南](#开发指南)
- [目录结构](#目录结构)
- [智能体须知](#智能体须知)
- [致谢与赞助](#致谢与赞助)
- [许可证](#许可证)

---

## 背景与定位

[Herdr](https://github.com/herdrdev/herdr) 是一款为 AI 编码智能体（如 Claude Code、Codex、Cursor、OpenCode、Grok 等）设计的终端复用器与多机工作流运行时。它在后台守护终端进程，具备多机连接、窗格管理、会话断点恢复以及智能体状态无感探测等能力。

**本仓库（herdr-i18n）是专门维护简体中文全量本地化的复刻分支。** 在完整保留上游所有功能、底层 PTY/终端引擎、API 与协议特性的前提下，全面汉化了 TUI 交互界面、CLI 命令行输出、配置模板与交互提示，让中文开发者拥有开箱即用、无语言门槛的现代化智能体运行环境。

---

## 与上游的关系

本仓库为长期独立跟踪上游的本地化复刻项目：

1. **不向上游推送 PR**：所有汉化与中文适配均在本仓库闭环维护，避免干扰上游纯英文项目治理与发布节奏。
2. **跟踪上游发布**：持续拉取上游 `master` 分支的最新特性与修复，通过自动化的本地化检测脚本（`just localization-check`）发现并补齐上游新增的文案。
3. **禁用官方自更新**：上游自带的官方二进制自更新和自动版本检查已被显式禁用（`src/i18n.rs::LOCALIZED_BUILD`），防止执行更新操作后官方英文二进制覆盖本地编译的中文版。更新方式为拉取本仓库最新源码并重新编译。
4. **架构保持同构**：本仓库严格遵循上游的架构规范与协议边界，底层代码逻辑与协议编解码器完全一致。

---

## 核心特性

- **工作不中断，后台静默守护**：关闭客户端或 SSH 意外断开时，Herdr 在后台服务中保持终端持续运行；服务或系统重启后，可完整恢复保存的窗口布局并唤醒智能体会话。
- **多机合一，统一全局透视**：在单一窗口中同时纳管本地环境与远程 SSH 机器，聚合智能体运行状态树，支持跨机器独立重连。
- **状态感知，告别盲目找寻**：实时无损侦测并标明智能体运行状态（`working` 运行中、`blocked` 阻塞等待确认、`idle` 空闲就绪），谁被卡住需要用户输入一目了然。
- **智能体原生驱动**：提供纯净的 Socket API 与 CLI 接口，智能体可自主创建窗格、互相传递 Prompt、等待其他依赖智能体完工，构建自动化多智能体协作链。
- **兼容现有开发工作流**：不包装、不阉割智能体生态；原生托管已有终端进程，无论运行 Claude Code、Codex 还是自定义脚本均无缝衔接。
- **键鼠双一等公民**：既有 tmux 风格的高效前缀快捷键体系，又完整支持现代终端的鼠标点击、拖拽分屏、窗口缩放。
- **轻量纯粹，拒绝 Electron**：单一纯 Rust 二进制交付，极致启动速度与低资源占用。

---

## 安装

### 方式一：从本仓库源码构建（推荐）

确保本地已安装 Rust（stable）以及 Zig 0.16.0（用于编译 vendored libghostty-vt）：

```bash
git clone https://github.com/Moody-Sin/herdr-i18n.git
cd herdr-i18n
git switch zh-CN
cargo build --release
```

编译产物位于 `target/release/herdr`，可将其放置在系统 `PATH` 路径中（如 `/usr/local/bin`）。

### 方式二：上游官方安装脚本（英文原版）

> 提示：上游官方安装脚本会安装官方英文二进制，且会启用自动更新功能。

```bash
curl -fsSL https://herdr.dev/install.sh | sh
```

或使用 macOS Homebrew / Windows PowerShell：
```bash
brew install herdr
```

---

## 快速开始

在需要开展编码工作的代码库根目录下启动：

```bash
herdr
```

常见快捷操作：
- `ctrl+b %`：垂直分割当前窗格
- `ctrl+b "`：水平分割当前窗格
- `ctrl+b q`：安全分离（Detach）会话，终端继续在后台运行
- `ctrl+b ?`：打开内置全量快捷键与交互帮助指南
- `herdr`：重新连接（Attach）回到先前的会话

---

## 文档

完整的进阶指引与配置参考可参阅官方文档：

- [快速上手指南 (Quick Start)](https://herdr.dev/zh-cn/docs/quick-start/)
- [核心概念与架构 (Concepts)](https://herdr.dev/zh-cn/docs/concepts/)
- [受支持的智能体清单 (Agents)](https://herdr.dev/zh-cn/docs/agents/)
- [快捷键与按键绑定 (Keyboard)](https://herdr.dev/zh-cn/docs/keyboard/)
- [配置文件规范 (Configuration)](https://herdr.dev/zh-cn/docs/configuration/)
- [会话保存与恢复 (Session State)](https://herdr.dev/zh-cn/docs/session-state/)
- [远程机器连接 (Remote Machines)](https://herdr.dev/zh-cn/docs/connecting-machines/)
- [Socket API 接口参考 (Socket API)](https://herdr.dev/zh-cn/docs/socket-api/)

---

## 本地化机制

本仓库所有的中文化文本统一维护在 [`src/i18n/zh-CN.json`](src/i18n/zh-CN.json) 映射表中，并遵循以下机制：

1. **查找模式**：Rust 源码中通过 `crate::tr!("English text")`（静态文案）和 `crate::trf!("Message {name}", name = value)`（动态插值）进行匹配。英文原文即为字典 Key，若条目缺失则平滑回退至英文，绝不发生运行时崩溃。
2. **规范术语统一**：所有界面的专业术语翻译均严格对照 [`src/i18n/GLOSSARY.md`](src/i18n/GLOSSARY.md)（例如：*worktree* 译为“工作树”，*pane* 译为“窗格”，*agent* 译为“智能体”）。
3. **严格的自动化保障**：通过 `just localization-check` 扫描源代码 AST，确保所有面向用户的输出均走 `tr!`，并且严格验证插值占位符对称性与未登记英文字面量。

---

## 开发指南

使用 `just` 作为统一的任务运行器：

```bash
# 运行单元测试与集成测试
just test

# 运行代码格式、静态检查、维护测试与本地化缺漏检查
just check

# 专项执行本地化覆盖率与有效性检查
just localization-check
```

### 同步上游流程

当上游发布新版本需要同步时：

```bash
git fetch origin
git merge origin/master
# 运行检查发现上游新增的文案，并在 src/i18n/zh-CN.json 中补齐
just localization-check
just check
```

---

## 目录结构

详细的源码设计与各层职责请参阅 [`src/README.md`](src/README.md)。概览如下：

```text
├── assets/             # 静态多媒体资源与图标
├── docs/               # 网站文档与多版本快照
├── scripts/            # 维护脚本、本地化检查及自动化测试
├── src/                # 核心 Rust 源代码（见 src/README.md）
│   ├── api/            # Socket API 与 JSON-RPC 服务协议
│   ├── app/            # 核心业务状态机（AppState）、工作区与窗格逻辑
│   ├── cli/            # 命令行解析与子命令处理（已全量本地化）
│   ├── client/         # TUI 客户端渲染与终端交互
│   ├── detect/         # 智能体屏幕无感侦测与规则引擎
│   ├── i18n/           # 简体中文语言包、术语对照表与翻译文档
│   ├── server/         # 后台守护进程与无头服务
│   └── terminal/       # 虚拟终端解析与底层状态管理
├── vendor/             # 依赖源码树（如 vendored libghostty-vt）
├── CHANGELOG.md        # 版本更新日志
├── CONTRIBUTING.md     # 贡献指引
├── LESSONS.md          # 经验教训积累文档
└── README.md           # 项目主说明文档
```

---

## 智能体须知

协助本仓库开发的 AI 智能体在进行修改前必须阅读 [`AGENTS.md`](./AGENTS.md)，遵守约定式提交、语义化版本与简体中文回复规范；在提交代码前使用 `just check` 运行全量验证。

---

## 致谢与赞助

感谢每一位支持和赞助 Herdr 项目的开发者，历史赞助者名单详见 [SPONSORS.md](./SPONSORS.md)。

- 上游发起与维护者：[herdrdev/herdr](https://github.com/herdrdev/herdr)
- 赞助伙伴：[Terminal Trove](https://terminaltrove.com/)

---

## 许可证

本项目基于 [Apache License 2.0](LICENSE) 协议开源。
