pub fn is_palindrome1(string: &str) -> bool {
    string.chars().eq(string.chars().rev())
}

pub fn is_palindrome2(string: &str) -> bool {
    if string.is_empty(){
        return true;
    }

    let mut left = 0;
    let mut right = string.len() - 1;
    while left < right {
        if string.chars().nth(left) != string.chars().nth(right) {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

pub fn is_palindrome3(string: &str) -> bool {
    let i = 0;
    is_palindrome3_helper(string, i)
}

fn is_palindrome3_helper(string: &str, i: isize) -> bool {
    let j = string.len() as isize - 1 - i;
    if i >= j {
        return true;
    }
    string.chars().nth(i as usize) == string.chars().nth(j as usize)
        && is_palindrome3_helper(string, i + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome1() {
        assert_eq!(is_palindrome1("abcdedcba"), true);
        assert_eq!(is_palindrome1("abcde"), false);
        assert_eq!(is_palindrome1("a"), true);
        assert_eq!(is_palindrome1(""), true);
    }

    #[test]
    fn test_is_palindrome2() {
        assert_eq!(is_palindrome2("abcdedcba"), true);
        assert_eq!(is_palindrome2("abcde"), false);
        assert_eq!(is_palindrome2("a"), true);
        assert_eq!(is_palindrome2(""), true);
    }

    #[test]
    fn test_is_palindrome3() {
        assert_eq!(is_palindrome3("abcdedcba"), true);
        assert_eq!(is_palindrome3("abcde"), false);
        assert_eq!(is_palindrome3("a"), true);
        assert_eq!(is_palindrome3(""), true);
    }
}
