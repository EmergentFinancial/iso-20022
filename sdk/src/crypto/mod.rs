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

use iso_20022_dsig::{dsig::DigestValueType, xpath::XPath};

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

/// Xml signature trait provides common methods for
/// constructing valid XML signatures with various
/// signing algorithms.
pub trait XmlSignature {
    /// Public Key Type for the Signature
    type PublicKey;

    /// Set the signed information of the signatured
    /// URI is the section of the XML document to be signed.
    /// XPath is the transformations on the URI of the document to be signed.
    /// The digest value is the hash value of the xpath evaluated document to be signed
    fn set_signed_info(
        self,
        uri: String,
        x_path_transformations: Vec<XPath>,
        digest_value: DigestValueType,
        public_key: Self::PublicKey,
    ) -> Self;
}

/// Message signer provides a common interface for signing messages
/// with different algorithms.
pub struct MessageSigner<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    inner: A,
}

impl<A> MessageSigner<A>
where
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
{
    /// Create a new message signer
    pub fn new() -> Self {
        Self {
            inner: A::default(),
        }
    }
}

// Implement Deref for MessageSigner
impl<A> std::ops::Deref for MessageSigner<A>
where
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
{
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
