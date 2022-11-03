/// Find the ten next entries on the scoreboard after the amount in the input.
pub fn one(input: &str) -> crate::Result<String> {
    let target: usize = input.parse()?;
    let (mut elf1, mut elf2) = (0, 1);
    let mut recipes: Vec<u8> = Vec::with_capacity(target + 20);
    recipes.extend([3, 7]);

    while recipes.len() < (target + 10) {
        let sum = recipes[elf1] + recipes[elf2];
        if sum >= 10 {
            recipes.push(1);
        }
        recipes.push(sum % 10);
        elf1 = (elf1 + recipes[elf1] as usize + 1) % recipes.len();
        elf2 = (elf2 + recipes[elf2] as usize + 1) % recipes.len();
    }

    Ok(String::from_utf8(
        recipes[target..target + 10]
            .iter()
            .map(|&b| b + b'0')
            .collect(),
    )?)
}

/// Find the number of entries before the input shows up on the scoreboard.
pub fn two(input: &str) -> crate::Result<usize> {
    let target: Vec<_> = input.as_bytes().iter().map(|&b| b - b'0').collect();

    let (mut elf1, mut elf2) = (0, 1);
    let mut recipes: Vec<u8> = Vec::with_capacity(1_000_000);
    recipes.extend([3, 7]);

    while !recipes.ends_with(target.as_slice()) {
        let sum = recipes[elf1] + recipes[elf2];
        if sum >= 10 {
            recipes.push(1);
            if recipes.ends_with(target.as_slice()) {
                break;
            }
        }
        recipes.push(sum % 10);
        elf1 = (elf1 + recipes[elf1] as usize + 1) % recipes.len();
        elf2 = (elf2 + recipes[elf2] as usize + 1) % recipes.len();
    }

    Ok(recipes.len() - target.len())
}
