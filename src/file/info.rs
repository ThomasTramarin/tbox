use std::{
    fs::{metadata, Metadata},
    path::Path,
    time::SystemTime,
};

use chrono::{DateTime, Utc};
use clap::{Arg, ArgMatches, Command};
use colored::Colorize;

pub fn file_info_command() -> Command {
    Command::new("info")
        .about("Get information about a file")
        .arg(
            Arg::new("pathname")
                .required(true)
                .index(1)
                .help("The path of the file to get information about"),
        )
}

pub fn file_info(matches: &ArgMatches) {
    // Get the pathname
    let pathname = matches.get_one::<String>("pathname").unwrap();

    // Check if the file exists
    let path = Path::new(pathname);
    if !path.exists() {
        println!("{}", "File does not exist.".red());
        return;
    }

    // Check if the path is a file
    if !path.is_file() {
        println!("{}", "The specified path is not a file.".red());
        return;
    }

    match metadata(path) {
        Ok(metadata) => print_info(metadata),
        Err(e) => eprintln!("{}", format!("Error: {}", e).red()),
    }
}

fn print_info(metadata: Metadata) {
    // Retrieve and display the size of the file
    println!("{}", "Information about the file:".bold());
    println!("Size: {}", format_size(metadata.len()));

    // Retrieve and display file permissions
    let permissions = metadata.permissions();
    let permissions_str = if permissions.readonly() {
        "Readonly"
    } else {
        "Writable"
    };
    println!("Permissions: {}", permissions_str.green());

    // Retrieve and display the last modification time
    match metadata.modified() {
        Ok(time) => {
            let modified_time = format_time(time);
            println!("Last modified: {}", modified_time.yellow());
        }
        Err(_) => println!("{}", "Unable to retrieve modification time.".red()),
    }

    // Retrieve and display the creation time
    match metadata.created() {
        Ok(time) => {
            let created_time = format_time(time);
            println!("Created: {}", created_time.yellow());
        }
        Err(_) => println!("{}", "Unable to retrieve creation time.".red()),
    }
}

// Function to format the system time into a human-readable format
fn format_time(system_time: SystemTime) -> String {
    let datetime: DateTime<Utc> = DateTime::from(system_time);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

// Function to format the size of a file
fn format_size(size: u64) -> String {
    if size >= 1_000_000_000 {
        format!("{:.2} GB", size as f64 / 1_000_000_000.0)
    } else if size >= 1_000_000 {
        format!("{:.2} MB", size as f64 / 1_000_000.0)
    } else if size >= 1_000 {
        format!("{:.2} KB", size as f64 / 1_000.0)
    } else {
        format!("{} bytes", size)
    }
}
