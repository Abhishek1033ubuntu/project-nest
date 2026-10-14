use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

pub struct ModelSecurityVerifier {
    pub public_key: VerifyingKey,
}

impl ModelSecurityVerifier {
    pub fn new(public_key_hex: &str) -> Result<Self, String> {
        let key_bytes = hex::decode(public_key_hex)
            .map_err(|e| format!("Invalid public key hex format: {}", e))?;
        
        let key_array: [u8; 32] = key_bytes
            .try_into()
            .map_err(|_| "Public key must be exactly 32 bytes (64 hex characters)")?;

        let public_key = VerifyingKey::from_bytes(&key_array)
            .map_err(|e| format!("Failed to parse Ed25519 public key: {}", e))?;

        Ok(Self { public_key })
    }

    pub fn verify_onnx_binary(
        &self,
        file_path: &Path,
        expected_sha256_hex: &str,
        signature_hex: &str,
    ) -> Result<Vec<u8>, String> {
        let file_bytes = fs::read(file_path)
            .map_err(|e| format!("Failed to read model file at {:?}: {}", file_path, e))?;

        let computed_hash = Sha256::digest(&file_bytes);
        let computed_hash_hex = hex::encode(computed_hash);

        if computed_hash_hex.to_lowercase() != expected_sha256_hex.to_lowercase() {
            return Err(format!(
                "SHA-256 Checksum mismatch! Expected: {}, Computed: {}",
                expected_sha256_hex, computed_hash_hex
            ));
        }

        let sig_bytes = hex::decode(signature_hex)
            .map_err(|e| format!("Invalid signature hex format: {}", e))?;
        
        let sig_array: [u8; 64] = sig_bytes
            .try_into()
            .map_err(|_| "Ed25519 signature must be 64 bytes")?;

        let signature = Signature::from_bytes(&sig_array);

        self.public_key
            .verify(&file_bytes, &signature)
            .map_err(|_| "Cryptographic Signature Verification FAILED!".to_string())?;

        Ok(file_bytes)
    }
}
