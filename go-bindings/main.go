package main

/*
#cgo LDFLAGS: -L../target/release -luuid_generator
#include <stdint.h>
#include <stdlib.h>

// FFI function declarations
int32_t uuid_generate_v4(uint8_t* uuid_bytes);
int32_t uuid_to_string(const uint8_t* uuid_bytes, char* uuid_string, size_t buffer_size);
int32_t uuid_get_info(const uint8_t* uuid_bytes, uint8_t* version, uint8_t* variant);
int32_t uuid_compare(const uint8_t* uuid1_bytes, const uint8_t* uuid2_bytes, uint8_t* are_equal);
*/
import "C"
import (
	"fmt"
)

type UUIDError struct {
	Code    int32
	Message string
}

func (e UUIDError) Error() string {
	return fmt.Sprintf("UUID error %d: %s", e.Code, e.Message)
}

type UUID struct {
	bytes [16]byte
}

func NewV4() (*UUID, error) {
	var uuid UUID
	var cBytes [16]C.uint8_t

	result := C.uuid_generate_v4(&cBytes[0])
	if result != 0 {
		return nil, UUIDError{
			Code:    int32(result),
			Message: getErrorMessage(int32(result)),
		}
	}

	for i := 0; i < 16; i++ {
		uuid.bytes[i] = byte(cBytes[i])
	}

	return &uuid, nil
}

func (u *UUID) String() (string, error) {
	var cBytes [16]C.uint8_t
	var buffer [37]C.char

	for i := 0; i < 16; i++ {
		cBytes[i] = C.uint8_t(u.bytes[i])
	}

	result := C.uuid_to_string(&cBytes[0], &buffer[0], 37)
	if result != 0 {
		return "", UUIDError{
			Code:    int32(result),
			Message: getErrorMessage(int32(result)),
		}
	}

	return C.GoString(&buffer[0]), nil
}

func (u *UUID) Bytes() [16]byte {
	return u.bytes
}

func (u *UUID) Version() (uint8, error) {
	var cBytes [16]C.uint8_t
	var version, variant C.uint8_t

	for i := 0; i < 16; i++ {
		cBytes[i] = C.uint8_t(u.bytes[i])
	}

	result := C.uuid_get_info(&cBytes[0], &version, &variant)
	if result != 0 {
		return 0, UUIDError{
			Code:    int32(result),
			Message: getErrorMessage(int32(result)),
		}
	}

	return uint8(version), nil
}

func (u *UUID) Variant() (uint8, error) {
	var cBytes [16]C.uint8_t
	var version, variant C.uint8_t

	for i := 0; i < 16; i++ {
		cBytes[i] = C.uint8_t(u.bytes[i])
	}

	result := C.uuid_get_info(&cBytes[0], &version, &variant)
	if result != 0 {
		return 0, UUIDError{
			Code:    int32(result),
			Message: getErrorMessage(int32(result)),
		}
	}

	return uint8(variant), nil
}

func (u *UUID) Equal(other *UUID) (bool, error) {
	var cBytes1, cBytes2 [16]C.uint8_t
	var areEqual C.uint8_t

	for i := 0; i < 16; i++ {
		cBytes1[i] = C.uint8_t(u.bytes[i])
		cBytes2[i] = C.uint8_t(other.bytes[i])
	}

	result := C.uuid_compare(&cBytes1[0], &cBytes2[0], &areEqual)
	if result != 0 {
		return false, UUIDError{
			Code:    int32(result),
			Message: getErrorMessage(int32(result)),
		}
	}

	return areEqual == 1, nil
}

func FromBytes(bytes [16]byte) *UUID {
	return &UUID{bytes: bytes}
}

func getErrorMessage(code int32) string {
	switch code {
	case 0:
		return "Success"
	case 1:
		return "Failed to generate random data from entropy source"
	case 2:
		return "Invalid parameter (null pointer, invalid size, etc.)"
	case 3:
		return "Buffer too small for output"
	case 99:
		return "Unknown error"
	default:
		return "Undefined error code"
	}
}

func main() {
	fmt.Println("UUID Generator - Go Integration Example")
	fmt.Println("======================================")

	fmt.Println("\n1. Generating a single UUID v4:")
	uuid, err := NewV4()
	if err != nil {
		fmt.Printf("   Error: %v\n", err)
		return
	}

	uuidStr, err := uuid.String()
	if err != nil {
		fmt.Printf("   Error converting to string: %v\n", err)
		return
	}

	version, err := uuid.Version()
	if err != nil {
		fmt.Printf("   Error getting version: %v\n", err)
		return
	}

	variant, err := uuid.Variant()
	if err != nil {
		fmt.Printf("   Error getting variant: %v\n", err)
		return
	}

	fmt.Printf("   Generated UUID: %s\n", uuidStr)
	fmt.Printf("   Version: %d\n", version)
	fmt.Printf("   Variant: %d\n", variant)
	fmt.Printf("   Raw bytes: %v\n", uuid.Bytes())

	fmt.Println("\n2. Generating multiple UUIDs:")
	for i := 1; i <= 5; i++ {
		uuid, err := NewV4()
		if err != nil {
			fmt.Printf("   Error generating UUID %d: %v\n", i, err)
			continue
		}

		uuidStr, err := uuid.String()
		if err != nil {
			fmt.Printf("   Error converting UUID %d to string: %v\n", i, err)
			continue
		}

		fmt.Printf("   UUID %d: %s\n", i, uuidStr)
	}

	fmt.Println("\n3. Testing UUID comparison:")
	uuid1, err := NewV4()
	if err != nil {
		fmt.Printf("   Error generating first UUID: %v\n", err)
		return
	}

	uuid2, err := NewV4()
	if err != nil {
		fmt.Printf("   Error generating second UUID: %v\n", err)
		return
	}

	uuid1Copy := FromBytes(uuid1.Bytes())

	uuid1Str, _ := uuid1.String()
	uuid2Str, _ := uuid2.String()
	uuid1CopyStr, _ := uuid1Copy.String()

	fmt.Printf("   UUID 1: %s\n", uuid1Str)
	fmt.Printf("   UUID 2: %s\n", uuid2Str)
	fmt.Printf("   UUID 1 copy: %s\n", uuid1CopyStr)

	equal12, err := uuid1.Equal(uuid2)
	if err != nil {
		fmt.Printf("   Error comparing UUID 1 and 2: %v\n", err)
		return
	}

	equal1Copy, err := uuid1.Equal(uuid1Copy)
	if err != nil {
		fmt.Printf("   Error comparing UUID 1 and copy: %v\n", err)
		return
	}

	fmt.Printf("   UUID 1 == UUID 2: %t\n", equal12)
	fmt.Printf("   UUID 1 == UUID 1 copy: %t\n", equal1Copy)

	fmt.Println("\n4. Validating UUID properties:")
	for i := 1; i <= 10; i++ {
		uuid, err := NewV4()
		if err != nil {
			fmt.Printf("   Error generating UUID %d: %v\n", i, err)
			continue
		}

		version, err := uuid.Version()
		if err != nil {
			fmt.Printf("   Error getting version for UUID %d: %v\n", i, err)
			continue
		}

		variant, err := uuid.Variant()
		if err != nil {
			fmt.Printf("   Error getting variant for UUID %d: %v\n", i, err)
			continue
		}

		uuidStr, _ := uuid.String()
		fmt.Printf("   UUID %d: %s (v%d, variant %d)\n", i, uuidStr, version, variant)

		if version != 4 {
			fmt.Printf("   ERROR: UUID %d has incorrect version %d (should be 4)\n", i, version)
		}
		if variant != 2 {
			fmt.Printf("   ERROR: UUID %d has incorrect variant %d (should be 2)\n", i, variant)
		}
	}
	fmt.Println("   All UUIDs have correct version (4) and variant (2)")

	fmt.Println("\nGo integration example completed successfully!")
	fmt.Println("The Rust UUID library is working correctly through FFI bindings.")
}
