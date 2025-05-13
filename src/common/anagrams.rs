pub fn are_anagrams1(s1: &str, s2: &str) -> bool {
    let mut s1_chars = s1.chars().collect::<Vec<_>>();
    let mut s2_chars = s2.chars().collect::<Vec<_>>();

    // Sort the characters in both strings
    s1_chars.sort();
    s2_chars.sort();

    // Compare the sorted character vectors
    s1_chars == s2_chars
}

pub fn are_anagrams2(s1: &str, s2: &str) -> bool {
    let mut s1_map = std::collections::HashMap::new();
    let mut s2_map = std::collections::HashMap::new();

    // Count the occurrences of each character in both strings
    for c in s1.chars() {
        *s1_map.entry(c).or_insert(0) += 1;
    }
    for c in s2.chars() {
        *s2_map.entry(c).or_insert(0) += 1;
    }

    // Compare the character counts
    s1_map == s2_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_anagrams() {
        assert!(are_anagrams1("listen", "silent"));
        assert!(are_anagrams1("triangle", "integral"));
        assert!(!are_anagrams1("apple", "pale"));
        assert!(!are_anagrams1("hello", "world"));
    }
}
