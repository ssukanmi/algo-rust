// O(n) time | O(1) space
pub fn is_valid_subsequence1(array: Vec<i32>, sequence: Vec<i32>) -> bool {
    let mut seq_idx = 0;
    let mut arr_idx = 0;
    while arr_idx < array.len() && seq_idx < sequence.len() {
        if array[arr_idx] == sequence[seq_idx] {
            seq_idx += 1;
        }
        arr_idx += 1;
    }
    seq_idx == sequence.len()
}

pub fn is_valid_subsequence2(array: Vec<i32>, sequence: Vec<i32>) -> bool {
    let mut seq_idx = 0;
    for v in array {
        if seq_idx == sequence.len() {
            break;
        }
        if v == sequence[seq_idx] {
            seq_idx += 1;
        }
    }
    seq_idx == sequence.len()
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_is_valid_subsequence1() {
        assert_eq!(
            is_valid_subsequence1(vec![5, 1, 22, 25, 6, -1, 8, 10], vec![1, 6, -1, 10]),
            true
        );
        assert_eq!(
            is_valid_subsequence1(vec![5, 1, 22, 25, 6, -1, 8, 10], vec![5, 22, 25]),
            true
        );
    }

    #[test]
    fn test_is_valid_subsequence2() {
        assert_eq!(
            is_valid_subsequence2(vec![5, 1, 22, 25, 6, -1, 8, 10], vec![1, 6, -1, 10]),
            true
        );
        assert_eq!(
            is_valid_subsequence2(vec![5, 1, 22, 25, 6, -1, 8, 10], vec![5, 22, 25]),
            true
        );
    }
}
