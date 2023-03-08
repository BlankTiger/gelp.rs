use clap::{arg, Command};
use color_eyre::Report;
use gelper::setup::setup;
use std::process;
use tracing::info;

// enum Action {
//     Reset,
//     AddAll,
//     Clean,
//     CommitOver,
//     Commit,
// }

fn main() -> Result<(), Report> {
    setup()?;

    let matches = cli().get_matches();

    if let Some(("go-back", sub_matches)) = matches.subcommand() {
        let num_of_commits = sub_matches.get_one::<String>("n").expect("required");
        info!("Going back {num_of_commits} commits");
        process::Command::new("git")
            .arg("reset")
            .arg("HEAD~".to_owned() + num_of_commits)
            .spawn()?;
    }

    if let Some(("add-all", sub_matches)) = matches.subcommand() {}

    Ok(())
}

fn cli() -> Command {
    Command::new("gelper")
        .about("Helper for my commonly used git commands")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("go-back")
                .about("Go back n number of commits")
                .arg(arg!(<n> "Number of commits"))
                .arg_required_else_help(true),
        )
}
