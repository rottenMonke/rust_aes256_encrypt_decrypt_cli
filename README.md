# rust_aes256_encrypt_decrypt_cli
Simple cli tool for encrypting/decrypting a file

It uses Aes256GcmSiv for decryption/encryption and blake3 for creating a hash out of user secret (256 bit) for the cipher key
and first 96 of the hashed secret bit for the nonce.

# test in repo

**ENCRYPT**
(creates a file)
```
cargo run -- ./path/to/fileToEncrypt.txt ./pathToEncryptedData secret
```


**DECRYPT**

writes to file (creates a file)

```
cargo run -- --d ./path/to/fileToDecrypt.txt ./pathTodecryptedData secret
```

Output to console

```
cargo run -- --d --p ./path/to/fileToDecrypt.txt secret
```
