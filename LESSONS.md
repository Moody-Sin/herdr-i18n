# 经验教训 (Lessons Learned)

## 构建与工具链
- **Zig 0.16.0 编译 vendored libghostty-vt**：`ghostty-vt` 依赖 Zig 编译，系统必须安装 `zig 0.16.0`。
- **构建网络与本地代理**：Zig 在下载依赖包时若经由代理服务器可能会由于请求头/协议出现 400 Bad Request，必要时使用直连或清空 proxy 环境变量 (`http_proxy=""`) 进行构建。
- **任务运行器**：使用 `just` 作为任务运行器，测试执行通过 `cargo nextest`。
