pub fn length_of_last_word(s: String) -> i32 {
    let parts = s.split_whitespace();
    match parts.last() {
        Some(s) => s.len() as i32,
        _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_insert_1() {
        assert_eq!(length_of_last_word("Hello world".to_string()), 5);
    }
}