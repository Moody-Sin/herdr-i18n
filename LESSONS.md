# 经验教训 (Lessons Learned)

## 构建与工具链
- **Zig 0.16.0 编译 vendored libghostty-vt**：`ghostty-vt` 依赖 Zig 编译，系统必须安装 `zig 0.16.0`。
- **构建网络与本地代理**：Zig 在下载依赖包时若经由代理服务器可能会由于请求头/协议出现 400 Bad Request，必要时使用直连或清空 proxy 环境变量 (`http_proxy=""`) 进行构建。
- **任务运行器**：使用 `just` 作为任务运行器，测试执行通过 `cargo nextest`。
- **本地化白名单与提取**：CLI 子命令的 `Leaf` 和枚举校验 `parse_choice` 均属于用户可见文案，应由 `scripts/check_localization.py` AST/正则规则自动覆盖与检查，严禁将用户可见英文字面量登记在 `localization-cli-allowlist.txt` 中。
- **扫描路径必须包含根层 `src/cli.rs`**：`scripts/check_localization.py` 的未翻译扫描之前只遍历了 `SRC / "cli"` 子目录，漏掉了根层的 `src/cli.rs`，导致该文件数十处直接打印文案长期漏检。扩展扫描路径时需显式包含 `src/cli.rs`。
- **包装 `tr!` 后同步单元测试断言**：把错误信息从英文字面量改为 `crate::tr!(...)` 后，断言该字符串的单元测试会因返回中文而失败。断言应写成 `crate::tr!("...")`，中英两种形态都成立，避免把测试绑死在一种语言上。`check_localization.py` 的未翻译扫描只覆盖 `src/cli/`，`src/client/`、`src/remote/` 仍会出现漏译，需按同一标准人工排查。
- **`tests/server_headless.rs` 的信号竞态抖动（非本仓库 i18n 改动引入）**：`server_survives_hangup_and_logs_why_it_stops` 在全量并发下偶发失败（日志缺 `server shutdown initiated`），隔离运行约 1/6 概率复现。根因是 `src/server/headless/bootstrap.rs:40` 先绑定 API socket，而 `src/server/headless.rs:417` 之后才安装 SIGTERM 处理器；测试在 `wait_for_socket` 返回后立刻发 SIGTERM，落在这个窗口内就会被默认处置直接终止、来不及记录关闭原因。判断失败是否与自己的改动相关时，先看 `git diff --name-only` 是否触及该启动与信号路径。
- **改智能体检测清单必须同时升 `version` 并同步 `distribution/agent-detection/`**：本分支为 `kimi.toml`/`qwen.toml` 加了中文规则却沿用上游版本号，结果官方远程清单（`manifest_check` 默认开启，从 `herdr.dev` 拉取）以**同版本优先**静默覆盖本地清单，中文审批面板从 `blocked` 退回 `idle`。`src/detect/manifest.rs` 的 `read_remote_manifest` 只在远程版本**严格小于** bundled 时才回退，同版本一律用远程。规则改动后必须：① 升 `version` 与 `updated_at`；② 同步改 `src/detect/manifests/` 和 `distribution/agent-detection/` 两处（检查器要求同版本内容一致）；③ 在线核对 `curl https://herdr.dev/agent-detection/<agent>.toml` 的版本是否已被本地超过。验证手法：用 `XDG_STATE_HOME` 隔离目录植入官方清单做前后对比，看 `herdr agent explain --json` 的 `manifest_source` 与 `state`。
- **根目录 `CHANGELOG.md` 与 `docs/next/CHANGELOG.md` 必须逐字节一致**：上游约定两者相同，`just release-docs-check` 会 `diff` 校验，但 `just check`/CI **不**跑该检查，所以漂移会长期潜伏。在根 CHANGELOG 的 `Unreleased` 加中文条目时，必须同步写入 `docs/next/CHANGELOG.md`；历次同步上游（如 `2efe6653`、`46fd8f5a`）都是两文件同时 +N 行，遵循同一惯例即可。
