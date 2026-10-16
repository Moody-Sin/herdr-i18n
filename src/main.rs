use std::io;

pub(crate) const HERDR_ENV_VAR: &str = "HERDR_ENV";
pub(crate) const HERDR_ENV_VALUE: &str = "1";
const NESTED_HERDR_MESSAGES: [&str; 6] = [
    "检测到嵌套启动。我们需要更深入……才怪。",
    "递归是一条通往诸多能力的道路，有些人认为这些能力……不太自然。",
    "你只顾着想自己能不能做到，却没想过该不该做。——马尔科姆博士",
    "已禁用递归调用 Herdr。某处的调用栈终于松了一口气。",
    "已拒绝递归下降。Herdr 真的不能开太多层。",
    "检测到递归。未找到终止条件。已中止。",
];

mod agent_resume;
mod agent_view_eval;
mod api;
mod app;
mod build_info;
mod checksum;
mod cli;
mod client;
mod config;
mod copy_mode;
mod detect;
mod events;
use ghostty_vt as ghostty;
mod handoff_runtime;
mod i18n;
mod input;
mod integration;
mod ipc;
mod kitty_graphics;
mod layout;
mod logging;
mod metadata_tokens;
mod noninteractive_process;
mod pane;
use ghostty_vt::pane_graphics_files;
mod persist;
mod platform;
mod plugin_command;
mod plugin_installations;
mod plugin_paths;
mod popup_size;
mod product_announcements;
mod protocol;
mod pty;
mod raw_input;
mod release_notes;
mod remote;
mod render_prof;
mod render_signal;
mod selection;
mod server;
mod session;
mod sound;
mod terminal;
mod terminal_effects;
mod terminal_modes;
mod terminal_notify;
mod terminal_theme;
mod thread_spawn;
mod ui;
mod update;
mod workspace;
mod worktree;

const DEFAULT_CONFIG: &str = r##"# herdr 配置文件
# 请将此文件保存到 ~/.config/herdr/config.toml

# 启动时显示首次运行通知引导。
# 未配置时也会显示新手引导；配置完成后可设为 false。
# onboarding = true

[theme]
# 内置主题：catppuccin, terminal, tokyo-night, dracula, nord,
#          gruvbox, one-dark, solarized, kanagawa, rose-pine,
#          vesper
# name = "catppuccin"

# 跟随外部终端的浅色/深色外观自动切换 Herdr 界面主题。
# 除非设为 true，否则保持现有的手动指定主题行为。
# auto_switch = false
# dark_name = "catppuccin"
# light_name = "catppuccin-latte"

# 在基础主题之上覆盖单个颜色标识（token）。
# 支持：十六进制 (#rrggbb)、命名颜色、rgb(r,g,b) 或 panel_bg = "reset"
# [theme.custom]
# sidebar_bg = "#181825"
# active_row_bg = "#1e1e2e"
# selection_bg = "#313244"
# panel_bg = "reset"
# accent = "#f5c2e7"
# red = "#ff6188"
# green = "#a6e3a1"

# 当启用 auto_switch 时，针对特定外观叠加覆盖层。
# [theme.custom.light]
# panel_bg = "#eff1f5"
# text = "#4c4f69"
#
# [theme.custom.dark]
# panel_bg = "#1e1e2e"
# text = "#cdd6f4"

[terminal]
# 新建交互式窗格使用的可执行文件。
# 为空时依次读取 $SHELL，后回退为 /bin/sh。
# default_shell = ""

# 新建交互式窗格 shell 的启动模式："auto"、"login" 或 "non_login"。
# "auto" 在 macOS 上使用登录 shell（login shell），在其他平台上保持现有行为。
# shell_mode = "auto"

# 新建窗格、标签页和工作区在未明确提供 --cwd 时的初始工作目录策略。
# "follow" 继承源窗格/工作区；"home" 使用 $HOME；
# "current" 使用 Herdr 进程当前所在目录；或指定绝对路径如 "~/Projects"。
# new_cwd = "follow"

# 在兼容 Kitty 图形协议的外部终端中渲染窗格图像。
# kitty_graphics = true

[update]
# 后台版本检查与 `herdr update` 使用的更新通道。
# 稳定版默认使用 "stable"。Windows 预览版默认使用 "preview"，
# 以确保现有预览版安装在明确切换前保持在预览通道。
# channel = "stable"

# 在后台检查 herdr.dev 是否有新的 Herdr 版本。
# version_check = true

# 在后台检查 herdr.dev 是否有远程智能体检测清单更新。
# manifest_check = true

[keys]
# 进入前缀模式的前缀键（默认："ctrl+b"）
# 示例："ctrl+b", "f12", "esc", "-"
# 动作快捷键使用显式语法："prefix+n" 需要前缀键；
# "ctrl+alt+n" 为直接生效的终端模式全局快捷键。
# 接受的按键语法：普通字母键、ctrl/shift/alt/cmd/super 修饰键，以及 enter/tab/esc/left/right/up/down 等特殊键。
# 亦支持命名标点符号，如 minus, comma, ampersand, plus, backtick。
# 最可靠的直接快捷键是 ctrl+字母、功能键（F1-F12）和显式修饰组合。
# alt+...、cmd/super 以及带修饰键的标点可能受终端/tmux 设置影响。
# prefix = "ctrl+b"

# 前缀模式动作
# help = "prefix+?"
# settings = "prefix+s"
# detach = "prefix+q"
# reload_config = "prefix+shift+r"
# open_notification_target = "prefix+o"
# workspace_picker = "prefix+w"
# goto = "prefix+g"
# new_workspace = "prefix+shift+n"
# new_worktree = "prefix+shift+g"
# open_worktree = ""    # 可选，默认未设置
# remove_worktree = ""  # 可选，默认未设置；触发删除确认弹窗
# rename_workspace = "prefix+shift+w"
# close_workspace = "prefix+shift+d"
# previous_workspace = "" # 可选，默认未设置
# next_workspace = ""     # 可选，默认未设置
# previous_agent = ""     # 可选，默认未设置
# next_agent = ""         # 可选，默认未设置
# focus_agent = ""        # 可选索引快捷键，例如 "prefix+alt+1..9"
# remote_image_paste = "ctrl+v" # 仅在 herdr --remote 中有效；为空禁用原生按键图像粘贴
# new_tab = "prefix+c"
# rename_tab = "prefix+shift+t"
# previous_tab = "prefix+p"
# next_tab = "prefix+n"
# move_tab_previous = ""   # 可选，例如 "alt+shift+left" 将标签页往前移动
# move_tab_next = ""       # 可选，例如 "alt+shift+right" 将标签页往后移动
# switch_tab = "prefix+1..9"
# switch_workspace = ""   # 可选索引快捷键，例如 "prefix+shift+1..9"
# close_tab = "prefix+shift+x"
# rename_pane = "prefix+shift+p"
# edit_scrollback = "prefix+e"
# clear_pane = ""                  # 未绑定；例如 "prefix+ctrl+k"
# focus_pane_left = "prefix+h"
# focus_pane_down = "prefix+j"
# focus_pane_up = "prefix+k"
# focus_pane_right = "prefix+l"
# cycle_pane_next = "prefix+tab"
# cycle_pane_previous = "prefix+shift+tab"
# last_pane = ""          # 可选，默认未设置；例如绑定 "prefix+tab" 实现全局往返切换
# split_vertical = "prefix+v"
# split_horizontal = "prefix+minus"
# close_pane = "prefix+x"
# zoom = "prefix+z"       # 历史别名：fullscreen
# resize_mode = "prefix+r"
# resize_pane_left = ""   # 可选，例如 "ctrl+shift+alt+left" 无需进入调整大小模式直接调整
# resize_pane_down = ""   # 可选，例如 "ctrl+shift+alt+down"
# resize_pane_up = ""     # 可选，例如 "ctrl+shift+alt+up"
# resize_pane_right = ""  # 可选，例如 "ctrl+shift+alt+right"
# toggle_sidebar = "prefix+b"

# 导航模式移动。当导航模式打开时，这些局部快捷键优先。
# 独立于 focus_pane_*。此处请勿包含 prefix+、esc、enter、tab 或 1..9。
# navigate_workspace_up = "up"
# navigate_workspace_down = "down"
# navigate_pane_left = "h"      # 左方向键始终聚焦左侧窗格
# navigate_pane_down = "j"
# navigate_pane_up = "k"
# navigate_pane_right = "l"     # 右方向键始终聚焦右侧窗格

# 自定义命令使用相同的按键语法。
# type = "shell" 在后台分离运行。
# type = "pane" 打开临时窗格并在命令退出时关闭。
# type = "popup" 打开会话模态终端弹窗，不改变原有标签页布局。
# 弹窗 width 与 height 支持终端单元格数或百分比（如 "80%"）。
# 在 Windows 上，命令字符串通过 cmd.exe /d /c 执行。
# [[keys.command]]
# key = "prefix+alt+g"
# type = "popup"
# command = "lazygit"
# width = "80%"
# height = "80%"

# 历史索引快捷键配置仍被解析以保持向后兼容。
# 新配置建议优先使用 switch_tab、switch_workspace 和 focus_agent。
# [keys.indexed]
# tabs = ""       # 例如 "ctrl" 使 ctrl+1..9 直接切换标签页
# workspaces = "" # 例如 "ctrl+shift" 使 ctrl+shift+1..9 直接切换工作区
# agents = ""     # 例如 "alt" 使 alt+1..9 直接聚焦智能体行

# 未连接客户端时虚拟终端的大小。
# 连接的客户端始终使用自身的终端尺寸。
[server]
# 仅限 Windows：允许同账户下的普通客户端控制已提权的服务器。
# 修改后需要重启服务器。
# allow_unelevated_clients = false
# headless_cols = 120
# headless_rows = 40

# [worktrees]
# directory = "~/.herdr/worktrees"

[ui]
# 侧边栏宽度（根据工作区名称自适应缩放，此项设置默认宽度）
# sidebar_width = 26

# 展开时侧边栏最小宽度（列数）
# sidebar_min_width = 18

# 展开时侧边栏最大宽度（列数）
# sidebar_max_width = 36

# 启动时默认折叠侧边栏。更改将在下次启动时生效。
# sidebar_start_collapsed = false

# 折叠侧边栏显示样式："compact" 保留狭窄的状态轨，"hidden" 完全隐藏（宽度为0）。
# sidebar_collapsed_mode = "compact"

# 终端宽度小于或等于此值时，Herdr 将使用单列移动端布局。
# 折叠屏、平板或宽屏手机终端可调大此阈值。
# mobile_width_threshold = 64

# 捕获鼠标输入以使用 Herdr 鼠标交互 UI。
# 设为 false 让外部终端处理正常点击（如按住 Cmd 点击 URL）。
# lazygit、btop 等窗格程序在主动请求鼠标时仍可正常接收。
# mouse_capture = true

# 自动复制鼠标选中的文本。
# 设为 false 将保留拖拽或双击选词，直到按 Ctrl+C（或主机转发的 Cmd+C）复制并清除选区。
# copy_on_select = true

# 主机光标策略："auto"、"native" 或 "drawn"。
# "auto" 在原生 Windows 构建和 WSL 上绘制 Herdr 自己的光标以避免 ConPTY 光标闪烁，其他平台使用原生终端光标。
# "native" 始终使用外部终端光标。"drawn" 始终将 Herdr 光标作为终端单元格内容绘制。
# host_cursor = "auto"

# 可选修饰键：按住该修饰键右键拖拽时，将事件转发给窗格应用而非打开 Herdr 窗格菜单。
# 为空/off 禁用此行为。Shift 有意不支持，因为终端通常保留 Shift+鼠标。
# right_click_passthrough_modifier = ""

# 外部终端重新获得焦点时强制全量重绘。
# 设为 false 可减少切回 Herdr 时的可见闪烁。
# 权衡：极少数终端表面损坏可能残留，直至下一次完整重绘。
# redraw_on_focus_gained = true

# 鼠标滚轮每刻度滚动的窗格回滚行数。
# mouse_scroll_lines = 3

# 关闭工作区前请求确认
# confirm_close = true

# 创建新标签页前提示输入名称。
# 设为 false 将立即使用自动生成的名称创建标签页。
# prompt_new_tab_name = true

# 交互式创建工作区前提示输入名称。
# prompt_new_workspace_name = false

# 在分割窗格周围绘制边框。
# "auto" 仅在存在分割窗格时绘制；"always" 在单个孤立窗格时也绘制边框（仅当 pane_outer_borders 启用时）；"off" 禁用边框。
# 仍兼容旧布尔值：true = "auto", false = "off"。
# pane_borders = "auto"

# 沿窗格区域外边缘绘制外边框。
# 禁用时为类似 tmux 的内部无外框分割线样式。
# pane_outer_borders = true

# 在终端窗格旁绘制交互式滚动条。
# 设为 false 可回收滚动条列，避免其进入终端原生选择区。
# pane_scrollbars = true

# 保持分割窗格视觉分隔，而非共享分割线。
# pane_gaps = true

# 未手动设置窗格名称时，在分割窗格边框显示检测到/上报的智能体标签。
# show_agent_labels_on_pane_borders = false

# 当工作区仅有 1 个标签页时隐藏标签栏。
# 仍可使用配置的快捷键创建新标签页。
# hide_tab_bar_when_single_tab = false

# 桌面端标签栏位置："top"（顶部）或 "bottom"（底部）。
# tab_bar_position = "top"

# 桌面端标签栏右侧按顺序排列的状态项。
# 支持类型：zoom, hostname, datetime, text, 和 command。
# hostname, datetime, 和 command 项在 Herdr 服务器上解析。
# tab_bar_right = []
# tab_bar_right_separator = " "

# Herdr 写入外部终端的窗口标题（由窗口管理器显示在标题栏、标签或分组栏）。
# 支持的标记有 {hostname}, {workspace}, {tab}, {pane}, 和 {terminal_title}；{{ 与 }} 表示字面量大括号。
# 标题在 Herdr 服务器端渲染，即使远程客户端连接，{hostname} 也是窗格运行所在的主机名。
# 设为 "" 则保持外部终端标题不变。
# window_title = "{hostname}: {workspace}"

# 智能体侧边栏面板排序方式："spaces"（按空间分组）或 "priority"（关注队列）。
# "workspaces" 作为 "spaces" 的别名同样被支持。
# agent_panel_sort = "spaces"

# 智能体状态指示符："dots" 保留紧凑的彩色圆点；"symbols" 使用
# 区分度高的静态符号分别表示已阻塞、工作中、已完成、空闲和未知状态。
# status_indicators = "dots"

# 高亮、边框和导航界面的强调色（accent）。
# 支持：十六进制 (#89b4fa)、命名颜色 (cyan, blue, magenta) 或 rgb(r,g,b)
# accent = "cyan"

# 展开的智能体行。内置标识有 state_icon, state_text, machine, workspace, tab,
# pane, agent, terminal_title, 和 terminal_title_stripped。
# 通过窗格元数据上报的自定义值使用 $name 标记。
# 标记可定制样式：{ token = "workspace", fg = "#89b4fa", bold = true, dim = false }。
# 省略的样式字段保留上下文默认值。
# [ui.sidebar.agents]
# 智能体条目之间的空行数。设为 1 恢复早期的宽松间距。
# row_gap = 0
# rows = [["state_icon", "machine", "workspace", "tab"], ["agent"]]
# 可选的规范智能体 ID 可为匹配的智能体替换默认行配置。
# [ui.sidebar.agents.rows_by_agent]
# claude = [["state_icon", "machine", "workspace", "tab"], ["terminal_title_stripped"], ["agent"]]

# 展开的工作区（Space）行。内置标识有 state_icon, state_text, workspace, branch, 和 git_status。
# 通过工作区元数据上报的自定义值使用 $name 标记，例如 $jj_status。
# 行内标记样式支持严格的 #RGB/#RRGGBB 前景色以及 bold、dim 布尔值。
# [ui.sidebar.spaces]
# 工作区条目之间的空行数。设为 1 恢复早期的宽松间距。
# row_gap = 0
# rows = [["state_icon", "workspace"], ["branch", "git_status"]]

# 后台通知弹窗投递设置
[ui.toast]
# off = 禁用弹窗通知
# herdr = 显示应用内通知弹窗
# terminal = 请求外部终端显示桌面通知
# system = 直接请求操作系统原生通知服务
# delivery = "off"
# delay_seconds = 1

[ui.toast.herdr]
# position = "bottom-right"

[ui.toast.clipboard]
# enabled = true
# position = "bottom-center"

# 当智能体在后台工作区改变状态时播放提示音
[ui.sound]
# enabled = true
# 可选自定义 mp3 音频文件。相对路径相对于此配置文件的所在目录解析。
# path = "sounds/notification.mp3"   # 适用于所有声音通知的通用 mp3 文件
# done_path = "sounds/done.mp3"      # 仅覆盖任务完成通知声音
# request_path = "sounds/request.mp3" # 仅覆盖需要关注通知声音

# 单个智能体声音覆盖：default | on | off
# 默认情况下 droid 处于静音状态。
# [ui.sound.agents]
# droid = "off"

[session]
# 在 Herdr 服务器重启后，将受支持的 AI 智能体窗格恢复到其原生的对话会话中。
# 需要上报 session ref 的官方集成支持。
# resume_agents_on_restore = true
# 自动恢复智能体之间的启动间隔毫秒数；0 表示同时启动不留间隔。
# startup_per_agent_delay_ms = 100

[remote]
# Herdr 是否管理 `herdr --remote` 使用的 ssh 配置。
# 为 true（默认）时，Herdr 通过生成的配置运行远程 ssh，该配置优先包含
# 你的 ~/.ssh/config，并添加 ServerAliveInterval/ServerAliveCountMax
# 作为回退（你自定义的保持连接参数仍然优先），以在空闲网络/NAT超时中存活。
# Herdr 还使用私有的每连接 OpenSSH control socket 复用首次认证连接。
# 设为 false 将直接原样使用你的 ssh 配置运行普通 ssh。
# manage_ssh_config = true

[experimental]
# 允许在受 Herdr 管理的窗格内部再次嵌套启动 herdr。
# allow_nested = false
# 在完整服务器重启之间保存最近的窗格屏幕历史记录。
pane_history = false
# 当处于前缀模式时，临时将系统输入法切换为 ASCII 兼容模式，
# 以便在开启中文等输入法时前缀快捷键仍能正确响应；退出前缀模式时恢复原输入法。
# macOS 上选择兼容 ASCII 的键盘布局；Windows 上在韩文输入法的韩/英模式间切换。
# 仅限 macOS 和 Windows；尽力而为。默认：false。
# switch_ascii_input_source_in_prefix = false
# 将焦点窗格的光标暴露给外部终端，以便在 TUI 自行绘制光标（Claude Code、pi、codex）时，
# macOS 输入法仍能跟踪候选词浮窗位置。
# 权衡：对于隐藏光标且未绘制替代项的应用（如 vim normal mode），会多显示一个光标。
# reveal_hidden_cursor_for_cjk_ime = false
# 可选白名单：仅当焦点窗格检测到的智能体匹配此列表时才暴露光标。为空则对任意焦点窗格生效。
# 若列表无有效名称，则不生效。
# 接受的名称：pi, claude, codex, gemini, cursor, devin, cline, opencode,
# copilot, kimi, kiro, droid, amp, grok, hermes, kilo, qodercli, qoder, qwen,
# qwen-code, letta, letta-code, maki。
# cjk_ime_agents = []
# 当 reveal_hidden_cursor_for_cjk_ime 为 true 时渲染的光标形状。
# 可选值：block, steady_block (默认), underline, steady_underline, bar, steady_bar。
# cjk_ime_cursor_shape = "steady_block"

[advanced]
# 每个窗格终端保留的最大回滚缓冲区大小（字节）。
# 遵循 Ghostty 默认的 scrollback-limit 行为。
# scrollback_limit_bytes = 10000000
"##;

// Bundled at build time so the printed skill always matches this binary's release.
const SKILL: &str = include_str!("../skills/herdr/SKILL.md");

fn should_block_nested(config: &config::Config) -> bool {
    should_block_nested_for_env(config, std::env::var(HERDR_ENV_VAR).ok().as_deref())
}

fn should_block_nested_for_env(config: &config::Config, herdr_env: Option<&str>) -> bool {
    !config.experimental.allow_nested && herdr_env == Some(HERDR_ENV_VALUE)
}

fn random_nested_message() -> &'static str {
    use std::time::{SystemTime, UNIX_EPOCH};

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.subsec_nanos() as usize)
        .unwrap_or(0);
    let index = (nanos ^ (std::process::id() as usize)) % NESTED_HERDR_MESSAGES.len();
    NESTED_HERDR_MESSAGES[index]
}

fn exit_if_nested_disabled(config: &config::Config) {
    if should_block_nested(config) {
        eprintln!(
            "\x1b[1m{}\x1b[0m",
            crate::tr!("error: nested herdr is disabled by default.")
        );
        eprintln!(
            "{}",
            crate::tr!("see configuration if you want to enable it.")
        );
        eprintln!();
        eprintln!("\x1b[2m\"{}\"\x1b[0m", random_nested_message());
        std::process::exit(1);
    }
}

/// Pads to a terminal-column width, not a `char` count.
///
/// `{:<32}` counts `char`s, so a command carrying a translated `<子命令>`
/// placeholder (one `char`, two columns each) is padded short and pushes the
/// description column out of alignment.
fn pad_display_width(text: &str, width: usize) -> String {
    let padding = width.saturating_sub(unicode_width::UnicodeWidthStr::width(text)) + 1;
    format!("{text}{}", " ".repeat(padding))
}

fn args_as_utf8<I>(args: I) -> Result<Vec<String>, String>
where
    I: IntoIterator<Item = std::ffi::OsString>,
{
    args.into_iter()
        .enumerate()
        .map(|(index, arg)| {
            arg.into_string()
                .map_err(|_| format!("argument {index} is not valid UTF-8"))
        })
        .collect()
}

fn finish_cli(outcome: io::Result<cli::CommandOutcome>) -> io::Result<()> {
    match outcome {
        Ok(cli::CommandOutcome::Handled(code)) => std::process::exit(code),
        Ok(cli::CommandOutcome::NotCli) => Ok(()),
        Err(err) if cli::protocol_mismatch_was_reported(&err) => std::process::exit(1),
        Err(err) if cli::server_not_running_was_reported(&err) => {
            if let Some(response) = cli::server_not_running_reported_response(&err) {
                if let Ok(json) = serde_json::to_string(response) {
                    eprintln!("{json}");
                }
            }
            std::process::exit(1);
        }
        Err(err) => Err(err),
    }
}

fn main() -> io::Result<()> {
    i18n::initialize();
    let raw_args: Vec<String> = match args_as_utf8(std::env::args_os()) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", crate::trf!("error: {error}", error = err));
            eprintln!("{}", crate::tr!("run 'herdr --help' for usage"));
            std::process::exit(2);
        }
    };
    #[cfg(windows)]
    if let Some(result) = platform::maybe_activate_desktop_notification(&raw_args) {
        return result;
    }
    if let Some(outcome) = cli::maybe_run_machine(&raw_args) {
        return finish_cli(outcome);
    }
    let args = match session::configure_from_args(&raw_args) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", crate::trf!("error: {error}", error = err));
            eprintln!("{}", crate::tr!("run 'herdr --help' for usage"));
            std::process::exit(2);
        }
    };
    let (args, remote_launch) = match remote::extract_remote_args(&args) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("{}", crate::trf!("error: {error}", error = err));
            eprintln!("{}", crate::tr!("run 'herdr --help' for usage"));
            std::process::exit(2);
        }
    };

    if remote_launch.is_some()
        && args.get(1).is_some()
        && !args.iter().any(|a| {
            matches!(
                a.as_str(),
                "--help" | "-h" | "--version" | "-V" | "--default-config" | "--skill"
            )
        })
    {
        eprintln!(
            "{}",
            crate::tr!("error: --remote can only be used with the default launch command")
        );
        eprintln!("{}", crate::tr!("run 'herdr --help' for usage"));
        std::process::exit(2);
    }

    finish_cli(cli::maybe_run(&args))?;

    if args.get(1).map(String::as_str) == Some("remote-api-bridge") {
        return remote::run_remote_api_bridge(&args[2..]);
    }

    // Subcommands and flags (no TUI, no logging needed)
    if args.get(1).map(|s| s.as_str()) == Some("remote-client-bridge") {
        return remote::run_remote_client_bridge(&args[2..]);
    }

    if args.get(1).map(|s| s.as_str()) == Some("server") {
        return server::headless::run_server();
    }

    // Hidden client mode: connect to an existing server's client socket.
    if args.get(1).map(|s| s.as_str()) == Some("client") {
        let loaded_config = config::Config::load();
        exit_if_nested_disabled(&loaded_config.config);
        return client::run_client();
    }

    if args.get(1).map(|s| s.as_str()) == Some("update") {
        let options = match update::parse_self_update_args(&args[2..]) {
            Ok(options) => options,
            Err(err) if err.starts_with("usage:") => {
                eprintln!("{err}");
                std::process::exit(0);
            }
            Err(err) => {
                eprintln!("{err}");
                eprintln!("{}", crate::tr!("usage: herdr update [--handoff]"));
                std::process::exit(2);
            }
        };
        match update::self_update(options) {
            Ok(_) => return Ok(()),
            Err(e) => {
                // A localized build refuses self-update by design; that refusal is
                // reported as-is rather than wrapped as a failure. Branch on the
                // build flag instead of matching the message text, because the
                // message is translated at runtime and no English prefix survives.
                if crate::i18n::LOCALIZED_BUILD {
                    eprintln!("{e}");
                } else {
                    eprintln!("{}", crate::trf!("update failed: {error}", error = e));
                }
                std::process::exit(1);
            }
        }
    }

    if args.iter().any(|a| a == "--help" || a == "-h") {
        platform::begin_cli_output();
        println!(
            "{}",
            crate::tr!("herdr — terminal workspace manager for AI coding agents")
        );
        println!();
        println!("{} herdr [选项]", crate::tr!("Usage:"));
        println!("       herdr --session <名称> [选项]");
        println!("       herdr --machine <标签或 ID> <命令>");
        println!("       herdr --remote <SSH 目标> [--session <名称>]");
        println!("       herdr session attach <名称>");
        println!("       herdr completion zsh");
        println!("       herdr update [--handoff]");
        println!("       herdr channel set <stable|preview>");
        println!("       herdr machine <子命令> ...");
        println!("       herdr server stop");
        println!("       herdr server reload-config");
        println!("       herdr api <子命令> ...");
        println!("       herdr completion <shell>");
        println!("       herdr config <子命令> ...");
        println!("       herdr channel <子命令> ...");
        println!("       herdr workspace <子命令> ...");
        println!("       herdr worktree <子命令> ...");
        println!("       herdr tab <子命令> ...");
        println!("       herdr notification <子命令> ...");
        println!("       herdr agent <子命令> ...");
        println!("       herdr pane <子命令> ...");
        println!("       herdr session <子命令> ...");
        println!("       herdr integration <子命令> ...");
        println!();
        println!("{}", crate::tr!("Common commands:"));
        for (command, description) in [
            (
                "herdr",
                crate::tr!("Launch or attach to the persistent session"),
            ),
            (
                "herdr status [server|client]",
                crate::tr!("Show local client and running server status"),
            ),
            (
                "herdr update",
                crate::tr!("Self-update is disabled for this local Chinese build"),
            ),
            (
                "herdr completion zsh",
                crate::tr!("Generate shell completions for zsh"),
            ),
            (
                "herdr server stop",
                crate::tr!("Stop the running server via the API socket"),
            ),
            (
                "herdr channel set <stable|preview>",
                crate::tr!("Choose the stable or preview update channel"),
            ),
            (
                "herdr server reload-config",
                crate::tr!("Reload config.toml in the running server"),
            ),
            (
                "herdr config reset-keys",
                crate::tr!("Back up config.toml and remove custom keybindings"),
            ),
            (
                "herdr channel <子命令>",
                crate::tr!("Manage the stable or preview update channel"),
            ),
            (
                "herdr machine <子命令>",
                crate::tr!("Manage saved SSH machines"),
            ),
            (
                "herdr api <子命令>",
                crate::tr!("Inspect socket API metadata and live runtime state"),
            ),
            (
                "herdr workspace <子命令>",
                crate::tr!("Workspace helpers over the socket API"),
            ),
            (
                "herdr worktree <子命令>",
                crate::tr!("Git worktree helpers over the socket API"),
            ),
            (
                "herdr tab <子命令>",
                crate::tr!("Tab helpers over the socket API"),
            ),
            (
                "herdr notification <子命令>",
                crate::tr!("Notification helpers over the socket API"),
            ),
            (
                "herdr agent <子命令>",
                crate::tr!("Agent/terminal helpers over the socket API"),
            ),
            (
                "herdr pane <子命令>",
                crate::tr!("Pane control helpers over the socket API"),
            ),
            (
                "herdr session <子命令>",
                crate::tr!("Manage named persistent sessions"),
            ),
            (
                "herdr integration <子命令>",
                crate::tr!("Manage built-in agent integrations"),
            ),
        ] {
            // `{command:<32}` pads by `char` count, so a command containing a
            // translated `<子命令>` placeholder would be padded short and push
            // every description in the list out of alignment.
            println!("  {}{}", pad_display_width(command, 32), description);
        }
        println!();
        println!("{}", crate::tr!("Advanced commands:"));
        println!(
            "  {:<32} {}",
            "herdr server",
            crate::tr!("Run as headless server")
        );
        println!();
        println!("{}", crate::tr!("Options:"));
        println!(
            "  --session <名称>    {}",
            crate::tr!("Use or create a named persistent session")
        );
        println!(
            "  --machine <标签或 ID>  {}",
            crate::tr!("Run an API command on a saved SSH machine")
        );
        println!(
            "  --remote <SSH 目标>   {}",
            crate::tr!("Attach through SSH to a remote Herdr server")
        );
        println!("  --remote-keybindings <local|server>");
        println!(
            "                      {}",
            crate::tr!("Keybindings for --remote app attach (default: local)")
        );
        println!(
            "  --handoff           {}",
            crate::tr!("Opt into live handoff for update or remote attach")
        );
        println!(
            "  --default-config    {}",
            crate::tr!("Print default configuration and exit")
        );
        println!(
            "  --skill             {}",
            crate::tr!("Print the agent skill file and exit")
        );
        println!(
            "  --version, -V       {}",
            crate::tr!("Print version and exit")
        );
        println!("  --help, -h          {}", crate::tr!("Show this help"));
        println!();
        println!(
            "{}",
            crate::trf!("Config: {path}", path = config::config_path().display())
        );
        println!(
            "{}",
            crate::trf!("Logs: {paths}", paths = logging::help_log_paths_summary())
        );
        println!(
            "{}",
            crate::tr!("Env: HERDR_CONFIG_PATH overrides config file path")
        );
        println!("{}", crate::tr!("Home: https://herdr.dev"));
        println!();
        println!("{}", cli::AGENT_HELP_FOOTER);
        return Ok(());
    }

    if args.iter().any(|a| a == "--version" || a == "-V") {
        platform::begin_cli_output();
        println!("herdr {}", crate::build_info::version());
        return Ok(());
    }

    if args.iter().any(|a| a == "--default-config") {
        platform::begin_cli_output();
        print!("{DEFAULT_CONFIG}");
        return Ok(());
    }

    if args.iter().any(|a| a == "--skill") {
        platform::begin_cli_output();
        print!("{SKILL}");
        return Ok(());
    }

    // Reject unknown flags
    let known_flags = [
        "--session",
        "--machine",
        "--remote",
        "--remote-keybindings",
        "--version",
        "-V",
        "--default-config",
        "--skill",
        "--help",
        "-h",
    ];
    for arg in &args[1..] {
        let arg_name = arg.split_once('=').map(|(name, _)| name).unwrap_or(arg);
        if arg.starts_with('-') && !known_flags.contains(&arg_name) {
            eprintln!("{}", crate::trf!("unknown option: {option}", option = arg));
            eprintln!("{}", crate::tr!("run 'herdr --help' for usage"));
            std::process::exit(2);
        }
        if !arg.starts_with('-')
            && ![
                "server",
                "client",
                "remote-client-bridge",
                "update",
                "status",
                "config",
                "channel",
                "machine",
                "workspace",
                "worktree",
                "pane",
                "session",
                "integration",
            ]
            .contains(&arg.as_str())
        {
            eprintln!(
                "{}",
                crate::trf!("unknown command: {command}", command = arg)
            );
            eprintln!("{}", crate::tr!("run 'herdr --help' for usage"));
            std::process::exit(2);
        }
    }

    if let Some(remote_launch) = remote_launch {
        let remote_target = remote_launch.target.clone();
        if let Err(err) = remote::run_remote(remote_launch) {
            eprintln!("error: {err}");
            remote::print_remote_error_hint(&err, &remote_target);
            std::process::exit(1);
        }
        return Ok(());
    }

    let loaded_config = config::Config::load();
    exit_if_nested_disabled(&loaded_config.config);

    let saved_federation =
        client::endpoint::EndpointCatalog::load().is_ok_and(|catalog| catalog.has_enabled_ssh());
    if let Err(err) = server::autodetect::auto_detect_launch(saved_federation) {
        eprintln!("herdr: {err}");
        std::process::exit(1);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_lists_ui_accent_before_nested_tables() {
        let accent_marker = "# accent = \"cyan\"";
        assert_eq!(DEFAULT_CONFIG.matches(accent_marker).count(), 1);

        let accent = DEFAULT_CONFIG.find(accent_marker).unwrap();
        let sidebar = DEFAULT_CONFIG.find("# [ui.sidebar.agents]").unwrap();

        assert!(accent < sidebar);
    }

    #[test]
    fn nested_herdr_blocks_when_env_is_set() {
        let config = config::Config::default();
        assert!(should_block_nested_for_env(&config, Some(HERDR_ENV_VALUE)));
    }

    #[test]
    fn nested_herdr_does_not_block_when_allowed() {
        let config: config::Config =
            toml::from_str("[experimental]\nallow_nested = true\n").unwrap();
        assert!(!should_block_nested_for_env(&config, Some(HERDR_ENV_VALUE)));
    }

    #[test]
    fn nested_herdr_does_not_block_without_env() {
        let config = config::Config::default();
        assert!(!should_block_nested_for_env(&config, None));
    }

    #[test]
    fn random_nested_message_comes_from_known_set() {
        let message = random_nested_message();
        assert!(NESTED_HERDR_MESSAGES.contains(&message));
    }

    #[test]
    fn nested_message_strings_no_longer_repeat_herdr_prefix() {
        assert!(NESTED_HERDR_MESSAGES
            .iter()
            .all(|message| !message.starts_with("herdr:")));
    }

    #[cfg(unix)]
    fn invalid_utf8_arg() -> std::ffi::OsString {
        use std::os::unix::ffi::OsStringExt;
        std::ffi::OsString::from_vec(vec![0xff])
    }

    #[cfg(windows)]
    fn invalid_utf8_arg() -> std::ffi::OsString {
        use std::os::windows::ffi::OsStringExt;
        std::ffi::OsString::from_wide(&[0xd800])
    }

    #[test]
    fn args_as_utf8_passes_through_valid_arguments() {
        let args = ["herdr", "pane", "get", "pane-1"].map(std::ffi::OsString::from);
        assert_eq!(
            args_as_utf8(args).unwrap(),
            ["herdr", "pane", "get", "pane-1"]
        );
    }

    #[test]
    fn args_as_utf8_reports_the_offending_argument_instead_of_panicking() {
        let args = vec![
            std::ffi::OsString::from("herdr"),
            std::ffi::OsString::from("pane"),
            invalid_utf8_arg(),
        ];
        assert_eq!(
            args_as_utf8(args).unwrap_err(),
            "argument 2 is not valid UTF-8"
        );
    }
}
