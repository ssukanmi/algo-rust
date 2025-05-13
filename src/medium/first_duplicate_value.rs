pub fn first_duplicate_value1(array: Vec<i32>) -> i32 {
    let mut min_idx = array.len();
    for i in 0..array.len() {
        for j in i + 1..array.len() {
            if array[i] == array[j] {
                min_idx = min_idx.min(j);
            }
        }
    }
    if min_idx == array.len() {
        return -1;
    }
    array[min_idx]
}

pub fn first_duplicate_value2(array: Vec<i32>) -> i32 {
    let mut seen = std::collections::HashSet::new();
    for &value in array.iter() {
        if seen.contains(&value) {
            return value;
        }
        seen.insert(value);
    }
    -1
}

pub fn first_duplicate_value3(mut array: Vec<i32>) -> i32 {
    for idx in 0..array.len() {
        let v = array[idx].abs();
        if array[v as usize - 1] < 0 {
            return v;
        }
        array[v as usize - 1] *= -1;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_duplicate_value() {
        assert_eq!(first_duplicate_value1(vec![2, 1, 5, 3, 3, 2]), 3);
        assert_eq!(first_duplicate_value1(vec![1, 2, 3, 4]), -1);
        assert_eq!(first_duplicate_value1(vec![1, 2, 3, 4, 5, 6, 7]), -1);
        assert_eq!(first_duplicate_value1(vec![1, 2, 3, 4, 5, 6, 7, 8]), -1);
    }

    #[test]
    fn test_first_duplicate_value2() {
        assert_eq!(first_duplicate_value2(vec![2, 1, 5, 3, 3, 2]), 3);
        assert_eq!(first_duplicate_value2(vec![1, 2, 3, 4]), -1);
        assert_eq!(first_duplicate_value2(vec![1, 2, 3, 4, 5, 6, 7]), -1);
        assert_eq!(first_duplicate_value2(vec![1, 2, 3, 4, 5, 6, 7, 8]), -1);
    }

    #[test]
    fn test_first_duplicate_value3() {
        assert_eq!(first_duplicate_value3(vec![2, 1, 5, 3, 3, 2]), 3);
        assert_eq!(first_duplicate_value3(vec![1, 2, 3, 4]), -1);
        assert_eq!(first_duplicate_value3(vec![1, 2, 3, 4, 5, 6, 7]), -1);
        assert_eq!(first_duplicate_value3(vec![1, 2, 3, 4, 5, 6, 7, 8]), -1);
    }
}
