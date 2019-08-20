// vigenere cipher breaking fun for Coursera Cryptography class
use std::env;
use std::fs;
use std::io;
use std::collections::HashMap;
use itertools::Itertools;

// Load and parse a text file containing printed hex of a payload.
// That is: "FFA0" -> [255, 10]
fn parse(file: &str) -> Result<Vec<u8>, io::Error> {
    let s = fs::read_to_string(file)?;
    let chars = s.chars();

    let mut result: Vec<u8> = Vec::with_capacity((s.len() / 2) + 1);

    for (a, b) in chars.tuples() {
        let mut chunk = String::new();
        chunk.push(a);
        chunk.push(b);

        let int = u8::from_str_radix(chunk.as_str(), 16).unwrap();
        result.push(int);
    }
    Ok(result)
}

/// Measure the fraction of how many u8 values in the Vec are
/// valid ASCII values.
fn asciiness(v: &Vec<u8>) -> f32 {
    let printable = v.into_iter()
        .fold(0, |acc, n| acc + n.is_ascii_graphic() as u8);

    printable as f32 / v.len() as f32
}

/// Sample a Vector, creating a new subset of every n-th element
fn stripe<T: Copy>(v: &Vec<T>, n: usize) -> Vec<T> {
    if n < 2 {
        return v.clone();
    }

    let mut result: Vec<T> = Vec::with_capacity((v.len() / n) + 1 as usize);
    let idx: Vec<usize> = (0..v.len()).filter(|i| i % n == 0).collect();

    for i in idx {
        result.push(v[i]);
    }
    result
}

fn freq_of_vec<T>(v: &Vec<T>) -> HashMap<&T, f32>
where T: std::hash::Hash + std::cmp::Eq {
    let n = v.len();
    let mut result = HashMap::new();

    for x in v {
        if result.contains_key(x) {
            result.insert(x, result.get(x).unwrap() + 1.0);
        } else {
            result.insert(x, 1.0);
        }
    }

    for (_, val) in result.iter_mut() {
        *val = (*val / n as f32).powi(2);
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: [filename]");
        return;
    }

    let file = &args[1];
    // let n = usize::from_str_radix(&args[2], 10).expect("Invalid usize value!");

    match parse(file) {
        Ok(data) => {
            for i in 1..64 {
                let s = stripe(&data, i);
                let f = freq_of_vec(&s);

                let num_keys = f.keys().len();
                let sum: f32 = f.values().fold(0_f32, |acc, v| acc + v);
                println!("{:?}: {:?} w/ {}", i, sum, num_keys);
            }
        }
        Err(e) => println!("Error!: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stripe_identity() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(stripe(&v, 0), v);
        assert_eq!(stripe(&v, 1), v);
    }

    #[test]
    fn stripe_n() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(stripe(&v, 2), [1, 3, 5]);
        assert_eq!(stripe(&v, 3), [1, 4]);
        assert_eq!(stripe(&v, 4), [1, 5]);
        assert_eq!(stripe(&v, 5), [1]);
        assert_eq!(stripe(&v, 6), [1]);
    }

    #[test]
    fn freq_of_vec_test() {
        let v = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
        let m = freq_of_vec(&v);

        assert!(m.contains_key(&1));
        assert!(m.contains_key(&2));
        assert!(m.contains_key(&3));
        assert!(m.contains_key(&4));

        assert_eq!(m.get(&1).unwrap(), &0.1_f32);
        assert_eq!(m.get(&2).unwrap(), &0.2_f32);
        assert_eq!(m.get(&3).unwrap(), &0.3_f32);
        assert_eq!(m.get(&4).unwrap(), &0.4_f32);
    }
}
