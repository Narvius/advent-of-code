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
