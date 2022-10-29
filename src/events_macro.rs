//! Contains a macro used to concisely include all solution files, as well as make them
//! available via numbered arguments.

/// Expands into a static variable named CONTENTS that holds all solutions, as well as the `mod`
/// tree that includes the relevant files. The macro call itself contains that list of modules
/// and files. Should be used within the root module.
///
/// Follows the conventions that:
/// - solutions are in src/solutions/(module)/dayXX.rs
/// - each dayXX.rs file contains two public functions called `one` and `two`
/// - each one of those takes a `&str` argument and returns a [`Result<T>`](crate::Result).
macro_rules! events {
    ($($module:ident::{$($day:ident),*};)*) => {
        mod solutions {$(
            pub mod $module {$(
                pub mod $day;
            )*}
        )*}

        /// The full space of Advent of Code solutions.
        pub static CONTENTS: &'static [&'static [Solution]] = &[$(
            &[$((
                |input| $crate::solutions::$module::$day::one(input).map(|v| format!("{v}")),
                |input| $crate::solutions::$module::$day::two(input).map(|v| format!("{v}")),
                include_str!(concat!(
                    "data/",
                    stringify!($module),
                    "/",
                    stringify!($day),
                    ".txt"
                )),
            )),*]
        ),*];
    }
}
