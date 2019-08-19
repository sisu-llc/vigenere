// vigenere cipher breaking fun for Coursera Cryptography class
use std::env;
use std::fs;
use std::io;

fn parse(file: &str) -> Result<Vec<u8>, io::Error> {
    let s = fs::read_to_string(file)?;

    let r: Vec<u8> = (1..s.len() as u8)
        .filter(|i| i % 2 == 0)
        .collect();

    let mut chars = s.chars();
    let mut result: Vec<u8> = Vec::with_capacity(r.len());

    for _ in r {
        let a = chars.next().unwrap();
        let b = chars.next().unwrap();
        let mut chunk = String::new();
        chunk.push(a);
        chunk.push(b);

        let int = u8::from_str_radix(chunk.as_str(), 16).unwrap();
        result.push(int);
    }
    Ok(result)
}

fn stripe<T: Copy>(v: Vec<T>, n: usize) -> Vec<T> {
    let mut result: Vec<T> = Vec::with_capacity((v.len() / n) + 1 as usize);
    let idx: Vec<usize> = (1..v.len()).filter(|i| i % n == 0).collect();

    for i in idx {
        result.push(v[i]);
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: [filename] [stripe index]");
        return;
    }

    let file = &args[1];
    let n = usize::from_str_radix(&args[2], 10).expect("Invalid usize value!");

    match parse(file) {
        Ok(data) => {
            println!("{:?}", stripe(data, n));
        }
        Err(e) => println!("Error!: {}", e),
    }
}
