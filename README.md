# Advent of Code solutions

A collection of my Advent of Code solutions in Rust. Most of them are backfilled as I was
learning the language. My [previous repo](https://github.com/Narvius/aoc-old) has C# code I wrote
live for 2018 through 2020; 2021 and beyond is written in Rust, live.

`main.rs` contains a list of all solutions that are implemented; `runner.rs` contains a bunch
of plumbing that makes it possible to dynamically run any of the solutions from the command
line, alongside timing information.

Note that all the input text files are compiled into the output binary. This means the binary
is a completely stand-alone blackbox with no exterior dependencies (except the platform it was
compiled on, of course). When trying it with your own input, just switch the .txt files out
before compiling.

I attempt to sufficiently document and comment code, in particular, naming known algorithms
I implement and the like; if something is unclear, I'd be happy to try to decipher why I did
something and explain it better.

## Compiling & Running

Just `cargo run` or `cargo build`, and then running the executable.

If no command line arguments are provided, the highest day with a solution from the assumed year
(see `main.rs`) is run. One argument allows you to pick a day from the assumed year. Two arguments
select the year and day, in that order. The year can be provided in full (2021) or just the
last two digits (21).

Either argument can be a period ('.'), which means "all". `cargo run . 1` would run the day 1
solution for every year; `cargo run . .` would run every single solution in the entire project.