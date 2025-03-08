# Tbox CLI
Tbox is an all-in-one **command-line interface** (CLI) tool that helps you with everything you need, from file management to note-taking, all in a single command. 🚀

> Note: This project is still under development. 👨‍💻

## Table of contents

1. [File management](#file-management)  
    1.1. [Create](#create-a-file)  
    1.2. [Read](#read-a-file)  
    1.3  [Delete](#delete-a-file)  
    1.4. [Write](#write-a-file)
2. [Folder Management](#folder-management)  
    2.1. [Create](#create-a-folder)  
    2.2 [Delete](#delete-a-folder)  

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
- `-t, --tail`: Display only the last 10 lines  (only works if --lines is not used)
- `-H, --head`: Display only the first 10 lines  (only works if --lines is not used)
- `-g, --grep`: Filters the displayed content by matching lines that contain the specified pattern. This can be used to quickly locate occurrences of a word or phrase in a large file.

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
- Show the first 10 lines:
```
tbox file read file.txt -H
```
- Show the last 10 lines:
```
tbox file read file.txt -t
```
- Search for a pattern in the file and display line numbers
```
tbox file read file.txt -g "error" -n
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
- Delete a file in another directory without asking for confirmation:
```sh
tbox file delete ../file.txt -f
```

### Write a file
Write content to a file with the option to overwrite or append content. It also supports skipping confirmation prompts and specifying a line number for insertion.

#### Usage
```sh
tbox file write <pathname> <content> [options]
```

#### Options
- `-a, --append`: Append content to the file instead of overwriting it
- `-f, --force`: Skip confirmation prompts and force the write operation

#### Examples
- Overwrite the content of a file:
```sh
tbox file write file.txt "This is the new content"
```
- Append content to the file:
```sh
tbox file write file.txt "This is appended content" -a
```
- Force write without confirmation
```sh
tbox file write file.txt "This is the new content" -f
```

## Folder management

### Create a folder
Create a new folder in the specified pathname.

#### Usage
```sh
tbox folder create <pathname>
```

#### Examples
- Create a folder in the current directory:
```sh
tbox folder create myFolder
```
- Create a folder in another directory:
```sh
tbox folder create ./Desktop/myFolder
```

### Delete a folder

#### Usage
```sh
tbox folder delete <pathname> [options]
```

#### Options
- `-f, --force`: Delete the folder without asking for confirmation.
- `-a, --all`: Delete a folder and all its contents.


#### Examples
- Delete a folder (only if it's empty)
```sh
tbox folder delete myFolder
```
- Delete a folder and all its contents (with confirmation):
```sh
tbox folder delete myFolder -a
```
- Delete a folder and all its contents without confirmation:
```sh
tbox folder delete myFolder -a -f
```