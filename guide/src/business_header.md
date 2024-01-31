# Business Application Header

The business application header of the ISO 20022 standard is a set of fields that are used to identify the message type and includes the sender and recipient of the message, along with additional values.

The XML tag name for the business header is `AppHdr`. The inner element type for the application header `AppHdr` is `BusinessApplicationHeaderV03`. The latest scheme version of the business application header is version 3.

The `AppHdr<A, B>` accepts two generic parameter types for the `SignatureEnvelope` used by the primary business application header and the related business application header field, `BusinessApplicationHeader7`. 

See the [Signature](./signature.md) section of the documentation for further information for signing the business header


See the [docs.rs](https://docs.rs/iso-20022-head/latest/iso_20022_head/head_001_001_03/index.html) documentation for available Rust structs for the header scheme.


## Builder

The `iso-20022-sdk` re-exports the `iso-20022-head` crate under the `prelude` module as `head`. There exists a builder struct for the header, `BusinessApplicationHeaderV03Builder`, that can be used to set the individuals fields for the header.


```rust

use iso_20022_sdk::prelude::head::{self, BusinessApplicationHeaderV03Builder as BizHdrBldr};

// Build a business header
let mut header = BizHdrBldr::default();
    
// Set the sender (from) field of the header
header.fr(head::Party44Choice { ... })

// Add additional header fields ...

// Once the application header fields are added, 
// set the application header for the message.
let msg = Message::<_>::builder()
    .set_app_hdr(head::AppHdr { value: header});

```

As a convenience, the `Message` struct provides top-level helper methods for setting the sender and recipient fields of the application header.

```rust

use iso_20022_sdk::prelude::*;

let msg = Message::<_>::builder()
    // Set the sender
    .set_sender(head::Party44Choice { ... })
    // Set the recipient
    .set_recipient(head::Party44Choice { ... });

```

The `Message` struct also provides a top-level getter method for returning the application header of the message, `.app_hdr()`. The method will return an `Option` with the inner value of the header, if one exists.

```rust

use iso_20022_sdk::prelude::*;

let msg = Message::<_>::builder()
    // Set header values ...
    .set_sender(head::Party44Choice { ... })
    .set_recipient(head::Party44Choice { ... });

// Retrieve the application header from the message
if let Some(header) = msg.app_hdr() {
    // Do something with the header
}


```