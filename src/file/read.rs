use std::{
    fs::{read_to_string, File},
    path::Path,
};

use clap::{Arg, ArgAction, ArgMatches, Command};
use colored::Colorize;

pub fn read_file_command() -> Command {
    Command::new("read")
        .about("Read a file with the path provided")
        .arg(
            Arg::new("pathname")
                .required(true)
                .index(1)
                .help("The path of the file to read"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Show line numbers")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .value_name("START:END")
                .help("Show only a range of lines (e.g., 10:20)"),
        )
}

pub fn read_file(matches: &ArgMatches) {
    // Get the pathname
    let pathname = matches.get_one::<String>("pathname").unwrap();
    let path = Path::new(pathname);

    // Check if the file exists
    if !path.exists() {
        println!("{}", "Error: File does not exist.".red());
        return;
    }

    // Read the file
    match read_to_string(pathname) {
        Ok(content) => {
            let lines: Vec<&str> = content.lines().collect();
            let total_lines = lines.len();

            // Check if the file is empty
            if total_lines == 0 {
                println!("{}", "The file is empty.".yellow());
                return;
            }

            // Default values for start and end
            let mut start = 1;
            let mut end = total_lines;

            // If the user wants to show a specific range of lines
            if let Some(range) = matches.get_one::<String>("lines") {
                // Check if the range is valid
                if !range.contains(":") {
                    println!(
                        "{}",
                        "Invalid range format. Use START:END, START: or :END".red()
                    );
                    return;
                }

                // Split the range
                let parts: Vec<&str> = range.split(":").collect();

                // Check if the range is valid
                if parts.len() != 2 {
                    println!(
                        "{}",
                        "Invalid range format. Use START:END, START: or :END".red()
                    );
                    return;
                }

                // Check if the start and end are valid
                if !parts[0].is_empty() {
                    if let Ok(parsed_start) = parts[0].parse::<usize>() {
                        if parsed_start > 0 && parsed_start <= total_lines {
                            start = parsed_start;
                        } else {
                            println!("{}", "Invalid start line.".red());
                            return;
                        }
                    } else {
                        println!("{}", "Invalid start line format.".red());
                        return;
                    }
                }

                if !parts[1].is_empty() {
                    if let Ok(parsed_end) = parts[1].parse::<usize>() {
                        if parsed_end > 0 && parsed_end <= total_lines {
                            end = parsed_end;
                        } else {
                            println!("{}", "Invalid end line.".red());
                            return;
                        }
                    } else {
                        println!("{}", "Invalid end line format.".red());
                        return;
                    }
                }
            }

            // Check if the start is greater than the end
            if start > end {
                println!("{}", "Invalid range: start is greater than end.".red());
                return;
            }

            // Flag number
            let show_number = matches.get_flag("number");

            // Print the content
            println!("{}", "File content".green());
            for (i, line) in lines
                .iter()
                .enumerate()
                .skip(start - 1)
                .take(end - start + 1)
            {
                if show_number {
                    let width = total_lines.to_string().len();
                    println!(
                        "{}:    {}",
                        format!("{:width$}", i + 1, width = width).blue(),
                        line
                    );
                } else {
                    println!("{}", line);
                }
            }
        }
        Err(e) => println!("{}: {}", "Error reading file:".red(), e.to_string().red()),
    }
}
