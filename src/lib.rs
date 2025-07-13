//! # UUID Generator Library
//!
//! A pure Rust implementation of UUID v4 generation following RFC 4122 and RFC 9562.
//! This library provides transparent UUID generation without external dependencies,
//! showing the complete process from entropy collection to final formatting.
//!
//! ## Features
//! - Pure Rust implementation with no external dependencies
//! - Cryptographically secure random number generation using system entropy
//! - RFC 4122 and RFC 9562 compliant UUID v4 generation
//! - C-compatible FFI bindings for Go integration
//! - Comprehensive test coverage
//! - Well-documented implementation showing the UUID generation process
//!
//! ## Example
//! ```rust
//! use uuid_generator::Uuid;
//!
//! // Generate a new random UUID v4
//! let uuid = Uuid::new_v4().expect("Failed to generate UUID");
//! println!("Generated UUID: {}", uuid);
//! ```

pub mod ffi;

use std::fmt;
use std::fs::File;
use std::io::Read;

/// UUID structure representing a 128-bit universally unique identifier
/// 
/// The UUID is stored in big-endian byte order as specified by RFC 4122/9562.
/// The internal layout follows the standard UUID format:
/// - time_low (32 bits)
/// - time_mid (16 bits) 
/// - time_hi_and_version (16 bits, includes 4-bit version)
/// - clock_seq_hi_and_reserved (8 bits, includes 2-bit variant)
/// - clock_seq_low (8 bits)
/// - node (48 bits)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Uuid {
    /// Internal 128-bit representation in big-endian byte order
    bytes: [u8; 16],
}

/// Errors that can occur during UUID generation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UuidError {
    /// Failed to read from system entropy source
    EntropyError(String),
    /// Invalid UUID format or data
    InvalidFormat(String),
}

impl fmt::Display for UuidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UuidError::EntropyError(msg) => write!(f, "Entropy error: {}", msg),
            UuidError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}

impl std::error::Error for UuidError {}

impl Uuid {
    /// Creates a new UUID v4 using cryptographically secure random data
    /// 
    /// This function demonstrates the complete UUID v4 generation process:
    /// 1. Collect 128 bits of cryptographically secure random data
    /// 2. Set the version field (bits 48-51) to 0b0100 (4)
    /// 3. Set the variant field (bits 64-65) to 0b10
    /// 4. Return the properly formatted UUID
    /// 
    /// # Returns
    /// - `Ok(Uuid)` - A newly generated UUID v4
    /// - `Err(UuidError)` - If entropy collection fails
    /// 
    /// # Example
    /// ```rust
    /// # use uuid_generator::Uuid;
    /// let uuid = Uuid::new_v4().expect("Failed to generate UUID");
    /// println!("New UUID: {}", uuid);
    /// ```
    pub fn new_v4() -> Result<Self, UuidError> {
        // Step 1: Generate 128 bits (16 bytes) of cryptographically secure random data
        let mut random_bytes = [0u8; 16];
        Self::fill_random_bytes(&mut random_bytes)?;
        
        // Step 2: Set version field to 4 (0b0100) in bits 48-51 (byte 6, upper 4 bits)
        // Clear the upper 4 bits and set to 0100 (4)
        random_bytes[6] = (random_bytes[6] & 0x0f) | 0x40;
        
        // Step 3: Set variant field to 10 (0b10) in bits 64-65 (byte 8, upper 2 bits)  
        // Clear the upper 2 bits and set to 10
        random_bytes[8] = (random_bytes[8] & 0x3f) | 0x80;
        
        Ok(Uuid {
            bytes: random_bytes,
        })
    }
    
    /// Fills a byte array with cryptographically secure random data from system entropy
    /// 
    /// This function demonstrates how to collect entropy without external dependencies:
    /// - On Unix-like systems: reads from /dev/urandom
    /// - Implements proper error handling for entropy collection failures
    /// 
    /// # Arguments
    /// - `buffer` - Mutable byte slice to fill with random data
    /// 
    /// # Returns
    /// - `Ok(())` - Successfully filled buffer with random data
    /// - `Err(UuidError)` - If entropy source is unavailable or fails
    fn fill_random_bytes(buffer: &mut [u8]) -> Result<(), UuidError> {
        // Use /dev/urandom for cryptographically secure random bytes
        // /dev/urandom is preferred over /dev/random as it doesn't block
        // and provides cryptographically secure pseudorandom data
        let mut file = File::open("/dev/urandom")
            .map_err(|e| UuidError::EntropyError(format!("Failed to open /dev/urandom: {}", e)))?;
            
        file.read_exact(buffer)
            .map_err(|e| UuidError::EntropyError(format!("Failed to read random bytes: {}", e)))?;
            
        Ok(())
    }
    
    /// Returns the raw bytes of the UUID in big-endian order
    /// 
    /// # Returns
    /// A 16-byte array containing the UUID in network byte order
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }
    
    /// Returns the version field of the UUID (should be 4 for UUID v4)
    /// 
    /// # Returns
    /// The version number (0-15) extracted from bits 48-51
    pub fn version(&self) -> u8 {
        (self.bytes[6] & 0xf0) >> 4
    }
    
    /// Returns the variant field of the UUID (should be 2 for RFC 4122 UUIDs)
    /// 
    /// # Returns  
    /// The variant number extracted from the upper bits of byte 8
    pub fn variant(&self) -> u8 {
        match self.bytes[8] {
            x if x & 0x80 == 0 => 0, // 0xxx - Reserved for NCS backward compatibility
            x if x & 0xc0 == 0x80 => 2, // 10xx - RFC 4122 variant (this specification)
            x if x & 0xe0 == 0xc0 => 6, // 110x - Reserved for Microsoft backward compatibility
            _ => 7, // 111x - Reserved for future definition
        }
    }
    
    /// Creates a UUID from a byte array
    /// 
    /// # Arguments
    /// - `bytes` - 16-byte array containing UUID data
    /// 
    /// # Returns
    /// A new UUID instance
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Uuid { bytes }
    }
}

impl fmt::Display for Uuid {
    /// Formats the UUID in the standard 8-4-4-4-12 hexadecimal string representation
    /// 
    /// Format: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    /// Example: 550e8400-e29b-41d4-a716-446655440000
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3],
            self.bytes[4], self.bytes[5],
            self.bytes[6], self.bytes[7],
            self.bytes[8], self.bytes[9],
            self.bytes[10], self.bytes[11], self.bytes[12], self.bytes[13], self.bytes[14], self.bytes[15]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_v4_generation() {
        let uuid = Uuid::new_v4().expect("Should generate UUID successfully");
        
        // Test version field is 4
        assert_eq!(uuid.version(), 4, "UUID version should be 4");
        
        // Test variant field is 2 (RFC 4122)
        assert_eq!(uuid.variant(), 2, "UUID variant should be 2 (RFC 4122)");
    }
    
    #[test]
    fn test_uuid_format() {
        let uuid = Uuid::new_v4().expect("Should generate UUID successfully");
        let uuid_str = format!("{}", uuid);
        
        // Test format: 8-4-4-4-12 characters with dashes
        assert_eq!(uuid_str.len(), 36, "UUID string should be 36 characters long");
        assert_eq!(uuid_str.chars().nth(8), Some('-'), "Character 8 should be dash");
        assert_eq!(uuid_str.chars().nth(13), Some('-'), "Character 13 should be dash");
        assert_eq!(uuid_str.chars().nth(18), Some('-'), "Character 18 should be dash");
        assert_eq!(uuid_str.chars().nth(23), Some('-'), "Character 23 should be dash");
        
        // Test all characters are valid hex or dashes
        for (i, c) in uuid_str.chars().enumerate() {
            if i == 8 || i == 13 || i == 18 || i == 23 {
                assert_eq!(c, '-', "Position {} should be dash", i);
            } else {
                assert!(c.is_ascii_hexdigit(), "Position {} should be hex digit, found '{}'", i, c);
            }
        }
    }
    
    #[test]
    fn test_uuid_uniqueness() {
        let uuid1 = Uuid::new_v4().expect("Should generate first UUID");
        let uuid2 = Uuid::new_v4().expect("Should generate second UUID");
        
        // UUIDs should be different (extremely high probability)
        assert_ne!(uuid1, uuid2, "Generated UUIDs should be unique");
    }
    
    #[test]
    fn test_uuid_from_bytes() {
        let bytes = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0x4d, 0xef,
                     0x81, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let uuid = Uuid::from_bytes(bytes);
        
        assert_eq!(uuid.as_bytes(), &bytes);
        assert_eq!(uuid.version(), 4); // Version extracted from byte 6
    }
    
    #[test]
    fn test_multiple_generations() {
        // Generate multiple UUIDs to test consistency
        for _ in 0..100 {
            let uuid = Uuid::new_v4().expect("Should generate UUID");
            assert_eq!(uuid.version(), 4);
            assert_eq!(uuid.variant(), 2);
        }
    }
}
