use hex::{encode, decode};

use k256::{
    ecdsa::{signature::{Signer, Verifier}, Signature, SigningKey, VerifyingKey}, sha2, SecretKey
};

use rand::rngs::OsRng;

use serde::{Serialize, Deserialize};

use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Clone)]
struct Node {
    ipv4_address: String,
    port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
struct Input {
    amount: u64,
    // other fields like previous_output, etc.
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

// generate a new keypair and return (private_key, public_key) as hex strings.
pub fn generate_key_pair() -> (String, String) {
    let secret_key = SecretKey::random(&mut OsRng);
    let signing_key = SigningKey::from(&secret_key);
    let verifying_key = signing_key.verifying_key();

    let private_hex = encode(secret_key.to_bytes());
    let public_hex = encode(verifying_key.to_sec1_bytes());

    (private_hex, public_hex)
}

// sign a message with a hex-encoded private key and return the signature and public key.
pub fn sign(private_key_hex: &str, message: &str) -> (String, String) {
    let private_bytes = decode(private_key_hex).unwrap();
    let private_array: [u8; 32] = private_bytes.try_into().expect("Expected 32-byte private key");
    let signing_key = SigningKey::from_bytes(&private_array.into()).unwrap();
    let signature: Signature = signing_key.sign(message.as_bytes());
    let verifying_key = signing_key.verifying_key();

    (
        encode(signature.to_der()),
        encode(verifying_key.to_sec1_bytes()),
    )
}

// verify a hex signature with a hex public key and message.
pub fn verify(public_key_hex: &str, message: &str, signature_hex: &str) -> bool {
    let public_bytes = decode(public_key_hex).unwrap();
    let signature_bytes = decode(signature_hex).unwrap();
    let public_key = VerifyingKey::from_sec1_bytes(&public_bytes).unwrap();
    let signature: Signature = Signature::from_der(&signature_bytes).unwrap();

    public_key.verify(message.as_bytes(), &signature).is_ok()
}


/// Tauri command version that returns hex
pub fn double_sha256(data: &[u8]) -> Vec<u8> {
    let first_hash = Sha256::digest(data);
    let second_hash = Sha256::digest(&first_hash);
    second_hash.to_vec()
}

/// Condenses all transactions into a single hash called the 'merkle root'
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