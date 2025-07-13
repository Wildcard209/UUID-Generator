# C# bindings for UUID Generator

## Overview

C# bindings for the Rust UUID Generator library, providing RFC 4122 and RFC 9562 compliant UUID v4 generation.

## Installation

1. Build the Rust library:
   ```bash
   cd ../..
   cargo build --release
   ```

2. Build the C# project:
   ```bash
   dotnet build
   ```

3. Run the example:
   ```bash
   dotnet run
   ```

## Usage

### Basic Usage

```csharp
using UuidGenerator;

// Generate a UUID v4
var uuid = UuidGenerator.Generate();
Console.WriteLine($"UUID: {uuid}");
Console.WriteLine($"Version: {uuid.Version}");  // Should be 4
Console.WriteLine($"Variant: {uuid.Variant}");  // Should be 2
```

### Advanced Usage

```csharp
using UuidGenerator;

// Create UUID from bytes
var uuid = UuidGenerator.Generate();
var uuidBytes = uuid.Bytes;
var newUuid = UuidGenerator.FromBytes(uuidBytes);

// Compare UUIDs
Console.WriteLine($"UUIDs equal: {uuid == newUuid}");
Console.WriteLine($"UUIDs equal (method): {uuid.Equals(newUuid)}");

// Get version and variant info
var (version, variant) = uuid.GetInfo();
Console.WriteLine($"Version: {version}, Variant: {variant}");
```

## API Reference

### Static Methods

- `UuidGenerator.Generate()` - Generate a new UUID v4
- `UuidGenerator.FromBytes(byte[])` - Create UUID from 16 bytes

### `Uuid` Class

#### Properties
- `Bytes` - Raw 16-byte array (get-only)
- `Version` - Get version (4 for UUID v4)
- `Variant` - Get variant (2 for RFC 4122)

#### Methods
- `ToString()` - String representation (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx)
- `Equals(Uuid)` - Compare with another UUID
- `Equals(object)` - Compare with any object
- `GetHashCode()` - Get hash code
- `GetInfo()` - Get (version, variant) tuple

#### Operators
- `==` and `!=` - Equality operators

### Exceptions

- `UuidException` - Base exception
- `EntropyException` - Failed to generate random data
- `InvalidParameterException` - Invalid parameter
- `BufferTooSmallException` - Buffer too small
- `UnknownErrorException` - Unknown error

## Requirements

- .NET 6.0+
- Built Rust library (libuuid_generator.so/.dylib/uuid_generator.dll)
- Platform with P/Invoke support
