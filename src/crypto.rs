fn vigenere(text: &[u8], key: &[u8]) -> Vec<u8> {
    let len = text.len();
    let keylen = key.len();

    let mut result = Vec::with_capacity(len);

    for i in 0..len {
        let p = text[i] ^ key[i % keylen];
        result.push(p);
    }

    result
}

#[allow(dead_code)]
pub fn encrypt(plaintext: &[u8], key: &[u8]) -> Vec<u8> {
    vigenere(plaintext, key)
}

pub fn decrypt(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    vigenere(ciphertext, key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decrypt_inverses_encrypt() {
        let plaintext = String::from("This is a super secret message.");
        let key = "password";

        let ciphertext = encrypt(plaintext.as_bytes(), key.as_bytes());
        assert_eq!(decrypt(&ciphertext, key.as_bytes()), plaintext.as_bytes());
    }
}
