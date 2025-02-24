use clap::{ArgMatches, Command};
use create::{create_file, create_file_command};

use crate::utils::prints::unknown_command;

mod create;

pub fn file_commands() -> Command {
    Command::new("file")
        .about("Command related to managing files")
        .subcommand(create_file_command())
}

pub fn handle_file_command(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("create", sub_matches)) => create_file(sub_matches),
        _ => unknown_command(),
    }
}
