package com.uuidgenerator;

/**
 * Example usage of the Java UUID generator bindings.
 */
public class Example {
    public static void main(String[] args) {
        System.out.println("UUID Generator - Java Example");
        System.out.println("=".repeat(40));

        try {
            System.out.println("\n1. Basic UUID Generation:");
            Uuid uuid = UuidGenerator.generate();
            System.out.println("   Generated UUID: " + uuid);
            System.out.println("   UUID type: " + uuid.getClass().getSimpleName());
            System.out.println("   Version: " + uuid.getVersion());
            System.out.println("   Variant: " + uuid.getVariant());

            System.out.println("\n2. Multiple UUID Generation:");
            for (int i = 0; i < 5; i++) {
                System.out.println("   UUID " + (i + 1) + ": " + UuidGenerator.generate());
            }

            System.out.println("\n3. UUID Properties:");
            Uuid testUuid = UuidGenerator.generate();
            System.out.println("   UUID: " + testUuid);
            System.out.println("   String representation: " + testUuid.toString());
            System.out.print("   Raw bytes: ");
            for (byte b : testUuid.getBytes()) {
                System.out.printf("%02x", b);
            }
            System.out.println();
            System.out.println("   Raw bytes length: " + testUuid.getBytes().length);
            int[] info = testUuid.getInfo();
            System.out.println("   Version and variant: [" + info[0] + ", " + info[1] + "]");

            System.out.println("\n4. UUID Comparison:");
            Uuid uuid1 = UuidGenerator.generate();
            Uuid uuid2 = UuidGenerator.generate();
            Uuid uuid3 = UuidGenerator.fromBytes(uuid1.getBytes());

            System.out.println("   UUID1: " + uuid1);
            System.out.println("   UUID2: " + uuid2);
            System.out.println("   UUID3: " + uuid3 + " (created from UUID1 bytes)");
            System.out.println("   UUID1.equals(UUID2): " + uuid1.equals(uuid2));
            System.out.println("   UUID1.equals(UUID3): " + uuid1.equals(uuid3));

            System.out.println("\n5. Error Handling:");
            try {
                Uuid invalidUuid = UuidGenerator.fromBytes(new byte[]{1, 2, 3});
            } catch (IllegalArgumentException e) {
                System.out.println("   Caught expected error: " + e.getMessage());
            }

            System.out.println("\n6. Hash Codes:");
            Uuid hashUuid1 = UuidGenerator.generate();
            Uuid hashUuid2 = UuidGenerator.fromBytes(hashUuid1.getBytes());
            System.out.println("   UUID1: " + hashUuid1);
            System.out.println("   UUID2: " + hashUuid2 + " (same bytes)");
            System.out.println("   Hash1: " + hashUuid1.hashCode());
            System.out.println("   Hash2: " + hashUuid2.hashCode());
            System.out.println("   Hashes equal: " + (hashUuid1.hashCode() == hashUuid2.hashCode()));

            System.out.println("\nDone!");

        } catch (Exception e) {
            System.err.println("Error: " + e.getMessage());
            System.err.println("Make sure to build the Rust library first and place the shared library in the Java library path!");
            e.printStackTrace();
            System.exit(1);
        }
    }
}
