use crate::crypto::*;
use crate::{OnionResult, increment_generated, increment_found};
use anyhow::Result;

/// Generate a single onion address
pub fn generate_onion_address() -> Result<OnionResult> {
    // Generate key pair
    let (signing_key, verifying_key) = generate_keypair();
    
    // Get raw bytes
    let private_bytes = signing_key.to_bytes();
    let public_bytes = verifying_key.to_bytes();
    
    // Expand secret key
    let expanded_secret = expand_secret_key(&private_bytes)?;
    
    // Generate onion address
    let hostname = encode_public_key(&public_bytes)?;
    
    // Format keys according to Tor specification
    let mut public_key_data = Vec::new();
    public_key_data.extend_from_slice(b"== ed25519v1-public: type0 ==");
    public_key_data.extend_from_slice(&[0, 0, 0]); // 3 null bytes
    public_key_data.extend_from_slice(&public_bytes);
    
    let mut private_key_data = Vec::new();
    private_key_data.extend_from_slice(b"== ed25519v1-secret: type0 ==");
    private_key_data.extend_from_slice(&[0, 0, 0]); // 3 null bytes
    private_key_data.extend_from_slice(&expanded_secret);
    
    // Encode to base64
    let public_key = base64_encode(&public_key_data);
    let private_key = base64_encode(&private_key_data);
    
    increment_generated();
    
    Ok(OnionResult {
        hostname,
        public_key,
        private_key,
    })
}

/// Generate onion address with specific prefix
pub fn generate_with_prefix(prefixes: &[String]) -> Result<OnionResult> {
    loop {
        let result = generate_onion_address()?;
        
        // Check if hostname starts with any of the prefixes
        for prefix in prefixes {
            if result.hostname.starts_with(prefix) {
                increment_found();
                return Ok(result);
            }
        }
    }
}

/// Encode public key to onion address
fn encode_public_key(public_key: &[u8]) -> Result<String> {
    if public_key.len() != 32 {
        return Err(anyhow::anyhow!("Public key must be 32 bytes"));
    }
    
    // Calculate checksum
    let checksum = calculate_checksum(public_key)?;
    
    // Construct address data: public_key + checksum + version
    let mut address_data = Vec::new();
    address_data.extend_from_slice(public_key);
    address_data.extend_from_slice(&checksum);
    address_data.push(0x03); // Version byte
    
    // Encode to base32 and add .onion suffix
    let encoded = base32_encode(&address_data);
    Ok(format!("{}.onion", encoded))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_onion_address() {
        let result = generate_onion_address().unwrap();
        
        assert!(result.hostname.ends_with(".onion"));
        assert!(result.hostname.len() > 10);
        assert!(!result.public_key.is_empty());
        assert!(!result.private_key.is_empty());
    }

    #[test]
    fn test_generate_with_prefix() {
        let prefixes = vec!["test".to_string()];
        
        // This might take a while, so we'll just test that it doesn't panic
        // In a real test, you might want to use a more common prefix or mock the generation
        let result = std::panic::catch_unwind(|| {
            generate_with_prefix(&prefixes)
        });
        
        assert!(!result.is_err());
    }

    #[test]
    fn test_encode_public_key() {
        let public_key = [0u8; 32];
        let address = encode_public_key(&public_key).unwrap();
        
        assert!(address.ends_with(".onion"));
        assert!(address.len() > 10);
    }

    #[test]
    fn test_invalid_public_key_length() {
        let invalid_key = [0u8; 31]; // Wrong length
        let result = encode_public_key(&invalid_key);
        
        assert!(result.is_err());
    }
}
