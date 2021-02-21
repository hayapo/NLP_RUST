extern crate log;
extern crate env_logger;
extern crate nlp_rust;
extern crate serde_json;
extern crate rand;

use nlp_rust::chapter01::answer;
use nlp_rust::chapter02::answer as answer_02;
use std::collections::BTreeMap;

fn main() {
    env_logger::init();
    log::info!("start!!");
    // Chapter 01
    println!("-- Chapter01");
    //Question 00
    let orig00 = "stressed";
    println!("---- 00 文字列の逆順");
    println!(
        "Ans_00(\"{}\") -> {}",
        orig00,
        answer::num_00(orig00)
    );
    //Question 01
    let orig01 = "パタトクカシーー";
    println!("---- 01 パタトクカシーー");
    println!(
        "Ans_01(\"{}\") -> {}",
        orig01,
        answer::num_01(orig01)

    );

    //Question 02
    let first_string = "パトカー";
    let second_string = "タクシー";
    println!("---- 02 Mix two string");
    println!(
        "mix_two_str(\"{}\", \"{}\") -> {}",
        first_string,
        second_string,
        answer::mix_string(first_string, second_string)
    );

    //Question 03
    let orig03 = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    println!("---- 03 Pi");
    println!(
        "pi(\"{}\") -> {}",
        orig03,
        format!("{:?}", &answer::pi(orig03))
    );

    //Question 04
    let original = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let idx_symbols: Vec<usize> = vec![1, 5, 6, 7, 8, 9, 15, 16, 19];
    println!("---- 04 Chemical Element");
    println!(
        "symbol_of_element(\"{}\", {}) -> ",
        original,
        format!("{:?}", &idx_symbols)
    );
    print_map_to_json(answer::chemical_symbols(original, idx_symbols));

    //Question 05
    let original = "I am an NLPer";
    println!("---- 05 N-gram (word, char)");
    println!(
        "word_ngram(\"{}\", 2) -> {}",
        original,
        format!("{:?}", &answer::word_ngram(original, 2))
    );
    println!(
        "char_ngram(\"{}\", 2) -> {}",
        original,
        format!("{:?}", &answer::char_ngram(original, 2))
    );

    //Question 06
    let orig06_1 = "paraparaparadise";
    let orig06_2 = "paragraph";
    println!("---- 06 Set (Union, Intersection, Difference, Include check)");
    println!(
        "union -> {}",
        format!(
            "{:?}",
            &answer::union_ngram_set(
                answer::char_ngram_set(orig06_1, 2),
                &answer::char_ngram_set(orig06_2, 2)
            )
        )
    );
    println!(
        "intersection -> {}",
        format!(
            "{:?}",
            &answer::intersection_ngram_set(
                answer::char_ngram_set(orig06_1, 2),
                &answer::char_ngram_set(orig06_2, 2)
            )
        )
    );
    println!(
        "difference X - Y -> {}",
        format!(
            "{:?}",
            &answer::difference_ngram_set(
                answer::char_ngram_set(orig06_1, 2),
                &answer::char_ngram_set(orig06_2, 2)
            )
        )
    );
    println!(
        "difference Y - X -> {}",
        format!(
            "{:?}",
            &answer::difference_ngram_set(
                answer::char_ngram_set(orig06_2, 2),
                &answer::char_ngram_set(orig06_1, 2)
            )
        )
    );

    println!(
        "\"{}\" contains \"se\"? -> {}",
        orig06_1,
        answer::char_ngram_set(orig06_1, 2).contains("se")
    );
    println!(
        "\"{}\" contains \"se\"? -> {}",
        orig06_2,
        answer::char_ngram_set(orig06_2, 2).contains("se")
    );
    
    //Question07
    let orig07_x = 0;
    let orig07_y = "y";
    let orig07_z = 2.0;
    println!("---- 07 Generate Sentences");
    println!("x = {}, y = {}, z ={:?}", orig07_x, orig07_y, orig07_z);
    println!(
        "{}",
        answer::generate_sentence(orig07_x, orig07_y, orig07_z)
    );

    //Question08
    let orig08 = "AaBbCc";
    let _orig08_ciphered = "AzByCx";
    println!("---- 08 Cipher");
    println!(">>>>>暗号化");
    println!("原文={},暗号文={}", orig08, answer::cipher(orig08));
    println!(">>>>>復号化");
    println!("原文={},暗号文={}", _orig08_ciphered, answer::cipher(_orig08_ciphered));

    //Question08
    let original_sentence = "I couldn’t believe that I could actually understand what I was reading : the phenomenal power of the human mind.";
    println!("---- 09 Typoglycemia");
    println!("{}", answer::typoglycemia(original_sentence));

    // Chapter 02
    println!("-- Chapter02");
    println!("{}", answer_02::word_count("data/popular-names.txt"))
}

fn print_map_to_json(map: BTreeMap<String, usize>) {
    println!("{}", serde_json::to_string_pretty(&map).unwrap());
}

