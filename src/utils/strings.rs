// sort string
pub fn sort_string(string: &str) -> String {
    let mut chars: Vec<char> = string.chars().collect();
    chars.sort();
    chars.iter().collect()
}
