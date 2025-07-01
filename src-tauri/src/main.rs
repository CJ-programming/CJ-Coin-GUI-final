/*
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
*/

/*
mod crypto;

use crypto::{generate_key_pair, sign, verify};


use crypto::{generate_aes_key_to_file, load_aes_key_from_file, aes_encrypt, aes_decrypt};
use crypto::{generate_key_pair_to_file, load_key_pair_from_file};

use hex::encode;

const ITERATIONS: u32 = 600_000;
#[test]
fn unit_tests() {
	let _result = generate_aes_key_to_file("aes_key_data.json", "hello".to_string(), ITERATIONS);
    println!("Key saved successfully.");

    let loaded = load_aes_key_from_file("aes_key_data.json").unwrap();

	let key = loaded.0;

    println!("Loaded KeyData: {:?}", key);

	// let (cipher_text, iv) = aes_encrypt(key, String::from("hello"));

	let _ = generate_key_pair_to_file("key_pair_data.json", key);

	let (encrypted_private_key, public_key, iv) = load_key_pair_from_file("key_pair_data.json").expect("Could not load key pair from file");

	let private_key = aes_decrypt(&key, iv, encrypted_private_key);
}
*/

mod crypto_utils;
use crypto_utils::decrypt_file;
use crypto_utils::generate_aes_key_to_file;
use crypto_utils::generate_key_pair_to_file;

fn main() {
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![
			crypto_utils::generate_aes_key_to_file,
			crypto_utils::generate_key_pair_to_file,
			crypto_utils::decrypt_file,
        ])
        .run(context)
        .expect("error while running tauri application");
}
