// Copyright 2023 Emergent Financial, LLC - All Rights Reserved
//
//
// This software is licensed under the Emergent Financial Limited Public License Version 1.0
// (EF-LPLv1). You may use, copy, modify, and distribute this software under the terms and
// conditions of the EF-LPL. For more information, please refer to the full text of the license
// at https://github.com/emergentfinancial/ef-lpl.
//
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
//! # Message Envelope
//!
//! The `Message` type is a high-level abstraction of the ISO-20022 message envelope `BizMsgEnvlp` and its child elements `Hdr`, `Doc`, `Ref` and `SplmtryData`.
//!
//! The `Message` type provides a builder pattern for constructing a message envelope and its child elements. The `Message` type also provides methods for serializing the message envelope to XML and deserializing the message envelope from XML.
//!
//! ```rust
//!
//! // Import common types and traits in the prelude
//! use iso_20022_sdk::prelude::*;
//!
//!
//! // The `builder` method will return `Message` instance
//! // after setting default values, e.g. envelope namespace
//! //
//! // The initial `Doc` type of the `Message` can be elided
//! // using `::<_>` turbofish syntax. The compiler will
//! // infer the type of the `Doc` based on the `set_document()`
//! // method.
//! //
//! // If a type is required from the compiler, the `::<Document>`
//! // turbofish syntax can be used to specify the enumerable iso-20022 document
//! // type of the `Doc` element. This value can then be later overridden
//! // using the `set_document()` method.
//! let msg = Message::<_>::builder()
//!    
//!    
//!     // Setting the type of document is done using the `set_document()` method.
//!     // In practice, the document type will likely be the result of the document
//!     // builder for the target namespace, e.g.
//!     //
//!     // `documents::pacs::pacs_008_001_07::Document`
//!     //
//!     // The example below uses the default values for the document builder
//!     // for the `pacs.008.001.10` namespace.
//!     //
//!     // NOTE: document namespaces are feature gated and must be enabled
//!     // for the example to work, e.g. `pacs` feature must be enabled in
//!     // Cargo.toml file.
//!     .set_document(Document::from_namespace("pacs.008.001.10"))
//!     // Call the `to_xml` method to serialize the `Message` type to XML
//!     .to_xml();
//!
//!
//! ```
//!
//! #### Example XML Output
//! ```xml
//! <!-- Example XML Instance -->
//! <BizMsgEnvlp xmlns="urn:iso:std:iso:20022:tech:xsd:nvlp.001.001.01">
//!     <Hdr>
//!         <!-- Business Application Header (head.001.001.03) -->
//!     </Hdr>
//!     <Doc>
//!         <!-- Document (ISO-20022 Message) -->
//!     </Doc>
//!     <Ref>
//!         <!-- Reference Information -->
//!     </Ref>
//!     <SplmtryData>
//!         <!-- Supplementary Data (Generic Type) -->
//!     </SplmtryData>
//! </BizMsgEnvlp>
//! ```
use std::io::BufReader;

use crate::dsig::xpath::XPath;
use crate::head::head_001_001_03::{self as head};
use crate::nvlp::nvlp_001_001_01::{self as nvlp};
use crate::prelude::MessageSigner;

use xml::{reader::XmlEvent, EventReader};

use crate::documents::{Dmkr, Document};

/// Default Business Message Identifier
/// ISO 20022 messages have a root `Document`
/// element.
const DEFAULT_BIZ_MSG_IDR: &str = "Document";

/// Default Envelope Type
pub type DefaultMsgEnvlp<Sig> = nvlp::BizMsgEnvlp<head::AppHdr<Sig, Sig>, Document, Dmkr, Dmkr>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error Serializing / Deserializing XML
    #[error(transparent)]
    XmlSerDe(#[from] quick_xml::de::DeError),
    /// SXD Document Error
    #[error(transparent)]
    XsdDocument(#[from] sxd_document::parser::Error),
    // /// SXD XPath Error
    // #[error(transparent)]
    // XsdXPath(#[from] sxd_xpath::Error),
    // /// Signing Error
    // #[error(transparent)]
    // Signing(#[from] signature::Error),
    /// Message Signer Error
    #[error(transparent)]
    Crypto(#[from] crate::crypto::Error),
    #[error("Invalid Document Type")]
    InvalidDocumentType,
}

#[derive(Debug, Clone, Default)]
pub struct Message<
    'a,
    Doc: std::fmt::Debug
        + Default
        + Clone
        + PartialEq
        + ::serde::Serialize
        + ::serde::Deserialize<'a>
        + ::validator::Validate,
    Sig: std::fmt::Debug
        + Default
        + Clone
        + PartialEq
        + ::serde::Serialize
        + ::serde::Deserialize<'a>
        + ::validator::Validate,
> {
    /// XML string representing the inner type. Used internally to parse the inner type.
    /// An incoming message will use this field for helping to determine what the
    /// inner type is.
    ///
    /// use the `to_xml()` method to get the XML string representation of the message
    /// inner type.
    pub xml_string: &'a str,
    /// Internal representation of the message envelope
    pub inner: nvlp::BizMsgEnvlp<head::AppHdr<Sig, Sig>, Doc, Dmkr, Dmkr>,
}

impl<'a, Doc, Sig> Message<'a, Doc, Sig>
where
    Doc: std::fmt::Debug
        + Default
        + Clone
        + PartialEq
        + ::serde::Serialize
        + ::serde::Deserialize<'a>
        + ::validator::Validate,
    Sig: std::fmt::Debug
        + Default
        + Clone
        + PartialEq
        + ::serde::Serialize
        + ::serde::Deserialize<'a>
        + ::validator::Validate
        + crate::crypto::XmlSignature,
{
    /// `msgIdr` refers to the `MsgDefIdr` field of the `AppHdr` element
    /// of the message envelope. E.g. pacs.008.001.10
    pub fn new(msg_id: &str, recipient_id: &str, sender_id: &str) -> Message<'a, Document, Sig> {
        let msg = Message::<Document, Sig>::builder()
            .set_cre_dt()
            .set_msg_def_idr(head::Max35Text {
                value: msg_id.to_string(),
            })
            .set_biz_msg_idr(head::Max35Text {
                value: DEFAULT_BIZ_MSG_IDR.to_string(),
            })
            .set_recipient_org_id(head::OrganisationIdentification29 {
                othr: vec![head::GenericOrganisationIdentification1 {
                    id: head::Max35Text {
                        value: recipient_id.to_string(),
                    },
                    ..Default::default()
                }],
                ..Default::default()
            })
            .set_sender_org_id(head::OrganisationIdentification29 {
                othr: vec![head::GenericOrganisationIdentification1 {
                    id: head::Max35Text {
                        value: sender_id.to_string(),
                    },
                    ..Default::default()
                }],
                ..Default::default()
            })
            // Set the document type for the message, this will overwrite the
            // type set in the turbofish.
            .set_document(Document::from_namespace(msg_id));

        msg
    }

    pub fn builder() -> Self {
        let envlp = Self::default();

        // Automatically set the envlp and header namespaces
        envlp.set_namespace()
    }

    /// Return the application header from the message envelope
    pub fn app_hdr(&self) -> Option<head::AppHdr<Sig, Sig>> {
        self.inner.value.hdr.clone().map(|hdr| hdr.value)
    }

    /// Set the application header AppHdr of the message
    /// Note, this will overwrite the existing AppHdr
    pub fn set_app_hdr(self, app_hdr: head::AppHdr<Sig, Sig>) -> Self {
        let mut msg = self;

        // Set the AppHdr
        msg.inner.value.hdr = Some(nvlp::LaxPayload { value: app_hdr });

        msg
    }

    /// Set the recipient of the message
    pub fn set_recipient(self, recipient: head::Party44Choice) -> Self {
        let mut app_hdr = self.app_hdr().unwrap_or_default();
        app_hdr.value.to = recipient;

        self.set_app_hdr(app_hdr)
    }

    /// Set the recipient organization id of the message.
    /// This is a simplified version of `set_recipient` that only takes an organization id.
    /// Use the `set_recipient_fi_id()` method to set a financial institutiton id.
    /// Note, this will overwrite any existing recipient.
    pub fn set_recipient_org_id(self, org_id: head::OrganisationIdentification29) -> Self {
        self.set_recipient(head::Party44Choice {
            value: head::Party44ChoiceEnum {
                org_id: Some(head::PartyIdentification135 {
                    id: Some(head::Party38Choice {
                        value: head::Party38ChoiceEnum {
                            org_id: Some(org_id),
                            ..Default::default()
                        },
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        })
    }

    /// Set the recipient financial institution id of the message.
    /// This is a simplified version of `set_recipient` that only takes a financial institution id.
    /// Use the `set_recipient_org_id()` method to set an organization id.
    /// Note, this will overwrite any existing recipient.
    pub fn set_recipient_fi_id(
        self,
        fin_instn_id: head::FinancialInstitutionIdentification18,
    ) -> Self {
        self.set_recipient(head::Party44Choice {
            value: head::Party44ChoiceEnum {
                fi_id: Some(head::BranchAndFinancialInstitutionIdentification6 {
                    fin_instn_id,
                    ..Default::default()
                }),
                ..Default::default()
            },
        })
    }

    /// Set the recipient private individual id of the message.
    /// This is a simplified version of `set_recipient` that only takes a private individual id.
    /// Use the `set_recipient_org_id()` method to set an organization id or the `set_recipient_fi_id()`
    /// method to set a financial institution id.
    /// Note, this will overwrite any existing recipient.
    pub fn set_recipient_prvt_id(self, prvt_id: head::PersonIdentification13) -> Self {
        self.set_recipient(head::Party44Choice {
            value: head::Party44ChoiceEnum {
                org_id: Some(head::PartyIdentification135 {
                    id: Some(head::Party38Choice {
                        value: head::Party38ChoiceEnum {
                            prvt_id: Some(prvt_id),
                            ..Default::default()
                        },
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        })
    }

    /// Set the sender of the message
    pub fn set_sender(self, sender: head::Party44Choice) -> Self {
        let mut app_hdr = self.app_hdr().unwrap_or_default();
        app_hdr.value.fr = sender;

        self.set_app_hdr(app_hdr)
    }

    /// Set the sender organization id of the message.
    /// This is a simplified version of `set_sender` that only takes an organization id.
    /// Use the `set_sender_fi_id()` method to set a financial institutiton id.
    /// Note, this will overwrite any existing sender.
    pub fn set_sender_org_id(self, org_id: head::OrganisationIdentification29) -> Self {
        self.set_sender(head::Party44Choice {
            value: head::Party44ChoiceEnum {
                org_id: Some(head::PartyIdentification135 {
                    id: Some(head::Party38Choice {
                        value: head::Party38ChoiceEnum {
                            org_id: Some(org_id),
                            ..Default::default()
                        },
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        })
    }

    /// Set the sender financial institution id of the message.
    /// This is a simplified version of `set_sender` that only takes a financial institution id.
    /// Use the `set_sender_org_id()` method to set an organization id.
    /// Note, this will overwrite any existing sender.
    pub fn set_sender_fi_id(
        self,
        fin_instn_id: head::FinancialInstitutionIdentification18,
    ) -> Self {
        self.set_sender(head::Party44Choice {
            value: head::Party44ChoiceEnum {
                fi_id: Some(head::BranchAndFinancialInstitutionIdentification6 {
                    fin_instn_id,
                    ..Default::default()
                }),
                ..Default::default()
            },
        })
    }

    /// Set the sender private individual id of the message.
    /// This is a simplified version of `set_sender` that only takes a private individual id.
    /// Use the `set_sender_org_id()` method to set an organization id or the `set_sender_fi_id()`
    /// method to set a financial institution id.
    /// Note, this will overwrite any existing sender.
    pub fn set_sender_prvt_id(self, prvt_id: head::PersonIdentification13) -> Self {
        self.set_sender(head::Party44Choice {
            value: head::Party44ChoiceEnum {
                org_id: Some(head::PartyIdentification135 {
                    id: Some(head::Party38Choice {
                        value: head::Party38ChoiceEnum {
                            prvt_id: Some(prvt_id),
                            ..Default::default()
                        },
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        })
    }

    /// e.g. `Document`
    pub fn set_biz_msg_idr(self, idr: head::Max35Text) -> Self {
        let mut app_hdr = self.app_hdr().unwrap_or_default();
        app_hdr.value.biz_msg_idr = idr;

        self.set_app_hdr(app_hdr)
    }

    /// e.g. `pacs.008.001.10`
    pub fn set_msg_def_idr(self, idr: head::Max35Text) -> Self {
        let mut app_hdr = self.app_hdr().unwrap_or_default();
        app_hdr.value.msg_def_idr = idr;

        self.set_app_hdr(app_hdr)
    }

    /// Set the created date time of the message.
    /// This will be set to the current UTC time.
    pub fn set_cre_dt(self) -> Self {
        let mut app_hdr = self.app_hdr().unwrap_or_default();
        app_hdr.value.cre_dt = head::IsoDateTime {
            value: chrono::Utc::now(),
        };

        self.set_app_hdr(app_hdr)
    }

    /// Set the xml namespace of the message and business header.
    pub fn set_namespace(self) -> Self {
        let mut envlp = self;

        // Set the envelope namespace
        envlp.inner.value.xmlns = nvlp::namespace();

        // Set the header namespace
        let mut app_hdr = envlp.app_hdr().unwrap_or_default();
        app_hdr.value.xmlns = head::namespace();

        envlp.set_app_hdr(app_hdr)
    }

    /// Set the document of the message.
    /// Note, the document must set its own namespace value.
    /// By default, all root iso-20022 message documents have
    /// an attribute field, `xmlns`, that is used to set the document namespace.
    /// The document namespace must be set before calling this method.
    pub fn set_document(self, doc: Doc) -> Self {
        let mut envlp = self;
        envlp.inner.value.doc.value = doc;

        envlp
    }

    /// Sign the document at an optional xpath, e.g. `/Document/AcctOpngInstr`
    /// If no xpath is provided, the entire document will be signed, e.g. `/Document`
    /// Note, this will overwrite any existing signature.
    /// ```rust
    /// use iso_20022_sdk::prelude::*;
    ///
    /// let msg = Message::<_>::builder()
    ///     // Document must be set before signing
    ///     .set_document(doc)
    ///     // Sign the entire document
    ///     .sign_document(&signer, None);
    ///
    ///
    /// ```
    ///
    /// NOTE: The implemented `signer` must return a signature that implements the
    /// `XmlSignature` trait.
    pub fn sign_document(
        self,
        signer: MessageSigner<Doc, Sig, impl signature::Signer<Sig>>,
        x_path_transformations: Vec<XPath>,
    ) -> Result<Self, Error> {
        let signature = signer
            .set_document(self.document())
            .set_x_path_data(x_path_transformations)?
            .try_sign()?;

        // Set the signature in the application header
        let mut app_hdr = self.app_hdr().unwrap_or_default();
        app_hdr.value.sgntr = Some(head::SignatureEnvelope { value: signature });

        // Set the application header and return the envelope
        Ok(self.set_app_hdr(app_hdr))
    }

    /// Set the related business reference of the message.
    pub fn set_rltd(self, rltd: head::BusinessApplicationHeader7<Sig>) -> Self {
        let mut app_hdr = self.app_hdr().unwrap_or_default();
        app_hdr.value.rltd.push(rltd);

        self.set_app_hdr(app_hdr)
    }

    /// Return the envelope document.
    pub fn document(&self) -> Doc {
        self.inner.value.doc.value.clone()
    }

    /// Return the signature of the envelope.
    pub fn signature(&self) -> Option<Sig> {
        self.app_hdr()
            .and_then(|app_hdr| app_hdr.value.sgntr)
            .map(|sgntr| sgntr.value)
    }

    /// Return the serialized xml string of the inner type.
    pub fn to_xml(&self) -> Result<String, Error> {
        let xml_string = quick_xml::se::to_string(&self.inner)?;

        Ok(xml_string)
    }

    /// parse the header from the xml string
    pub fn from_xml(xml_string: &'a str) -> Result<Self, Error> {
        let inner = quick_xml::de::from_str(xml_string)?;

        println!("inner: {:?}", inner);

        let mut msg = Self { xml_string, inner };

        println!("msg: {:?}", msg);

        // Parse the msg into the inner type;
        msg.parse()?;

        Ok(msg)
    }

    fn parse(&mut self) -> Result<(), Error> {
        // Use xml-reader to parse the xml string and find the `MsgDefIdr` element in the `head.001.001.03` namespace.
        let buf_reader = BufReader::new(self.xml_string.as_bytes());
        let event_reader = EventReader::new(buf_reader);

        println!("Parsing xml string...");

        for e in event_reader {
            match e {
                Ok(XmlEvent::ProcessingInstruction { name, data }) => {
                    println!("name: {:?}", name);
                    println!("data: {:?}", data);
                }
                Ok(XmlEvent::StartElement {
                    name:
                        xml::name::OwnedName {
                            local_name,
                            namespace,
                            ..
                        },
                    ..
                }) => {
                    println!("local_name: {:?}", local_name);
                    println!("namespace: {:?}", namespace);
                }
                Ok(XmlEvent::Characters(data)) => {
                    println!("data: {:?}", data);
                }
                _ => (),
            }

            // buf.clear();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use iso_20022_pacs::pacs_008_001_10::FiToFiCustomerCreditTransferV10;

    use super::*;
    use crate::prelude::{ecdsa::EcdsaSignature, pacs, DocumentType, MessageSigner};

    #[test]
    fn test_message_builder() -> Result<(), Error> {
        // Create a random P256 Message Signer
        let signer = MessageSigner::random_p256();

        // Message Details
        let msg_id = "pacs.008.001.10";
        let recipient_id = "b3033215-3a30-48ee-b194-5c02e08a5fb3";
        let sender_id = "b3033215-3a30-48ee-b194-5c02e08a5fb3";

        // Create a document
        // Alternativately, you can create a document from a string
        // e.g., Document::from_namespace("pacs.008.001.10");
        // However, this will require dereferencing the inner document
        // value before mutating the document.
        // It is preferable to use the `DocumentType::pacs` method below
        // to construct documents.
        let doc = Document {
            value: Some(DocumentType::pacs(pacs::Document::pacs_008_001_10(
                pacs::pacs_008_001_10::Document {
                    // Add your document elements here...
                    fi_to_fi_cstmr_cdt_trf: FiToFiCustomerCreditTransferV10 {
                        ..Default::default()
                    },
                    xmlns: pacs::pacs_008_001_10::namespace(),
                },
            ))),
        };

        // Create the message with the msg_id, recipient_id, and sender_id
        let msg = Message::<Document, _>::new(msg_id, recipient_id, sender_id)
            // Set the document
            .set_document(doc)
            // Sign the document
            .sign_document(signer, vec![])?;

        let xml = msg.to_xml()?;

        println!("xml: {}", xml);

        Ok(())
    }

    #[test]
    fn test_parse_message() -> Result<(), Error> {
        let file = std::fs::read_to_string("examples/pacs.xml").expect("Unable to read file");
        let msg = Message::<Document, EcdsaSignature>::from_xml(&file)?;

        println!("msg: {:?}", msg);

        Ok(())
    }
}
