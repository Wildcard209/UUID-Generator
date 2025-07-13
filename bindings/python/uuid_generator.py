"""
Python bindings for the UUID Generator library.

This module provides a Python interface to the Rust UUID generator
library through FFI bindings.
"""

import ctypes
import os
import sys
from pathlib import Path
from typing import Optional, Tuple

# Error codes from the Rust library
class UuidError(Exception):
    """Base exception for UUID generation errors."""
    pass

class EntropyError(UuidError):
    """Failed to generate random data from entropy source."""
    pass

class InvalidParameterError(UuidError):
    """Invalid parameter passed to UUID function."""
    pass

class BufferTooSmallError(UuidError):
    """Buffer too small for output."""
    pass

class UnknownError(UuidError):
    """Unknown error occurred."""
    pass

# Error code mapping
ERROR_CODES = {
    1: EntropyError,
    2: InvalidParameterError,
    3: BufferTooSmallError,
    99: UnknownError,
}

class UuidGenerator:
    """
    A Python wrapper for the Rust UUID generator library.
    
    This class provides RFC 4122 and RFC 9562 compliant UUID v4 generation
    with cryptographically secure randomness.
    """
    
    def __init__(self, library_path: Optional[str] = None):
        """
        Initialize the UUID generator.
        
        Args:
            library_path: Path to the shared library. If None, attempts to find it automatically.
        """
        if library_path is None:
            library_path = self._find_library()
        
        self._lib = ctypes.CDLL(library_path)
        self._setup_functions()
    
    def _find_library(self) -> str:
        """Find the UUID generator shared library."""
        current_dir = Path(__file__).parent
        possible_paths = [
            current_dir / "../../target/release/libuuid_generator.so",
            current_dir / "../../target/debug/libuuid_generator.so",
            current_dir / "libuuid_generator.so",
            Path("./libuuid_generator.so"),
            Path("./target/release/libuuid_generator.so"),
        ]
        
        if sys.platform == "darwin":
            possible_paths.extend([
                current_dir / "../../target/release/libuuid_generator.dylib",
                current_dir / "../../target/debug/libuuid_generator.dylib",
                current_dir / "libuuid_generator.dylib",
            ])
        elif sys.platform == "win32":
            possible_paths.extend([
                current_dir / "../../target/release/uuid_generator.dll",
                current_dir / "../../target/debug/uuid_generator.dll",
                current_dir / "uuid_generator.dll",
            ])
        
        for path in possible_paths:
            if path.exists():
                return str(path)
        
        raise FileNotFoundError(
            "Could not find UUID generator library. "
            "Please build the Rust library first with 'cargo build --release'"
        )
    
    def _setup_functions(self):
        """Setup function signatures for the C library."""
        self._lib.uuid_generate_v4.argtypes = [ctypes.POINTER(ctypes.c_uint8)]
        self._lib.uuid_generate_v4.restype = ctypes.c_int32
        
        self._lib.uuid_to_string.argtypes = [
            ctypes.POINTER(ctypes.c_uint8),
            ctypes.c_char_p,
            ctypes.c_size_t
        ]
        self._lib.uuid_to_string.restype = ctypes.c_int32
        
        self._lib.uuid_get_info.argtypes = [
            ctypes.POINTER(ctypes.c_uint8),
            ctypes.POINTER(ctypes.c_uint8),
            ctypes.POINTER(ctypes.c_uint8)
        ]
        self._lib.uuid_get_info.restype = ctypes.c_int32
        
        self._lib.uuid_compare.argtypes = [
            ctypes.POINTER(ctypes.c_uint8),
            ctypes.POINTER(ctypes.c_uint8),
            ctypes.POINTER(ctypes.c_uint8)
        ]
        self._lib.uuid_compare.restype = ctypes.c_int32
    
    def _check_error(self, result: int):
        """Check result code and raise appropriate exception."""
        if result != 0:
            error_class = ERROR_CODES.get(result, UnknownError)
            raise error_class(f"UUID operation failed with error code {result}")
    
    def generate(self) -> 'Uuid':
        """
        Generate a new UUID v4.
        
        Returns:
            A new Uuid object.
            
        Raises:
            EntropyError: If random data generation failed.
            UnknownError: If an unknown error occurred.
        """
        uuid_bytes = (ctypes.c_uint8 * 16)()
        result = self._lib.uuid_generate_v4(uuid_bytes)
        self._check_error(result)
        return Uuid(bytes(uuid_bytes), self)
    
    def from_bytes(self, uuid_bytes: bytes) -> 'Uuid':
        """
        Create a UUID from raw bytes.
        
        Args:
            uuid_bytes: 16 bytes representing the UUID.
            
        Returns:
            A Uuid object.
            
        Raises:
            ValueError: If uuid_bytes is not exactly 16 bytes.
        """
        if len(uuid_bytes) != 16:
            raise ValueError("UUID bytes must be exactly 16 bytes")
        return Uuid(uuid_bytes, self)

class Uuid:
    """
    Represents a UUID with various utility methods.
    """
    
    def __init__(self, uuid_bytes: bytes, generator: UuidGenerator):
        """
        Initialize a UUID.
        
        Args:
            uuid_bytes: 16 bytes representing the UUID.
            generator: The UuidGenerator instance.
        """
        if len(uuid_bytes) != 16:
            raise ValueError("UUID bytes must be exactly 16 bytes")
        self._bytes = uuid_bytes
        self._generator = generator
    
    @property
    def bytes(self) -> bytes:
        """Get the raw bytes of the UUID."""
        return self._bytes
    
    def __str__(self) -> str:
        """Get the string representation of the UUID."""
        uuid_array = (ctypes.c_uint8 * 16)(*self._bytes)
        buffer = ctypes.create_string_buffer(37)  # 36 chars + null terminator
        
        result = self._generator._lib.uuid_to_string(
            uuid_array, buffer, ctypes.sizeof(buffer)
        )
        self._generator._check_error(result)
        
        return buffer.value.decode('utf-8')
    
    def __repr__(self) -> str:
        """Get the representation of the UUID."""
        return f"Uuid('{str(self)}')"
    
    def __eq__(self, other) -> bool:
        """Check if two UUIDs are equal."""
        if not isinstance(other, Uuid):
            return False
        
        uuid1_array = (ctypes.c_uint8 * 16)(*self._bytes)
        uuid2_array = (ctypes.c_uint8 * 16)(*other._bytes)
        are_equal = ctypes.c_uint8()
        
        result = self._generator._lib.uuid_compare(
            uuid1_array, uuid2_array, ctypes.byref(are_equal)
        )
        self._generator._check_error(result)
        
        return bool(are_equal.value)
    
    def __hash__(self) -> int:
        """Get hash of the UUID."""
        return hash(self._bytes)
    
    def version(self) -> int:
        """Get the version of the UUID (should be 4 for UUID v4)."""
        uuid_array = (ctypes.c_uint8 * 16)(*self._bytes)
        version = ctypes.c_uint8()
        variant = ctypes.c_uint8()
        
        result = self._generator._lib.uuid_get_info(
            uuid_array, ctypes.byref(version), ctypes.byref(variant)
        )
        self._generator._check_error(result)
        
        return int(version.value)
    
    def variant(self) -> int:
        """Get the variant of the UUID (should be 2 for RFC 4122)."""
        uuid_array = (ctypes.c_uint8 * 16)(*self._bytes)
        version = ctypes.c_uint8()
        variant = ctypes.c_uint8()
        
        result = self._generator._lib.uuid_get_info(
            uuid_array, ctypes.byref(version), ctypes.byref(variant)
        )
        self._generator._check_error(result)
        
        return int(variant.value)
    
    def info(self) -> Tuple[int, int]:
        """
        Get version and variant information.
        
        Returns:
            A tuple of (version, variant).
        """
        return (self.version(), self.variant())

# Convenience functions
_default_generator = None

def get_generator() -> UuidGenerator:
    """Get the default UUID generator instance."""
    global _default_generator
    if _default_generator is None:
        _default_generator = UuidGenerator()
    return _default_generator

def uuid4() -> Uuid:
    """Generate a new UUID v4 using the default generator."""
    return get_generator().generate()

def from_bytes(uuid_bytes: bytes) -> Uuid:
    """Create a UUID from bytes using the default generator."""
    return get_generator().from_bytes(uuid_bytes)

# Example usage
if __name__ == "__main__":
    uuid = uuid4()
    print(f"Generated UUID: {uuid}")
    print(f"UUID bytes: {uuid.bytes.hex()}")
    print(f"Version: {uuid.version()}")
    print(f"Variant: {uuid.variant()}")
    
    print("\nGenerating 5 UUIDs:")
    for i in range(5):
        print(f"  {uuid4()}")
