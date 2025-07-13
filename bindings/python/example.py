#!/usr/bin/env python3
"""
Example usage of the Python UUID generator bindings.
"""

import sys
import os

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from uuid_generator import uuid4, UuidGenerator, from_bytes

def main():
    print("UUID Generator - Python Example")
    print("=" * 40)
    
    print("\n1. Basic UUID Generation:")
    uuid = uuid4()
    print(f"   Generated UUID: {uuid}")
    print(f"   UUID type: {type(uuid)}")
    print(f"   Version: {uuid.version()}")
    print(f"   Variant: {uuid.variant()}")
    
    print("\n2. Multiple UUID Generation:")
    for i in range(5):
        print(f"   UUID {i+1}: {uuid4()}")
    
    print("\n3. UUID Properties:")
    uuid = uuid4()
    print(f"   UUID: {uuid}")
    print(f"   String representation: {str(uuid)}")
    print(f"   Raw bytes: {uuid.bytes.hex()}")
    print(f"   Raw bytes length: {len(uuid.bytes)}")
    print(f"   Version and variant: {uuid.info()}")
    
    print("\n4. UUID Comparison:")
    uuid1 = uuid4()
    uuid2 = uuid4()
    uuid3 = from_bytes(uuid1.bytes)
    
    print(f"   UUID1: {uuid1}")
    print(f"   UUID2: {uuid2}")
    print(f"   UUID3: {uuid3} (created from UUID1 bytes)")
    print(f"   UUID1 == UUID2: {uuid1 == uuid2}")
    print(f"   UUID1 == UUID3: {uuid1 == uuid3}")
    
    print("\n5. Custom Generator:")
    try:
        generator = UuidGenerator()
        custom_uuid = generator.generate()
        print(f"   Custom UUID: {custom_uuid}")
    except Exception as e:
        print(f"   Error: {e}")
        print("   Make sure to build the Rust library first!")
    
    print("\n6. Error Handling:")
    try:
        invalid_uuid = from_bytes(b"too_short")
    except ValueError as e:
        print(f"   Caught expected error: {e}")
    
    print("\nDone!")

if __name__ == "__main__":
    main()
