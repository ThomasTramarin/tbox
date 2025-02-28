use clap::{ArgMatches, Command};
use create::{create_file, create_file_command};
use delete::{delete_file, delete_file_command};
use read::{read_file, read_file_command};
use write::{write_file, write_file_command};

use crate::utils::prints::unknown_command;

mod create;
mod delete;
mod read;
mod write;

pub fn file_commands() -> Command {
    Command::new("file")
        .about("Command related to managing files")
        .subcommand(create_file_command())
        .subcommand(delete_file_command())
        .subcommand(read_file_command())
        .subcommand(write_file_command())
}

pub fn handle_file_command(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("create", sub_matches)) => create_file(sub_matches),
        Some(("delete", sub_matches)) => delete_file(sub_matches),
        Some(("read", sub_matches)) => read_file(sub_matches),
        Some(("write", sub_matches)) => write_file(sub_matches),
        _ => unknown_command(),
    }
}
