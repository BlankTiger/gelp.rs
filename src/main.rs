use clap::{arg, Command};
use color_eyre::Report;
use gelp::setup::setup;
use std::process;
use tracing::info;

fn main() -> Result<(), Report> {
    setup()?;

    let matches = cli().get_matches();
    let mut base_command = process::Command::new("git");

    if let Some(("go-back", sub_matches)) = matches.subcommand() {
        let num_of_commits = sub_matches.get_one::<String>("n").expect("required");
        info!("Going back {num_of_commits} commits");
        base_command
            .arg("reset")
            .arg("HEAD~".to_owned() + num_of_commits);
    }

    if let Some(("clean", _)) = matches.subcommand() {
        info!("Running git clean -fdx");
        base_command.arg("clean").arg("-fdx");
    }

    if let Some(("reset", _)) = matches.subcommand() {
        info!("Running git reset --hard");
        base_command.arg("reset").arg("--hard");
    }

    if let Some(("ff", _)) = matches.subcommand() {
        info!("Running git pull");
        base_command.arg("pull");
    }

    if let Some(("over", sub_matches)) = matches.subcommand() {
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
                .about("Go back n number of commits without removing files created in commits")
                .arg(arg!(<n> "Number of commits")),
        )
        .subcommand(Command::new("clean").about(
            "Clean current directory out of all files that are not tracked and that are ignored",
        ))
        .subcommand(
            Command::new("reset").about("Reset current directory to the state of the last commit"),
        )
        .subcommand(
            Command::new("ff")
                .about("Fast forward current branch to the latest commit on that branch"),
        )
        .subcommand(
            Command::new("over")
                .about("Overwrite last commit with current changes")
                .arg(
                    arg!(<change_msg> "Should commit message be changed")
                        .default_value("no")
                        .required(false),
                ),
        )
}
