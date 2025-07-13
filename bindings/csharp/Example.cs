using System;

namespace UuidGenerator.Example
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("UUID Generator - C# Example");
            Console.WriteLine(new string('=', 40));

            try
            {
                Console.WriteLine("\n1. Basic UUID Generation:");
                var uuid = UuidGenerator.Generate();
                Console.WriteLine($"   Generated UUID: {uuid}");
                Console.WriteLine($"   UUID type: {uuid.GetType().Name}");
                Console.WriteLine($"   Version: {uuid.Version}");
                Console.WriteLine($"   Variant: {uuid.Variant}");

                Console.WriteLine("\n2. Multiple UUID Generation:");
                for (int i = 0; i < 5; i++)
                {
                    Console.WriteLine($"   UUID {i + 1}: {UuidGenerator.Generate()}");
                }

                Console.WriteLine("\n3. UUID Properties:");
                var testUuid = UuidGenerator.Generate();
                Console.WriteLine($"   UUID: {testUuid}");
                Console.WriteLine($"   String representation: {testUuid.ToString()}");
                Console.WriteLine($"   Raw bytes: {BitConverter.ToString(testUuid.Bytes).Replace("-", "")}");
                Console.WriteLine($"   Raw bytes length: {testUuid.Bytes.Length}");
                var info = testUuid.GetInfo();
                Console.WriteLine($"   Version and variant: ({info.Version}, {info.Variant})");

                Console.WriteLine("\n4. UUID Comparison:");
                var uuid1 = UuidGenerator.Generate();
                var uuid2 = UuidGenerator.Generate();
                var uuid3 = UuidGenerator.FromBytes(uuid1.Bytes);

                Console.WriteLine($"   UUID1: {uuid1}");
                Console.WriteLine($"   UUID2: {uuid2}");
                Console.WriteLine($"   UUID3: {uuid3} (created from UUID1 bytes)");
                Console.WriteLine($"   UUID1 == UUID2: {uuid1 == uuid2}");
                Console.WriteLine($"   UUID1 == UUID3: {uuid1 == uuid3}");
                Console.WriteLine($"   UUID1.Equals(UUID2): {uuid1.Equals(uuid2)}");
                Console.WriteLine($"   UUID1.Equals(UUID3): {uuid1.Equals(uuid3)}");

                Console.WriteLine("\n5. Error Handling:");
                try
                {
                    var invalidUuid = UuidGenerator.FromBytes(new byte[] { 1, 2, 3 });
                }
                catch (ArgumentException ex)
                {
                    Console.WriteLine($"   Caught expected error: {ex.Message}");
                }

                Console.WriteLine("\n6. Hash Codes:");
                var hashUuid1 = UuidGenerator.Generate();
                var hashUuid2 = UuidGenerator.FromBytes(hashUuid1.Bytes);
                Console.WriteLine($"   UUID1: {hashUuid1}");
                Console.WriteLine($"   UUID2: {hashUuid2} (same bytes)");
                Console.WriteLine($"   Hash1: {hashUuid1.GetHashCode()}");
                Console.WriteLine($"   Hash2: {hashUuid2.GetHashCode()}");
                Console.WriteLine($"   Hashes equal: {hashUuid1.GetHashCode() == hashUuid2.GetHashCode()}");

                Console.WriteLine("\nDone!");
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error: {ex.Message}");
                Console.WriteLine("Make sure to build the Rust library first and place the shared library in the same directory!");
                Environment.Exit(1);
            }
        }
    }
}
