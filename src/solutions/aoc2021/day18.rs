/// Sum up all the snailfish numbers in the input, find the magnitude of the result.
pub fn one(input: &str) -> crate::Result<i32> {
    Ok(input
        .lines()
        .map(Expr::from_input)
        .reduce(|a, b| Expr::add(&a, &b))
        .ok_or("no input")?
        .magnitude())
}

/// Find the highest magnitude obtainable from adding any two different snailfish numbers from the
/// input.
pub fn two(input: &str) -> crate::Result<i32> {
    let expressions: Vec<_> = input.lines().map(Expr::from_input).collect();

    let mut best = 0;
    for i in 0..expressions.len() {
        for j in 0..expressions.len() {
            if i == j {
                continue;
            }
            best = best.max(Expr::add(&expressions[i], &expressions[j]).magnitude());
        }
    }
    Ok(best)
}

/// A node of an `Expr` binary tree.
#[derive(Copy, Clone)]
enum Node {
    /// An null value for a node.
    Empty,
    /// A tree node that has children.
    Pair,
    /// A leaf node with a numeric value.
    Value(u8),
}

/// An array-based binary tree with `Node` as a node type.
struct Expr {
    nodes: [Node; 63],
}

impl Expr {
    /// Parses a line of puzzle input into a binary tree.
    fn from_input(s: &str) -> Self {
        let mut expr = Self {
            nodes: [Node::Empty; 63],
        };
        let mut i = 0;

        for c in s.as_bytes() {
            match c {
                b'[' => {
                    expr.nodes[i] = Node::Pair;
                    i = i * 2 + 1;
                }
                b',' => i += 1,
                b']' => i = (i - 1) / 2,
                _ => expr.nodes[i] = Node::Value(c - b'0'),
            }
        }
        expr
    }

    /// Adds two expressions.
    fn add(lhs: &Self, rhs: &Self) -> Self {
        let mut expr = Self {
            nodes: [Node::Empty; 63],
        };

        expr.nodes[0] = Node::Pair;

        // Copy the left subtree into the new tree, shifted down a level.
        let mut queue = vec![(0, 1)];
        while let Some((source, left)) = queue.pop() {
            expr.nodes[left] = lhs.nodes[source];
            if let Node::Pair = lhs.nodes[source] {
                queue.push((2 * source + 1, 2 * left + 1));
                queue.push((2 * source + 2, 2 * left + 2));
            }
        }

        // Copy the right subtree into the new tree, shifted down a level.
        queue.push((0, 2));
        while let Some((source, right)) = queue.pop() {
            expr.nodes[right] = rhs.nodes[source];
            if let Node::Pair = rhs.nodes[source] {
                queue.push((2 * source + 1, 2 * right + 1));
                queue.push((2 * source + 2, 2 * right + 2));
            }
        }

        // Fully reduce the expression.
        loop {
            expr.explode();
            if !expr.split_once() {
                break;
            }
        }

        expr
    }

    /// Calculates the magnitude of the entire snailfish number, as described by the puzzle.
    fn magnitude(&self) -> i32 {
        fn recursive_magnitude(e: &Expr, i: usize) -> i32 {
            match e.nodes[i] {
                Node::Pair => {
                    3 * recursive_magnitude(e, 2 * i + 1) + 2 * recursive_magnitude(e, 2 * i + 2)
                }
                Node::Value(n) => n as i32,
                _ => unreachable!(),
            }
        }

        recursive_magnitude(self, 0)
    }

    /// Finds the closest neighbour in the binary tree according to exploding rules in the puzzle.
    fn neighbour(&self, mut i: usize, left: bool) -> Option<usize> {
        // Root node has no neighbours either direction.
        if i == 0 {
            return None;
        }

        let parity = left as usize;

        // There's two options, we are either a left child or a right child. If the direction we're
        // going matches the direction of child we are, that means we have to take the nearest value
        // from an adjacent subtree. We can tell if we're a left child or right child by whether the
        // index is odd (left child) or even (right child).

        // That means we first have to move UP the tree until we are the opposite type of child (if
        // we were left, until we're right, and vice versa).
        while i % 2 == parity {
            i = (i - 1) / 2;
            // If we reach the root node that means that there is no adjacent subtree.
            if i == 0 {
                return None;
            }
        }

        // Now we're the "correct" type of child--that is, if we're going left we're a right child,
        // and if we're going right, we're a left child. All that's left to do is to visit our
        // neighbour by going up once, and then down to the neighbour...
        i = (i - 1) / 2;
        i = i * 2 + 2 - parity;

        // And finally keep drilling down if the neighbour is not a leaf.
        while let Node::Pair = self.nodes[i] {
            i = i * 2 + 1 + parity;
        }
        Some(i)
    }

    /// Explodes all pairs in this expression that are eligible for it. For details, see the puzzle
    /// description.
    fn explode(&mut self) {
        // Check all depth 4 nodes if they're pairs, and if they are, explode them.
        for i in 15..31 {
            if let Node::Pair = self.nodes[i] {
                if let Some(left) = self.neighbour(i, true) {
                    self.nodes[left] = match (self.nodes[left], self.nodes[i * 2 + 1]) {
                        (Node::Value(a), Node::Value(b)) => Node::Value(a + b),
                        _ => unreachable!(),
                    }
                }

                if let Some(right) = self.neighbour(i, false) {
                    self.nodes[right] = match (self.nodes[right], self.nodes[i * 2 + 2]) {
                        (Node::Value(a), Node::Value(b)) => Node::Value(a + b),
                        _ => unreachable!(),
                    }
                }

                self.nodes[i] = Node::Value(0);
                self.nodes[2 * i + 1] = Node::Empty;
                self.nodes[2 * i + 2] = Node::Empty;
            }
        }
    }

    /// Splits the leftmost value in this expression that is eligible for it. For details, see the
    /// puzzle description. Returns whether a split took place. This is required, because a
    /// successful split might create a new pair eligible for explosion, which takes priority over
    /// splitting.
    fn split_once(&mut self) -> bool {
        let mut stack = vec![0];
        while let Some(i) = stack.pop() {
            match self.nodes[i] {
                Node::Pair => {
                    stack.push(2 * i + 2);
                    stack.push(2 * i + 1);
                }
                Node::Value(n) => {
                    if n >= 10 {
                        self.nodes[i] = Node::Pair;
                        self.nodes[2 * i + 1] = Node::Value(n / 2);
                        self.nodes[2 * i + 2] = Node::Value((n + 1) / 2);
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }
}
