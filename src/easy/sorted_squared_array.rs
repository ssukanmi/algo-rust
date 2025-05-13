// 0 (nlongn) time | O(1) space
pub fn sorted_squared_array1(array: Vec<i32>) -> Vec<i32> {
    let mut new_arr = array.into_iter().map(|x| x * x).collect::<Vec<i32>>();

    new_arr.sort();
    new_arr
}

// if array has been initially sorted
// O(n) time | O(n) space
pub fn sorted_squared_array2(array: Vec<i32>) -> Vec<i32> {
    let mut new_arr = vec![0; array.len()];
    let mut left_idx = 0;
    let mut right_idx = array.len() as isize - 1;

    for arr_idx in (0..array.len()).rev() {
        let left_val = array[left_idx];
        let right_val = array[right_idx as usize];
        if left_val.abs() > right_val.abs() {
            new_arr[arr_idx] = left_val * left_val;
            left_idx += 1;
        } else {
            new_arr[arr_idx] = right_val * right_val;
            right_idx -= 1;
        }
    }
    new_arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_squared_array1() {
        println!("{:?}", sorted_squared_array2(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_sorted_squared_array2() {}
}
