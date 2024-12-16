//! A general-purpose 2D grid with constant width and height.

use std::borrow::{BorrowMut, Cow};

/// A general-purpose 2D grid with constant width and height.
pub struct Grid<'a, T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    width: usize,
    height: usize,
    data: Vec<Cow<'a, [T]>>,
}

impl<'a, T> std::ops::Index<(i32, i32)> for Grid<'a, T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    type Output = T;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        self.get(index).expect("index in bounds")
    }
}

impl<'a, T> std::ops::IndexMut<(i32, i32)> for Grid<'a, T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn index_mut(&mut self, index: (i32, i32)) -> &mut Self::Output {
        self.get_mut(index).expect("index in bounds")
    }
}

impl<'a> Grid<'a, u8> {
    /// Advent of Code frequently gives you maps drawn from characters. With this call, it can be
    /// converted into a [`Grid`] with minimal calls and allocations.
    pub fn from_input(input: &'a str) -> Self {
        Self::new(
            input
                .trim()
                .lines()
                .map(|line| Cow::from(line.as_bytes()))
                .collect(),
        )
    }
}

impl<'a, T> Grid<'a, T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    /// Constructs a new grid from data. Panics if rows have divergent widths.
    fn new(data: Vec<Cow<'a, [T]>>) -> Self {
        let width = data.first().expect("non-empty input").as_ref().len();
        let height = data.len();

        assert!(data.iter().all(|line| line.len() == width), "uneven grid");

        Self {
            width,
            height,
            data,
        }
    }

    /// Constructs a new grid from an iterator of rows, where each row is also an iterator. Panics
    /// if rows have divergent widths.
    pub fn from_iters(iters: impl Iterator<Item = impl Iterator<Item = T>>) -> Self {
        Self::new(iters.map(|iter| Cow::Owned(iter.collect())).collect())
    }

    /// Gets the width of the grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Gets the height of the grid.
    pub fn height(&self) -> usize {
        self.height
    }

    /// An iterator over all coordinates in the grid.
    pub fn coordinates(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| (x as i32, y as i32)))
    }

    /// An iterator over all coordinate/element pairs in the grid.
    pub fn iter_with_position(&'a self) -> impl Iterator<Item = ((i32, i32), &'a T)> + 'a {
        self.coordinates()
            .map(move |p| (p, self.get(p).expect("coordinate in bounds")))
    }

    /// An iterator over all elements in the grid.
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> + 'a {
        self.data.iter().flat_map(|line| line.as_ref())
    }

    /// Get a reference to the cell at position `p`.
    pub fn get(&self, p: (i32, i32)) -> Option<&T> {
        let (x, y) = (usize::try_from(p.0).ok()?, usize::try_from(p.1).ok()?);
        self.data.get(y).and_then(|line| line.get(x))
    }

    /// Get a mutable reference to the cell at position `p`.
    pub fn get_mut(&mut self, p: (i32, i32)) -> Option<&mut T> {
        let (x, y) = (usize::try_from(p.0).ok()?, usize::try_from(p.1).ok()?);
        self.data
            .get_mut(y)
            .and_then(|line| line.to_mut().get_mut(x))
    }
}

impl<'a, T> Grid<'a, T>
where
    [T]: ToOwned<Owned = Vec<T>>,
    T: Clone,
{
    /// Creates a grid populated with clones of the provided `elem`.
    pub fn from_elem(width: usize, height: usize, elem: T) -> Self {
        Self::new(vec![Cow::Owned(vec![elem; width]); height])
    }

    /// Equivalent to `self.get(p).cloned()`.
    pub fn at(&self, p: (i32, i32)) -> Option<T> {
        self.get(p).cloned()
    }
}
