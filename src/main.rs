#![warn(missing_docs)]

//! A project containing my (Phil, narvius@gmail.com) solutions to Advent of Code, written in Rust.
//! Many of those solutions do not represent my first time ever solving the problems, but rather,
//! the best Rust solution I am able to write as of... time of writing.

use std::{
    borrow::Cow,
    time::{Duration, Instant},
};

use crate::contents::CONTENTS;

mod contents;
mod solutions;

/// During 0 or 1 argument invocations, this year is assumed. Changes based on what I work on,
/// for convenience's sake.
const ASSUMED_YEAR: usize = 2015;

/// The highest year for which there can be solutions. Changes every December.
const LAST_YEAR: usize = 2021;

fn main() {
    fn run() -> Result<()> {
        let args: Vec<_> = std::env::args().skip(1).collect();

        use Input::*;
        match args.len() {
            0 => {
                // The highest day with a solution.
                for day in (1..=25).rev() {
                    if contents::CONTENTS[ASSUMED_YEAR - 2015][day - 1].is_some() {
                        return eval([ASSUMED_YEAR], [day]);
                    }
                }
                Err(Error::NoSolutions)
            }
            1 => {
                // A number to run a specific day, or . to run all days.
                match Input::from_day(&args[0])? {
                    All => eval([ASSUMED_YEAR], 1..=25),
                    Specific(day) => eval([ASSUMED_YEAR], [day]),
                }
            }
            2 => {
                // [year, day]. Year can be given either as 2015 or 15 (if it's less than 100, 2000
                // is implicitly added to it). Both year and day can still be '.' for "all".
                let year = Input::from_year(&args[0])?;
                let day = Input::from_day(&args[1])?;

                match (year, day) {
                    (All, All) => eval(2015..=LAST_YEAR, 1..=25),
                    (Specific(year), All) => eval([year], 1..=25),
                    (All, Specific(day)) => eval(2015..=LAST_YEAR, [day]),
                    (Specific(year), Specific(day)) => eval([year], [day]),
                }
            }
            _ => Err(Error::WrongArgCount(args.len()))?,
        }
    }

    if let Err(why) = run() {
        println!("{}", why);
    }
}

/// Possible command line inputs.
enum Input {
    /// Corresponds to the user supplying a number. Not guara
    Specific(usize),
    All,
}

impl Input {
    /// Parses a command line argument for day into an [`Input`] value.
    fn from_day(arg: &str) -> Result<Input> {
        if arg == "." {
            Ok(Input::All)
        } else if let Ok(day) = arg.parse() {
            if 1 <= day && day <= 25 {
                Ok(Input::Specific(day))
            } else {
                Err(Error::OutOfRange(day, 1, 25))
            }
        } else {
            Err(Error::InvalidArg(arg.to_string()))
        }
    }

    /// Parses a command line argument for year into an [`Input`] value. Also accepts values from
    /// 15 and up, in addition to the actual expected range of 2015 and up.
    fn from_year(arg: &str) -> Result<Input> {
        if arg == "." {
            Ok(Input::All)
        } else if let Ok(mut year) = arg.parse() {
            if year < 100 {
                year += 2000;
            }

            if 2015 <= year && year <= LAST_YEAR {
                Ok(Input::Specific(year))
            } else {
                Err(Error::OutOfRange(year, 2015, LAST_YEAR))
            }
        } else {
            Err(Error::InvalidArg(arg.to_string()))
        }
    }
}

/// Error type for this program.
#[derive(Clone, Debug, Eq, PartialEq)]
enum Error {
    WrongArgCount(usize),
    OutOfRange(usize, usize, usize),
    InvalidArg(String),
    NoSolutions,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::WrongArgCount(count) => write!(
                f,
                "expected between 0 and 2 arguments (inclusive); got {}",
                count
            ),
            Error::InvalidArg(content) => write!(
                f,
                "argument expected to be \".\" or a number; got {}",
                content
            ),
            Error::OutOfRange(actual, min, max) => write!(
                f,
                "argument expected to be in range ({}..={}); got {}",
                min, max, actual
            ),
            Error::NoSolutions => write!(f, "no solutions found"),
        }
    }
}

impl std::error::Error for Error {}

type Result<T> = std::result::Result<T, Error>;

/// Runs a range of puzzle solutions. Returns `Ok` if at least one solution was ran.
fn eval<Y, D>(years: Y, days: D) -> Result<()>
where
    Y: IntoIterator<Item = usize>,
    D: Clone + IntoIterator<Item = usize>,
{
    let mut runtime = Duration::new(0, 0);
    for year in years {
        for day in days.clone() {
            if !valid_input(year, day) {
                eprintln!("out of range value in eval: ({}, {})", year, day);
                continue;
            }

            runtime += eval_single(year, day).unwrap_or_else(|| Duration::new(0, 0));
        }
    }

    if runtime.as_secs_f64() > 0.0 {
        println!("\nTotal runtime: {}", format_duration(runtime));
        Ok(())
    } else {
        Err(Error::NoSolutions)
    }
}

/// Runs the solution for a given day. Returns the runtime, if anything was executed.
fn eval_single(year: usize, day: usize) -> Option<Duration> {
    if !valid_input(year, day) {
        eprintln!("out of range input to eval_single ({}, {})", year, day);
        return None;
    }

    let mut runtime = Duration::new(0, 0);
    if let Some((a, b, s)) = CONTENTS[year - 2015][day - 1] {
        for (part, f) in [('a', a), ('b', b)] {
            let start = Instant::now();
            let result = f(s);
            let end = Instant::now();

            match result {
                Ok(val) => {
                    runtime += end - start;
                    let d = format_duration(end - start);
                    println!("Day {:04}-{:02}{}  [{}]  = {}", year, day, part, d, val);
                }
                Err(why) => println!("Day {:04}-{:02}{}  [ FAILED ]  = {}", year, day, part, why),
            }
        }
    }

    (runtime.as_secs_f64() > 0.0).then(|| runtime)
}

/// Formats a [`Duration`](std::time::Duration) for output.
fn format_duration(d: Duration) -> Cow<'static, str> {
    let d = d.as_secs_f64();
    if d < 0.001 {
        Cow::Borrowed("< 0.001s")
    } else {
        Cow::Owned(format!("{:>7.3}s", d))
    }
}

/// Checks whether all input numbers are within their respective valid ranges.
fn valid_input(year: usize, day: usize) -> bool {
    (2015..=LAST_YEAR).contains(&year) && (1..=25).contains(&day)
}
