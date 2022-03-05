# seek

Search string files (blend of find and grep).

Excercise in Rust.

(c) Copyright 2022 txtor

GNU GENERAL PUBLIC LICENSE, Version 3

## Usage
    seek content
search all string files in the current directory which contain the string 'content'. If content is an empty string, all lines are matched.

## Brainstorming

### Recursive
    seek -r ...

recursively search the current directory and its descendants

### Filter
    seek -f name ...

filter for files/directories whith "name" contained in their full name. `^name` considers only names beggining with "name" and `name$` only 
names ending with "name".

### Target
    seek -t target ...
list of targets to search for. Default: `content`. Supported targets:
1. content: the file content
1. path: full file name

### Output
    seek -o fields ...
list of output fields separated by a non-word character. Determines which fields and which separator to output. Default: `path:num:line`. Supported fields:
1. path: full file name
1. name: file name (without path)
1. dir: file path (without file name)
1. mdate: file modification date
1. cdate: file creation date
1. num: matching line number
1. line: full matching line 
1. count: number of matches in the current file
1. target: the name of the matching target
