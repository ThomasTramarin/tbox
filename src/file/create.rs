use std::{fs::File, path::Path};

use clap::{Arg, ArgMatches, Command};
use colored::Colorize;

pub fn create_file_command() -> Command {
    Command::new("create")
        .about("Create a new file with the path provided")
        .arg(
            Arg::new("pathname")
                .required(true)
                .index(1)
                .help("The path of the file to create"),
        )
}

pub fn create_file(matches: &ArgMatches) {
    // Get the pathname
    let pathname = matches.get_one::<String>("pathname").unwrap();
    let path = Path::new(pathname);

    // Check if the file already exists
    if path.exists() {
        println!("{}", "File already exists.".red());
        return;
    }

    // Create a file
    match File::create(path) {
        Ok(_) => println!("{}", "File created successfully.".green()),
        Err(e) => println!("Error creating file: {}", e.to_string().red()),
    }
}
