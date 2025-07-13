package com.uuidgenerator;

import java.util.Arrays;

/**
 * Represents a UUID with various utility methods.
 * 
 * This class provides RFC 4122 and RFC 9562 compliant UUID v4 generation
 * through JNI bindings to the Rust library.
 */
public class Uuid {
    private final byte[] bytes;

    /**
     * Initialize a UUID from bytes
     * 
     * @param bytes 16 bytes representing the UUID
     * @throws IllegalArgumentException if bytes is not exactly 16 bytes
     */
    public Uuid(byte[] bytes) {
        if (bytes == null) {
            throw new IllegalArgumentException("UUID bytes cannot be null");
        }
        if (bytes.length != 16) {
            throw new IllegalArgumentException("UUID bytes must be exactly 16 bytes");
        }
        this.bytes = Arrays.copyOf(bytes, 16);
    }

    /**
     * Get the raw bytes of the UUID
     * 
     * @return Copy of the UUID bytes
     */
    public byte[] getBytes() {
        return Arrays.copyOf(bytes, 16);
    }

    /**
     * Get the version of the UUID
     * 
     * @return Version number (should be 4 for UUID v4)
     * @throws UuidException if operation fails
     */
    public int getVersion() throws UuidException {
        byte[] info = UuidGenerator.getInfo(bytes);
        return info[0] & 0xFF;
    }

    /**
     * Get the variant of the UUID
     * 
     * @return Variant number (should be 2 for RFC 4122)
     * @throws UuidException if operation fails
     */
    public int getVariant() throws UuidException {
        byte[] info = UuidGenerator.getInfo(bytes);
        return info[1] & 0xFF;
    }

    /**
     * Get version and variant information
     * 
     * @return Array containing [version, variant]
     * @throws UuidException if operation fails
     */
    public int[] getInfo() throws UuidException {
        byte[] info = UuidGenerator.getInfo(bytes);
        return new int[] { info[0] & 0xFF, info[1] & 0xFF };
    }

    /**
     * Convert UUID to string representation
     * 
     * @return UUID in format xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
     * @throws UuidException if operation fails
     */
    @Override
    public String toString() {
        try {
            return UuidGenerator.toString(bytes);
        } catch (UuidException e) {
            throw new RuntimeException("Failed to convert UUID to string", e);
        }
    }

    /**
     * Check if this UUID equals another UUID
     * 
     * @param other Other UUID to compare
     * @return True if UUIDs are equal
     */
    public boolean equals(Uuid other) {
        if (other == null) {
            return false;
        }
        try {
            return UuidGenerator.compare(bytes, other.bytes);
        } catch (UuidException e) {
            throw new RuntimeException("Failed to compare UUIDs", e);
        }
    }

    /**
     * Check if this UUID equals another object
     * 
     * @param obj Object to compare
     * @return True if objects are equal
     */
    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        return equals((Uuid) obj);
    }

    /**
     * Get hash code for the UUID
     * 
     * @return Hash code based on UUID bytes
     */
    @Override
    public int hashCode() {
        return Arrays.hashCode(bytes);
    }
}
