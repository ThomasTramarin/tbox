use std::{
    fs::{create_dir, File},
    path::Path,
};

use clap::{Arg, ArgMatches, Command};
use colored::Colorize;

pub fn create_folder_command() -> Command {
    Command::new("create")
        .about("Create a new folder with the path provided")
        .arg(
            Arg::new("pathname")
                .required(true)
                .index(1)
                .help("The path of the folder to create"),
        )
}

pub fn create_folder(matches: &ArgMatches) {
    // Get the pathname
    let pathname = matches.get_one::<String>("pathname").unwrap();
    let path = Path::new(pathname);

    // Check if the folder already exists
    if path.exists() {
        println!("{}", "Folder already exists.".red());
        return;
    }

    // Create a folder
    match create_dir(path) {
        Ok(_) => println!("{}", "Folder created successfully.".green()),
        Err(e) => println!(
            "{}",
            format!("Error creating folder: {}", e.to_string()).red()
        ),
    }
}
