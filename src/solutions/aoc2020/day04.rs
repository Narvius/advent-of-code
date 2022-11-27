/// Counts the number of passports that have all the required fields.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(parse_passports(input)
        .into_iter()
        .filter(has_required_fields)
        .count())
}

/// Counts the number of passports that have valid fields (that is, matching some arbitrary
/// specs provided by the puzzle).
pub fn two(input: &str) -> crate::Result<usize> {
    Ok(parse_passports(input)
        .into_iter()
        .filter(has_valid_fields)
        .count())
}

/// Parses a list of passports from the input.
fn parse_passports(input: &str) -> Vec<Passport> {
    let lines: Vec<_> = input.lines().collect();
    lines
        .split(|line| line.is_empty())
        .filter_map(|chunk| {
            let mut p = Passport::default();
            for token in chunk.iter().flat_map(|line| line.split_whitespace()) {
                let (key, val) = token.split_once(':')?;
                match key {
                    "byr" => p.birth_year = val.parse().ok()?,
                    "iyr" => p.issue_year = val.parse().ok()?,
                    "eyr" => p.expiration_year = val.parse().ok()?,
                    "hgt" => {
                        let num_pos = val
                            .chars()
                            .position(|c| c.is_alphabetic())
                            .unwrap_or(val.len());
                        let (num, unit) = val.split_at(num_pos);
                        p.height = (num.parse().ok()?, unit);
                    }
                    "hcl" => p.hair_colour = val,
                    "ecl" => p.eye_colour = val,
                    "pid" => p.passport_id = val,
                    "cid" => p.country_id = val,
                    _ => None?,
                }
            }
            Some(p)
        })
        .collect()
}

/// A single passport from puzzle input.
#[derive(Default)]
struct Passport<'a> {
    birth_year: i32,
    issue_year: i32,
    expiration_year: i32,
    eye_colour: &'a str,
    hair_colour: &'a str,
    passport_id: &'a str,
    country_id: &'a str,
    height: (i32, &'a str),
}

/// Checks if a passport has all required fields.
fn has_required_fields(p: &Passport) -> bool {
    p.birth_year > 0
        && p.issue_year > 0
        && p.expiration_year > 0
        && p.height.0 > 0
        && !p.eye_colour.is_empty()
        && !p.hair_colour.is_empty()
        && !p.passport_id.is_empty()
}

/// Checks if a passport has valid fields.
fn has_valid_fields(p: &Passport) -> bool {
    p.passport_id.len() == 9
        && p.hair_colour.starts_with('#')
        && p.hair_colour[1..].chars().all(|c| c.is_ascii_hexdigit())
        && matches!(
            p,
            Passport {
                birth_year: (1920..=2002),
                issue_year: (2010..=2020),
                expiration_year: (2020..=2030),
                height: ((150..=193), "cm") | ((59..=76), "in"),
                eye_colour: "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth",
                ..
            }
        )
}
