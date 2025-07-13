//! # Foreign Function Interface (FFI) for Go Integration
//!
//! This module provides C-compatible functions that can be called from Go.
//! All functions are marked as `extern "C"` and use C-compatible types.
//!
//! ## Memory Safety
//! - All pointers passed from Go must be valid for the duration of the call
//! - The caller is responsible for allocating sufficient memory for output buffers
//! - Functions return error codes to indicate success/failure
//!
//! ## Go Integration Example
//! ```go
//! /*
//! #cgo LDFLAGS: -L. -luuid_generator
//! #include <stdint.h>
//! 
//! int32_t uuid_generate_v4(uint8_t* uuid_bytes);
//! int32_t uuid_to_string(const uint8_t* uuid_bytes, char* uuid_string, size_t buffer_size);
//! */
//! import "C"
//! import "unsafe"
//! 
//! func GenerateUUID() (string, error) {
//!     var uuidBytes [16]C.uint8_t
//!     result := C.uuid_generate_v4(&uuidBytes[0])
//!     if result != 0 {
//!         return "", fmt.Errorf("failed to generate UUID: error code %d", result)
//!     }
//!     
//!     var buffer [37]C.char // 36 chars + null terminator
//!     result = C.uuid_to_string(&uuidBytes[0], &buffer[0], 37)
//!     if result != 0 {
//!         return "", fmt.Errorf("failed to convert UUID to string: error code %d", result)
//!     }
//!     
//!     return C.GoString(&buffer[0]), nil
//! }
//! ```

use crate::{Uuid, UuidError};
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::slice;

/// Error codes returned by FFI functions
#[repr(C)]
pub enum UuidFfiError {
    /// Success
    Success = 0,
    /// Failed to generate random data from entropy source
    EntropyFailure = 1,
    /// Invalid parameter (null pointer, invalid size, etc.)
    InvalidParameter = 2,
    /// Buffer too small for output
    BufferTooSmall = 3,
    /// Unknown error
    UnknownError = 99,
}

/// Generates a new UUID v4 and writes the bytes to the provided buffer
///
/// # Parameters
/// - `uuid_bytes`: Pointer to a 16-byte buffer where the UUID will be written
///
/// # Returns
/// - `0` (Success) if UUID was generated successfully
/// - `1` (EntropyFailure) if random data generation failed
/// - `2` (InvalidParameter) if uuid_bytes is null
///
/// # Safety
/// The caller must ensure that `uuid_bytes` points to a valid 16-byte buffer.
#[no_mangle]
pub extern "C" fn uuid_generate_v4(uuid_bytes: *mut u8) -> c_int {
    if uuid_bytes.is_null() {
        return UuidFfiError::InvalidParameter as c_int;
    }

    match Uuid::new_v4() {
        Ok(uuid) => {
            unsafe {
                let buffer = slice::from_raw_parts_mut(uuid_bytes, 16);
                buffer.copy_from_slice(uuid.as_bytes());
            }
            UuidFfiError::Success as c_int
        }
        Err(UuidError::EntropyError(_)) => UuidFfiError::EntropyFailure as c_int,
        Err(_) => UuidFfiError::UnknownError as c_int,
    }
}

/// Converts UUID bytes to a null-terminated string representation
///
/// # Parameters
/// - `uuid_bytes`: Pointer to a 16-byte UUID
/// - `uuid_string`: Pointer to a buffer where the string will be written
/// - `buffer_size`: Size of the string buffer (must be at least 37 bytes)
///
/// # Returns
/// - `0` (Success) if conversion was successful
/// - `2` (InvalidParameter) if any pointer is null
/// - `3` (BufferTooSmall) if buffer_size < 37
///
/// # Safety
/// The caller must ensure that:
/// - `uuid_bytes` points to a valid 16-byte UUID
/// - `uuid_string` points to a valid buffer of at least `buffer_size` bytes
/// - `buffer_size` is accurate
#[no_mangle]
pub extern "C" fn uuid_to_string(
    uuid_bytes: *const u8,
    uuid_string: *mut c_char,
    buffer_size: usize,
) -> c_int {
    if uuid_bytes.is_null() || uuid_string.is_null() {
        return UuidFfiError::InvalidParameter as c_int;
    }

    if buffer_size < 37 {
        return UuidFfiError::BufferTooSmall as c_int;
    }

    unsafe {
        let uuid_bytes_slice = slice::from_raw_parts(uuid_bytes, 16);
        let mut uuid_array = [0u8; 16];
        uuid_array.copy_from_slice(uuid_bytes_slice);
        
        let uuid = Uuid::from_bytes(uuid_array);
        let uuid_str = format!("{}", uuid);
        
        let uuid_cstring = match std::ffi::CString::new(uuid_str) {
            Ok(s) => s,
            Err(_) => return UuidFfiError::UnknownError as c_int,
        };
        
        let uuid_bytes = uuid_cstring.as_bytes_with_nul();
        if uuid_bytes.len() > buffer_size {
            return UuidFfiError::BufferTooSmall as c_int;
        }
        
        ptr::copy_nonoverlapping(
            uuid_bytes.as_ptr() as *const c_char,
            uuid_string,
            uuid_bytes.len(),
        );
    }

    UuidFfiError::Success as c_int
}

/// Validates UUID bytes and returns version and variant information
///
/// # Parameters
/// - `uuid_bytes`: Pointer to a 16-byte UUID
/// - `version`: Pointer to where the version will be written
/// - `variant`: Pointer to where the variant will be written
///
/// # Returns
/// - `0` (Success) if validation was successful
/// - `2` (InvalidParameter) if any pointer is null
///
/// # Safety
/// The caller must ensure that all pointers are valid.
#[no_mangle]
pub extern "C" fn uuid_get_info(
    uuid_bytes: *const u8,
    version: *mut u8,
    variant: *mut u8,
) -> c_int {
    if uuid_bytes.is_null() || version.is_null() || variant.is_null() {
        return UuidFfiError::InvalidParameter as c_int;
    }

    unsafe {
        let uuid_bytes_slice = slice::from_raw_parts(uuid_bytes, 16);
        let mut uuid_array = [0u8; 16];
        uuid_array.copy_from_slice(uuid_bytes_slice);
        
        let uuid = Uuid::from_bytes(uuid_array);
        
        *version = uuid.version();
        *variant = uuid.variant();
    }

    UuidFfiError::Success as c_int
}

/// Compares two UUIDs for equality
///
/// # Parameters
/// - `uuid1_bytes`: Pointer to first 16-byte UUID
/// - `uuid2_bytes`: Pointer to second 16-byte UUID
/// - `are_equal`: Pointer to where the result will be written (1 if equal, 0 if not)
///
/// # Returns
/// - `0` (Success) if comparison was successful
/// - `2` (InvalidParameter) if any pointer is null
///
/// # Safety
/// The caller must ensure that all pointers are valid.
#[no_mangle]
pub extern "C" fn uuid_compare(
    uuid1_bytes: *const u8,
    uuid2_bytes: *const u8,
    are_equal: *mut u8,
) -> c_int {
    if uuid1_bytes.is_null() || uuid2_bytes.is_null() || are_equal.is_null() {
        return UuidFfiError::InvalidParameter as c_int;
    }

    unsafe {
        let uuid1_slice = slice::from_raw_parts(uuid1_bytes, 16);
        let uuid2_slice = slice::from_raw_parts(uuid2_bytes, 16);
        
        let mut uuid1_array = [0u8; 16];
        let mut uuid2_array = [0u8; 16];
        uuid1_array.copy_from_slice(uuid1_slice);
        uuid2_array.copy_from_slice(uuid2_slice);
        
        let uuid1 = Uuid::from_bytes(uuid1_array);
        let uuid2 = Uuid::from_bytes(uuid2_array);
        
        *are_equal = if uuid1 == uuid2 { 1 } else { 0 };
    }

    UuidFfiError::Success as c_int
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_ffi_uuid_generate_v4() {
        let mut uuid_bytes = [0u8; 16];
        let result = uuid_generate_v4(uuid_bytes.as_mut_ptr());
        
        assert_eq!(result, UuidFfiError::Success as c_int);
        
        assert!(uuid_bytes.iter().any(|&b| b != 0));
        
        let uuid = Uuid::from_bytes(uuid_bytes);
        assert_eq!(uuid.version(), 4);
        assert_eq!(uuid.variant(), 2);
    }

    #[test]
    fn test_ffi_uuid_generate_v4_null_pointer() {
        let result = uuid_generate_v4(ptr::null_mut());
        assert_eq!(result, UuidFfiError::InvalidParameter as c_int);
    }

    #[test]
    fn test_ffi_uuid_to_string() {
        let mut uuid_bytes = [0u8; 16];
        let gen_result = uuid_generate_v4(uuid_bytes.as_mut_ptr());
        assert_eq!(gen_result, UuidFfiError::Success as c_int);

        let mut buffer = [0i8; 37];
        let result = uuid_to_string(
            uuid_bytes.as_ptr(),
            buffer.as_mut_ptr(),
            buffer.len(),
        );
        
        assert_eq!(result, UuidFfiError::Success as c_int);
        
        let c_str = unsafe { CStr::from_ptr(buffer.as_ptr()) };
        let uuid_str = c_str.to_str().unwrap();
        assert_eq!(uuid_str.len(), 36);
        assert_eq!(uuid_str.chars().nth(8), Some('-'));
        assert_eq!(uuid_str.chars().nth(13), Some('-'));
        assert_eq!(uuid_str.chars().nth(18), Some('-'));
        assert_eq!(uuid_str.chars().nth(23), Some('-'));
    }

    #[test]
    fn test_ffi_uuid_to_string_buffer_too_small() {
        let uuid_bytes = [0u8; 16];
        let mut buffer = [0i8; 36];
        
        let result = uuid_to_string(
            uuid_bytes.as_ptr(),
            buffer.as_mut_ptr(),
            buffer.len(),
        );
        
        assert_eq!(result, UuidFfiError::BufferTooSmall as c_int);
    }

    #[test]
    fn test_ffi_uuid_get_info() {
        let mut uuid_bytes = [0u8; 16];
        let gen_result = uuid_generate_v4(uuid_bytes.as_mut_ptr());
        assert_eq!(gen_result, UuidFfiError::Success as c_int);

        let mut version = 0u8;
        let mut variant = 0u8;
        
        let result = uuid_get_info(
            uuid_bytes.as_ptr(),
            &mut version,
            &mut variant,
        );
        
        assert_eq!(result, UuidFfiError::Success as c_int);
        assert_eq!(version, 4);
        assert_eq!(variant, 2);
    }

    #[test]
    fn test_ffi_uuid_compare() {
        let mut uuid1_bytes = [0u8; 16];
        let mut uuid2_bytes = [0u8; 16];
        
        uuid_generate_v4(uuid1_bytes.as_mut_ptr());
        uuid_generate_v4(uuid2_bytes.as_mut_ptr());
        
        let mut are_equal = 0u8;
        let result = uuid_compare(
            uuid1_bytes.as_ptr(),
            uuid2_bytes.as_ptr(),
            &mut are_equal,
        );
        
        assert_eq!(result, UuidFfiError::Success as c_int);
        assert_eq!(are_equal, 0);
        
        let result = uuid_compare(
            uuid1_bytes.as_ptr(),
            uuid1_bytes.as_ptr(),
            &mut are_equal,
        );
        
        assert_eq!(result, UuidFfiError::Success as c_int);
        assert_eq!(are_equal, 1);
    }
}
