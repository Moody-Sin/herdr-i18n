# 本地化文案

此目录保存 Herdr 本地界面使用的简体中文文案。英文原文是消息键，Rust 界面通过 `crate::tr!("English text")` 查找译文；带动态值的文案使用 `crate::trf!("Message {name}", name = value)`。缺失译文时程序显示英文原文，不会报错。

## 新增或修改文案

新增界面文案时，用 `tr!`（无动态值）或 `trf!`（有动态值）包裹，并同步在 `zh-CN.json` 中添加条目。`trf!` 的占位符名称必须与英文原文一致。

命令语法行（如 `herdr pane split ...`）有意保留英文，只翻译说明文字和标签。界面里出现的中文标签如果依赖前导/尾随空格来对齐，请把空格写进 `tr!` 的键里，例如 `crate::tr!(" esc back ")`。

## 检查

运行 `just localization-check`（即 `scripts/check_localization.py`），它会检查：

- 每个 `tr!`/`trf!` 键、以及 `keybind_help.rs` 中每个 `entry(.., "label")` 标签都有译文；
- `trf!` 的实参名称与消息占位符一致，且中英占位符集合一致；
- `src/cli/` 中的界面文案要么已本地化，要么显式登记在 `scripts/localization-cli-allowlist.txt`。

允许清单只用于**非用户可见**的字符串：内部不变量断言（`unreachable!`、`expect`）和机器可读取值。用户能看到的文字必须走 `tr!`，不要加进允许清单。

## 翻译术语与词汇表

本地化翻译请严格遵守 `src/i18n/GLOSSARY.md` 中的统一术语规范，保持全项目术语与心智模型一致（例如：worktree 统一译为“工作树”，pane 统一译为“窗格”，agent 统一译为“智能体”等）。

## 自更新

本复刻仓库禁用 Herdr 官方二进制自更新和自动版本检查（`src/i18n.rs::LOCALIZED_BUILD`），避免官方二进制覆盖本地汉化。升级时同步上游 `master`、合并到 `zh-CN` 分支、按 `just localization-check` 补齐译文，最后重新构建。
