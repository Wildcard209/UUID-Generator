/**
 * Example usage of the Node.js UUID generator bindings.
 */

const { uuid4, UuidGenerator, fromBytes, Uuid } = require('./index');

function main() {
    console.log('UUID Generator - Node.js Example');
    console.log('='.repeat(40));
    
    try {
        console.log('\n1. Basic UUID Generation:');
        const uuid = uuid4();
        console.log(`   Generated UUID: ${uuid}`);
        console.log(`   UUID type: ${uuid.constructor.name}`);
        console.log(`   Version: ${uuid.version()}`);
        console.log(`   Variant: ${uuid.variant()}`);
        
        console.log('\n2. Multiple UUID Generation:');
        for (let i = 0; i < 5; i++) {
            console.log(`   UUID ${i + 1}: ${uuid4()}`);
        }
        
        console.log('\n3. UUID Properties:');
        const testUuid = uuid4();
        console.log(`   UUID: ${testUuid}`);
        console.log(`   String representation: ${testUuid.toString()}`);
        console.log(`   Raw bytes: ${testUuid.bytes.toString('hex')}`);
        console.log(`   Raw bytes length: ${testUuid.bytes.length}`);
        console.log(`   Info: ${JSON.stringify(testUuid.info())}`);
        
        console.log('\n4. UUID Comparison:');
        const uuid1 = uuid4();
        const uuid2 = uuid4();
        const uuid3 = fromBytes(uuid1.bytes);
        
        console.log(`   UUID1: ${uuid1}`);
        console.log(`   UUID2: ${uuid2}`);
        console.log(`   UUID3: ${uuid3} (created from UUID1 bytes)`);
        console.log(`   UUID1 equals UUID2: ${uuid1.equals(uuid2)}`);
        console.log(`   UUID1 equals UUID3: ${uuid1.equals(uuid3)}`);
        
        console.log('\n5. Custom Generator:');
        const generator = new UuidGenerator();
        const customUuid = generator.generate();
        console.log(`   Custom UUID: ${customUuid}`);
        
        console.log('\n6. JSON Serialization:');
        const jsonUuid = uuid4();
        console.log(`   UUID: ${jsonUuid}`);
        console.log(`   JSON: ${JSON.stringify({ uuid: jsonUuid })}`);
        
        console.log('\n7. Error Handling:');
        try {
            fromBytes(Buffer.from('too_short'));
        } catch (error) {
            console.log(`   Caught expected error: ${error.message}`);
        }
        
        console.log('\nDone!');
        
    } catch (error) {
        console.error('Error:', error.message);
        console.error('Make sure to build the Rust library first with: cargo build --release');
        process.exit(1);
    }
}

if (require.main === module) {
    main();
}
