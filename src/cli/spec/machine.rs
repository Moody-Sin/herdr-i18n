use clap::{Arg, Command};

use super::{json_flag, option};

pub(super) fn command() -> Command {
    Command::new("machine")
        .about(crate::tr!("Manage saved SSH machines"))
        .subcommand(
            Command::new("list")
                .about(crate::tr!("List saved SSH machines"))
                .arg(json_flag()),
        )
        .subcommand(
            Command::new("status")
                .about(crate::tr!("Check saved machines without prompting for authentication"))
                .arg(Arg::new("machine").value_name("LABEL_OR_ID"))
                .arg(json_flag()),
        )
        .subcommand(
            Command::new("reconnect")
                .about(crate::tr!("Authenticate a saved machine in this terminal and verify connectivity"))
                .arg(Arg::new("machine").value_name("LABEL_OR_ID").required(true)),
        )
        .subcommand(
            Command::new("add")
                .about(crate::tr!(
                    "Prepare the remote Herdr server and save an SSH machine"
                ))
                .arg(
                    Arg::new("ssh-target")
                        .value_name("SSH_TARGET")
                        .required(true)
                        .help_heading("参数"),
                )
                .arg(
                    option("label", "LABEL").help(crate::tr!(
                        "Set the machine label shown in the sidebar (defaults to the SSH host, or host/session)"
                    )),
                )
                .arg(option("remote-session", "NAME").help(crate::tr!(
                    "Select a session explicitly (default without an interactive terminal)"
                ))),
        )
        .subcommand(
            profile_command("rename", crate::tr!("Rename a saved SSH machine")).arg(
                option("label", "LABEL")
                    .required(true)
                    .help(crate::tr!("Set the machine label shown in the sidebar")),
            ),
        )
        .subcommand(profile_command(
            "remove",
            crate::tr!("Remove a saved SSH machine"),
        ))
        .subcommand(profile_command(
            "enable",
            crate::tr!("Enable a saved SSH machine"),
        ))
        .subcommand(profile_command(
            "disable",
            crate::tr!("Disable a saved SSH machine"),
        ))
}

fn profile_command(name: &'static str, about: &'static str) -> Command {
    Command::new(name).about(about).arg(
        Arg::new("profile-id")
            .value_name("PROFILE_ID")
            .required(true)
            .help_heading("参数"),
    )
}
