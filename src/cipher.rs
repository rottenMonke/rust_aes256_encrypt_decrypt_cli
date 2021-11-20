use aes_gcm_siv::aead::{Aead, NewAead};
use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce}; // Or Aes128GcmSiv
use blake3;

pub struct Cipher {
    pub cipher: Aes256GcmSiv,
    pub nonce: [u8; 12],
}

impl Cipher {
    pub fn encrypt(&self, data: String) -> Vec<u8> {
        self.cipher
            .encrypt(Nonce::from_slice(&self.nonce), data.as_ref())
            .expect("Failed to encrypt!")
    }
    pub fn decrypt(&self, data: Vec<u8>) -> Vec<u8> {
        self.cipher
            .decrypt(Nonce::from_slice(&self.nonce), data.as_ref())
            .expect("Failed to decrypt!")
    }
}

pub fn make_cipher(user_input: String) -> Cipher {
    let hashed_user_input = blake3::hash(user_input.as_bytes());
    let hashed_user_input_as_bytes = hashed_user_input.as_bytes();
    let key = Key::from_slice(hashed_user_input_as_bytes);
    let mut nonce: [u8; 12] = [0; 12];

    // xDD
    for i in 0..12 {
        nonce[i] = hashed_user_input_as_bytes[i]
    }

    let cipher = Aes256GcmSiv::new(key);
    Cipher {
        cipher: cipher,
        nonce: nonce,
    }
}
