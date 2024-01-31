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

// Re-export the iso 20022 tsin module
pub use iso_20022_tsin::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename = "Document")]
pub enum Document {
    // tsin
    tsin_001_001_01(iso_20022_tsin::tsin_001_001_01::Document),
    tsin_002_001_01(iso_20022_tsin::tsin_002_001_01::Document),
    tsin_003_001_01(iso_20022_tsin::tsin_003_001_01::Document),
    tsin_005_001_01(iso_20022_tsin::tsin_005_001_01::Document<Dmkr, Dmkr>),
    tsin_006_001_01(iso_20022_tsin::tsin_006_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    tsin_007_001_01(iso_20022_tsin::tsin_007_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    tsin_008_001_01(iso_20022_tsin::tsin_008_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    tsin_009_001_01(iso_20022_tsin::tsin_009_001_01::Document<Dmkr, Dmkr, Dmkr>),
    tsin_010_001_01(iso_20022_tsin::tsin_010_001_01::Document<Dmkr, Dmkr, Dmkr>),
    tsin_011_001_01(iso_20022_tsin::tsin_011_001_01::Document<Dmkr, Dmkr, Dmkr>),
    tsin_012_001_01(iso_20022_tsin::tsin_012_001_01::Document<Dmkr, Dmkr, Dmkr>),
    tsin_013_001_01(iso_20022_tsin::tsin_013_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    #[default]
    Unknown,
}

impl Document {
    /// Set the namespace of the document
    pub fn set_namespace(self) -> Self {
        let mut doc = self;

        match &mut doc {
            Self::tsin_001_001_01(d) => d.xmlns = iso_20022_tsin::tsin_001_001_01::namespace(),
            Self::tsin_002_001_01(d) => d.xmlns = iso_20022_tsin::tsin_002_001_01::namespace(),
            Self::tsin_003_001_01(d) => d.xmlns = iso_20022_tsin::tsin_003_001_01::namespace(),
            Self::tsin_005_001_01(d) => d.xmlns = iso_20022_tsin::tsin_005_001_01::namespace(),
            Self::tsin_006_001_01(d) => d.xmlns = iso_20022_tsin::tsin_006_001_01::namespace(),
            Self::tsin_007_001_01(d) => d.xmlns = iso_20022_tsin::tsin_007_001_01::namespace(),
            Self::tsin_008_001_01(d) => d.xmlns = iso_20022_tsin::tsin_008_001_01::namespace(),
            Self::tsin_009_001_01(d) => d.xmlns = iso_20022_tsin::tsin_009_001_01::namespace(),
            Self::tsin_010_001_01(d) => d.xmlns = iso_20022_tsin::tsin_010_001_01::namespace(),
            Self::tsin_011_001_01(d) => d.xmlns = iso_20022_tsin::tsin_011_001_01::namespace(),
            Self::tsin_012_001_01(d) => d.xmlns = iso_20022_tsin::tsin_012_001_01::namespace(),
            Self::tsin_013_001_01(d) => d.xmlns = iso_20022_tsin::tsin_013_001_01::namespace(),
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
            // tsin
            "tsin.001.001.01" => Document::tsin_001_001_01(Default::default()),
            "tsin.002.001.01" => Document::tsin_002_001_01(Default::default()),
            "tsin.003.001.01" => Document::tsin_003_001_01(Default::default()),
            "tsin.005.001.01" => Document::tsin_005_001_01(Default::default()),
            "tsin.006.001.01" => Document::tsin_006_001_01(Default::default()),
            "tsin.007.001.01" => Document::tsin_007_001_01(Default::default()),
            "tsin.008.001.01" => Document::tsin_008_001_01(Default::default()),
            "tsin.009.001.01" => Document::tsin_009_001_01(Default::default()),
            "tsin.010.001.01" => Document::tsin_010_001_01(Default::default()),
            "tsin.011.001.01" => Document::tsin_011_001_01(Default::default()),
            "tsin.012.001.01" => Document::tsin_012_001_01(Default::default()),
            "tsin.013.001.01" => Document::tsin_013_001_01(Default::default()),
            _ => {
                return Err(crate::message::Error::UnsupportedDocumentType(
                    s.to_string(),
                ))
            }
        };

        Ok(doc.set_namespace())
    }
}
