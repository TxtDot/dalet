use std::fs;

use dalet::{
    abstractions::{HeadingLevel, Tag, ToDaletl},
    daletpack::*,
};

fn main() {
    let dalet_page: Vec<Tag> = vec![Tag::H("I am heading".to_owned(), HeadingLevel::One)];

    let data = encode(dalet_page.to_daletl()).unwrap();

    println!("{:#?}", data);
    println!("{}", data.len());

    let bits: Vec<_> = data.iter().map(|n| format!("{:b}", n)).collect();

    println!("{}", bits.join(""));
    // 11010000100111011010010010010000110000101101101001000011010011001010110000101100101101001011011111001111111111
    fs::write("./test.daletpack", data).unwrap();
}
