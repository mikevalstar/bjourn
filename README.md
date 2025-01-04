# bjourn

A commnd line Bullet List tool

## Installation

```bash
cargo install --locked bjourn
```

## Usage

```
A simple journaling tool

USAGE:
        bjourn [action] [args]

ACTIONS:
        -a, --add, add [text]
                Add a new entry with the given text
        -h, --help, help
                Print this help message
        -l, --list, list [optional date]
                List all entries for the given date, defaults to today
        -r, --remove, remove [id]
                Remove the entry with the given id
        -v, --version, version
                Print the version of bjourn
```

## Features

### Add

Add a new bullet point to the list

```bash
bjourn add This is a new bullet point
```

### Help

Print the help message

```bash
bjourn help
```

### List

You can view the day's bullet points by specifying the day or view today's bullet points without specifying a day.

```bash
bjourn
```

```bash
bjourn list 2025-01-01
```

When piping to another command, the output is formatted as a simple list of bullet points

```bash
bjourn list 2025-01-01 | pbcopy

bjourn list | cat
* woke up and had breakfast
* added version 0.2.1 of bjourn
```

## Development Notes

Run:

```bash
cargo run
```

Run with arguments:

```bash
cargo run -- add This is a new bullet point
```

Debug Mode

```bash
DEBUG=true cargo run
```

## ENV variables

`DEBUG` - Set to `true` to print debug messages
`BJOURN_USAGE` - Set to `false` to print the usage message when running bjourn with no arguments

## Installation (Local)

```bash
cargo install --path .
```
