
fn vigenere(text: &[u8], key: &[u8], assert_ascii: bool) -> Result<Vec<u8>, String> {
    let len = text.len();
    let keylen = key.len();

    let mut result = Vec::with_capacity(len);

    for i in 0..len {
        let p = text[i] ^ key[i % keylen];
        if assert_ascii && !(19 < p && p < 128) {
            return Err(String::from("invalid character detected in output"));
        }
        result.push(p);
    }

    Ok(result)
}

#[allow(dead_code)]
pub fn encrypt(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    vigenere(plaintext, key, false)
}

pub fn decrypt(ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    vigenere(ciphertext, key, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decrypt_inverses_encrypt() {
        let plaintext = String::from("This is a super secret message.");
        let key = "password";

        let ciphertext = encrypt(plaintext.as_bytes(), key.as_bytes()).unwrap();
        assert_eq!(decrypt(&ciphertext, key.as_bytes()).unwrap(), plaintext.as_bytes());
    }

    #[test]
    fn zero_key_is_noop_cause_of_xor() {
        let key = [0_u8];
        let plaintext = String::from("Super secret!");

        let ciphertext = encrypt(plaintext.as_bytes(), &key).unwrap();
        assert_eq!(String::from_utf8(ciphertext).unwrap(), plaintext);
    }
}
