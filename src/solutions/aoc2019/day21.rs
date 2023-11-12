use crate::common::intcode::v2::*;

/// Program the springdroid to reach the end.
pub fn one(input: &str) -> crate::Result<Int> {
    #[rustfmt::skip]
    let script = &[
        // Only jump if we would go over a hole.
        "OR A T", "AND B T", "AND C T", "NOT T J",
        // Don't jump if there's a hole where we land.
        "AND D J",
        // Execute script.
        "WALK",
    ];
    execute_script(input, script)
}

/// Program the springdroid to reach the end in extended sensor mode.
pub fn two(input: &str) -> crate::Result<Int> {
    #[rustfmt::skip]
    let script = &[
        // Only jump if we would go over a hole.
        "OR A T", "AND B T", "AND C T", "NOT T J",
        // Don't jump if there's a hole where we land.
        "AND D J",
        // Don't jump if we would checkmate ourselves (immediately have to
        // jump again, but would land in a hole).
        "OR E T", "OR H T", "AND T J",
        // Execute in extended sensor mode.
        "RUN",
    ];
    execute_script(input, script)
}

/// Executes the provided springscript `script` using the provided `input`
/// Intcode program.
fn execute_script(input: &str, script: &[&str]) -> crate::Result<Int> {
    let mut p = Program::with_capacity(input, 2200)?;

    for line in script {
        for byte in line.bytes() {
            p.input.push_back(byte as Int);
        }
        p.input.push_back(10);
    }

    p.run()?;
    if p.output.back() > Some(&256) {
        return p.output.pop_back().ok_or("no result".into());
    }

    // Diagnostic output if the script fails.
    for byte in p.output {
        print!("{}", byte as u8 as char);
    }
    Err("script failed".into())
}
