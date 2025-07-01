use aes::Aes256;

use block_modes::{BlockMode, CBC};
use block_modes::block_padding::Pkcs7;
use pbkdf2::pbkdf2_hmac_array;

use rand::{thread_rng, Rng};

use std::fs;
use std::io;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn hex_to_bytes(hex_str: &str) -> [u8; 32] {
	let str_vec = decode(hex_str).expect("Unable to decode hex string.");
	let bytes: [u8; 32] = str_vec.try_into().expect("Expected a 32-byte vector");

	bytes
}

pub fn generate_aes_key_to_file(path: &str, password: &str, iterations: u32) -> io::Result<()> {
    let (key, salt, iterations) = generate_key(password, iterations);
    
    let data = AesKeyData {
        hmac_array: encode(key),
        salt: encode(&salt),
        iterations,
    };

    let json = serde_json::to_string_pretty(&data)?;
    fs::write(path, json)?;

    Ok(())
}

pub fn load_aes_key_from_file(path: &str) -> io::Result<(String, String, u32)> {
    let content = fs::read_to_string(path)?;
    let data: AesKeyData = serde_json::from_str(&content)?;

    let hmac_array = data.hmac_array;
    let salt = data.salt;

    Ok((hmac_array, salt, data.iterations))
}

pub fn generate_key_pair_to_file(path: &str, hex_key: &str) -> io::Result<()>{
	let vec_key: Vec<u8> = decode(hex_key).expect("Error while decoding hex key.");
	let key: [u8; 32] = vec_key.try_into().expect("Expected a 32-byte vector");
	let (private_key, public_key) = generate_key_pair();

	let (encrypted_private_key, iv) = aes_encrypt(key, &private_key);

	let data = KeyPairData {
		encrypted_private_key: encrypted_private_key,
		public_key: public_key,
		iv: iv,
	};

	let json = serde_json::to_string_pretty(&data)?;
	fs::write(path, json)?;

	Ok(())
}

pub fn load_key_pair_from_file(path: &str) -> io::Result<(Vec<u8>, Vec<u8>, [u8; 32])> {
    let content = fs::read_to_string(path)?;
    let data: KeyPairData = serde_json::from_str(&content)?;

    let encrypted_private_key = decode(&data.encrypted_private_key).expect("Invalid hex private key");

    let public_key = decode(&data.public_key).expect("Invalid hex public key");

	let iv_vec = decode(&data.iv).expect("Invalid hex iv");
	let iv: [u8; 32] = iv_vec.try_into().expect("Expected a 32-byte vector");

    Ok((encrypted_private_key, public_key, iv))
}

fn generate_random_bytes(size: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; size];
    thread_rng().fill(&mut buffer[..]);
    buffer
}

fn generate_salt() -> Vec<u8> {
	let salt = generate_random_bytes(8);
	salt
}

fn generate_iv() -> Vec<u8> {
	let iv = generate_random_bytes(16);
	iv
}

fn derive_key(password: &[u8], salt: &[u8], iterations: u32) -> [u8; 32] {
	let key = pbkdf2_hmac_array::<Sha256,32>(&password, salt, iterations);

	key
}

pub fn generate_key(password: &str, iterations: u32) -> ([u8; 32], Vec<u8>, u32) {
	let password_bytes: &[u8] = password.as_bytes();

	let salt = generate_salt();

	let key = derive_key(password_bytes, &salt, iterations);
	(key, salt, iterations)
}

pub fn aes_encrypt(key: [u8; 32], plain_text: &str) -> (String, String) {
	let iv = generate_iv();

	let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
	let cipher_text = cipher.encrypt_vec(&plain_text.as_bytes());

	(encode(cipher_text), encode(iv))
}

pub fn aes_decrypt(hex_key: &str, hex_iv: &str, cipher_text: Vec<u8>) -> String {
	let iv = hex_to_bytes(hex_iv);
	let key = hex_to_bytes(hex_key);
	let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
	let decrypted_cipher_text = cipher.decrypt_vec(&cipher_text).unwrap();

	let plain_text = str::from_utf8(&decrypted_cipher_text).unwrap(); 
	plain_text.to_string()
}

pub fn decrypt_file<P: AsRef<Path>>(path: P, password: &[u8], hex_iv: &str) -> Vec<u8> {
    // Load AES key metadata
    let key_data_str = fs::read_to_string("aes_key_data.json").expect("Failed to read key data");
    let key_data: KeyData = serde_json::from_str(&key_data_str).expect("Failed to parse key data");

    let salt = hex_to_bytes(&key_data.salt).expect("Invalid salt");
    let derived_key = derive_key(password, &salt, key_data.iterations);
    let hex_key = hex::encode(derived_key);

    // Load ciphertext
    let cipher_text = fs::read(path).expect("Failed to read ciphertext file");

    // Decrypt using AES-256-CBC
    let decrypted = aes_decrypt(&hex_key, hex_iv, cipher_text);
    decrypted.into_bytes()
}