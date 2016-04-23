fn main() {
    let string1 = "abcba".to_string();
    let string2 = "abccba".to_string();
    let string3 = "abcdba".to_string();
    assert_eq!(Some(true), is_palindrome(&string1));
    assert_eq!(Some(true), is_palindrome(&string2));
    assert_eq!(Some(false), is_palindrome(&string3));
    assert_eq!(None, is_palindrome(&"".to_string()));
}

fn is_palindrome(string: &String) -> Option<bool> {
    if string.is_empty() {
        return None;
    }
    let mut result = true;
    let len = string.len();
    for i in 0..len / 2 {
        if string.chars().nth(i) != string.chars().nth(len - 1 - i) {
            result = false;
        }
    }
    Some(result)
}
