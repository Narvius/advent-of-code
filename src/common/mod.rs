pub mod intcode;

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
