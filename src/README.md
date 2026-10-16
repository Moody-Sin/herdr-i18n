# herdr 核心源码目录 (`src/`)

> Herdr 运行时、TUI 客户端、后台服务与智能体交互引擎的核心 Rust 实现。

本目录包含 Herdr 的完整源码实现。遵循标准分层架构，保证状态机与运行时解耦、渲染与计算解耦、跨平台代码严格隔离，并全量集成了简体中文本地化能力。

---

## 目录

- [架构设计原则](#架构设计原则)
- [模块导航与职责划分](#模块导航与职责划分)
- [核心工作流与数据流](#核心工作流与数据流)
- [本地化 (i18n) 集成](#本地化-i18n-集成)
- [测试与验证规范](#测试与验证规范)
- [相关文档](#相关文档)

---

## 架构设计原则

在阅读与修改源码时，应遵守以下核心准则：

1. **状态与运行时解耦 (State is separated from runtime)**：
   `AppState` 是纯数据模型，不依赖异步执行或真实的 PTY 伪终端。`PaneState` 与底层进程运行时 `PaneRuntime` 独立分离，单元测试无需真实终端即可高效执行。
2. **渲染纯粹性 (Render is pure)**：
   `compute_view()` 负责几何计算与布局计算，而 `render()` 只接收不可变引用 `&AppState` 进行屏幕绘制。严禁在渲染流程中修改状态。
3. **消除上帝对象 (No god objects)**：
   业务逻辑严格拆分。例如 `app/` 目录下按状态 (`state.rs`)、创建动作 (`creation.rs`)、运行时 (`runtime.rs`)、动作处理 (`actions.rs`) 等独立模块解耦组织；终端输入解析独立在 `input/` 目录。
4. **平台代码严格隔离 (Platform code is isolated)**：
   所有操作系统特定实现统一收敛在 `src/platform/<os>.rs`（如 `linux.rs`, `macos.rs`, `windows.rs`），公共特征与协议收敛在 `src/platform/mod.rs`。核心业务逻辑严禁直接散落 `#[cfg(target_os)]`。
5. **探测机制无感解耦 (Detection is decoupled)**：
   智能体状态探测器仅基于屏幕缓冲快照进行事实匹配，绝不干涉终端解析器或视口滚动状态。

---

## 模块导航与职责划分

| 目录/文件 | 核心职责与说明 |
| :--- | :--- |
| [`api/`](api/) | 提供外部通信的 Socket API、JSON-RPC 服务、Schema 协议定义及订阅中心。供 CLI、智能体或第三方工具调用。 |
| [`app/`](app/) | 核心领域状态机。维护全局 `AppState`、工作区（Workspace）、标签页（Tab）、窗格（Pane）树形结构及生命周期。 |
| [`cli/`](cli/) | 命令行界面入口与各级子命令实现（如 `herdr pane`、`herdr agent`、`herdr machine` 等），已实现全量中文本地化。 |
| [`client/`](client/) | TUI 客户端实现。管理终端 Raw 模式、鼠标捕获、按键分发、端点连接激活与多机状态流转。 |
| [`config/`](config/) | 用户与系统配置文件解析、热重载以及键位映射绑定。 |
| [`detect/`](detect/) | 智能体状态无感探测引擎。根据内置与用户自定义的规则清单（TOML），精准探测智能体处于运行、阻塞抑或空闲。 |
| [`i18n/`](i18n/) | 简体中文本地化基础设施。包含静态/动态翻译宏（`tr!` / `trf!`）、中文词典文件（`zh-CN.json`）与专业术语表（`GLOSSARY.md`）。 |
| [`input/`](input/) | 底层终端按键、鼠标事件解析与前缀键（Prefix）序列处理引擎。 |
| [`integration/`](integration/) | 外部智能体与工具链集成资产（如 Claude Code, Codex, Cursor 等配置或补丁注入）。 |
| [`kitty_graphics/`](kitty_graphics/) | 针对现代终端的 Kitty Graphics 协议图像渲染适配。 |
| [`pane/`](pane/) | 窗格几何布局、拆分树计算、视口滚动与焦点聚焦算法。 |
| [`persist/`](persist/) | 布局与会话持久化引擎。负责原子写入、快照保存以及崩溃/重启后的状态恢复。 |
| [`platform/`](platform/) | 跨平台系统抽象层。屏蔽 Unix 与 Windows 在进程管理、信号处理、命名管道与文件描述符上的差异。 |
| [`protocol/`](protocol/) | 客户端与后台守护服务之间的序列化传输协议（Wire 编解码协议）。 |
| [`pty/`](pty/) | 跨平台伪终端（PTY）创建、读写与生命周期抽象。 |
| [`remote/`](remote/) | 远程 SSH 机器连接与会话转发。 |
| [`server/`](server/) | 后台守护进程（Headless Server）。托管所有运行中的终端进程与 Socket 监听。 |
| [`terminal/`](terminal/) | 虚拟终端仿真核心。集成 vendored libghostty-vt，提供屏幕网格计算与属性管理。 |
| [`ui/`](ui/) | TUI 界面组件与绘制管线。实现状态栏、侧边栏、弹窗（Modal）、快捷键帮助列表的纯绘制。 |
| [`workspace/`](workspace/) | 工作区上下文聚合，管理 Git 工作树（Worktree）绑定与多工程切换。 |

---

## 核心工作流与数据流

```text
终端输入 (Terminal Input)
       │
       ▼
 [src/client/input.rs]  ──> 按键/鼠标解码，前缀键匹配
       │
       ▼
 [src/client/shell.rs]  ──> 发送 RPC / 操作请求
       │
       ▼ (Socket Wire 协议)
 [src/server/]          ──> 后台服务分发指令
       │
       ├─> [src/app/state.rs]       ──> 状态机状态更新 (AppState)
       ├─> [src/pty/ & terminal/]   ──> 进程 I/O 驱动，终端模拟器状态刷新
       └─> [src/detect/]            ──> 屏幕快照分析，更新智能体状态
       │
       ▼
 [src/ui/ & client/]    ──> compute_view() 计算视口 ──> render() 纯净渲染输出到终端
```

---

## 本地化 (i18n) 集成

在 `src/` 中新增或修改面向用户的文本时，必须遵循以下步骤：

1. **宏包裹**：
   - 静态文本：使用 `crate::tr!("Example text")`。
   - 带参数文本：使用 `crate::trf!("Hello {user}", user = name)`。
2. **字典同步**：
   - 将英文原文作为键，在 [`src/i18n/zh-CN.json`](i18n/zh-CN.json) 中加入对应的中文译文。
   - 术语使用需严格遵循 [`src/i18n/GLOSSARY.md`](i18n/GLOSSARY.md)。
3. **验证检查**：
   - 提交前运行 `just localization-check`（执行 `scripts/check_localization.py`）。该检查会扫描源码并校验是否存在未汉化的用户可见文本。

---

## 测试与验证规范

- **单元测试**：放置在各模块内部的 `#[cfg(test)] mod tests` 中。业务逻辑通过 `AppState::test_new()` 等无 PTY 假人构造快速运行。
- **运行命令**：
  ```bash
  # 运行源码单元测试
  cargo nextest run
  
  # 全量验证（格式化、测试与脚本校验）
  just check
  ```

---

## 相关文档

- 根目录项目说明：[`../README.md`](../README.md)
- 本地化指南与词汇表：[`i18n/README.md`](i18n/README.md) 及 [`i18n/GLOSSARY.md`](i18n/GLOSSARY.md)
- 编码智能体规范：[`../AGENTS.md`](../AGENTS.md)
