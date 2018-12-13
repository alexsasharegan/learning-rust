# Create A CLI Command In Rust

For our first collaborative project, we're going to start with building a CLI to
copy files.

## Goals

The idea behind collaborative projects is to get people working on the same
thing so we can all share our problems/solutions, creative ideas, and help each
other along the way.

This project aims to get you familiar with the standard library and system IO in
the form of stdin/out/err and the file system.

## Requirements

The command can be named whatever you choose, but it should follow the format:

```sh
rust-cp [-nvh] src dest
```

It should also support input from stdin as the `src` when src is absent from the
arguments.

The flags should support these flags:

- `-n` Do not overwrite an existing file
- `-v` Verbose output (log whatever you like to stderr)
- `-h` Print the command usage

\_Note: While you would use a library like [clap](https://crates.io/crates/clap)
for building cli applications, we want to stick to the basics of the standard
library for now.\_
