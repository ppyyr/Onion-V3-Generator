use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use sha3::{Digest, Sha3_256, Sha3_512};
use anyhow::Result;

/// Generate a new Ed25519 key pair
pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

/// Expand the secret key according to Tor's specification
pub fn expand_secret_key(secret_key: &[u8]) -> Result<Vec<u8>> {
    if secret_key.len() != 32 {
        return Err(anyhow::anyhow!("Secret key must be 32 bytes"));
    }

    let mut hasher = Sha3_512::new();
    hasher.update(secret_key);
    let hash = hasher.finalize();
    
    let mut expanded = hash.to_vec();
    
    // Apply the required bit manipulations
    expanded[0] &= 248;
    expanded[31] &= 127;
    expanded[31] |= 64;
    
    Ok(expanded)
}

/// Calculate the checksum for the onion address
pub fn calculate_checksum(public_key: &[u8]) -> Result<Vec<u8>> {
    if public_key.len() != 32 {
        return Err(anyhow::anyhow!("Public key must be 32 bytes"));
    }

    let mut hasher = Sha3_256::new();
    hasher.update(b".onion checksum");
    hasher.update(public_key);
    hasher.update(&[0x03]); // Version byte
    
    let hash = hasher.finalize();
    Ok(hash[..2].to_vec()) // Take first 2 bytes
}

/// Encode data using base32 (without padding)
pub fn base32_encode(data: &[u8]) -> String {
    base32::encode(base32::Alphabet::Rfc4648 { padding: false }, data).to_lowercase()
}

/// Encode data using base64
pub fn base64_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let (signing_key, verifying_key) = generate_keypair();
        assert_eq!(signing_key.to_bytes().len(), 32);
        assert_eq!(verifying_key.to_bytes().len(), 32);
    }

    #[test]
    fn test_expand_secret_key() {
        let secret = [0u8; 32];
        let expanded = expand_secret_key(&secret).unwrap();
        assert_eq!(expanded.len(), 64);
        
        // Check bit manipulations
        assert_eq!(expanded[0] & 7, 0); // Last 3 bits should be 0
        assert_eq!(expanded[31] & 128, 0); // MSB should be 0
        assert_eq!(expanded[31] & 64, 64); // Second MSB should be 1
    }

    #[test]
    fn test_checksum_calculation() {
        let public_key = [0u8; 32];
        let checksum = calculate_checksum(&public_key).unwrap();
        assert_eq!(checksum.len(), 2);
    }

    #[test]
    fn test_base32_encoding() {
        let data = b"hello world";
        let encoded = base32_encode(data);
        assert!(!encoded.is_empty());
        assert!(!encoded.contains('='));
    }
}
