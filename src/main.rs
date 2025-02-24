use clap::Command;
use file::{file_commands, handle_file_command};
use utils::prints::unknown_command;

mod file;
mod utils;

fn main() {
    let matches = Command::new("tbox")
        .about("A simple and powerful CLI tool that bundles multiple useful commands into a single executable, making everyday tasks faster and more efficient.")
        .after_help("The last cli tool you will ever need.")
        .subcommand(file_commands())
        .get_matches();

    match matches.subcommand() {
        Some(("file", sub_matches)) => handle_file_command(sub_matches),
        _ => unknown_command(),
    }
}
