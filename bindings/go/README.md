# Go bindings for UUID Generator

## Overview

Go bindings for the Rust UUID Generator library, providing RFC 4122 and RFC 9562 compliant UUID v4 generation through CGO FFI bindings.

## Installation

1. Build the Rust library:
   ```bash
   cd ../..
   cargo build --release
   ```

2. Run the example:
   ```bash
   LD_LIBRARY_PATH=../../target/release go run main.go
   ```

   Or on macOS:
   ```bash
   DYLD_LIBRARY_PATH=../../target/release go run main.go
   ```

## Usage

### Basic Usage

```go
package main

import (
    "fmt"
    "log"
)

func main() {
    // Generate a new UUID v4
    uuid, err := NewV4()
    if err != nil {
        log.Fatal(err)
    }
    
    // Convert to string
    uuidStr, err := uuid.String()
    if err != nil {
        log.Fatal(err)
    }
    
    fmt.Printf("UUID: %s\n", uuidStr)
    
    // Get properties
    version, _ := uuid.Version()
    variant, _ := uuid.Variant()
    fmt.Printf("Version: %d, Variant: %d\n", version, variant)
}
```

### Advanced Usage

```go
// Create UUID from bytes
uuid1, _ := NewV4()
bytes := uuid1.Bytes()
uuid2 := FromBytes(bytes)

// Compare UUIDs
equal, err := uuid1.Equal(uuid2)
if err != nil {
    log.Fatal(err)
}
fmt.Printf("UUIDs equal: %t\n", equal)
```

## API Reference

### Functions

- `NewV4() (*UUID, error)` - Generate a new UUID v4
- `FromBytes(bytes [16]byte) *UUID` - Create UUID from 16 bytes

### `UUID` Type

#### Methods
- `String() (string, error)` - Get string representation
- `Bytes() [16]byte` - Get raw bytes
- `Version() (uint8, error)` - Get version (4 for UUID v4)
- `Variant() (uint8, error)` - Get variant (2 for RFC 4122)
- `Equal(other *UUID) (bool, error)` - Compare with another UUID

### Error Handling

- `UUIDError` - Custom error type with code and message
- Error codes match the Rust library FFI error codes
- All methods that can fail return proper Go errors

## Requirements

- Go 1.21+
- CGO enabled
- Built Rust library (libuuid_generator.so/.dylib/.dll)
- Unix-like system with `/dev/urandom` support

## Building

```bash
# Build Rust library first
cd ../.. && cargo build --release

# Run Go example
LD_LIBRARY_PATH=../../target/release go run main.go

# Or build Go binary
LD_LIBRARY_PATH=../../target/release go build -o uuid-example main.go
```
