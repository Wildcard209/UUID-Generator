//! # JNI bindings for Java integration
//!
//! This module provides JNI-compatible functions that can be called from Java.
//! The functions follow JNI naming conventions and handle JNI types.

use crate::{Uuid, UuidError};
use jni::objects::{JByteArray, JClass};
use jni::sys::{jbyteArray, jint, JNIEnv};
use std::ptr;

/// JNI function: Generate a new UUID v4
/// Java signature: private static native int nativeGenerate(byte[] uuidBytes);
#[no_mangle]
pub extern "system" fn Java_com_uuidgenerator_UuidGenerator_nativeGenerate(
    mut env: JNIEnv,
    _class: JClass,
    uuid_bytes: jbyteArray,
) -> jint {
    let byte_array = unsafe { JByteArray::from_raw(uuid_bytes) };
    
    let len = match env.get_array_length(&byte_array) {
        Ok(len) => len,
        Err(_) => return 2,
    };
    
    if len != 16 {
        return 2;
    }
    
    let uuid = match Uuid::new() {
        Ok(uuid) => uuid,
        Err(UuidError::EntropyError) => return 1,
        Err(_) => return 99,
    };
    
    let uuid_bytes_slice = uuid.as_bytes();
    
    match env.set_byte_array_region(&byte_array, 0, unsafe {
        std::slice::from_raw_parts(uuid_bytes_slice.as_ptr() as *const i8, 16)
    }) {
        Ok(_) => 0,
        Err(_) => 2,
    }
}

/// JNI function: Convert UUID bytes to string
/// Java signature: private static native int nativeToString(byte[] uuidBytes, byte[] buffer);
#[no_mangle]
pub extern "system" fn Java_com_uuidgenerator_UuidGenerator_nativeToString(
    mut env: JNIEnv,
    _class: JClass,
    uuid_bytes: jbyteArray,
    buffer: jbyteArray,
) -> jint {
    let uuid_array = unsafe { JByteArray::from_raw(uuid_bytes) };
    let buffer_array = unsafe { JByteArray::from_raw(buffer) };
    
    let uuid_len = match env.get_array_length(&uuid_array) {
        Ok(len) => len,
        Err(_) => return 2,
    };
    
    let buffer_len = match env.get_array_length(&buffer_array) {
        Ok(len) => len,
        Err(_) => return 2,
    };
    
    if uuid_len != 16 {
        return 2;
    }
    
    if buffer_len < 37 {
        return 3;
    }
    
    let mut uuid_bytes_buf = [0u8; 16];
    match env.get_byte_array_region(&uuid_array, 0, unsafe {
        std::slice::from_raw_parts_mut(uuid_bytes_buf.as_mut_ptr() as *mut i8, 16)
    }) {
        Ok(_) => {},
        Err(_) => return 2,
    }
    
    let uuid = match Uuid::from_bytes(uuid_bytes_buf) {
        Ok(uuid) => uuid,
        Err(_) => return 2,
    };
    
    let uuid_string = uuid.to_string();
    let string_bytes = uuid_string.as_bytes();
    
    let mut output_buffer = vec![0i8; 37];
    for (i, &byte) in string_bytes.iter().enumerate() {
        if i < 36 {
            output_buffer[i] = byte as i8;
        }
    }
    
    match env.set_byte_array_region(&buffer_array, 0, &output_buffer) {
        Ok(_) => 0,
        Err(_) => 2,
    }
}

/// JNI function: Get UUID info (version and variant)
/// Java signature: private static native int nativeGetInfo(byte[] uuidBytes, byte[] info);
#[no_mangle]
pub extern "system" fn Java_com_uuidgenerator_UuidGenerator_nativeGetInfo(
    mut env: JNIEnv,
    _class: JClass,
    uuid_bytes: jbyteArray,
    info: jbyteArray,
) -> jint {
    let uuid_array = unsafe { JByteArray::from_raw(uuid_bytes) };
    let info_array = unsafe { JByteArray::from_raw(info) };
    
    let uuid_len = match env.get_array_length(&uuid_array) {
        Ok(len) => len,
        Err(_) => return 2,
    };
    
    let info_len = match env.get_array_length(&info_array) {
        Ok(len) => len,
        Err(_) => return 2,
    };
    
    if uuid_len != 16 || info_len != 2 {
        return 2;
    }
    
    let mut uuid_bytes_buf = [0u8; 16];
    match env.get_byte_array_region(&uuid_array, 0, unsafe {
        std::slice::from_raw_parts_mut(uuid_bytes_buf.as_mut_ptr() as *mut i8, 16)
    }) {
        Ok(_) => {},
        Err(_) => return 2,
    }
    
    let uuid = match Uuid::from_bytes(uuid_bytes_buf) {
        Ok(uuid) => uuid,
        Err(_) => return 2,
    };
    
    let version = uuid.version() as i8;
    let variant = uuid.variant() as i8;
    
    let info_data = [version, variant];
    match env.set_byte_array_region(&info_array, 0, &info_data) {
        Ok(_) => 0,
        Err(_) => 2,
    }
}

/// JNI function: Compare two UUIDs
/// Java signature: private static native int nativeCompare(byte[] uuid1Bytes, byte[] uuid2Bytes, byte[] result);
#[no_mangle]
pub extern "system" fn Java_com_uuidgenerator_UuidGenerator_nativeCompare(
    mut env: JNIEnv,
    _class: JClass,
    uuid1_bytes: jbyteArray,
    uuid2_bytes: jbyteArray,
    result: jbyteArray,
) -> jint {
    // Convert JNI byte arrays
    let uuid1_array = unsafe { JByteArray::from_raw(uuid1_bytes) };
    let uuid2_array = unsafe { JByteArray::from_raw(uuid2_bytes) };
    let result_array = unsafe { JByteArray::from_raw(result) };
    
    // Check array lengths
    let uuid1_len = match env.get_array_length(&uuid1_array) {
        Ok(len) => len,
        Err(_) => return 2,
    };
    
    let uuid2_len = match env.get_array_length(&uuid2_array) {
        Ok(len) => len,
        Err(_) => return 2,
    };
    
    let result_len = match env.get_array_length(&result_array) {
        Ok(len) => len,
        Err(_) => return 2,
    };
    
    if uuid1_len != 16 || uuid2_len != 16 || result_len != 1 {
        return 2;
    }
    
    let mut uuid1_bytes_buf = [0u8; 16];
    let mut uuid2_bytes_buf = [0u8; 16];
    
    match env.get_byte_array_region(&uuid1_array, 0, unsafe {
        std::slice::from_raw_parts_mut(uuid1_bytes_buf.as_mut_ptr() as *mut i8, 16)
    }) {
        Ok(_) => {},
        Err(_) => return 2,
    }
    
    match env.get_byte_array_region(&uuid2_array, 0, unsafe {
        std::slice::from_raw_parts_mut(uuid2_bytes_buf.as_mut_ptr() as *mut i8, 16)
    }) {
        Ok(_) => {},
        Err(_) => return 2,
    }
    
    let uuid1 = match Uuid::from_bytes(uuid1_bytes_buf) {
        Ok(uuid) => uuid,
        Err(_) => return 2,
    };
    
    let uuid2 = match Uuid::from_bytes(uuid2_bytes_buf) {
        Ok(uuid) => uuid,
        Err(_) => return 2,
    };
    
    let are_equal = if uuid1 == uuid2 { 1i8 } else { 0i8 };
    
    match env.set_byte_array_region(&result_array, 0, &[are_equal]) {
        Ok(_) => 0,
        Err(_) => 2,
    }
}
