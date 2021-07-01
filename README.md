# Memo
A simple utility to memoize commands.

< 100 lines of code

### How it works
The first thing this program does is creating a sha256 hash from the key the user provided.
Then it will check if a temp file exists with that hash as a name, if it does, the contents of the file will be copied to stdout.
If the temp file could not be found, the provided command will be executed, and the output will be written to a temp file and printed to stdout.

## Usage
`memo <key> <command> <args...>`
