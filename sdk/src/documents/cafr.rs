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

use super::Dmkr;

// Re-export the iso 20022 cafr module
pub use iso_20022_cafr::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename = "Document")]
pub enum Document {
    // cafr
    cafr_001_001_02(iso_20022_cafr::cafr_001_001_02::Document<Dmkr>),
    cafr_002_001_02(iso_20022_cafr::cafr_002_001_02::Document<Dmkr>),
    cafr_003_001_02(iso_20022_cafr::cafr_003_001_02::Document<Dmkr>),
    cafr_004_001_02(iso_20022_cafr::cafr_004_001_02::Document<Dmkr>),
    #[default]
    Unknown,
}

impl Document {
    /// Set the namespace of the document
    pub fn set_namespace(self) -> Self {
        let mut doc = self;

        match &mut doc {
            Self::cafr_001_001_02(d) => d.xmlns = iso_20022_cafr::cafr_001_001_02::namespace(),
            Self::cafr_002_001_02(d) => d.xmlns = iso_20022_cafr::cafr_002_001_02::namespace(),
            Self::cafr_003_001_02(d) => d.xmlns = iso_20022_cafr::cafr_003_001_02::namespace(),
            Self::cafr_004_001_02(d) => d.xmlns = iso_20022_cafr::cafr_004_001_02::namespace(),
            _ => {
                unimplemented!()
            }
        };

        doc
    }
}

impl TryFrom<&str> for Document {
    type Error = crate::message::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // cafr
            "cafr.001.001.02" => Document::cafr_001_001_02(Default::default()),
            "cafr.002.001.02" => Document::cafr_002_001_02(Default::default()),
            "cafr.003.001.02" => Document::cafr_003_001_02(Default::default()),
            "cafr.004.001.02" => Document::cafr_004_001_02(Default::default()),
            _ => {
                return Err(crate::message::Error::UnsupportedDocumentType(
                    s.to_string(),
                ))
            }
        };

        Ok(doc.set_namespace())
    }
}
