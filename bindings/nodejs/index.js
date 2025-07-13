/**
 * Node.js bindings for the UUID Generator library.
 * 
 * Provides RFC 4122 and RFC 9562 compliant UUID v4 generation
 * through FFI bindings to the Rust library.
 */

const koffi = require('koffi');
const path = require('path');
const fs = require('fs');

// Error codes
const ERROR_CODES = {
    0: 'Success',
    1: 'EntropyFailure',
    2: 'InvalidParameter', 
    3: 'BufferTooSmall',
    99: 'UnknownError'
};

/**
 * Custom error classes for UUID operations
 */
class UuidError extends Error {
    constructor(message, code) {
        super(message);
        this.name = 'UuidError';
        this.code = code;
    }
}

class EntropyError extends UuidError {
    constructor(message) {
        super(message, 1);
        this.name = 'EntropyError';
    }
}

class InvalidParameterError extends UuidError {
    constructor(message) {
        super(message, 2);
        this.name = 'InvalidParameterError';
    }
}

class BufferTooSmallError extends UuidError {
    constructor(message) {
        super(message, 3);
        this.name = 'BufferTooSmallError';
    }
}

class UnknownError extends UuidError {
    constructor(message) {
        super(message, 99);
        this.name = 'UnknownError';
    }
}

/**
 * Find the UUID generator shared library
 */
function findLibrary() {
    const possiblePaths = [
        path.join(__dirname, '../../target/release/libuuid_generator.so'),
        path.join(__dirname, '../../target/debug/libuuid_generator.so'),
        path.join(__dirname, 'libuuid_generator.so'),
        './libuuid_generator.so',
        './target/release/libuuid_generator.so'
    ];

    if (process.platform === 'darwin') {
        possiblePaths.push(
            path.join(__dirname, '../../target/release/libuuid_generator.dylib'),
            path.join(__dirname, '../../target/debug/libuuid_generator.dylib'),
            path.join(__dirname, 'libuuid_generator.dylib')
        );
    } else if (process.platform === 'win32') {
        possiblePaths.push(
            path.join(__dirname, '../../target/release/uuid_generator.dll'),
            path.join(__dirname, '../../target/debug/uuid_generator.dll'),
            path.join(__dirname, 'uuid_generator.dll')
        );
    }

    for (const libraryPath of possiblePaths) {
        if (fs.existsSync(libraryPath)) {
            return libraryPath;
        }
    }

    throw new Error(
        'Could not find UUID generator library. ' +
        'Please build the Rust library first with "cargo build --release"'
    );
}

/**
 * Load the UUID generator library
 */
function loadLibrary() {
    const libraryPath = findLibrary();
    
    const lib = koffi.load(libraryPath);
    
    const uuid_generate_v4 = lib.func('uuid_generate_v4', 'int32', ['uint8 *']);
    const uuid_to_string = lib.func('uuid_to_string', 'int32', ['uint8 *', 'char *', 'size_t']);
    const uuid_get_info = lib.func('uuid_get_info', 'int32', ['uint8 *', 'uint8 *', 'uint8 *']);
    const uuid_compare = lib.func('uuid_compare', 'int32', ['uint8 *', 'uint8 *', 'uint8 *']);
    
    return {
        uuid_generate_v4,
        uuid_to_string,
        uuid_get_info,
        uuid_compare
    };
}

/**
 * Check error code and throw appropriate exception
 */
function checkError(result) {
    if (result !== 0) {
        const errorName = ERROR_CODES[result] || 'UnknownError';
        const message = `UUID operation failed: ${errorName} (code ${result})`;
        
        switch (result) {
            case 1:
                throw new EntropyError(message);
            case 2:
                throw new InvalidParameterError(message);
            case 3:
                throw new BufferTooSmallError(message);
            default:
                throw new UnknownError(message);
        }
    }
}

/**
 * UUID class representing a UUID with utility methods
 */
class Uuid {
    /**
     * Create a UUID from bytes
     * @param {Buffer} bytes - 16 bytes representing the UUID
     * @param {Object} lib - FFI functions object
     */
    constructor(bytes, lib) {
        if (!Buffer.isBuffer(bytes) || bytes.length !== 16) {
            throw new Error('UUID bytes must be a Buffer of exactly 16 bytes');
        }
        this._bytes = Buffer.from(bytes);
        this._lib = lib;
    }

    /**
     * Get the raw bytes of the UUID
     * @returns {Buffer} The UUID bytes
     */
    get bytes() {
        return Buffer.from(this._bytes);
    }

    /**
     * Get the string representation of the UUID
     * @returns {string} UUID in format xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
     */
    toString() {
        const buffer = Buffer.alloc(37);
        
        const result = this._lib.uuid_to_string(this._bytes, buffer, buffer.length);
        checkError(result);
        
        const nullIndex = buffer.indexOf(0);
        return buffer.toString('utf8', 0, nullIndex > 0 ? nullIndex : 36);
    }

    /**
     * Get the version of the UUID
     * @returns {number} Version number (should be 4 for UUID v4)
     */
    version() {
        const versionBuffer = Buffer.alloc(1);
        const variantBuffer = Buffer.alloc(1);
        
        const result = this._lib.uuid_get_info(this._bytes, versionBuffer, variantBuffer);
        checkError(result);
        
        return versionBuffer[0];
    }

    /**
     * Get the variant of the UUID
     * @returns {number} Variant number (should be 2 for RFC 4122)
     */
    variant() {
        const versionBuffer = Buffer.alloc(1);
        const variantBuffer = Buffer.alloc(1);
        
        const result = this._lib.uuid_get_info(this._bytes, versionBuffer, variantBuffer);
        checkError(result);
        
        return variantBuffer[0];
    }

    /**
     * Get version and variant information
     * @returns {Object} Object with version and variant properties
     */
    info() {
        const versionBuffer = Buffer.alloc(1);
        const variantBuffer = Buffer.alloc(1);
        
        const result = this._lib.uuid_get_info(this._bytes, versionBuffer, variantBuffer);
        checkError(result);
        
        return {
            version: versionBuffer[0],
            variant: variantBuffer[0]
        };
    }

    /**
     * Check if this UUID equals another UUID
     * @param {Uuid} other - Another UUID to compare with
     * @returns {boolean} True if UUIDs are equal
     */
    equals(other) {
        if (!(other instanceof Uuid)) {
            return false;
        }

        const resultBuffer = Buffer.alloc(1);
        
        const result = this._lib.uuid_compare(this._bytes, other._bytes, resultBuffer);
        checkError(result);
        
        return Boolean(resultBuffer[0]);
    }

    /**
     * JSON representation
     * @returns {string} UUID string
     */
    toJSON() {
        return this.toString();
    }

    /**
     * Inspect representation for console.log
     * @returns {string} Formatted representation
     */
    [Symbol.for('nodejs.util.inspect.custom')]() {
        return `Uuid { '${this.toString()}' }`;
    }
}

/**
 * UUID Generator class
 */
class UuidGenerator {
    /**
     * Create a new UUID generator
     * @param {string} [libraryPath] - Optional path to the shared library
     */
    constructor(libraryPath = null) {
        if (libraryPath) {
            const lib = koffi.load(libraryPath);
            this._lib = {
                uuid_generate_v4: lib.func('uuid_generate_v4', 'int32', ['uint8 *']),
                uuid_to_string: lib.func('uuid_to_string', 'int32', ['uint8 *', 'char *', 'size_t']),
                uuid_get_info: lib.func('uuid_get_info', 'int32', ['uint8 *', 'uint8 *', 'uint8 *']),
                uuid_compare: lib.func('uuid_compare', 'int32', ['uint8 *', 'uint8 *', 'uint8 *'])
            };
        } else {
            this._lib = loadLibrary();
        }
    }

    /**
     * Generate a new UUID v4
     * @returns {Uuid} A new UUID object
     */
    generate() {
        const uuidBytes = Buffer.alloc(16);
        
        const result = this._lib.uuid_generate_v4(uuidBytes);
        checkError(result);
        
        return new Uuid(uuidBytes, this._lib);
    }

    /**
     * Create a UUID from bytes
     * @param {Buffer|Array} bytes - 16 bytes representing the UUID
     * @returns {Uuid} A UUID object
     */
    fromBytes(bytes) {
        if (Array.isArray(bytes)) {
            bytes = Buffer.from(bytes);
        }
        return new Uuid(bytes, this._lib);
    }
}

// Default generator instance
let defaultGenerator = null;

/**
 * Get the default UUID generator instance
 * @returns {UuidGenerator} Default generator
 */
function getGenerator() {
    if (!defaultGenerator) {
        defaultGenerator = new UuidGenerator();
    }
    return defaultGenerator;
}

/**
 * Generate a new UUID v4 using the default generator
 * @returns {Uuid} A new UUID object
 */
function uuid4() {
    return getGenerator().generate();
}

/**
 * Create a UUID from bytes using the default generator
 * @param {Buffer|Array} bytes - 16 bytes representing the UUID
 * @returns {Uuid} A UUID object
 */
function fromBytes(bytes) {
    return getGenerator().fromBytes(bytes);
}

// Export everything
module.exports = {
    // Classes
    Uuid,
    UuidGenerator,
    
    // Error classes
    UuidError,
    EntropyError,
    InvalidParameterError,
    BufferTooSmallError,
    UnknownError,
    
    // Convenience functions
    uuid4,
    fromBytes,
    getGenerator,
    
    // Constants
    ERROR_CODES
};
