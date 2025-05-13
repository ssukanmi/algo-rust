// sort array in place
pub fn sort_array<T: Ord>(arr: &mut [T]) {
    arr.sort();
}

// sort array and return new array
pub fn sort_array_new<T>(arr: &[T]) -> Vec<T>
where
    T: Ord + Clone,
{
    let mut sorted = arr.to_vec();
    sorted.sort();
    sorted
}

// create array of length n
pub fn create_array<T>(n: usize) -> Vec<T>
where
    T: Default + Clone,
{
    vec![T::default(); n]
}
