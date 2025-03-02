use clap::{ArgMatches, Command};
use create::{create_folder, create_folder_command};
use delete::{delete_folder, delete_folder_command};

use crate::utils::prints::unknown_command;

mod create;
mod delete;

pub fn folder_commands() -> Command {
    Command::new("folder")
        .about("Command related to managing folders")
        .subcommand(create_folder_command())
        .subcommand(delete_folder_command())
}

pub fn handle_folder_command(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("create", sub_matches)) => create_folder(sub_matches),
        Some(("delete", sub_matches)) => delete_folder(sub_matches),
        _ => unknown_command(),
    }
}
