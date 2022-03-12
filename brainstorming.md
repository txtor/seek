# Brainstorming

## Target
    seek -t:<target> ...
target to search for. Default `line`. Supported targets:
1. content: the file complete contents
1. line: single line in content
1. word: single word in content
1. path: full file name
1. name: file name (without path)
1. dir: file path (without file name)
1. mdate: file modification date
1. cdate: file creation date
1. num: line number

## Match
    seek -m:<operator> ...
match operation to use. Default `contains`. Supported operations:
1. begin: the target content begins with the query string
1. end: the target content ends with the query string
1. contains: the target content contains the query string
1. eq: the target content is equal to the query string
1. gt: the target content is greater than the query string
1. gte: the target content is greater than or equal to the query string
1. lt: the target content is less than the query string
1. lte: the target content is less than or equal to the query string

## Comparer
    seek c:<comparer> ...
comparer function to use when matching. Default `lexic`. Supported:
1. lexic: compare strings in lexical order case-insensitive 
1. case: compare strings in lexical order case-sensitive
1. numeric: compare strings as numbers
1. regex: interpret query string as a regular expression

## Output
    seek -o:<type> <format> ...
defines the format of the output. Default type `psv`. Supported types:
1. psv: punctuation separated values
1. json: json array

PSV: `<format>` is the list of output fields separated by a punctuation character. Determines which fields to output and how to separate them. Default: `path:num:line`. Supported fields:
1. path: full file name
1. name: file name (without path)
1. dir: file path (without file name)
1. mdate: file modification date
1. cdate: file creation date
1. num: matching line number
1. line: full matching line 
1. count: number of matches in the current file
1. target: the name of the matching target

## Configuration
    /etc/seek
    ~/.seek
    ./.seek
this files are read at startup in this order and contain the default arguments for every `seek` call. 

Syntax: each line contains an argument as expressed in the command line.
Lines beginning with `#` are ignored.

## Alias
Alias for command line arguments can be defined at compile time.
Examples: 

    search in the given directory
    -d <dir>
    -t:dir <dir>

    search in the given directory and recursively its subdirectories
    -r <dir>
    -t:dir -m:begin <dir>

    search files with a given extension
    -e <ext>
    -t:name -m:end <ext>

## To think about
- Search for files by permission (not readable by u, executable, ...)
- Search for broken symbolic links

## To do
- Better error message on error trying to open a directory: mention the directory name.
- All error message should begin with the program name (seek)
