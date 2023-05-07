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
//!
//! ```toml
//! # Cargo.toml
//!
//! [dependencies]
//! iso-20022-sdk = { version = "0.1.0", features = ["remt"] }
//! ```
//!
//! Now you can create a `Document` from the `remt.001.001.01` namespace:
//!
//! ```rust
//! use iso_20022_sdk::Document;
//!
//! let mut doc = Document::from_namespace("remt.001.001.01")?;
//!
//! ```
#[cfg(feature = "acmt")]
pub mod acmt;
#[cfg(feature = "admi")]
pub mod admi;
#[cfg(feature = "auth")]
pub mod auth;
#[cfg(feature = "caaa")]
pub mod caaa;
#[cfg(feature = "caad")]
pub mod caad;
#[cfg(feature = "caam")]
pub mod caam;
#[cfg(feature = "cafc")]
pub mod cafc;
#[cfg(feature = "cafm")]
pub mod cafm;
#[cfg(feature = "cafr")]
pub mod cafr;
#[cfg(feature = "cain")]
pub mod cain;
#[cfg(feature = "camt")]
pub mod camt;
#[cfg(feature = "canm")]
pub mod canm;
#[cfg(feature = "casp")]
pub mod casp;
#[cfg(feature = "casr")]
pub mod casr;
#[cfg(feature = "catm")]
pub mod catm;
#[cfg(feature = "catp")]
pub mod catp;
#[cfg(feature = "colr")]
pub mod colr;
#[cfg(feature = "fxtr")]
pub mod fxtr;
#[cfg(feature = "pacs")]
pub mod pacs;
#[cfg(feature = "pain")]
pub mod pain;
#[cfg(feature = "reda")]
pub mod reda;
#[cfg(feature = "remt")]
pub mod remt;
#[cfg(feature = "secl")]
pub mod secl;
#[cfg(feature = "seev")]
pub mod seev;
#[cfg(feature = "semt")]
pub mod semt;
#[cfg(feature = "sese")]
pub mod sese;
#[cfg(feature = "setr")]
pub mod setr;
#[cfg(test)]
mod tests;
#[cfg(feature = "tsin")]
pub mod tsin;
#[cfg(feature = "tsmt")]
pub mod tsmt;
#[cfg(feature = "tsrv")]
pub mod tsrv;

// default xmlns prefix for iso-20022 documents
const DEFAULT_XLMNS_PREFIX: &str = "urn:iso:std:iso:20022:tech:xsd:";

/// Document Marker (Dmkr) is a type that is used as a default value
/// for `any` element types. It effectively is a default document type.
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
pub struct Dmkr {
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Enumeration of iso-20022 documents
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename = "Document")]
pub enum DocumentType {
    #[cfg(feature = "acmt")]
    acmt(acmt::Document),
    #[cfg(feature = "admi")]
    admi(admi::Document),
    #[cfg(feature = "auth")]
    auth(auth::Document),
    #[cfg(feature = "caaa")]
    caaa(caaa::Document),
    #[cfg(feature = "caad")]
    caad(caad::Document),
    #[cfg(feature = "caam")]
    caam(caam::Document),
    #[cfg(feature = "cafc")]
    cafc(cafc::Document),
    #[cfg(feature = "cafm")]
    cafm(cafm::Document),
    #[cfg(feature = "cafr")]
    cafr(cafr::Document),
    #[cfg(feature = "cain")]
    cain(cain::Document),
    #[cfg(feature = "camt")]
    camt(camt::Document),
    #[cfg(feature = "canm")]
    canm(canm::Document),
    #[cfg(feature = "casp")]
    casp(casp::Document),
    #[cfg(feature = "casr")]
    casr(casr::Document),
    #[cfg(feature = "catm")]
    catm(catm::Document),
    #[cfg(feature = "catp")]
    catp(catp::Document),
    #[cfg(feature = "colr")]
    colr(colr::Document),
    #[cfg(feature = "fxtr")]
    fxtr(fxtr::Document),
    #[cfg(feature = "pacs")]
    pacs(pacs::Document),
    #[cfg(feature = "pain")]
    pain(pain::Document),
    #[cfg(feature = "reda")]
    reda(reda::Document),
    #[cfg(feature = "remt")]
    remt(remt::Document),
    #[cfg(feature = "secl")]
    secl(secl::Document),
    #[cfg(feature = "seev")]
    seev(seev::Document),
    #[cfg(feature = "semt")]
    semt(semt::Document),
    #[cfg(feature = "sese")]
    sese(sese::Document),
    #[cfg(feature = "setr")]
    setr(setr::Document),
    #[cfg(feature = "tsin")]
    tsin(tsin::Document),
    #[cfg(feature = "tsmt")]
    tsmt(tsmt::Document),
    #[cfg(feature = "tsrv")]
    tsrv(tsrv::Document),
    Other(String),
    #[default]
    Unknown,
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::validator::Validate,
)]
#[serde(transparent)]
pub struct Document {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub value: Option<DocumentType>,
}

/// Return the Document based on the namespace
impl Document {
    pub fn from_namespace(namespace: &str) -> Self {
        let _schema = namespace.replace(DEFAULT_XLMNS_PREFIX, "");
        #[cfg(feature = "acmt")]
        if let Ok(doc) = acmt::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::acmt(doc)),
            };
        }
        #[cfg(feature = "admi")]
        if let Ok(doc) = admi::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::admi(doc)),
            };
        }
        #[cfg(feature = "auth")]
        if let Ok(doc) = auth::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::auth(doc)),
            };
        }
        #[cfg(feature = "caaa")]
        if let Ok(doc) = caaa::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::caaa(doc)),
            };
        }
        #[cfg(feature = "caad")]
        if let Ok(doc) = caad::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::caad(doc)),
            };
        }
        #[cfg(feature = "caam")]
        if let Ok(doc) = caam::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::caam(doc)),
            };
        }
        #[cfg(feature = "cafc")]
        if let Ok(doc) = cafc::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::cafc(doc)),
            };
        }
        #[cfg(feature = "cafm")]
        if let Ok(doc) = cafm::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::cafm(doc)),
            };
        }
        #[cfg(feature = "cafr")]
        if let Ok(doc) = cafr::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::cafr(doc)),
            };
        }
        #[cfg(feature = "cain")]
        if let Ok(doc) = cain::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::cain(doc)),
            };
        }
        #[cfg(feature = "camt")]
        if let Ok(doc) = camt::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::camt(doc)),
            };
        }
        #[cfg(feature = "canm")]
        if let Ok(doc) = canm::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::canm(doc)),
            };
        }
        #[cfg(feature = "casp")]
        if let Ok(doc) = casp::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::casp(doc)),
            };
        }
        #[cfg(feature = "casr")]
        if let Ok(doc) = casr::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::casr(doc)),
            };
        }
        #[cfg(feature = "catm")]
        if let Ok(doc) = catm::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::catm(doc)),
            };
        }
        #[cfg(feature = "catp")]
        if let Ok(doc) = catp::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::catp(doc)),
            };
        }
        #[cfg(feature = "colr")]
        if let Ok(doc) = colr::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::colr(doc)),
            };
        }
        #[cfg(feature = "fxtr")]
        if let Ok(doc) = fxtr::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::fxtr(doc)),
            };
        }
        #[cfg(feature = "pacs")]
        if let Ok(doc) = pacs::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::pacs(doc)),
            };
        }
        #[cfg(feature = "pain")]
        if let Ok(doc) = pain::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::pain(doc)),
            };
        }
        #[cfg(feature = "reda")]
        if let Ok(doc) = reda::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::reda(doc)),
            };
        }
        #[cfg(feature = "remt")]
        if let Ok(doc) = remt::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::remt(doc)),
            };
        }
        #[cfg(feature = "secl")]
        if let Ok(doc) = secl::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::secl(doc)),
            };
        }
        #[cfg(feature = "seev")]
        if let Ok(doc) = seev::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::seev(doc)),
            };
        }
        #[cfg(feature = "semt")]
        if let Ok(doc) = semt::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::semt(doc)),
            };
        }
        #[cfg(feature = "sese")]
        if let Ok(doc) = sese::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::sese(doc)),
            };
        }
        #[cfg(feature = "setr")]
        if let Ok(doc) = setr::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::setr(doc)),
            };
        }
        #[cfg(feature = "tsin")]
        if let Ok(doc) = tsin::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::tsin(doc)),
            };
        }
        #[cfg(feature = "tsmt")]
        if let Ok(doc) = tsmt::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::tsmt(doc)),
            };
        }
        #[cfg(feature = "tsrv")]
        if let Ok(doc) = tsrv::Document::try_from(schema.as_str()) {
            return Self {
                value: Some(DocumentType::tsrv(doc)),
            };
        }

        Self { value: None }
    }
}
