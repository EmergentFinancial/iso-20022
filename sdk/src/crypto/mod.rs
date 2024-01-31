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

use iso_20022_dsig::dsig::{Base64Binary, DigestValueType, SignedInfoType, ID};
use iso_20022_dsig::ecdsa::{EcPointType, HexBinary};
use iso_20022_dsig::xpath::{FilterType, XPath, XPathType};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sxd_document::parser;
use sxd_xpath::evaluate_xpath;

/// ECDSA signature algorithms.
#[cfg(feature = "ecdsa")]
pub mod ecdsa;

/// Default canonicalization method algorithm.
const DEFAULT_CANONICALIZATION_METHOD_ALGORITHM: &str = "http://www.w3.org/2001/10/xml-exc-c14n#";

/// Default signature method algorithm.
const DEFAULT_SIGNATURE_METHOD_ALGORITHM: &str = "http://www.w3.org/2001/04/xmldsig-more";

/// Default Digest Method algorithm
const DEFAULT_DIGEST_METHOD_ALGORITHM: &str = "http://www.w3.org/2001/04/xmlenc#sha256";

/// HMAC Output Length for SHA-256 hash (32 bytes)
const SHA_256_HMAC_OUTPUT_LENGTH: i64 = 32;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error Serializing / Deserializing XML
    #[error(transparent)]
    XmlSerDe(#[from] quick_xml::de::DeError),
    /// SXD Document Error
    #[error(transparent)]
    XsdDocument(#[from] sxd_document::parser::Error),
    /// SXD XPath Error
    #[error(transparent)]
    XsdXPath(#[from] sxd_xpath::Error),
    /// Signing Error
    #[error(transparent)]
    Signing(#[from] signature::Error),
}

/// Xml Public Key trait provides common methods for
/// converting public keys to XML Signature format.
/// i.e. EcPointValue coordinates for ECDSA keys.
pub trait XmlPublicKey: Clone {
    /// Public Key Type for the Signature
    type PublicKey;

    /// Convert the public key to XML Signature format
    fn to_ec_point(&self) -> EcPointType<HexBinary, HexBinary>;

    /// Convert the public key to ASN.1 DER document encoded as a byte vector
    fn to_der_bytes(self) -> Vec<u8>;

    /// Return the Object ID (OID) of the public key, e.g. 1.2.840.10045.2.1
    fn oid(&self) -> String;
}

/// Xml signature trait provides common methods for
/// constructing valid XML signatures with various
/// signing algorithms.
pub trait XmlSignature {
    /// Public Key Type for the Signature
    type PublicKey: XmlPublicKey;

    /// Digest Type for the Signature, e.g. Sha256
    type Digest;

    /// Set the signed information of the signatured
    /// URI is the section of the XML document to be signed.
    /// XPath is the transformations on the URI of the document to be signed.
    /// The digest value is the digest value of the xpath evaluated document to be signed
    fn set_signed_info(
        self,
        x_path_transformations: Vec<XPath>,
        digest_value: DigestValueType,
        public_key: Self::PublicKey,
        uri: Option<String>,
    ) -> Self;

    /// Set the Signature value
    fn set_signature_value(self, signature_value: Base64Binary, id: Option<String>) -> Self;

    /// Set the key info for the signature
    fn set_key_info(self, public_key: Self::PublicKey) -> Self;

    /// Return the digest value of the signature info
    fn digest_value(&self) -> Option<DigestValueType>;

    /// Hash the data to be signed and return the digest value
    fn hash(data: &[u8]) -> DigestValueType;
}

/// Message signer provides a common interface for signing messages
/// with different algorithms.
#[derive(Default, Clone, Debug)]
pub struct MessageSigner<
    Doc: std::fmt::Debug
        + Default
        + Clone
        + PartialEq
        + ::serde::Serialize
        // + ::serde::Deserialize<'a>
        + ::validator::Validate,
    Sig: std::fmt::Debug
        + Default
        + Clone
        + PartialEq
        + ::serde::Serialize
        // + ::serde::Deserialize<'a>
        + ::validator::Validate
        + XmlSignature,
    Signer: signature::Signer<Sig>,
> {
    /// Signing Key
    signer: Signer,

    /// Public / Verifier Key
    verifier: Sig::PublicKey,

    /// Document to be signed, prior to XPath transformations
    document: Doc,

    /// XPath Transformations to apply to the document
    x_path_transformations: Vec<XPath>,

    /// XPath Transformed Data
    x_path_data: Option<Vec<u8>>,

    /// URI path to the document to be signed
    uri: Option<String>,
}

impl<Doc, Sig, Signer> MessageSigner<Doc, Sig, Signer>
where
    Doc: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    Sig: std::fmt::Debug
        + Default
        + Clone
        + PartialEq
        + ::serde::Serialize
        + ::validator::Validate
        + XmlSignature,
    Signer: signature::Signer<Sig>,
{
    /// Create a new message signer
    pub fn new(signer: Signer, verifier: Sig::PublicKey) -> Self {
        Self {
            signer,
            verifier,
            document: Doc::default(),
            x_path_transformations: Vec::new(),
            x_path_data: None,
            uri: None,
        }
    }

    /// Set the document for the message signer
    pub fn set_document(self, document: Doc) -> Self {
        Self { document, ..self }
    }

    // /// Set the XPath transformations for the message signer
    // pub fn set_x_path_transformations(self, x_path_transformations: Vec<XPath>) -> Self {
    //     Self {
    //         x_path_transformations,
    //         ..self
    //     }
    // }

    /// Set the URI for the message signer
    pub fn set_uri(self, uri: Option<String>) -> Self {
        Self { uri, ..self }
    }

    /// Returns the pre-hash digest of the document data to be signed, after XPath transformations.
    pub fn set_x_path_data(self, x_path_transformations: Vec<XPath>) -> Result<Self, Error> {
        let xml = quick_xml::se::to_string(&self.document)?;
        let package = parser::parse(&xml)?;

        let doc = package.as_document();
        let mut uri = self.uri;

        // TODO: Handle deduplication
        // TODO: Intersection and Unions are handled as unions
        let mut data = x_path_transformations.iter().fold(
            String::new(),
            |mut data,
             XPath {
                 value: XPathType { value, filter },
             }| {
                let exp = evaluate_xpath(&doc, value)
                    // TODO: Identify a better way to handle XML string of evaluated xpath
                    .map(|s| s.into_string())
                    .unwrap_or_default();

                match filter {
                    FilterType::Subtract => {
                        data = data.replace(&exp, "");
                    }
                    FilterType::Intersect | FilterType::Union => data.push_str(&exp),
                    _ => {}
                };

                // Return the updated data to be signed
                data
            },
        );

        // By default, sign the entire document if no xpath transformations are found
        if data.is_empty() {
            data = xml;
            uri = Some(String::from("/Document"));
        }

        Ok(Self {
            x_path_data: Some(data.as_bytes().to_vec()),
            uri,
            x_path_transformations,
            ..self
        })
    }

    /// Sign the document
    pub fn try_sign(&self) -> Result<Sig, Error> {
        let digest_value = Sig::hash(&self.x_path_data.clone().unwrap_or_default());

        let signature = self
            .signer
            .try_sign(digest_value.value.as_bytes())
            .map_err(|e| Error::Signing(e))?
            .set_signed_info(
                self.x_path_transformations.clone(),
                digest_value,
                self.verifier.clone(),
                self.uri.clone(),
            )
            .set_key_info(self.verifier.clone());

        Ok(signature)
    }
}

#[cfg(feature = "ecdsa")]
impl<Doc> MessageSigner<Doc, ecdsa::EcdsaSignature, ecdsa::EcdsaSigner>
where
    Doc: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
{
    /// Create a random P256 Signer
    #[cfg(feature = "p256")]
    pub fn random_p256() -> MessageSigner<Doc, ecdsa::EcdsaSignature, ecdsa::EcdsaSigner> {
        use p256::ecdsa::SigningKey;
        use p256::ecdsa::VerifyingKey;
        use rand_core::OsRng;

        let sk = SigningKey::random(&mut OsRng);
        let public_key = ecdsa::EcdsaPublicKey::P256(VerifyingKey::from(&sk).into());
        let signing_key = ecdsa::EcdsaSigner::P256(sk);

        MessageSigner::new(signing_key, public_key)
    }
}
