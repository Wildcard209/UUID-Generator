//! Basic usage example of the UUID generator library
//!
//! This example shows how to generate UUIDs and access their properties.

use uuid_generator::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("UUID Generator Library - Basic Usage Example");
    println!("============================================");
    
    println!("\n1. Generating a single UUID v4:");
    let uuid = Uuid::new_v4()?;
    println!("   Generated UUID: {}", uuid);
    println!("   Version: {}", uuid.version());
    println!("   Variant: {}", uuid.variant());
    
    println!("\n2. Generating multiple UUIDs to demonstrate uniqueness:");
    for i in 1..=5 {
        let uuid = Uuid::new_v4()?;
        println!("   UUID {}: {}", i, uuid);
    }
    
    println!("\n3. Examining UUID structure:");
    let uuid = Uuid::new_v4()?;
    println!("   UUID: {}", uuid);
    println!("   Raw bytes: {:?}", uuid.as_bytes());
    println!("   Bytes in hex: {}", 
        uuid.as_bytes().iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" "));
    
    println!("\n4. Validating UUID properties (generating 10 UUIDs):");
    for i in 1..=10 {
        let uuid = Uuid::new_v4()?;
        println!("   UUID {}: {} (v{}, variant {})", 
            i, uuid, uuid.version(), uuid.variant());
        
        assert_eq!(uuid.version(), 4, "UUID should be version 4");
        assert_eq!(uuid.variant(), 2, "UUID should have RFC 4122 variant");
    }
    println!("   All UUIDs have correct version (4) and variant (2)");
    
    println!("\n5. UUID comparison:");
    let uuid1 = Uuid::new_v4()?;
    let uuid2 = Uuid::new_v4()?;
    let uuid1_copy = Uuid::from_bytes(*uuid1.as_bytes());
    
    println!("   UUID 1: {}", uuid1);
    println!("   UUID 2: {}", uuid2);
    println!("   UUID 1 copy: {}", uuid1_copy);
    println!("   UUID 1 == UUID 2: {}", uuid1 == uuid2);
    println!("   UUID 1 == UUID 1 copy: {}", uuid1 == uuid1_copy);
    
    println!("\n Basic usage example completed successfully!");
    Ok(())
}
