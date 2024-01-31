# Message Envelope

The `Message` type is a high-level abstraction of the ISO-20022 message envelope `BizMsgEnvlp` and its child elements `Hdr`, `Doc`, `Ref` and `SplmtryData`.

The `Message` type provides a builder pattern for constructing a message envelope and its child elements. The `Message` type also provides methods for serializing the message envelope to XML and deserializing the message envelope from XML.

```rust

// Import common types and traits in the prelude
use iso_20022_sdk::prelude::*;


// The `builder` method will return `Message` instance
// after setting default values, e.g. envelope namespace
//
// The initial `Doc` type of the `Message` can be elided
// using `::<_>` turbofish syntax. The compiler will 
// infer the type of the `Doc` based on the `set_document()`
// method.
//
// If a type is required from the compiler, the `::<Document>` 
// turbofish syntax can be used to specify the enumerable document
// type of the `Doc` element. This value can then be later overridden
// using the `set_document()` method.
let msg = Message::<_>::builder()
    
    
    // Setting the type of document is done using the `set_document()` method.
    // In practice, the document type will likely be the result of the document
    // builder for the target namespace, e.g. 
    //
    // `documents::pacs::pacs_008_001_07::Document`
    //
    // The example below uses the default values for the document builder 
    // for the `pacs.008.001.07` namespace.
    //
    // NOTE: document namespaces are feature gated and must be enabled
    // for the example to work, e.g. `pacs` feature must be enabled in
    // Cargo.toml file.
    .set_document(Document::from_namespace("pacs.008.001.07"));


```

## Overview

The `nvlp.001.001.01` message envelope `BizMsgEnvlp` is the root element of an ISO-20022 message. It contains the `Hdr` (Business Application Header), `Doc` (Document), `Ref` (Reference Information) and `SplmtryData` (Supplementary Data) elements.

### XML Schema Definition (XSD)

The following snippet is the XML Schema Definition (XSD) for the `BizMsgEnvlp` element.

```xml
<!-- XML Schema Definition (XSD) element -->

<xs:element name="BizMsgEnvlp" type="BusinessMessageEnvelopeV01"/>
<!-- ... -->
<xs:complexType name="BusinessMessageEnvelopeV01">
    <xs:sequence>
        <xs:element maxOccurs="1" minOccurs="0" name="Hdr" type="LaxPayload"/>
        <xs:element name="Doc" type="LaxPayload"/>
        <xs:element maxOccurs="unbounded" minOccurs="0" name="Ref" type="Reference22"/>
        <xs:element maxOccurs="unbounded" minOccurs="0" name="SplmtryData" type="SupplementaryData1"/>
    </xs:sequence>
</xs:complexType>
```

### Rust Type Definition

The following snippet is the Rust type definition for the `BizMsgEnvlp` struct.

```rust
// BizMsgEnvlp Rust Type Definition

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
#[serde(transparent)]
pub struct BizMsgEnvlp<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    pub value: BusinessMessageEnvelopeV01<A, B, C, D>,
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
#[serde(rename = "BizMsgEnvlp")]
pub struct BusinessMessageEnvelopeV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Hdr", skip_serializing_if = "Option::is_none")]
    pub hdr: Option<LaxPayload<A>>,
    #[validate]
    #[serde(rename = "Doc")]
    pub doc: LaxPayload<B>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Ref", default)]
    pub r#ref: Vec<Reference22<C>>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<D>>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}

```

The `Message` type is a wrapper around the `BizMsgEnvlp` type. It provides a simplified interface for interacting with a `BizMsgEnvlp` instance.



#### Example BizMsgEnvlp Instance

Using the `Message::to_xml` method, the `Message` type can be serialized to XML.

```rust
// Create a `Message` instance
let msg = Message::<Document>::default();

// Call the `to_xml` method to serialize the `Message` type to XML
let xml = msg.to_xml();

```


```xml
<!-- Example XML Instance -->
<BizMsgEnvlp xmlns="urn:iso:std:iso:20022:tech:xsd:nvlp.001.001.01">
    <Hdr>
        <!-- Business Application Header (head.001.001.03) -->
    </Hdr>
    <Doc>
        <!-- Document (ISO-20022 Message) -->
    </Doc>
    <Ref>
        <!-- Reference Information -->
    </Ref>
    <SplmtryData>
        <!-- Supplementary Data (Generic Type) -->
    </SplmtryData>
</BizMsgEnvlp>
```