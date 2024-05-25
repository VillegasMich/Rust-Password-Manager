mod cli;
mod file;
mod handlers;
mod models;
mod utils;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;
// use chacha20poly1305::{
//     aead::{Aead, AeadCore, KeyInit, OsRng},
//     ChaCha20Poly1305, Key,
// };

use std::io;

fn main() -> io::Result<()> {
    // Test encryption and decryption
    // !!! aes
    let aes_key = GenericArray::from([0u8; 16]);
    let mut block = GenericArray::from(b"1111111111111111".to_owned());
    let aes_cipher = Aes128::new(&aes_key);

    // let block_copy = block.clone();

    // Encrypt block in-place
    aes_cipher.encrypt_block(&mut block);
    println!("{:#?}", block);

    // And decrypt it back
    aes_cipher.decrypt_block(&mut block);
    println!("{}", String::from_utf8(block.to_vec()).unwrap());
    // assert_eq!(block, block_copy);

    // !!! ChaCha20Poly1305
    // let master_key = Key::from_slice(b"112609840327102003"); // needs to be len 32
    // let cipher = ChaCha20Poly1305::new(&master_key);
    // let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    // let ciphertext = cipher.encrypt(&nonce, "password".as_ref()).expect("");
    // println!("Encryption completed");
    // let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).expect("");
    // println!("{:#?}", &ciphertext);
    // println!("{}", String::from_utf8(plaintext).unwrap());
    // cli::parse()
    Ok(())
}
