/// Run the program until completion, get `b`.
pub fn one(input: &str) -> crate::Result<String> {
    Ok(simulate(input, 0, 0).to_string())
}

/// With `a` set to 1 initially, run the program until completion, get `b`.
pub fn two(input: &str) -> crate::Result<String> {
    Ok(simulate(input, 1, 0).to_string())
}

/// Runs the `program` with initial `a` and `b`, and returns the value of `b` after completion.
fn simulate(program: &str, mut a: usize, mut b: usize) -> usize {
    let program: Vec<_> = program.lines().collect();
    let mut ptr = 0usize;
    while let Some(line) = program.get(ptr) {
        let instruction = &line[0..3];
        let register = if &line[4..5] == "a" { &mut a } else { &mut b };
        match instruction {
            "hlf" => {
                *register /= 2;
                ptr += 1
            }
            "tpl" => {
                *register *= 3;
                ptr += 1
            }
            "inc" => {
                *register += 1;
                ptr += 1
            }
            jump if jump == "jmp"
                || (jump == "jio" && *register == 1)
                || (jump == "jie" && *register % 2 == 0) =>
            {
                let offset = line.split(' ').last().unwrap().parse::<i32>().unwrap();
                ptr = (ptr as i32 + offset) as usize;
            }
            _ => ptr += 1,
        }
    }
    b
}
