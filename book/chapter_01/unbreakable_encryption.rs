/// # "Unbreakable" Encryption
///
/// This implementation leans into `std::iter` to avoid the `new byte[]` allocations and for-loops in the Java implementation.
///
/// Java implementation: https://github.com/davecom/ClassicComputerScienceProblemsInJava/blob/master/CCSPiJ/src/chapter1/UnbreakableEncryption.java
///
use rand::prelude::*;

struct KeyPair {
    key_1: Vec<u8>,
    key_2: Vec<u8>,
}

impl KeyPair {
    fn random_key(length: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();

        let mut dummy = vec![0; length];

        for i in &mut dummy {
            *i = rng.gen();
        }

        dummy
    }

    fn encrypt(original: &str) -> KeyPair {
        let original_bytes = original.as_bytes();
        let dummy_key = Self::random_key(original_bytes.len());

        let encrypted_key: Vec<u8> = original_bytes
            .iter()
            .enumerate()
            .map(|(i, b)| b ^ dummy_key[i])
            .collect();

        KeyPair {
            key_1: dummy_key,
            key_2: encrypted_key,
        }
    }

    fn decrypt(self) -> Result<String, std::string::FromUtf8Error> {
        let decrypted: Vec<u8> = self
            .key_1
            .iter()
            .zip(self.key_2.iter())
            .map(|(i1, i2)| i1 ^ i2) // XOR decryption
            .collect();

        String::from_utf8(decrypted)
    }
}

fn main() {
    let kp = KeyPair::encrypt("My super secret password abc890xyz123 🔐");

    if let Ok(decrypted) = kp.decrypt() {
        println!("Decrypted: `{}`", decrypted);
    }
}
