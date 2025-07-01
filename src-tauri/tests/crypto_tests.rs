use app::crypto_utils::calculate_merkle_root;
use app::crypto_utils::double_sha256;
use app::crypto_utils::generate_key_pair;
use app::crypto_utils::sign;
use app::crypto_utils::verify;

use hex::decode;
use hex::encode; 

#[test]
fn test_double_sha256() {
    let hash = double_sha256(b"abc");
    let expected = decode("4f8b42c22dd3729b519ba6f68d2da7cc5b2d606d05daed5ad5128cc03e6c6358").unwrap();
    assert_eq!(hash, expected);
}

#[test]
fn test_merkle_root_bytes() {
    let txs = vec![
        b"a".to_vec(),
        b"b".to_vec(),
    ];

    let root = calculate_merkle_root(txs);
    let hex = encode(&root);
    assert_eq!(hex, "b767a3a12f5f8bb1949d163c51f9a42e6bda8dcd02d50353717f73d4338b1bf0");
}

#[test]
fn test_signature_verification() {
    let message = "hello";

    let (private_key_hex, public_key_hex) = generate_key_pair();

    let (signature_hex, _) = sign(&private_key_hex, message);

    let is_valid = verify(&public_key_hex, message, &signature_hex);

    assert!(is_valid, "Signature verification failed");
}