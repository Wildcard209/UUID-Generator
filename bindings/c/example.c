/**
 * @file example.c
 * @brief Example usage of the UUID Generator C bindings
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "uuid_generator.h"

/**
 * Print UUID bytes in hexadecimal format
 */
void print_uuid_bytes(const uint8_t* uuid_bytes) {
    for (int i = 0; i < 16; i++) {
        printf("%02x", uuid_bytes[i]);
    }
}

/**
 * Demonstrate basic UUID generation
 */
void demo_basic_generation(void) {
    printf("\n1. Basic UUID Generation:\n");
    
    uint8_t uuid[16];
    int result = uuid_generate_v4(uuid);
    
    if (result != UUID_SUCCESS) {
        printf("   Error generating UUID: %s\n", uuid_error_string(result));
        return;
    }
    
    char uuid_str[37];
    result = uuid_to_string(uuid, uuid_str, sizeof(uuid_str));
    
    if (result != UUID_SUCCESS) {
        printf("   Error converting UUID to string: %s\n", uuid_error_string(result));
        return;
    }
    
    uint8_t version, variant;
    result = uuid_get_info(uuid, &version, &variant);
    
    if (result != UUID_SUCCESS) {
        printf("   Error getting UUID info: %s\n", uuid_error_string(result));
        return;
    }
    
    printf("   Generated UUID: %s\n", uuid_str);
    printf("   Raw bytes: ");
    print_uuid_bytes(uuid);
    printf("\n");
    printf("   Version: %d\n", version);
    printf("   Variant: %d\n", variant);
}

/**
 * Demonstrate multiple UUID generation
 */
void demo_multiple_generation(void) {
    printf("\n2. Multiple UUID Generation:\n");
    
    for (int i = 0; i < 5; i++) {
        uint8_t uuid[16];
        char uuid_str[37];
        
        int result = uuid_generate_v4(uuid);
        if (result != UUID_SUCCESS) {
            printf("   Error generating UUID %d: %s\n", i + 1, uuid_error_string(result));
            continue;
        }
        
        result = uuid_to_string(uuid, uuid_str, sizeof(uuid_str));
        if (result != UUID_SUCCESS) {
            printf("   Error converting UUID %d to string: %s\n", i + 1, uuid_error_string(result));
            continue;
        }
        
        printf("   UUID %d: %s\n", i + 1, uuid_str);
    }
}

/**
 * Demonstrate UUID comparison
 */
void demo_uuid_comparison(void) {
    printf("\n3. UUID Comparison:\n");
    
    uint8_t uuid1[16], uuid2[16], uuid3[16];
    char uuid1_str[37], uuid2_str[37], uuid3_str[37];
    
    uuid_generate_v4(uuid1);
    uuid_generate_v4(uuid2);
    
    memcpy(uuid3, uuid1, 16);
    
    uuid_to_string(uuid1, uuid1_str, sizeof(uuid1_str));
    uuid_to_string(uuid2, uuid2_str, sizeof(uuid2_str));
    uuid_to_string(uuid3, uuid3_str, sizeof(uuid3_str));
    
    printf("   UUID1: %s\n", uuid1_str);
    printf("   UUID2: %s\n", uuid2_str);
    printf("   UUID3: %s (copy of UUID1)\n", uuid3_str);
    
    uint8_t equal;
    
    int result = uuid_compare(uuid1, uuid2, &equal);
    if (result == UUID_SUCCESS) {
        printf("   UUID1 == UUID2: %s\n", equal ? "true" : "false");
    }
    
    result = uuid_compare(uuid1, uuid3, &equal);
    if (result == UUID_SUCCESS) {
        printf("   UUID1 == UUID3: %s\n", equal ? "true" : "false");
    }
}

/**
 * Demonstrate error handling
 */
void demo_error_handling(void) {
    printf("\n4. Error Handling:\n");
    
    int result = uuid_generate_v4(NULL);
    printf("   Generate with NULL pointer: %s (code %d)\n", 
           uuid_error_string(result), result);
    
    uint8_t uuid[16];
    char small_buffer[10];
    uuid_generate_v4(uuid);
    result = uuid_to_string(uuid, small_buffer, sizeof(small_buffer));
    printf("   Convert with small buffer: %s (code %d)\n", 
           uuid_error_string(result), result);
    
    result = uuid_get_info(NULL, NULL, NULL);
    printf("   Get info with NULL pointers: %s (code %d)\n", 
           uuid_error_string(result), result);
    
    result = uuid_compare(NULL, NULL, NULL);
    printf("   Compare with NULL pointers: %s (code %d)\n", 
           uuid_error_string(result), result);
}

/**
 * Demonstrate UUID properties
 */
void demo_uuid_properties(void) {
    printf("\n5. UUID Properties:\n");
    
    uint8_t uuid[16];
    char uuid_str[37];
    uint8_t version, variant;
    
    int result = uuid_generate_v4(uuid);
    if (result != UUID_SUCCESS) {
        printf("   Error generating UUID: %s\n", uuid_error_string(result));
        return;
    }
    
    uuid_to_string(uuid, uuid_str, sizeof(uuid_str));
    uuid_get_info(uuid, &version, &variant);
    
    printf("   UUID: %s\n", uuid_str);
    printf("   Length: %d characters\n", (int)strlen(uuid_str));
    printf("   Dashes at positions: 8, 13, 18, 23\n");
    printf("   Version: %d (should be 4 for UUID v4)\n", version);
    printf("   Variant: %d (should be 2 for RFC 4122)\n", variant);
    printf("   Raw bytes: ");
    print_uuid_bytes(uuid);
    printf("\n");
    printf("   Raw bytes length: 16 bytes\n");
}

/**
 * Main function
 */
int main(void) {
    printf("UUID Generator - C Example\n");
    printf("========================================\n");
    
    demo_basic_generation();
    demo_multiple_generation();
    demo_uuid_comparison();
    demo_error_handling();
    demo_uuid_properties();
    
    printf("\nDone!\n");
    return 0;
}
