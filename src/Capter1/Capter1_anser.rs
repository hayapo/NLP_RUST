//第1章:準備運動

//00
pub fn reverse_string(input_string: &str) -> String {
    if input_string.len() > 0 {
        String::from_iter(original.chars().rev())
    } else {
        String::new()
    }
}
//01

#[cfg(test)]
mod tests {
    #[test]
    fn test_00() {
        let input_string = "Stressed";
        let expected = "dessertS";
        assert_eq!(expected,reverse_string(input_string))
    }
}