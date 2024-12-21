// Large swathes of solutions may be commented out any given time, which
// produces spurious unused warnings from this module.
#![allow(unused)]

use std::collections::{HashSet, VecDeque};

pub mod astar;
mod dir;
mod grid;
pub mod intcode;

pub use dir::{Dir, ALL_DIRS, CARDINAL};
pub use grid::Grid;

/// Returns an iterator over all points within range `size` of originm using taxicab distance.
pub fn diamond_deltas(size: usize) -> impl Iterator<Item = (i32, i32)> {
    let n = size as i32;
    (-n..=n).flat_map(move |y| {
        let m = n - y.abs();
        (-m..=m).map(move |x| (x, y))
    })
}

/// Checks whether a `sequence` has a cycle, using the idea of [Floyd's algorithm], but adapted for
/// usage with iterators rather than a known collection of items. If the given iterator is
/// infinite, only halts once a cycle is detected.
///
/// Uses additional memory (linear relative to size of cycle) to avoid cloning the input.
///
/// [Floyd's algorithm]: https://en.wikipedia.org/wiki/Cycle_detection#Floyd's_tortoise_and_hare
pub fn has_cycle<T>(mut sequence: impl Iterator<Item = T>) -> bool
where
    T: Eq,
{
    let mut items = VecDeque::new();

    while let (Some(a), Some(b)) = (sequence.next(), sequence.next()) {
        items.push_back(a);
        if items.pop_front().as_ref() == Some(&b) {
            return true;
        }
        items.push_back(b);
    }

    false
}

/// Produces the carthesian product of two iterators.
pub fn product<I1, I2, T1, T2>(i1: I1, i2: I2) -> impl Iterator<Item = (T1, T2)>
where
    I1: IntoIterator<Item = T1>,
    I2: IntoIterator<Item = T2> + Clone,
    T1: Clone,
{
    i1.into_iter()
        .flat_map(move |t1| i2.clone().into_iter().map(move |t2| (t1.clone(), t2)))
}

/// Returns all possible permutations of the numbers in `0..k`, using Heap's algorithm.
pub fn permutations(k: usize) -> Vec<Vec<usize>> {
    fn inner(k: usize, values: &mut [usize]) -> Vec<Vec<usize>> {
        let mut result = Vec::new();
        if k <= 1 {
            result.push(Vec::from(values));
        } else {
            result.extend(inner(k - 1, values));
            for i in 0..(k - 1) {
                if k % 2 == 0 {
                    values.swap(i, k - 1);
                } else {
                    values.swap(0, k - 1);
                }
                result.extend(inner(k - 1, values));
            }
        }
        result
    }

    inner(k, &mut (0..k).collect::<Vec<_>>())
}

/// Returns the `permutations` of a Vec.
pub fn permutations_of<T: Clone>(mut items: Vec<T>) -> Vec<Vec<T>> {
    fn inner<T: Clone>(k: usize, values: &mut [T]) -> Vec<Vec<T>> {
        let mut result = Vec::new();
        if k <= 1 {
            result.push(Vec::from(values));
        } else {
            result.extend(inner(k - 1, values));
            for i in 0..(k - 1) {
                if k % 2 == 0 {
                    values.swap(i, k - 1);
                } else {
                    values.swap(0, k - 1);
                }
                result.extend(inner(k - 1, values));
            }
        }
        result
    }

    inner(items.len(), &mut items)
}

/// Returns the greatest common denominator of `a` and `b`.
pub fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = match a.cmp(&b) {
        std::cmp::Ordering::Less => (b, a),
        std::cmp::Ordering::Equal => return a,
        std::cmp::Ordering::Greater => (a, b),
    };

    while b != 0 {
        std::mem::swap(&mut a, &mut b);
        b %= a;
    }

    a
}

/// Returns the least common multiple of `a` and `b`.
pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

/// Constructs a pixel display banner, of the kind used to display the answer in a lot of
/// image manipulation-type tasks.
pub fn pixel_display(
    width: usize,
    height: usize,
    mut f: impl FnMut(usize, usize) -> bool,
) -> String {
    let mut display = String::with_capacity((width + 1) * height);
    for y in 0..height {
        display.push('\n');
        for x in 0..width {
            display.push(if f(x, y) { '#' } else { '.' });
        }
    }
    display
}

/// Constructs a [pixel display](pixel_display) banner from a set of points.
pub fn pixel_display_from_set(points: HashSet<(i32, i32)>) -> String {
    let (lx, ly, hx, hy) = {
        let (mut lx, mut ly, mut hx, mut hy) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
        for &(x, y) in &points {
            lx = lx.min(x);
            ly = ly.min(y);
            hx = hx.max(x);
            hy = hy.max(y);
        }
        (lx, ly, hx, hy)
    };

    let width = (1 + hx - lx) as usize;
    let height = (1 + hy - ly) as usize;

    pixel_display(width, height, |x, y| {
        points.contains(&(lx + x as i32, ly + y as i32))
    })
}

/// Performs a breadth-first search of a value space, returning the
/// shortest amounts of step taken to reach an `end`.
///
/// `next` returns all reachable nodes from the given one, `end` checks
/// if a given node counts as the end.
pub fn bfs<N, I, Next, End>(start: N, mut next: Next, mut end: End) -> Option<usize>
where
    N: Clone + Eq + std::hash::Hash,
    I: Iterator<Item = N>,
    Next: FnMut(&N) -> I,
    End: FnMut(&N) -> bool,
{
    let mut visited = HashSet::from([start.clone()]);
    let mut queue = VecDeque::from([(start, 0)]);

    while let Some((node, steps)) = queue.pop_front() {
        for next in next(&node) {
            if visited.contains(&next) {
                continue;
            }
            if end(&next) {
                return Some(steps + 1);
            }
            visited.insert(next.clone());
            queue.push_back((next, steps + 1));
        }
    }

    None
}
