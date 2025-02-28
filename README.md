# Tbox CLI
Tbox is an all-in-one **command-line interface** (CLI) tool that helps you with everything you need, from file management to note-taking, all in a single command. 🚀

**Note: This project is still under development.** 👨‍💻

## Commands

### Files
- **`tbox file create <pathname>`**:
Creates a file at the specified path
**Note**: This command works only if the folder exists

- **`tbox file delete <pathname>`**:
Deletes the file at the specified path.
**Note**: This command works only if the folder exists

#### Read
Reads and displays the content of a file, optionally showing line numbers or a specific range of lines.

##### Usage
```sh
tbox file read <pathname> [options]
```

##### Options
- `-n, --number`: Show line numbers
- `-l, --lines START:END`: Display only a specific line range. Supports `START:`, `:END`, or `START:END`. If start and end have the same number, it will display only a single line

##### Examples
- Read a File:
```sh
tbox file read file.txt
```
- Show line numbers:
```sh
tbox file read file.txt -n
```
- Show lines 10 to 20:
```sh
tbox file read file.txt -l 10:20
```
- Show from line 5 onwards and display line numbers:
```sh
tbox file read file.txt -l 5: -n
```