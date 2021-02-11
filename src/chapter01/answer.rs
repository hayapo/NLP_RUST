use std::iter::FromIterator;
//第1章:準備運動

//00
pub fn num_00(input_string: &str) -> String {
    if input_string.len() > 0 {
        String::from_iter(input_string.chars().rev())
    } else {
        String::new()
    }
}
//01
pub fn num_01(input_string: &str) -> String {
    let iter = input_string
        .chars()
        .enumerate()
        .filter(|(i, _x)| i%2==0)
        .map(|(_i, x)| x);
    String::from_iter(iter)
}


#[cfg(test)]
mod tests {
    use crate::chapter01::answer::{
        num_00, num_01
    };

    #[test]
    fn test_00() {
        let input_string = "stressed";
        let expected = "desserts";
        assert_eq!(expected, num_00(input_string));
    }

    #[test]
    fn test_01() {
        let input_string = "パタトクカシーー";
        let expected = "パトカー";
        assert_eq!(expected, num_01(input_string));
    }
}