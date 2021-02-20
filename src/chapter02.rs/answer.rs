use rand::seq::SliceRandom;
use rand::thread_rng;
use std::iter::FromIterator;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
//第2章 UNIXコマンド
pub fn word_count(file_name: &str) -> usize {
    let f = File::open(file_name).expect("file not found");
    let buf = BufReader::new(f);
    return buf.lines().count();
}

#cfg[test]
mod tests {
    use crate::chapter02::answer::{
        count_uniq_words
    }
    use std::fs::{create_dir, remove_file, File};
    use std::io::{BufRead, BufReader, Read};

    const INPUT_PATH: &str = "data/popular-names.txt";
    const EXPECTED_PATH: &str = "data/chap02_expected/";
    const TMP_PATH: &str = "data/chap02_tmp/";
    const N: usize = 5;

    fn read_file_as_string(file_name: &str) -> String {
        let mut f = File::open(file_name).expect(format!("file not found. {}", file_name).as_str());
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("read error");
        return contents;
    }

    fn read_file_as_lines(file_name: &str) -> Vec<String> {
        let f = File::open(file_name).expect(format!("file not found. {}", file_name).as_str());
        let buf = BufReader::new(f);
        let lines: Vec<String> = buf.lines().map(|l| l.unwrap()).collect();
        return lines;
    }

    #[test]
    fn success_10_word_count() {
        let count = word_count(INPUT_PATH);
        let expected = read_file_as_string(format!("{}{}", EXPECTED_PATH, "10.txt").as_str());
        let expected: usize = expected.trim().parse().expect("parse error!");
        assert_eq!(expected, count);
    }
}