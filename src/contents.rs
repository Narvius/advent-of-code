use crate::solutions::aoc2015;

type TableOfContents = [Section; 7];
type Section = [Option<(SolutionFn, SolutionFn, &'static str)>; 25];
type SolutionFn = fn(&str) -> Result<String, String>;

pub static CONTENTS: TableOfContents = [
    AOC2015, AOC2016, AOC2017, AOC2018, AOC2019, AOC2020, AOC2021,
];

macro_rules! day {
    ($y:ident::$d:ident) => {
        Some((
            $y::$d::one,
            $y::$d::two,
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
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

static AOC2016: Section = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None,
];

static AOC2017: Section = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None,
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
