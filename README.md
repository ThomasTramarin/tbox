# Tbox CLI
Tbox is an all-in-one **command-line interface** (CLI) tool that helps you with everything you need, from file management to note-taking, all in a single command. 🚀

> Note: This project is still under development. 👨‍💻

## Table of contents

1. [File management](#file-management)  
    1.1. [Create](#create-a-file)  
    1.2. [Read](#read-a-file)  
    1.3  [Delete](#delete-a-file)  

## File management

### Create a file
Create a new file in the specified pathname

#### Usage
```sh
tbox file create <pathname>
```
#### Examples
- Create a file in the current directory
```sh
tbox file create file.txt
```
- Create a file in another directory
```sh
tbox file create ./Desktop/file.txt
```

### Read a file
Reads and displays the content of a file, optionally showing line numbers or a specific range of lines.

#### Usage
```sh
tbox file read <pathname> [options]
```

#### Options
- `-n, --number`: Show line numbers
- `-l, --lines START:END`: Display only a specific line range. Supports `START:`, `:END`, or `START:END`. If start and end have the same number, it will display only a single line

#### Examples
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

### Delete a file
Delete the specified file from the filesystem.

#### Usage
```sh
tbox file delete <pathaname> [options]
```

#### Options
- `-f, --force`: Delete the file without asking for confirmation

#### Examples
- Delete a file in the current directory:
```sh
tbox file delete file.txt
```
- Delete a file in another directory without confiemation:
```sh
tbox file delete ../file.txt -f
```