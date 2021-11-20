use std::fs;
use std::fs::File;
mod cipher;
use cipher::Cipher;
mod utils;
use std::io::prelude::*;

fn main() {
    let arguments = utils::get_arguments();

    let cipher = cipher::make_cipher(arguments.secret);

    if arguments.should_decrypt {
        let plaintext = decrypt(arguments.path_to_file, cipher);
        if arguments.should_print {
            print(plaintext);
        } else {
            write_to_file(arguments.path_to_file_to_write_to, plaintext);
        }
    } else {
        let ciphertext = encrypt(arguments.path_to_file, cipher);
        write_to_file(arguments.path_to_file_to_write_to, ciphertext);
    }
}

fn decrypt(path_to_file: String, cipher: Cipher) -> Vec<u8> {
    let buffer = fs::read(utils::make_path_to_file(path_to_file)).expect("Failed to read a file");
    cipher.decrypt(buffer)
}

fn encrypt(path_to_file: String, cipher: Cipher) -> Vec<u8> {
    let contents =
        fs::read_to_string(utils::make_path_to_file(path_to_file)).expect("Failed to read a file");
    cipher.encrypt(contents)
}

fn print(text: Vec<u8>) {
    println!("{}", String::from_utf8_lossy(&text));
}

fn write_to_file(path_to_file_to_write_to: Option<String>, data: Vec<u8>) {
    let mut file = File::create(utils::make_path_to_file(
        path_to_file_to_write_to.expect("Path to write is missing"),
    ))
    .unwrap();
    file.write(&data[..]).unwrap();
}
