use std::iter::FromIterator;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
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

//Question 03
pub fn pi(original: &str) -> Vec<usize> {
    original
        .split_whitespace()
        .map(|word| {
            word.chars()
                .filter(|x| x.is_alphabetic())
                .collect::<Vec<char>>()
                .len()
        }) 
        .collect::<Vec<usize>>()
}

//Question 04
pub fn chemical_symbols(sentence: &str, index_symbols: Vec<usize>) -> BTreeMap<String, usize> {
    let mut symbols: BTreeMap<String, usize> = BTreeMap::new();
    sentence
        .split_whitespace()
        .enumerate()
        .for_each(|(idx, word)| {
            let idx = idx + 1;
            if index_symbols.contains(&idx) {
                symbols.insert(String::from_iter(word.chars().take(1)), idx);
            } else {
                symbols.insert(String::from_iter(word.chars().take(2)), idx);
            }
        });
    return symbols;
}

//Question5
pub fn word_ngram(text: &str, n:usize) -> Vec<Vec<String>> {
    return text
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .windows(n)
            .map(|x| Vec::from(x.to_vec()))
            .collect::<Vec<Vec<String>>>();
}

pub fn char_ngram(text: &str, n: usize) -> Vec<String> {
    return text
            .chars()
            .collect::<Vec<char>>()
            .windows(n)
            .map(|x| String::from_iter(x.to_vec()))
            .collect::<Vec<String>>();
}

//Question6
pub fn char_ngram_set(text: &str, n: usize) -> BTreeSet<String> {
    let mut ngram_set = BTreeSet::new();
    char_ngram(text, n).iter().for_each(|x| {
        ngram_set.insert(x.to_string());
    });
    return ngram_set;
}

pub fn union_ngram_set(source_set: BTreeSet<String>, target_set: &BTreeSet<String>) -> BTreeSet<String> {
    return source_set
        .union(target_set)
        .cloned()
        .collect::<BTreeSet<String>>();
}

pub fn intersection_ngram_set(
    source_set: BTreeSet<String>,
    target_set: &BTreeSet<String>,
) -> BTreeSet<String> {
    return source_set
        .intersection(target_set)
        .cloned()
        .collect::<BTreeSet<String>>();
}

pub fn difference_ngram_set(
    source_set: BTreeSet<String>,
    target_set: &BTreeSet<String>,
) -> BTreeSet<String> {
    return source_set
        .difference(target_set)
        .cloned()
        .collect::<BTreeSet<String>>();
}

#[cfg(test)]
mod tests {
    use crate::chapter01::answer::{
        num_00, num_01, mix_string, pi, chemical_symbols, word_ngram, char_ngram, char_ngram_set, union_ngram_set, intersection_ngram_set, difference_ngram_set
    };

    use std::collections::BTreeMap;

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

    #[test]
    fn test_03(){
        let original = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
        let expected: Vec<usize> = vec!{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9};
        assert_eq!(expected, pi(original))
    }

    #[test]
    fn test_04(){
        let original = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
        let index_symbols: Vec<usize> = vec!{1, 5, 6, 7, 8, 9, 15, 16, 19};
        let expected_vec = vec![
            "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mi", "Al", "Si", "P", "S",
            "Cl", "Ar", "K", "Ca",
        ];
        let mut expected = BTreeMap::new();
        for (i, symbol) in expected_vec.iter().enumerate() {
            let idx = i + 1;
            expected.insert(symbol.to_string(), idx);
        }
        let actual = chemical_symbols(original, index_symbols);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_05() {
        let original = "I am an NLPer";
        let n = 2;
        let expected_word_tokens: Vec<Vec<&str>> = 
            vec![vec!["I", "am"], vec!["am", "an"], vec!["an", "NLPer"]];
        let actual_word_tokens = word_ngram(original, n);
        assert_eq!(expected_word_tokens, actual_word_tokens);

        //char n-gram
        let expected_char_tokens: Vec<&str> = vec![
            "I ", " a", "am", "m ", " a", "an", "n ", " N", "NL", "LP", "Pe", "er",
        ];
        let actual_char_tokens = char_ngram(original, n);
        assert_eq!(expected_char_tokens, actual_char_tokens)
    }

    #[test]
    fn test_06() {
        let original1 = "paraparaparadise";
        let original2 = "paragraph";
        
        let expected1_value = vec!["ad", "ap", "ar", "di", "is", "pa", "ra", "se"];
        let expected2_value = vec!["ag", "ap", "ar", "gr", "pa", "ph", "ra"];

        assert_eq!(
            expected1_value,
            char_ngram_set(original1, 2)
                .into_iter()
                .collect::<Vec<String>>()
        );
        assert_eq!(
            expected2_value,
            char_ngram_set(original2, 2)
                .into_iter()
                .collect::<Vec<String>>()
        );

        let expected_union = vec![
            "ad", "ag", "ap", "ar", "di", "gr", "is", "pa", "ph", "ra", "se",
        ];
        assert_eq!(
            expected_union,
            union_ngram_set(char_ngram_set(original1, 2), &char_ngram_set(original2, 2))
                .into_iter()
                .collect::<Vec<String>>()
        );

        let expected_intersection = vec!["ap", "ar", "pa", "ra"];
        assert_eq!(
            expected_intersection,
            intersection_ngram_set(char_ngram_set(original1, 2), &char_ngram_set(original2, 2))
                .into_iter()
                .collect::<Vec<String>>()
        );

        // origina1 - original2
        let expected_difference_1_minus_2 = vec!["ad", "di", "is", "se"];
        assert_eq!(
            expected_difference_1_minus_2,
            difference_ngram_set(char_ngram_set(original1, 2), &char_ngram_set(original2, 2))
                .into_iter()
                .collect::<Vec<String>>()
        );

        // original2 - origina1
        let expected_difference_2_minus_1 = vec!["ag", "gr", "ph"];
        assert_eq!(
            expected_difference_2_minus_1,
            difference_ngram_set(char_ngram_set(original2, 2), &char_ngram_set(original1, 2))
                .into_iter()
                .collect::<Vec<String>>()
        );

        // find "se" from each set
        assert_eq!(true, char_ngram_set(original1, 2).contains("se"));
        assert_eq!(false, char_ngram_set(original2, 2).contains("se"));
    }
}