# 本地化文案

此目录保存 Herdr 本地界面使用的简体中文文案。英文原文是消息键，Rust 界面通过 `crate::tr!("English text")` 查找译文；带动态值的文案使用 `crate::trf!("Message {name}", name = value)`。

新增或修改界面文案时，请同步更新 `zh-CN.json`。动态占位符名称必须与英文原文一致。运行 `just localization-check` 可检查缺失文案、重复键和占位符不匹配；缺失译文时程序会显示英文原文。

此分支禁用 Herdr 官方二进制自更新和自动版本检查，避免官方二进制覆盖本地汉化。升级时先同步 `origin` 的稳定标签，再合并到 `zh-CN` 分支、补齐检查器指出的译文，最后重新构建。
