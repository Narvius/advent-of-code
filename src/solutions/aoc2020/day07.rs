use std::collections::HashSet;

pub fn one(input: &str) -> crate::Result<usize> {
    let graph = parse(input).ok_or("parse failed")?;
    graph
        .possible_container_count("shiny gold")
        .ok_or_else(|| "no result".into())
}

pub fn two(input: &str) -> crate::Result<usize> {
    let graph = parse(input).ok_or("parse failed")?;
    graph
        .contained_bag_count("shiny gold")
        .ok_or_else(|| "no result".into())
}

/// An adjacency list-based graph describing which bags contain which, and how many.
#[derive(Default)]
struct Graph<'a> {
    nodes: Vec<&'a str>,
    contains: Vec<Vec<(usize, usize)>>,
    contained_in: Vec<Vec<usize>>,
}

impl Graph<'_> {
    /// Returns the number of bags that could, directly or indirectly, contain the given one.
    fn possible_container_count(&self, bag: &str) -> Option<usize> {
        let mut possible_containers = HashSet::new();
        let mut stack = self.contained_in[self.get_node(bag)?].clone();
        while let Some(bag) = stack.pop() {
            possible_containers.insert(bag);
            stack.extend(self.contained_in[bag].iter())
        }
        Some(possible_containers.len())
    }

    /// Returns the total number of bags contained by the given one.
    fn contained_bag_count(&self, bag: &str) -> Option<usize> {
        fn recurse(g: &Graph<'_>, bag: usize) -> usize {
            g.contains[bag]
                .iter()
                .map(|&(bag, count)| count * (1 + recurse(g, bag)))
                .sum()
        }

        Some(recurse(self, self.get_node(bag)?))
    }

    /// Returns the index of a node given its label.
    fn get_node(&self, bag: &str) -> Option<usize> {
        self.nodes.iter().position(|&node| node == bag)
    }
}

/// Parses the puzzle input into an adjacency list-based graph.
fn parse(input: &str) -> Option<Graph<'_>> {
    fn node_index<'a>(g: &mut Graph<'a>, item: &'a str) -> usize {
        match g.nodes.iter().position(|&node| node == item) {
            Some(p) => p,
            None => {
                g.nodes.push(item);
                g.contains.push(vec![]);
                g.contained_in.push(vec![]);
                g.nodes.len() - 1
            }
        }
    }

    let mut g = Graph::default();
    for line in input.lines() {
        if let Some((container, items)) = line.split_once(" bags contain ") {
            let container = node_index(&mut g, container);

            for item in items.trim_end_matches('.').split(", ") {
                if item == "no other bags" {
                    continue;
                }

                let (num, item) = item
                    .trim_end_matches(&['b', 'a', 'g', 's'][..])
                    .trim_end()
                    .split_once(' ')?;
                let item = node_index(&mut g, item);
                g.contains[container].push((item, num.parse().ok()?));
                g.contained_in[item].push(container);
            }
        }
    }
    Some(g)
}
