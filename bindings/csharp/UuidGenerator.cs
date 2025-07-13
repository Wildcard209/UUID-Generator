using System;
using System.Runtime.InteropServices;
using System.Text;

namespace UuidGenerator
{
    /// <summary>
    /// Exception thrown when UUID operations fail
    /// </summary>
    public class UuidException : Exception
    {
        public int ErrorCode { get; }

        public UuidException(string message, int errorCode) : base(message)
        {
            ErrorCode = errorCode;
        }
    }

    /// <summary>
    /// Exception thrown when entropy generation fails
    /// </summary>
    public class EntropyException : UuidException
    {
        public EntropyException(string message) : base(message, 1) { }
    }

    /// <summary>
    /// Exception thrown when invalid parameters are passed
    /// </summary>
    public class InvalidParameterException : UuidException
    {
        public InvalidParameterException(string message) : base(message, 2) { }
    }

    /// <summary>
    /// Exception thrown when buffer is too small
    /// </summary>
    public class BufferTooSmallException : UuidException
    {
        public BufferTooSmallException(string message) : base(message, 3) { }
    }

    /// <summary>
    /// Exception thrown for unknown errors
    /// </summary>
    public class UnknownErrorException : UuidException
    {
        public UnknownErrorException(string message) : base(message, 99) { }
    }

    /// <summary>
    /// Native methods imported from the Rust UUID generator library
    /// </summary>
    internal static class NativeMethods
    {
        private const string LibraryName = "uuid_generator";

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int uuid_generate_v4(byte[] uuidBytes);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int uuid_to_string(byte[] uuidBytes, StringBuilder uuidString, IntPtr bufferSize);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int uuid_get_info(byte[] uuidBytes, out byte version, out byte variant);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        internal static extern int uuid_compare(byte[] uuid1Bytes, byte[] uuid2Bytes, out byte areEqual);
    }

    /// <summary>
    /// Represents a UUID with various utility methods
    /// </summary>
    public sealed class Uuid : IEquatable<Uuid>
    {
        private readonly byte[] _bytes;

        /// <summary>
        /// Initialize a UUID from bytes
        /// </summary>
        /// <param name="bytes">16 bytes representing the UUID</param>
        public Uuid(byte[] bytes)
        {
            if (bytes == null)
                throw new ArgumentNullException(nameof(bytes));
            if (bytes.Length != 16)
                throw new ArgumentException("UUID bytes must be exactly 16 bytes", nameof(bytes));

            _bytes = new byte[16];
            Array.Copy(bytes, _bytes, 16);
        }

        /// <summary>
        /// Get the raw bytes of the UUID
        /// </summary>
        public byte[] Bytes
        {
            get
            {
                var result = new byte[16];
                Array.Copy(_bytes, result, 16);
                return result;
            }
        }

        /// <summary>
        /// Get the version of the UUID
        /// </summary>
        public byte Version
        {
            get
            {
                var result = NativeMethods.uuid_get_info(_bytes, out byte version, out byte variant);
                CheckError(result);
                return version;
            }
        }

        /// <summary>
        /// Get the variant of the UUID
        /// </summary>
        public byte Variant
        {
            get
            {
                var result = NativeMethods.uuid_get_info(_bytes, out byte version, out byte variant);
                CheckError(result);
                return variant;
            }
        }

        /// <summary>
        /// Get version and variant information
        /// </summary>
        /// <returns>Tuple containing version and variant</returns>
        public (byte Version, byte Variant) GetInfo()
        {
            var result = NativeMethods.uuid_get_info(_bytes, out byte version, out byte variant);
            CheckError(result);
            return (version, variant);
        }

        /// <summary>
        /// Convert UUID to string representation
        /// </summary>
        /// <returns>UUID in format xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx</returns>
        public override string ToString()
        {
            var buffer = new StringBuilder(37); // 36 chars + null terminator
            var result = NativeMethods.uuid_to_string(_bytes, buffer, new IntPtr(37));
            CheckError(result);
            return buffer.ToString();
        }

        /// <summary>
        /// Check if this UUID equals another UUID
        /// </summary>
        /// <param name="other">Other UUID to compare</param>
        /// <returns>True if UUIDs are equal</returns>
        public bool Equals(Uuid other)
        {
            if (other == null)
                return false;

            var result = NativeMethods.uuid_compare(_bytes, other._bytes, out byte areEqual);
            CheckError(result);
            return areEqual != 0;
        }

        /// <summary>
        /// Check if this UUID equals another object
        /// </summary>
        /// <param name="obj">Object to compare</param>
        /// <returns>True if objects are equal</returns>
        public override bool Equals(object obj)
        {
            return Equals(obj as Uuid);
        }

        /// <summary>
        /// Get hash code for the UUID
        /// </summary>
        /// <returns>Hash code</returns>
        public override int GetHashCode()
        {
            return BitConverter.ToInt32(_bytes, 0);
        }

        /// <summary>
        /// Equality operator
        /// </summary>
        public static bool operator ==(Uuid left, Uuid right)
        {
            if (ReferenceEquals(left, null))
                return ReferenceEquals(right, null);
            return left.Equals(right);
        }

        /// <summary>
        /// Inequality operator
        /// </summary>
        public static bool operator !=(Uuid left, Uuid right)
        {
            return !(left == right);
        }

        /// <summary>
        /// Check error code and throw appropriate exception
        /// </summary>
        /// <param name="result">Error code from native function</param>
        private static void CheckError(int result)
        {
            switch (result)
            {
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
                    throw new UuidException($"UUID operation failed with error code {result}", result);
            }
        }
    }

    /// <summary>
    /// UUID generator providing RFC 4122 and RFC 9562 compliant UUID v4 generation
    /// </summary>
    public static class UuidGenerator
    {
        /// <summary>
        /// Generate a new UUID v4
        /// </summary>
        /// <returns>A new UUID object</returns>
        /// <exception cref="EntropyException">Thrown when random data generation fails</exception>
        /// <exception cref="UnknownErrorException">Thrown when an unknown error occurs</exception>
        public static Uuid Generate()
        {
            var uuidBytes = new byte[16];
            var result = NativeMethods.uuid_generate_v4(uuidBytes);
            CheckError(result);
            return new Uuid(uuidBytes);
        }

        /// <summary>
        /// Create a UUID from bytes
        /// </summary>
        /// <param name="bytes">16 bytes representing the UUID</param>
        /// <returns>A UUID object</returns>
        /// <exception cref="ArgumentNullException">Thrown when bytes is null</exception>
        /// <exception cref="ArgumentException">Thrown when bytes is not 16 bytes long</exception>
        public static Uuid FromBytes(byte[] bytes)
        {
            return new Uuid(bytes);
        }

        /// <summary>
        /// Check error code and throw appropriate exception
        /// </summary>
        /// <param name="result">Error code from native function</param>
        private static void CheckError(int result)
        {
            switch (result)
            {
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
                    throw new UuidException($"UUID operation failed with error code {result}", result);
            }
        }
    }
}
