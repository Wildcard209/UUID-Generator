package com.uuidgenerator;

/**
 * Exception thrown when UUID operations fail
 */
public class UuidException extends Exception {
    private final int errorCode;

    public UuidException(String message, int errorCode) {
        super(message);
        this.errorCode = errorCode;
    }

    public int getErrorCode() {
        return errorCode;
    }
}

/**
 * Exception thrown when entropy generation fails
 */
class EntropyException extends UuidException {
    public EntropyException(String message) {
        super(message, 1);
    }
}

/**
 * Exception thrown when invalid parameters are passed
 */
class InvalidParameterException extends UuidException {
    public InvalidParameterException(String message) {
        super(message, 2);
    }
}

/**
 * Exception thrown when buffer is too small
 */
class BufferTooSmallException extends UuidException {
    public BufferTooSmallException(String message) {
        super(message, 3);
    }
}

/**
 * Exception thrown for unknown errors
 */
class UnknownErrorException extends UuidException {
    public UnknownErrorException(String message) {
        super(message, 99);
    }
}
