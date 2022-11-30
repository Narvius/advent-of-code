use std::collections::HashMap;

/// Counts the number of safe ingredients (without any allergens) in them across all recipes.
pub fn one(input: &str) -> crate::Result<usize> {
    let (allergens, recipes) = get_allergens_and_recipes(input).ok_or("parse failed")?;
    Ok(recipes
        .into_iter()
        .flatten()
        .filter(|&i| allergens.values().all(|&a| a != i))
        .count())
}

/// Finds the canonical list of allergens (the actual ingredients sorted by the alphabetical
/// order of the allergen they contain).
pub fn two(input: &str) -> crate::Result<String> {
    let (allergens, _) = get_allergens_and_recipes(input).ok_or("parse failed")?;
    let mut allergens: Vec<_> = allergens.into_iter().collect();
    allergens.sort_by_key(|(k, _)| *k);
    let allergens: Vec<_> = allergens.into_iter().map(|(_, v)| v).collect();
    Ok(allergens.join(","))
}

type Allergens<'a> = HashMap<&'a str, &'a str>;
type Recipes<'a> = Vec<Vec<&'a str>>;

/// Parses the puzzle input, finds which ingredients are which allergen, and returns that
/// mapping plus a list of all ingredients.
fn get_allergens_and_recipes(input: &str) -> Option<(Allergens<'_>, Recipes<'_>)> {
    let mut candidates = HashMap::new();
    let mut recipes = vec![];
    let mut allergens = HashMap::new();

    for line in input.lines() {
        let (ingredients, allergens) = line.trim_end_matches(')').split_once(" (contains ")?;
        let recipe: Vec<_> = ingredients.split_ascii_whitespace().collect();
        for allergen in allergens.split(", ") {
            candidates
                .entry(allergen)
                .or_insert_with(|| recipe.clone())
                .retain(|v| recipe.contains(v));
        }
        recipes.push(recipe);
    }

    while let Some((&allergen, _)) = candidates.iter().find(|(_, v)| v.len() == 1) {
        let candidate = candidates.remove(allergen)?.pop()?;
        allergens.insert(allergen, candidate);
        for v in candidates.values_mut() {
            v.retain(|&c| c != candidate);
        }
    }

    Some((allergens, recipes))
}
