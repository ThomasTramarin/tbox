use std::{
    fs::{self},
    io::stdin,
    path::Path,
};

use clap::{Arg, ArgAction, ArgMatches, Command};
use colored::Colorize;

pub fn clear_file_command() -> Command {
    Command::new("clear")
        .about("Clear the content of a file")
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

pub fn clear_file(matches: &ArgMatches) {
    // Get the pathname and force flag
    let pathname = matches.get_one::<String>("pathname").unwrap();
    let force = matches.get_flag("force");

    // Check if the file exists
    let path = Path::new(pathname);
    if !path.exists() {
        println!("{}", "File does not exist.".red());
        return;
    }

    // Confirm action before deleting
    if !force && confirm_overwrite(pathname) {
        println!("{}", "Clear canceled.".red());
        return;
    }

    // Clear the file
    match fs::write(pathname, b"") {
        Ok(_) => println!("{}", "File cleared successfully.".green()),
        Err(e) => println!("Error clearing file: {}", e.to_string().red()),
    }
}

fn confirm_overwrite(pathname: &str) -> bool {
    println!(
        "{}",
        format!(
            "Are you sure you want to clear the file {}? (y/n)",
            pathname
        )
        .yellow()
    );

    let mut input = String::new();

    return stdin().read_line(&mut input).is_err() || input.trim().to_lowercase() != "y";
}
