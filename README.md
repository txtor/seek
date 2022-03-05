# seek

Search string files (blend of find and grep).

Excercise in Rust.

(c) Copyright 2022 txtor

GNU GENERAL PUBLIC LICENSE, Version 3

## Usage
    seek <query string>
search all string files in the current directory which contain the query string. If content is an empty string, all lines are matched.

## Brainstorming

### Recursive
    seek -r ...

recursively search the current directory and its descendants

### Target
    seek -t target ...
target to search for. Supported targets:
1. content: the file complete contents
1. path: full file name
1. name: file name (without path)
1. dir: file path (without file name)
1. mdate: file modification date
1. cdate: file creation date
1. num: matching line number
1. line: full matching line 

### Match
    seek -m operator ...
match operation to use. Supported operations:
1. begin: the target content begins with the query string
1. end: the target content ends with the query string
1. contains: the target content contains the query string
1. eq: the target content is equal to the query string
1. gt: the target content is greater than the query string
1. gte: the target content is greater than or equal to the query string
1. lt: the target content is less than the query string
1. lte: the target content is less than or equal to the query string

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
