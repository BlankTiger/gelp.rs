use clap::{arg, Command};
use color_eyre::Report;
use gelp::setup::setup;
use std::process;
use tracing::info;

fn main() -> Result<(), Report> {
    setup()?;

    let matches = cli().get_matches();
    let mut base_command = process::Command::new("git");

    match matches.subcommand() {
        Some(("go-back", sub_matches)) | Some(("-g", sub_matches)) => {
            let num_of_commits = sub_matches.get_one::<String>("n").expect("required");
            info!("Going back {num_of_commits} commits");
            base_command
                .arg("reset")
                .arg("HEAD~".to_owned() + num_of_commits);
        }
        Some(("clean", _)) | Some(("-c", _)) => {
            base_command.arg("clean").arg("-fdx");
            info!("Running git clean -fdx");
        }
        Some(("reset", _)) | Some(("-r", _)) => {
            info!("Running git reset --hard");
            base_command.arg("reset").arg("--hard");
        }
        Some(("ff", _)) | Some(("-f", _)) => {
            info!("Running git pull");
            base_command.arg("pull");
        }
        Some(("over", sub_matches)) | Some(("-o", sub_matches)) => {
            let change_msg = sub_matches
                .get_one::<String>("change_msg")
                .expect("required");

            let should_change = matches!(change_msg.as_str(), "yes");
            info!(
                "Overwriting previous commit with current changes, new commit message: {should_change}"
            );
            base_command.arg("commit").arg("-a").arg("--amend");

            if !should_change {
                base_command.arg("--no-edit");
            }
        }
        _ => {}
    }

    base_command.spawn()?.wait()?;
    Ok(())
}

fn cli() -> Command {
    Command::new("gelp")
        .about("Helper for my commonly used git commands")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("go-back")
                .short_flag('g')
                .about("Go back n number of commits without removing files created in commits")
                .arg(arg!(<n> "Number of commits")),
        )
        .subcommand(Command::new("clean").short_flag('c').about(
            "Clean current directory out of all files that are not tracked and that are ignored",
        ))
        .subcommand(
            Command::new("reset")
                .short_flag('r')
                .about("Reset current directory to the state of the last commit"),
        )
        .subcommand(
            Command::new("ff")
                .short_flag('f')
                .about("Fast forward current branch to the latest commit on that branch"),
        )
        .subcommand(
            Command::new("over")
                .short_flag('o')
                .about("Overwrite last commit with current changes")
                .arg(
                    arg!(<change_msg> "Should commit message be changed")
                        .default_value("no")
                        .required(false),
                ),
        )
}
