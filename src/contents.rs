//! Contains all the plumbing necessary to run solutions "dynamically" from user input.

use crate::solutions::{aoc2015, aoc2016, aoc2017};

type TableOfContents = [Section; 7];
type Section = [Option<(SolutionFn, SolutionFn, &'static str)>; 25];
type SolutionFn = fn(&str) -> crate::Result<String>;

/// The full space of Advent of Code solutions.
pub static CONTENTS: TableOfContents = [
    AOC2015, AOC2016, AOC2017, AOC2018, AOC2019, AOC2020, AOC2021,
];

/// Expands to a tuple that ties together the two partial solution functions, as well as the puzzle
/// input.
///
/// Relies on the convention that the crate `aocXXX::dayXX` contains the functions `one` and
/// `two`, representing two parts of the solution respectively; and that there is a textfile with
/// the puzzle input at "src/data/aocXXX/dayXX.txt".
///
/// Textfiles are compiled into the program, and thus are only required at compile time.
macro_rules! day {
    ($y:ident::$d:ident) => {
        Some((
            |input| $y::$d::one(input).map(|v| format!("{v}")),
            |input| $y::$d::two(input).map(|v| format!("{v}")),
            include_str!(concat!(
                "data/",
                stringify!($y),
                "/",
                stringify!($d),
                ".txt"
            )),
        ))
    };
}

static AOC2015: Section = [
    day!(aoc2015::day01),
    day!(aoc2015::day02),
    day!(aoc2015::day03),
    day!(aoc2015::day04),
    day!(aoc2015::day05),
    day!(aoc2015::day06),
    day!(aoc2015::day07),
    day!(aoc2015::day08),
    day!(aoc2015::day09),
    day!(aoc2015::day10),
    day!(aoc2015::day11),
    day!(aoc2015::day12),
    day!(aoc2015::day13),
    day!(aoc2015::day14),
    day!(aoc2015::day15),
    day!(aoc2015::day16),
    day!(aoc2015::day17),
    day!(aoc2015::day18),
    day!(aoc2015::day19),
    day!(aoc2015::day20),
    day!(aoc2015::day21),
    day!(aoc2015::day22),
    day!(aoc2015::day23),
    day!(aoc2015::day24),
    day!(aoc2015::day25),
];

static AOC2016: Section = [
    day!(aoc2016::day01),
    day!(aoc2016::day02),
    day!(aoc2016::day03),
    day!(aoc2016::day04),
    day!(aoc2016::day05),
    day!(aoc2016::day06),
    day!(aoc2016::day07),
    day!(aoc2016::day08),
    day!(aoc2016::day09),
    day!(aoc2016::day10),
    day!(aoc2016::day11),
    day!(aoc2016::day12),
    day!(aoc2016::day13),
    day!(aoc2016::day14),
    day!(aoc2016::day15),
    day!(aoc2016::day16),
    day!(aoc2016::day17),
    day!(aoc2016::day18),
    day!(aoc2016::day19),
    day!(aoc2016::day20),
    day!(aoc2016::day21),
    day!(aoc2016::day22),
    day!(aoc2016::day23),
    day!(aoc2016::day24),
    day!(aoc2016::day25),
];

static AOC2017: Section = [
    day!(aoc2017::day01),
    day!(aoc2017::day02),
    day!(aoc2017::day03),
    day!(aoc2017::day04),
    day!(aoc2017::day05),
    day!(aoc2017::day06),
    day!(aoc2017::day07),
    day!(aoc2017::day08),
    day!(aoc2017::day09),
    day!(aoc2017::day10),
    day!(aoc2017::day11),
    day!(aoc2017::day12),
    day!(aoc2017::day13),
    day!(aoc2017::day14),
    day!(aoc2017::day15),
    day!(aoc2017::day16),
    day!(aoc2017::day17),
    day!(aoc2017::day18),
    day!(aoc2017::day19),
    day!(aoc2017::day20),
    day!(aoc2017::day21),
    None,
    None,
    None,
    None,
];

static AOC2018: Section = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None,
];

static AOC2019: Section = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None,
];

static AOC2020: Section = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None,
];

static AOC2021: Section = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None,
];
