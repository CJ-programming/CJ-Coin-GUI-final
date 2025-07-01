use std::{
    fs,
    io,
    path::Path,
};

use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex::{decode, encode};
use k256::{
	sha2,
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    SecretKey,
};
use pbkdf2::pbkdf2_hmac_array;
use rand::{rngs::OsRng, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};


type Aes256Cbc = Cbc<Aes256, Pkcs7>;

#[derive(Serialize, Deserialize)]
pub struct AesKeyData {
    pub hmac_array: String,
    pub salt: String,
    pub iterations: u32,
}

#[derive(Serialize, Deserialize)]
pub struct KeyPairData {
    encrypted_private_key: String,
    public_key: String,
    iv: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Node {
    ipv4_address: String,
    port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
struct Input {
    amount: u64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Output {
    amount: u64,
    address: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Transaction {
    version: u32,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    signature: Option<String>,
    txid: Option<String>,
    public_key: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct BlockHeader {
    version: u32,
    prev_hash: String,
    merkle_root: String,
    timestamp: u64,
    nbits: u32,
    nonce: u64,
    hash: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Block {
    header: BlockHeader,
    txs: Vec<Transaction>,
}

type KeyData = AesKeyData;

// Utility
pub fn hex_to_bytes(hex_str: &str) -> [u8; 32] {
    decode(hex_str)
        .expect("Unable to decode hex string.")
        .try_into()
        .expect("Expected a 32-byte vector")
}

fn generate_random_bytes(size: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; size];
    thread_rng().fill(&mut buffer[..]);
    buffer
}

fn generate_salt() -> Vec<u8> {
    generate_random_bytes(8)
}

fn generate_iv() -> Vec<u8> {
    generate_random_bytes(16)
}

fn derive_key(password: &[u8], salt: &[u8], iterations: u32) -> [u8; 32] {
    pbkdf2_hmac_array::<Sha256, 32>(password, salt, iterations)
}

// Key Generation
pub fn generate_key(password: &str, iterations: u32) -> ([u8; 32], Vec<u8>, u32) {
    let password_bytes = password.as_bytes();
    let salt = generate_salt();
    let key = derive_key(password_bytes, &salt, iterations);
    (key, salt, iterations)
}

#[tauri::command]
pub fn generate_aes_key_to_file(path: &str, password: &str, iterations: u32) -> Result<(), String> {
    let (key, salt, iterations) = generate_key(password, iterations);

    let data = AesKeyData {
        hmac_array: encode(key),
        salt: encode(&salt),
        iterations,
    };

    let json = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn load_aes_key_from_file(path: &str) -> io::Result<(String, String, u32)> {
    let content = fs::read_to_string(path)?;
    let data: AesKeyData = serde_json::from_str(&content)?;

    Ok((data.hmac_array, data.salt, data.iterations))
}

// AES Encryption
pub fn aes_encrypt(key: [u8; 32], plain_text: &str) -> (String, String) {
    let iv = generate_iv();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
    let cipher_text = cipher.encrypt_vec(plain_text.as_bytes());

    (encode(cipher_text), encode(iv))
}

pub fn aes_decrypt(hex_key: &str, hex_iv: &str, cipher_text: Vec<u8>) -> String {
    let key = hex_to_bytes(hex_key);
    let iv = hex_to_bytes(hex_iv);

    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
    let decrypted = cipher.decrypt_vec(&cipher_text).unwrap();

    String::from_utf8(decrypted).unwrap()
}

#[tauri::command]
pub fn decrypt_file(aes_key_data_path: String, key_pair_data_path: String, password: Vec<u8>) -> Vec<u8> {
    // load key data for salt and iterations
    let key_data_str = fs::read_to_string(&aes_key_data_path).expect("Failed to read key data");
    let key_data: KeyData = serde_json::from_str(&key_data_str).expect("Failed to parse key data");

    // load KeyPairData for IV
    let keypair_data_str = fs::read_to_string(key_pair_data_path).expect("Failed to read keypair data");
    let keypair_data: KeyPairData = serde_json::from_str(&keypair_data_str).expect("Failed to parse keypair data");
    let hex_iv = &keypair_data.iv;

    // derive key
    let salt = hex_to_bytes(&key_data.salt);
    let derived_key = derive_key(&password, &salt, key_data.iterations);
    let hex_key = encode(derived_key);

    // reads ciphertext
    let cipher_text = fs::read(&aes_key_data_path).expect("Failed to read ciphertext file");

    aes_decrypt(&hex_key, hex_iv, cipher_text).into_bytes()
}

/// generates private and public cryptographic keys
#[tauri::command]
pub fn generate_key_pair_to_file(path: &str, hex_key: &str) -> Result<(), String> {
    let vec_key = decode(hex_key).map_err(|e| e.to_string())?;
    let key: [u8; 32] = vec_key.try_into().map_err(|_| "Expected a 32-byte vector".to_string())?;

    let (private_key, public_key) = generate_key_pair();
    let (encrypted_private_key, iv) = aes_encrypt(key, &private_key);

    let data = KeyPairData {
        encrypted_private_key,
        public_key,
        iv,
    };

    let json = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;

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

pub fn generate_key_pair() -> (String, String) {
    let secret_key = SecretKey::random(&mut OsRng);
    let signing_key = SigningKey::from(&secret_key);
    let verifying_key = signing_key.verifying_key();

    (encode(secret_key.to_bytes()), encode(verifying_key.to_sec1_bytes()))
}

pub fn sign(private_key_hex: &str, message: &str) -> (String, String) {
    let private_bytes = decode(private_key_hex).unwrap();
    let private_array: [u8; 32] = private_bytes.try_into().expect("Expected 32-byte private key");

    let signing_key = SigningKey::from_bytes(&private_array.into()).unwrap();
    let signature: Signature = signing_key.sign(message.as_bytes());
    let verifying_key = signing_key.verifying_key();

    (encode(signature.to_der()), encode(verifying_key.to_sec1_bytes()))
}

pub fn verify(public_key_hex: &str, message: &str, signature_hex: &str) -> bool {
    let public_bytes = decode(public_key_hex).unwrap();
    let signature_bytes = decode(signature_hex).unwrap();

    let public_key = VerifyingKey::from_sec1_bytes(&public_bytes).unwrap();
    let signature = Signature::from_der(&signature_bytes).unwrap();

    public_key.verify(message.as_bytes(), &signature).is_ok()
}

// Hashing
pub fn double_sha256(data: &[u8]) -> Vec<u8> {
    let first_hash = Sha256::digest(data);
    Sha256::digest(&first_hash).to_vec()
}

pub fn calculate_merkle_root(txs: Vec<Vec<u8>>) -> Vec<u8> {
    if txs.is_empty() {
        return vec![0u8; 32];
    }

    if txs.len() == 1 {
        return double_sha256(&txs[0]);
    }

    let mut new_tx_list = Vec::new();
    let mut i = 0;

    while i < txs.len() {
        let tx_hash1 = double_sha256(&txs[i]);
        let tx_hash2 = if i + 1 < txs.len() {
            double_sha256(&txs[i + 1])
        } else {
            tx_hash1.clone()
        };

        let mut combined = tx_hash1.clone();
        combined.extend(tx_hash2);
        new_tx_list.push(combined);

        i += 2;
    }

    calculate_merkle_root(new_tx_list)
}