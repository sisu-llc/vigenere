// vigenere
use std::fs;

//const INPUT: &str = "/home/dave/src/coursera/cryptography/vigenere/input/cipher.txt";
const INPUT: &str = "/home/dave/src/coursera/cryptography/vigenere/input/test.txt";

fn main() {
    let s = fs::read_to_string(INPUT)
        .expect(format!("Could not read {:?}", INPUT).as_str());
    println!("{}", s);

    let r: Vec<u8> = (1..s.len() as u8).into_iter().filter(|i| i % 2 == 0).collect();

    let mut chars = s.chars();
    for _ in r {
        let a = chars.next().unwrap();
        let b = chars.next().unwrap();
        let mut chunk = String::new();
        chunk.push(a);
        chunk.push(b);
        let int = u8::from_str_radix(chunk.as_str(), 16).unwrap();
        println!("a: {}, b: {}, chunk: 0x{:x?}, int: {}, char: {}",
                 a, b, int, chunk, int as char);
    }
}
