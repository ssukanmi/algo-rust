pub fn insertion_sort(mut array: Vec<i32>) -> Vec<i32> {
    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && array[j] < array[j - 1] {
            array.swap(j, j - 1);
            j -= 1;
        }
    }
    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let array = vec![5, 2, 9, 1, 5, 6];
        let res = insertion_sort(array);
        assert_eq!(res, vec![1, 2, 5, 5, 6, 9]);
    }
}
