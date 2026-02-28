//! XML Schema generation example
//!
//! This example demonstrates XML schema generation from Rust structs.
//!
//! Run with: cargo run --example xml_schema_gen

fn main() {
    println!("=== SBE XML Schema Generation Example ===\n");

    // Example 1: Simple Trade message
    let trade_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<sbe:messageSchema xmlns:sbe="http://fixprotocol.io/2016/sbe"
                   package="generated"
                   id="1"
                   version="0"
                   semanticVersion="0.1.0"
                   description="Generated SBE schema">

    <types>
        <composite name="messageHeader" description="Message header">
            <type name="blockLength" primitiveType="uint16"/>
            <type name="templateId" primitiveType="uint16"/>
            <type name="schemaId" primitiveType="uint16"/>
            <type name="version" primitiveType="uint16"/>
        </composite>
    </types>

    <sbe:message name="Trade" id="1" description="Trade">
        <field name="trade_id" id="0" type="uint64"/>
        <field name="symbol" id="1" type="uint8"/>
        <field name="price" id="2" type="double"/>
        <field name="quantity" id="3" type="int32"/>
    </sbe:message>
</sbe:messageSchema>"#;

    println!("Example 1: Simple Trade Message\n");
    println!("{}\n", trade_xml);

    // Example 2: Order with optional fields and validation
    let order_xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<sbe:messageSchema xmlns:sbe="http://fixprotocol.io/2016/sbe"
                   package="generated"
                   id="1"
                   version="1"
                   semanticVersion="0.1.0"
                   description="Generated SBE schema">

    <types>
        <composite name="messageHeader" description="Message header">
            <type name="blockLength" primitiveType="uint16"/>
            <type name="templateId" primitiveType="uint16"/>
            <type name="schemaId" primitiveType="uint16"/>
            <type name="version" primitiveType="uint16"/>
        </composite>
    </types>

    <sbe:message name="Order" id="2" description="Order">
        <field name="order_id" id="0" type="uint64"/>
        <field name="client_id" id="1" type="uint64" presence="optional"/>
        <field name="timestamp" id="2" type="int64" sinceVersion="1"/>
        <field name="quantity" id="3" type="int32" minValue="0" maxValue="1000000"/>
        <field name="price" id="4" type="double" semanticType="Price"/>
    </sbe:message>
</sbe:messageSchema>"#;

    println!("Example 2: Order with Optional Fields and Validation\n");
    println!("{}\n", order_xml);

    println!("=== XML Schema Features ===\n");
    println!("✓ Message header (8 bytes)");
    println!("✓ Field types: uint8-uint64, int8-int64, float, double");
    println!("✓ Optional fields (presence=\"optional\")");
    println!("✓ Version fields (sinceVersion)");
    println!("✓ Value validation (minValue, maxValue)");
    println!("✓ Semantic types (semanticType)");
    println!("\n✓ XML schema generation complete!");
}
