pub fn encrypt(_plaintext: &[u8], _key: &[u8]) -> Vec<u8> {
    panic!("not implemented");
}

pub fn decrypt(_ciphertext: &[u8], _key: &[u8]) -> Vec<u8> {
    panic!("not implemented");
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
