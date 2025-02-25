use std::{
    fs::{remove_file, File},
    io::stdin,
    path::Path,
};

use clap::{Arg, ArgMatches, Command};
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

    // Get che confirmation from the user
    let mut confirm = String::new();
    println!("Are you sure you want to delete {}? (y/n)", pathname);
    stdin()
        .read_line(&mut confirm)
        .expect("Failed to read input");

    if confirm.trim().to_lowercase() == "y" {
        // Delete the file
        match remove_file(pathname) {
            Ok(_) => println!("{}", "File deleted successfully.".green()),
            Err(e) => println!("Error deleting file: {}", e.to_string().red()),
        }
    } else {
        println!("{}", "File deletion canceled.".blue());
    }
}
