//! A project containing my (Phil, narvius@gmail.com) solutions to Advent of Code, written in Rust.
//! Many of those solutions do not represent my first time ever solving the problems, but rather,
//! the best Rust solution I am able to write as of... time of writing.

mod common;
#[macro_use]
mod runner;

/// During 0 or 1 argument invocations, this year is assumed. Changes based on what I work on,
/// for convenience's sake.
const ASSUMED_YEAR: usize = 2021;

events! {
    //2015 => aoc2015::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25};
    //2016 => aoc2016::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25};
    //2017 => aoc2017::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25};
    //2018 => aoc2018::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25};
    //2019 => aoc2019::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25};
    //2020 => aoc2020::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25};
    2021 => aoc2021::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24};
    //2022 => aoc2022::{day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25};
}

/// The result type used throughout this app.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// A single full solution consisting of two solution functions and the text input for them.
type Solution = (SolutionFn, SolutionFn, &'static str);

/// A single solution function. Note that despite what the signature implies, the actual
/// implementations can return any type that implements [`Display`](std::fmt::Display), due
/// to it being ran through [`format`] (inside the [`events`] macro).
type SolutionFn = fn(&str) -> Result<String>;

fn main() {
    if let Err(why) = runner::run_from_cmd_args() {
        println!("{}", why);
    }
}
