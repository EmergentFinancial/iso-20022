# Serialization / Deserialization


Serializing and deserializing ISO 20022 messages can be handled in various formats. As the ISO 20022 messages are defined using the extensible schema definition (XSD) format, it is common for systems processing ISO 20022 messages to be serialized using XML strings. 

However, XML is not always the most suitable serialization format. To provide compatibility with other serialization formats, the `Message` type includes three formats for serializing and deserializing ISO 20022 messages, `XML`, `JSON` and `Bincode (bytes)`.


> NOTE: ISO-20022-SDK uses `serde` for serialization and deserialization. As a result, there are known limitations with serialization for `XML` and `Bincode` formats. Notably, when sequence fields are empty, `serde` has difficulty serializing and may throw an error. `JSON` is currently the most stable serialization / deserialization format, however, the format used will depend on the use case.

> NOTE: There is a known limitation of using the `iso_20022_sdk::prelude::Document` enum type when using `serde`. As a result, it is advised to use the exact `Document` type specific to the message scheme. For example, `iso_20022_pacs::pacs_008_001_10::Document`, `iso_20022_remt::remt_001_001_5::Document`, etc. The `iso_20022_sdk::prelude::Document` type is marked as deprecated for developer awareness.

```rust

use iso_20022_sdk::prelude::*;
use iso_20022_pacs::pacs_008_001_10::Document;

let msg = Message::<Document<Dmkr, Dmkr>>::default();

// XML Serialization / Deserialization
let xml = msg.to_xml()?;
let msg = Message::<Document<Dmkr, Dmkr>>::from_xml(&xml)?;

// JSON Serialization / Deserialization
let json = msg.to_json()?;
let msg = Message::<Document<Dmkr, Dmkr>>::from_json(&json)?;

// Bincode Serialization / Deserialization
let bytes = msg.to_bytes()?;
let msg = Message::<Document<Dmkr, Dmkr>>::from_bytes(&bytes)?;
```