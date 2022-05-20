// Finds the best possible cookie with exactly 100 units of ingredients.
pub fn one(input: &str) -> Result<String, String> {
    // Uses a greedy algorithm that at every step takes 1 unit of the ingredient that would lead to
    // the biggest improvement in score. Because the score would be zero without any ingredients,
    // making it impossible to make these greedy choices, we assume 1 of each ingredient.
    let ingredients: Vec<_> = parse(input).collect();
    let mut choices = vec![1; ingredients.len()];

    let (mut best, mut index) = (0, 0);
    for _ in ingredients.len()..100 {
        for i in 0..ingredients.len() {
            choices[i] += 1;
            let score = score(&ingredients, &choices);
            if score > best {
                (best, index) = (score, i);
            }
            choices[i] -= 1;
        }

        choices[index] += 1;
    }

    Ok(best.to_string())
}

// Finds the best possible cookie with exactly 100 units of ingredients and 500 calories.
pub fn two(input: &str) -> Result<String, String> {
    // Search through all possible recipes with brute force. The 500 calorie restriction is
    // extremely limiting, as such checking every possibility is viable.
    fn find_best_recipe_score(
        index: usize,
        ingredients: &[Vec<i32>],
        choices: &mut [i32],
        best: &mut i32,
        slots: i32,
        calories: i32,
    ) {
        if index == ingredients.len() - 1 {
            // We're on the last ingredient, no need to loop--just use as many of the final
            // ingredient as we have to.
            choices[index] = slots;
            let calories: i32 = (0..ingredients.len())
                .map(|i| ingredients[i][4] * choices[i])
                .sum();
            if calories == 500 {
                *best = score(ingredients, choices).max(*best);
            }
        } else {
            // A nested for loop for each ingredient.
            for choice in 0..slots.min(calories / ingredients[index][4]) {
                choices[index] = choice;
                find_best_recipe_score(
                    index + 1,
                    ingredients,
                    choices,
                    best,
                    slots - choice,
                    calories - choice * ingredients[index][4],
                );
            }
        }
    }

    let ingredients = {
        let mut ingredients: Vec<_> = parse(input).collect();
        ingredients.sort_by_key(|i| i32::MAX - i[4]);
        ingredients
    };

    let (mut best, mut choices) = (0, vec![0; ingredients.len()]);
    find_best_recipe_score(
        0,
        ingredients.as_slice(),
        choices.as_mut_slice(),
        &mut best,
        100,
        500,
    );
    Ok(best.to_string())
}

/// Determines the score for a cookie recipe.
fn score(ingredients: &[Vec<i32>], counts: &[i32]) -> i32 {
    let mut subscores = [0; 4];
    for (ingredient, count) in ingredients.iter().zip(counts.iter()) {
        for factor in 0..4 {
            subscores[factor] += ingredient[factor] * count;
        }
    }
    subscores.into_iter().take(4).map(|x| x.max(0)).product()
}

// Parses the puzzle input into a series of ingredient values.
fn parse(input: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    input.lines().map(|line| {
        line.split(&[' ', ','][..])
            .filter_map(|v| v.parse::<i32>().ok())
            .collect()
    })
}
