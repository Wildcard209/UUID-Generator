/**
 * @file uuid_generator.h
 * @brief C header file for the UUID Generator library
 * 
 * This header provides C bindings for the Rust UUID Generator library,
 * offering RFC 4122 and RFC 9562 compliant UUID v4 generation.
 */

#ifndef UUID_GENERATOR_H
#define UUID_GENERATOR_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Error codes returned by UUID functions
 */
typedef enum {
    UUID_SUCCESS = 0,           /**< Operation completed successfully */
    UUID_ENTROPY_FAILURE = 1,  /**< Failed to generate random data from entropy source */
    UUID_INVALID_PARAMETER = 2, /**< Invalid parameter (null pointer, invalid size, etc.) */
    UUID_BUFFER_TOO_SMALL = 3,  /**< Buffer too small for output */
    UUID_UNKNOWN_ERROR = 99     /**< Unknown error occurred */
} uuid_error_t;

/**
 * @brief Generate a new UUID v4
 * 
 * Generates a new RFC 4122 and RFC 9562 compliant UUID v4 with
 * cryptographically secure randomness.
 * 
 * @param uuid_bytes Pointer to a 16-byte buffer where the UUID will be written
 * @return UUID_SUCCESS on success, error code on failure
 * 
 * @note The caller must ensure that uuid_bytes points to a valid 16-byte buffer.
 * 
 * @example
 * ```c
 * uint8_t uuid[16];
 * int result = uuid_generate_v4(uuid);
 * if (result != UUID_SUCCESS) {
 *     // Handle error
 * }
 * ```
 */
int32_t uuid_generate_v4(uint8_t* uuid_bytes);

/**
 * @brief Convert UUID bytes to string representation
 * 
 * Converts a 16-byte UUID to its canonical string representation in the format:
 * xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
 * 
 * @param uuid_bytes Pointer to a 16-byte UUID
 * @param uuid_string Pointer to a buffer where the string will be written
 * @param buffer_size Size of the string buffer (must be at least 37 bytes)
 * @return UUID_SUCCESS on success, error code on failure
 * 
 * @note The caller must ensure that:
 * - uuid_bytes points to a valid 16-byte UUID
 * - uuid_string points to a valid buffer of at least buffer_size bytes
 * - buffer_size is at least 37 (36 characters + null terminator)
 * 
 * @example
 * ```c
 * uint8_t uuid[16];
 * char uuid_str[37];
 * uuid_generate_v4(uuid);
 * int result = uuid_to_string(uuid, uuid_str, sizeof(uuid_str));
 * if (result == UUID_SUCCESS) {
 *     printf("UUID: %s\n", uuid_str);
 * }
 * ```
 */
int32_t uuid_to_string(const uint8_t* uuid_bytes, char* uuid_string, size_t buffer_size);

/**
 * @brief Get UUID version and variant information
 * 
 * Extracts the version and variant fields from a UUID.
 * For UUID v4, version should be 4 and variant should be 2 (RFC 4122).
 * 
 * @param uuid_bytes Pointer to a 16-byte UUID
 * @param version Pointer to where the version will be written
 * @param variant Pointer to where the variant will be written
 * @return UUID_SUCCESS on success, error code on failure
 * 
 * @note The caller must ensure that all pointers are valid.
 * 
 * @example
 * ```c
 * uint8_t uuid[16];
 * uint8_t version, variant;
 * uuid_generate_v4(uuid);
 * int result = uuid_get_info(uuid, &version, &variant);
 * if (result == UUID_SUCCESS) {
 *     printf("Version: %d, Variant: %d\n", version, variant);
 * }
 * ```
 */
int32_t uuid_get_info(const uint8_t* uuid_bytes, uint8_t* version, uint8_t* variant);

/**
 * @brief Compare two UUIDs for equality
 * 
 * Compares two 16-byte UUIDs and determines if they are equal.
 * 
 * @param uuid1_bytes Pointer to first 16-byte UUID
 * @param uuid2_bytes Pointer to second 16-byte UUID
 * @param are_equal Pointer to where the result will be written (1 if equal, 0 if not)
 * @return UUID_SUCCESS on success, error code on failure
 * 
 * @note The caller must ensure that all pointers are valid.
 * 
 * @example
 * ```c
 * uint8_t uuid1[16], uuid2[16];
 * uint8_t equal;
 * uuid_generate_v4(uuid1);
 * uuid_generate_v4(uuid2);
 * int result = uuid_compare(uuid1, uuid2, &equal);
 * if (result == UUID_SUCCESS) {
 *     printf("UUIDs are %s\n", equal ? "equal" : "different");
 * }
 * ```
 */
int32_t uuid_compare(const uint8_t* uuid1_bytes, const uint8_t* uuid2_bytes, uint8_t* are_equal);

/**
 * @brief Get error message for error code
 * 
 * Returns a human-readable error message for the given error code.
 * 
 * @param error_code Error code returned by UUID functions
 * @return Pointer to static error message string
 * 
 * @example
 * ```c
 * int result = uuid_generate_v4(NULL);
 * if (result != UUID_SUCCESS) {
 *     printf("Error: %s\n", uuid_error_string(result));
 * }
 * ```
 */
static inline const char* uuid_error_string(int32_t error_code) {
    switch (error_code) {
        case UUID_SUCCESS:
            return "Success";
        case UUID_ENTROPY_FAILURE:
            return "Failed to generate random data from entropy source";
        case UUID_INVALID_PARAMETER:
            return "Invalid parameter";
        case UUID_BUFFER_TOO_SMALL:
            return "Buffer too small";
        case UUID_UNKNOWN_ERROR:
            return "Unknown error";
        default:
            return "Invalid error code";
    }
}

#ifdef __cplusplus
}
#endif

#endif /* UUID_GENERATOR_H */
