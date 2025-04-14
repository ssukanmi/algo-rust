use std::collections::HashSet;

// O (n^2) time | O(1) space
// Brute force easy approach
pub fn two_number_sum1(array: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..array.len() - 1 {
        let first_num = array[i];
        for j in i + 1..array.len() {
            let second_num = array[j];
            if first_num + second_num == target {
                return vec![first_num, second_num];
            }
        }
    }
    return vec![];
}

// O (n) time | O(n) space
pub fn two_number_sum2(array: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash: HashSet<i32> = HashSet::new();
    for i in array {
        let compliment = target - i;
        if hash.contains(&compliment) {
            return vec![compliment, i];
        }
        hash.insert(i);
    }
    return vec![];
}

// O (n log n) time | O(1) space
pub fn two_number_sum3(array: Vec<i32>, target: i32) -> Vec<i32> {
    let mut new_arr = array.clone();
    new_arr.sort();
    let mut l = 0;
    let mut r = new_arr.len() - 1;
    while l < r {
        let sum = new_arr[l] + new_arr[r];
        if sum == target {
            return vec![new_arr[l], new_arr[r]];
        } else if sum < target {
            l += 1;
        } else {
            r -= 1;
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_number_sum() {
        let array = vec![3, 5, -4, 8, 11, 1, -1, 6];
        let target = 10;
        let result = two_number_sum1(array, target);
        assert!(result == vec![11, -1] || result == vec![-1, 11]);
    }

    #[test]
    fn test_two_number_sum2() {
        let array = vec![3, 5, -4, 8, 11, 1, -1, 6];
        let target = 10;
        let result = two_number_sum2(array, target);
        assert!(result == vec![11, -1] || result == vec![-1, 11]);
    }

    #[test]
    fn test_two_number_sum3() {
        let array = vec![3, 5, -4, 8, 11, 1, -1, 6];
        let target = 10;
        let result = two_number_sum2(array, target);
        assert!(result == vec![11, -1] || result == vec![-1, 11]);
    }
}
