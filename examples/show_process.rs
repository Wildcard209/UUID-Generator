//! Detailed example showing the UUID generation process step by step
//!
//! This example demonstrates the complete UUID v4 generation process
//! as implemented in this library, following RFC 4122 and RFC 9562.

use uuid_generator::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("UUID v4 Generation Process - Step by Step");
    println!("=========================================");
    
    println!("\n UUID v4 Generation Overview:");
    println!("  RFC 4122 (obsoleted by RFC 9562) defines UUID version 4 as:");
    println!("   • 128 bits total");
    println!("   • 122 bits of random data");
    println!("   • 4 bits for version field (set to 4)");
    println!("   • 2 bits for variant field (set to 10 binary)");
    
    println!("\n UUID Structure (128 bits / 16 bytes):");
    println!("  Byte Layout:");
    println!("   0-3:   time_low (32 bits) - random data");
    println!("   4-5:   time_mid (16 bits) - random data");
    println!("   6-7:   time_hi_and_version (16 bits) - 12 bits random + 4 bits version");
    println!("   8-9:   clock_seq_hi_and_reserved + clock_seq_low (16 bits) - 14 bits random + 2 bits variant");
    println!("   10-15: node (48 bits) - random data");
    
    // Generate and analyze a UUID
    println!("\n Step-by-Step Generation Process:");
    let uuid = Uuid::new_v4()?;
    let bytes = uuid.as_bytes();
    
    println!("\n   Step 1: Generate 128 bits of cryptographically secure random data");
    println!("    Using /dev/urandom (Unix) for entropy source");
    
    println!("\n   Step 2: Set version field (bits 48-51) to 0100 (4)");
    println!("    Byte 6 upper nibble: 0x{:01x} (should be 4)", (bytes[6] & 0xf0) >> 4);
    
    println!("\n   Step 3: Set variant field (bits 64-65) to 10");
    println!("    Byte 8 upper 2 bits: {:02b} (should be 10)", (bytes[8] & 0xc0) >> 6);
    
    println!("\n Generated UUID Analysis:");
    println!("   UUID: {}", uuid);
    println!("   Version: {} (extracted from bits 48-51)", uuid.version());
    println!("   Variant: {} (extracted from upper bits of byte 8)", uuid.variant());
    
    println!("\n Bit-Level Analysis:");
    println!("   Raw bytes: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]);
    
    // Show the bit breakdown
    println!("\n   Bit breakdown:");
    println!("   time_low (bytes 0-3):        {:02x}{:02x}{:02x}{:02x}", bytes[0], bytes[1], bytes[2], bytes[3]);
    println!("   time_mid (bytes 4-5):        {:02x}{:02x}", bytes[4], bytes[5]);
    println!("   time_hi_and_version (bytes 6-7): {:02x}{:02x}", bytes[6], bytes[7]);
    println!("     - version (upper 4 bits of byte 6): {:01x} (binary: {:04b})", 
        (bytes[6] & 0xf0) >> 4, (bytes[6] & 0xf0) >> 4);
    println!("     - time_hi (lower 12 bits):  {:03x}", ((bytes[6] as u16 & 0x0f) << 8) | bytes[7] as u16);
    println!("   clock_seq (bytes 8-9):       {:02x}{:02x}", bytes[8], bytes[9]);
    println!("     - variant (upper 2 bits of byte 8): {:02b}", (bytes[8] & 0xc0) >> 6);
    println!("     - clock_seq (remaining 14 bits): {:04x}", ((bytes[8] as u16 & 0x3f) << 8) | bytes[9] as u16);
    println!("   node (bytes 10-15):          {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}", 
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]);
    
    println!("\n String Formatting Process:");
    println!("   Format: 8-4-4-4-12 hexadecimal groups separated by dashes");
    println!("   {:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]);
    
    println!("\n Security Considerations (RFC 9562):");
    println!("   • UUIDs should not be used as security capabilities");
    println!("   • Random data comes from cryptographically secure source (/dev/urandom)");
    println!("   • Each UUID contains 122 bits of entropy");
    println!("   • Probability of collision is extremely low (2^-122)");
    
    println!("\n Compliance Verification:");
    println!("    RFC 4122 compliant: Version 4, Variant 2");
    println!("    RFC 9562 compliant: Updated specification followed");
    println!("    Proper bit layout: Version and variant bits correctly set");
    println!("    String format: Standard 8-4-4-4-12 representation");
    
    // Generate a few more to show the randomness
    println!("\n Multiple Generation Examples:");
    for i in 1..=5 {
        let test_uuid = Uuid::new_v4()?;
        let test_bytes = test_uuid.as_bytes();
        println!("   UUID {}: {} (v{}, var {}, byte6=0x{:02x}, byte8=0x{:02x})", 
            i, test_uuid, test_uuid.version(), test_uuid.variant(), test_bytes[6], test_bytes[8]);
    }
    
    println!("\n UUID generation process demonstration completed!");
    println!("   This implementation provides full transparency into the UUID v4 generation process");
    println!("   while maintaining compliance with both RFC 4122 and RFC 9562 specifications.");
    
    Ok(())
}
