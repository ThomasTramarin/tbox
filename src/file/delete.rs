use std::{
    fs::{remove_file, File},
    io::stdin,
    path::Path,
};

use clap::{Arg, ArgAction, ArgMatches, Command};
use colored::Colorize;

pub fn delete_file_command() -> Command {
    Command::new("delete")
        .about("Delete a file with the path provided")
        .arg(
            Arg::new("pathname")
                .required(true)
                .index(1)
                .help("The path of the file to delete"),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .action(ArgAction::SetTrue)
                .help("Force deletion without confirmation"),
        )
}

pub fn delete_file(matches: &ArgMatches) {
    // Get the pathname
    let pathname = matches.get_one::<String>("pathname").unwrap();
    let path = Path::new(pathname);

    // Check if the file exists
    if !path.exists() {
        println!("{}", "File does not exist.".red());
        return;
    }

    // Get the flags
    let force = matches.get_flag("force");

    // Delete the file with confirmation
    if !force {
        println!(
            "{}",
            "Are you sure you want to delete this file? (y/n)".yellow()
        );
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            println!("{}", "Deletion canceled.".red());
            return;
        }
    }

    // Delete the file
    match remove_file(pathname) {
        Ok(_) => println!("{}", "File deleted successfully.".green()),
        Err(e) => println!("{}", format!("Error deleting file: {}", e).red()),
    }
}
