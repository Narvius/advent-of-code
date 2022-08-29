/// Find the sum of all metadata entries across the entire tree.
pub fn one(input: &str) -> crate::Result<i32> {
    let mut trees = vec![parse(input).ok_or("no tree parsed")?];
    let mut result = 0;
    while let Some(tree) = trees.pop() {
        trees.extend(tree.children);
        result += tree.metadata.into_iter().sum::<i32>();
    }
    Ok(result)
}

/// Calculate the value of the root node as described in the puzzle input.
pub fn two(input: &str) -> crate::Result<i32> {
    // Because the same child node may be indexed multiple times, we effectively
    // "multiply" the value calculated from some trees. Thus, in the list of
    // trees to be processed, we carry a multiplier alongside the tree itself.
    let mut trees = vec![(1, parse(input).ok_or("no tree parsed")?)];
    let mut result = 0;
    while let Some((multiplier, tree)) = trees.pop() {
        if tree.children.is_empty() {
            result += tree.metadata.into_iter().sum::<i32>() * multiplier;
        } else {
            for (index, child) in tree.children.into_iter().enumerate() {
                let count = tree
                    .metadata
                    .iter()
                    .filter(|&&i| i == (index + 1) as i32)
                    .count();
                if count > 0 {
                    trees.push((count as i32 * multiplier, child));
                }
            }
        }
    }
    Ok(result)
}

/// A tree structure as describes in the puzzle.
#[derive(Default)]
struct Tree {
    children: Vec<Tree>,
    metadata: Vec<i32>,
}

/// Parses the puzzle input into a tree.
fn parse(input: &str) -> Option<Tree> {
    fn make_node(stream: &mut impl Iterator<Item = i32>) -> Option<Tree> {
        let mut result = Tree::default();
        let children = stream.next()?;
        let metadata = stream.next()?;

        for _ in 0..children {
            result.children.push(make_node(stream)?);
        }
        for _ in 0..metadata {
            result.metadata.push(stream.next()?);
        }

        Some(result)
    }

    let mut tokens = input
        .split_ascii_whitespace()
        .filter_map(|n| n.parse::<i32>().ok());
    make_node(&mut tokens)
}
