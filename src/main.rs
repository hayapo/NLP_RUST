extern crate nlp_rust;

use nlp_rust::chapter01::answer;

fn main() {
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

    )
}