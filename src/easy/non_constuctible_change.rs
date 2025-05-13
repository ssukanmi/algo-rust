pub fn non_constuctible_change(coins: &Vec<i32>) -> i32 {
    let mut coins = coins.clone();
    coins.sort();
    let mut change = 0;
    for coin in coins {
        if coin > change + 1 {
            break;
        }
        change += coin;
    }
    change + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_constuctible_change() {
        assert_eq!(non_constuctible_change(&vec![1, 2, 5]), 4);
        assert_eq!(non_constuctible_change(&vec![5, 7, 1, 1, 2, 3, 22]), 20);
        assert_eq!(non_constuctible_change(&vec![1, 1, 1, 1]), 5);
        assert_eq!(non_constuctible_change(&vec![1]), 2);
        assert_eq!(non_constuctible_change(&vec![]), 1);
    }
}
