# Document

> NOTE: There is a known limitation of using the `iso_20022_sdk::prelude::Document` enum type when using `serde`. As a result, it is advised to use the exact `Document` type specific to the message scheme. For example, `iso_20022_pacs::pacs_008_001_10::Document`, `iso_20022_remt::remt_001_001_5::Document`, etc. 
> 
> The `iso_20022_sdk::prelude::Document` type is marked as deprecated for developer awareness.


When using `iso_20022_sdk::prelude::Message`, the document type must be known. 

The document for `Message` can be set at compile type, e.g., `Message::<Document<_, _>>`, or the document can be modified during runtime using the `Message::set_document()` method. 


```rust

use iso_20022_sdk::prelude::*;
use iso_20022_pacs::pacs_008_001_10::Document;

// Create an instance of a document, in this case the `pacs::008_001_10` document.
let mut doc = Document::<Dmkr, Dmkr>::default();

// Set the XML namespace for the document.
doc.xmlns = iso_20022_pacs::pacs_008_001_10::namespace();

// The message type can be elided when using the `set_document()` method
let msg = Message::<_>::builder()
    // Set the document type using the `set_document()` method.
    .set_document(doc)

// Tell the compiler how to handle the message document type
let xml = msg.to_xml()?;
let msg = Message::<Document<Dmkr, Dmkr>>::from_xml(&xml)?;


```