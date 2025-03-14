use std::{
    fs::{remove_dir, remove_dir_all},
    path::Path,
};

use clap::{Arg, ArgAction, ArgMatches, Command};
use colored::Colorize;

pub fn delete_folder_command() -> Command {
    Command::new("delete")
        .about("Delete a folder with the path provided")
        .arg(
            Arg::new("pathname")
                .required(true)
                .index(1)
                .help("The path of the folder to delete"),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .action(ArgAction::SetTrue)
                .help("Force deletion without confirmation"),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Delete all files in the folder")
                .action(ArgAction::SetTrue),
        )
}

pub fn delete_folder(matches: &ArgMatches) {
    // Get the pathname
    let pathname = matches.get_one::<String>("pathname").unwrap();
    let path = Path::new(pathname);

    // Check if the folder exists
    if !path.exists() {
        println!("{}", "Folder does not exist.".red());
        return;
    }

    // Asking for confirmation if force flag is not set
    let force = matches.get_flag("force");

    if !force {
        let mut input = String::new();

        println!(
            "{}",
            "Are you sure you want to delete this folder? (y/n)".yellow()
        );
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() != "y" {
            println!("{}", "Deletion canceled.".red());
            return;
        }
    }

    // Delete the folder
    let all = matches.get_flag("all");

    if all {
        match remove_dir_all(path) {
            Ok(_) => println!("{}", "Folder deleted successfully.".green()),
            Err(e) => println!("{}", format!("Error deleting folder: {}", e).red()),
        }
        return;
    }

    match remove_dir(path) {
        Ok(_) => println!("{}", "Folder deleted successfully.".green()),
        Err(e) => println!("{}", format!("Error deleting folder: {}", e).red()),
    }
}
