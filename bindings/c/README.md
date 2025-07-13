# C bindings for UUID Generator

## Overview

C header file and bindings for the Rust UUID Generator library, providing RFC 4122 and RFC 9562 compliant UUID v4 generation.

## Installation

1. Build the Rust library:
   ```bash
   cd ../..
   cargo build --release
   ```

2. Build the C example:
   ```bash
   make build
   ```

3. Run the example:
   ```bash
   make run
   ```

## Usage

### Basic Usage

```c
#include "uuid_generator.h"
#include <stdio.h>

int main() {
    uint8_t uuid[16];
    char uuid_str[37];
    
    // Generate UUID
    int result = uuid_generate_v4(uuid);
    if (result != UUID_SUCCESS) {
        printf("Error: %s\n", uuid_error_string(result));
        return 1;
    }
    
    // Convert to string
    result = uuid_to_string(uuid, uuid_str, sizeof(uuid_str));
    if (result != UUID_SUCCESS) {
        printf("Error: %s\n", uuid_error_string(result));
        return 1;
    }
    
    printf("UUID: %s\n", uuid_str);
    return 0;
}
```

### Advanced Usage

```c
#include "uuid_generator.h"
#include <stdio.h>

int main() {
    uint8_t uuid1[16], uuid2[16];
    uint8_t version, variant;
    uint8_t equal;
    
    // Generate two UUIDs
    uuid_generate_v4(uuid1);
    uuid_generate_v4(uuid2);
    
    // Get UUID info
    uuid_get_info(uuid1, &version, &variant);
    printf("Version: %d, Variant: %d\n", version, variant);
    
    // Compare UUIDs
    uuid_compare(uuid1, uuid2, &equal);
    printf("UUIDs equal: %s\n", equal ? "true" : "false");
    
    return 0;
}
```

## API Reference

### Functions

- `uuid_generate_v4(uint8_t* uuid_bytes)` - Generate UUID v4
- `uuid_to_string(const uint8_t* uuid_bytes, char* uuid_string, size_t buffer_size)` - Convert to string
- `uuid_get_info(const uint8_t* uuid_bytes, uint8_t* version, uint8_t* variant)` - Get version/variant
- `uuid_compare(const uint8_t* uuid1_bytes, const uint8_t* uuid2_bytes, uint8_t* are_equal)` - Compare UUIDs
- `uuid_error_string(int32_t error_code)` - Get error message

### Error Codes

- `UUID_SUCCESS` (0) - Operation successful
- `UUID_ENTROPY_FAILURE` (1) - Failed to generate random data
- `UUID_INVALID_PARAMETER` (2) - Invalid parameter
- `UUID_BUFFER_TOO_SMALL` (3) - Buffer too small
- `UUID_UNKNOWN_ERROR` (99) - Unknown error

### Data Types

- `uuid_error_t` - Enum of error codes
- `uint8_t` - 8-bit unsigned integer
- `int32_t` - 32-bit signed integer
- `size_t` - Size type

## Building

### Using Makefile

```bash
make build    # Build example
make run      # Build and run example
make clean    # Clean build artifacts
make help     # Show help
```

### Manual Building

```bash
# Build Rust library first
cd ../.. && cargo build --release

# Compile C program
gcc -Wall -Wextra -std=c99 -O2 -o example example.c \
    -L../../target/release -luuid_generator \
    -Wl,-rpath,../../target/release
```

## Requirements

- C99 compatible compiler (gcc, clang)
- Built Rust library (libuuid_generator.so/.dylib/uuid_generator.dll)
- POSIX-compatible system for Makefile
