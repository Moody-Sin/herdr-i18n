use clap::{Arg, Command};

pub(super) fn command() -> Command {
    Command::new("completion")
        .visible_alias("completions")
        .about(crate::tr!("Generate shell completion scripts"))
        .arg(
            Arg::new("shell")
                .value_name("SHELL")
                .required(true)
                .help_heading("参数")
                .value_parser(super::super::completion::SUPPORTED_SHELLS)
                .help(crate::tr!("Shell to generate completions for")),
        )
}
