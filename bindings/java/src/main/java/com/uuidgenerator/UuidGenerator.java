package com.uuidgenerator;

import com.sun.jna.Library;
import com.sun.jna.Native;
import java.nio.file.Paths;

/**
 * UUID generator providing RFC 4122 and RFC 9562 compliant UUID v4 generation
 */
public class UuidGenerator {
    
    // JNA interface for the native library
    public interface UuidGeneratorLibrary extends Library {
        int uuid_generate_v4(byte[] uuid_bytes);
        int uuid_to_string(byte[] uuid_bytes, byte[] uuid_string, int buffer_size);
        int uuid_get_info(byte[] uuid_bytes, byte[] version, byte[] variant);
        int uuid_compare(byte[] uuid1_bytes, byte[] uuid2_bytes, byte[] are_equal);
    }
    
    private static UuidGeneratorLibrary library = null;
    private static boolean libraryLoaded = false;

    static {
        loadLibrary();
    }

    /**
     * Load the native library using JNA
     */
    private static void loadLibrary() {
        if (libraryLoaded) {
            return;
        }

        try {
            library = Native.load("uuid_generator", UuidGeneratorLibrary.class);
            libraryLoaded = true;
            return;
        } catch (UnsatisfiedLinkError e) {
            // Library not found in system path
        }

        String[] possiblePaths = {
            "../../target/release/libuuid_generator.so",
            "../../target/debug/libuuid_generator.so",
            "./libuuid_generator.so",
            "target/release/libuuid_generator.so"
        };

        String osName = System.getProperty("os.name").toLowerCase();
        if (osName.contains("mac")) {
            String[] macPaths = {
                "../../target/release/libuuid_generator.dylib",
                "../../target/debug/libuuid_generator.dylib",
                "./libuuid_generator.dylib"
            };
            String[] allPaths = new String[possiblePaths.length + macPaths.length];
            System.arraycopy(possiblePaths, 0, allPaths, 0, possiblePaths.length);
            System.arraycopy(macPaths, 0, allPaths, possiblePaths.length, macPaths.length);
            possiblePaths = allPaths;
        } else if (osName.contains("win")) {
            String[] winPaths = {
                "../../target/release/uuid_generator.dll",
                "../../target/debug/uuid_generator.dll",
                "./uuid_generator.dll"
            };
            String[] allPaths = new String[possiblePaths.length + winPaths.length];
            System.arraycopy(possiblePaths, 0, allPaths, 0, possiblePaths.length);
            System.arraycopy(winPaths, 0, allPaths, possiblePaths.length, winPaths.length);
            possiblePaths = allPaths;
        }

        for (String path : possiblePaths) {
            try {
                if (Paths.get(path).toFile().exists()) {
                    library = Native.load(Paths.get(path).toAbsolutePath().toString(), UuidGeneratorLibrary.class);
                    libraryLoaded = true;
                    return;
                }
            } catch (UnsatisfiedLinkError e) {
                // Continue to next path
            }
        }

        throw new RuntimeException(
            "Could not find UUID generator library. " +
            "Please build the Rust library first with 'cargo build --release' " +
            "and ensure the shared library is accessible."
        );
    }

    /**
     * Generate a new UUID v4
     * 
     * @return A new UUID object
     * @throws UuidException if generation fails
     */
    public static Uuid generate() throws UuidException {
        byte[] uuidBytes = new byte[16];
        int result = library.uuid_generate_v4(uuidBytes);
        checkError(result);
        return new Uuid(uuidBytes);
    }

    /**
     * Create a UUID from bytes
     * 
     * @param bytes 16 bytes representing the UUID
     * @return A UUID object
     * @throws IllegalArgumentException if bytes is not exactly 16 bytes
     */
    public static Uuid fromBytes(byte[] bytes) {
        return new Uuid(bytes);
    }

    /**
     * Convert UUID bytes to string (internal use)
     * 
     * @param uuidBytes UUID bytes
     * @return UUID string
     * @throws UuidException if operation fails
     */
    static String toString(byte[] uuidBytes) throws UuidException {
        byte[] buffer = new byte[37];
        int result = library.uuid_to_string(uuidBytes, buffer, buffer.length);
        checkError(result);
        
        int length = 0;
        for (int i = 0; i < buffer.length; i++) {
            if (buffer[i] == 0) {
                length = i;
                break;
            }
        }
        return new String(buffer, 0, length);
    }

    /**
     * Get UUID info (internal use)
     * 
     * @param uuidBytes UUID bytes
     * @return Array containing [version, variant]
     * @throws UuidException if operation fails
     */
    static byte[] getInfo(byte[] uuidBytes) throws UuidException {
        byte[] version = new byte[1];
        byte[] variant = new byte[1];
        int result = library.uuid_get_info(uuidBytes, version, variant);
        checkError(result);
        return new byte[] { version[0], variant[0] };
    }

    /**
     * Compare two UUIDs (internal use)
     * 
     * @param uuid1Bytes First UUID bytes
     * @param uuid2Bytes Second UUID bytes
     * @return True if UUIDs are equal
     * @throws UuidException if operation fails
     */
    static boolean compare(byte[] uuid1Bytes, byte[] uuid2Bytes) throws UuidException {
        byte[] result = new byte[1];
        int errorCode = library.uuid_compare(uuid1Bytes, uuid2Bytes, result);
        checkError(errorCode);
        return result[0] != 0;
    }

    /**
     * Check error code and throw appropriate exception
     * 
     * @param result Error code from native function
     * @throws UuidException if error occurred
     */
    private static void checkError(int result) throws UuidException {
        switch (result) {
            case 0:
                return; // Success
            case 1:
                throw new EntropyException("Failed to generate random data from entropy source");
            case 2:
                throw new InvalidParameterException("Invalid parameter passed to UUID function");
            case 3:
                throw new BufferTooSmallException("Buffer too small for output");
            case 99:
                throw new UnknownErrorException("Unknown error occurred");
            default:
                throw new UuidException("UUID operation failed with error code " + result, result);
        }
    }
}
