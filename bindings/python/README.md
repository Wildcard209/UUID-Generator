# Python bindings for UUID Generator

## Overview

Python bindings for the Rust UUID Generator library, providing RFC 4122 and RFC 9562 compliant UUID v4 generation.

## Installation

1. Build the Rust library:
   ```bash
   cd ../..
   cargo build --release
   ```

2. Install Python dependencies (none required - uses only standard library)

3. Run the example:
   ```bash
   python example.py
   ```

## Usage

### Basic Usage

```python
from uuid_generator import uuid4

# Generate a UUID v4
uuid = uuid4()
print(f"UUID: {uuid}")
print(f"Version: {uuid.version()}")  # Should be 4
print(f"Variant: {uuid.variant()}")  # Should be 2
```

### Advanced Usage

```python
from uuid_generator import UuidGenerator, from_bytes

# Use custom generator
generator = UuidGenerator()
uuid = generator.generate()

# Create UUID from bytes
uuid_bytes = uuid.bytes
new_uuid = from_bytes(uuid_bytes)

# Compare UUIDs
print(f"UUIDs equal: {uuid == new_uuid}")
```

## API Reference

### Functions

- `uuid4()` - Generate a new UUID v4
- `from_bytes(bytes)` - Create UUID from 16 bytes
- `get_generator()` - Get default generator instance

### Classes

#### `UuidGenerator`
- `generate()` - Generate new UUID
- `from_bytes(bytes)` - Create UUID from bytes

#### `Uuid`
- `bytes` - Raw 16-byte representation
- `version()` - Get version (4 for UUID v4)
- `variant()` - Get variant (2 for RFC 4122)
- `info()` - Get (version, variant) tuple
- `__str__()` - String representation (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx)

### Exceptions

- `UuidError` - Base exception
- `EntropyError` - Failed to generate random data
- `InvalidParameterError` - Invalid parameter
- `BufferTooSmallError` - Buffer too small
- `UnknownError` - Unknown error

## Requirements

- Python 3.6+
- Built Rust library (libuuid_generator.so/.dylib/.dll)
- No external Python dependencies
