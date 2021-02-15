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

//02
pub fn mix_string(first_string: &str, second_string: &str) -> String {
    let mut mixed_string = String::from_iter(
        first_string
            .chars()
            .zip(second_string.chars())
            .map(|(x, y)| format!("{}{}", x, y)),
    );

    if first_string.chars().count() > second_string.chars().count() {
        first_string
            .chars()
            .skip(second_string.chars().count())
            .for_each(|x| mixed_string.push(x));
    } else if second_string.chars().count() > first_string.chars().count() {
        second_string
            .chars()
            .skip(first_string.chars().count())
            .for_each(|x| mixed_string.push(x));
    }
    return mixed_string;
}


#[cfg(test)]
mod tests {
    use crate::chapter01::answer::{
        num_00, num_01, mix_string
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

    #[test]

    fn test_02() {
        let first_string = "パトカー";
        let second_string = "タクシー";
        let expected = "パタトクカシーー";

        assert_eq!(expected, mix_string(first_string, second_string));

    }
}