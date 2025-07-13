# Node.js bindings for UUID Generator

## Overview

Node.js bindings for the Rust UUID Generator library, providing RFC 4122 and RFC 9562 compliant UUID v4 generation.

## Installation

1. Build the Rust library:
   ```bash
   cd ../..
   cargo build --release
   ```

2. Install Node.js dependencies:
   ```bash
   npm install
   ```

3. Run the example:
   ```bash
   node example.js
   ```

## Usage

### Basic Usage

```javascript
const { uuid4 } = require('uuid-generator-bindings');

// Generate a UUID v4
const uuid = uuid4();
console.log(`UUID: ${uuid}`);
console.log(`Version: ${uuid.version()}`);  // Should be 4
console.log(`Variant: ${uuid.variant()}`);  // Should be 2
```

### Advanced Usage

```javascript
const { UuidGenerator, fromBytes } = require('uuid-generator-bindings');

// Use custom generator
const generator = new UuidGenerator();
const uuid = generator.generate();

// Create UUID from bytes
const uuidBytes = uuid.bytes;
const newUuid = fromBytes(uuidBytes);

// Compare UUIDs
console.log(`UUIDs equal: ${uuid.equals(newUuid)}`);
```

## API Reference

### Functions

- `uuid4()` - Generate a new UUID v4
- `fromBytes(buffer)` - Create UUID from 16-byte Buffer
- `getGenerator()` - Get default generator instance

### Classes

#### `UuidGenerator`
- `generate()` - Generate new UUID
- `fromBytes(buffer)` - Create UUID from bytes

#### `Uuid`
- `bytes` - Raw 16-byte Buffer
- `version()` - Get version (4 for UUID v4)
- `variant()` - Get variant (2 for RFC 4122)
- `info()` - Get {version, variant} object
- `toString()` - String representation (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx)
- `equals(other)` - Compare with another UUID
- `toJSON()` - JSON representation

### Exceptions

- `UuidError` - Base exception
- `EntropyError` - Failed to generate random data
- `InvalidParameterError` - Invalid parameter
- `BufferTooSmallError` - Buffer too small
- `UnknownError` - Unknown error

## Requirements

- Node.js 14.0+
- Built Rust library (libuuid_generator.so/.dylib/.dll)
- ffi-napi and ref-napi packages
