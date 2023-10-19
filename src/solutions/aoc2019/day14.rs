use std::collections::HashMap;

/// Find the total amount of ORE required to produce 1 FUEL.
pub fn one(input: &str) -> crate::Result<usize> {
    Ok(cost(&parse(input)?, "ORE", "FUEL", 1))
}

/// Find the total amount of FUELS you can make from 1 trillion ORE.
///
/// Due to resources being produced in batches of incongruent sizes, we cannot simply divide
/// 1 trillion by the result of [`one`] to get the answer, though it's a decent first guess in
/// a search approaching the correct result.
pub fn two(input: &str) -> crate::Result<usize> {
    const ORE: usize = 1000000000000;

    let nodes = parse(input)?;

    // Use the cost of 1 fuel as a starting point to approximate the goal amount.
    let mut total_cost = cost(&nodes, "ORE", "FUEL", 1);
    let mut fuels = ORE / total_cost;
    total_cost = cost(&nodes, "ORE", "FUEL", fuels);

    while total_cost <= ORE {
        // Calculate the rough amount of additional fuels we should be able to make, given the
        // current average cost, then check again.
        let expected_possible = (ORE - total_cost) as f32 / (total_cost as f32 / fuels as f32);
        fuels += 1.max(expected_possible.floor() as usize);
        total_cost = cost(&nodes, "ORE", "FUEL", fuels);
    }

    // Once we overshoot, we slowly count back down until we're just under the allowance.
    while total_cost > ORE {
        fuels -= 1;
        total_cost = cost(&nodes, "ORE", "FUEL", fuels);
    }

    Ok(fuels)
}

/// A resource.
struct Node<'a> {
    /// How many of this resource are produced per batch.
    per_batch: usize,
    /// Products that use this as an ingredient; and how much of `self` they use per batch.
    used_in: Vec<(&'a str, usize)>,
}

/// Returns the total amount of `resource`s it takes to create `amount` `product`s, rounded up
/// to the nearest full batch.
fn cost(nodes: &HashMap<&str, Node>, resource: &str, product: &str, amount: usize) -> usize {
    if resource == product {
        amount
    } else {
        nodes[resource]
            .used_in
            .iter()
            .map(|&(resource, per_batch)| {
                let required_parents = cost(nodes, resource, product, amount);
                let batches =
                    (required_parents as f64 / nodes[resource].per_batch as f64).ceil() as usize;
                batches * per_batch
            })
            .sum()
    }
}

/// Parses the puzzle input into a map of resource names to their data.
fn parse(input: &str) -> crate::Result<HashMap<&str, Node>> {
    fn entry(entry: &str) -> crate::Result<(usize, &str)> {
        let (amount, material) = entry.split_once(' ').ok_or("invalid entry")?;
        Ok((amount.parse()?, material))
    }

    let mut links = vec![];
    let mut result = std::iter::once("1 NOTHING => 1 ORE")
        .chain(input.lines())
        .map(|line| {
            let (mats, product) = line.split_once(" => ").ok_or("invalid line")?;
            let (count, product) = entry(product)?;
            for mat in mats.split(", ").map(entry) {
                let (count, mat) = mat?;
                links.push((mat, count, product));
            }

            Ok((
                product,
                Node {
                    per_batch: count,
                    used_in: vec![],
                },
            ))
        })
        .collect::<crate::Result<HashMap<_, _>>>()?;

    for (from, cost, to) in links {
        result
            .entry(from)
            .and_modify(|node| node.used_in.push((to, cost)));
    }

    Ok(result)
}
