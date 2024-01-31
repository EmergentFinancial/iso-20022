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

// Re-export the iso 20022 caam module
pub use iso_20022_caam::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename = "Document")]
pub enum Document {
    // caam
    caam_001_001_03(iso_20022_caam::caam_001_001_03::Document),
    caam_002_001_03(iso_20022_caam::caam_002_001_03::Document),
    caam_003_001_03(iso_20022_caam::caam_003_001_03::Document),
    caam_004_001_03(iso_20022_caam::caam_004_001_03::Document),
    caam_005_001_02(iso_20022_caam::caam_005_001_02::Document),
    caam_006_001_02(iso_20022_caam::caam_006_001_02::Document),
    caam_007_001_01(iso_20022_caam::caam_007_001_01::Document),
    caam_008_001_01(iso_20022_caam::caam_008_001_01::Document),
    caam_009_001_02(iso_20022_caam::caam_009_001_02::Document),
    caam_010_001_02(iso_20022_caam::caam_010_001_02::Document),
    caam_011_001_01(iso_20022_caam::caam_011_001_01::Document),
    caam_012_001_01(iso_20022_caam::caam_012_001_01::Document),
    #[default]
    Unknown,
}

impl Document {
    /// Set the namespace of the document
    pub fn set_namespace(self) -> Self {
        let mut doc = self;

        match &mut doc {
            Self::caam_001_001_03(d) => d.xmlns = iso_20022_caam::caam_001_001_03::namespace(),
            Self::caam_002_001_03(d) => d.xmlns = iso_20022_caam::caam_002_001_03::namespace(),
            Self::caam_003_001_03(d) => d.xmlns = iso_20022_caam::caam_003_001_03::namespace(),
            Self::caam_004_001_03(d) => d.xmlns = iso_20022_caam::caam_004_001_03::namespace(),
            Self::caam_005_001_02(d) => d.xmlns = iso_20022_caam::caam_005_001_02::namespace(),
            Self::caam_006_001_02(d) => d.xmlns = iso_20022_caam::caam_006_001_02::namespace(),
            Self::caam_007_001_01(d) => d.xmlns = iso_20022_caam::caam_007_001_01::namespace(),
            Self::caam_008_001_01(d) => d.xmlns = iso_20022_caam::caam_008_001_01::namespace(),
            Self::caam_009_001_02(d) => d.xmlns = iso_20022_caam::caam_009_001_02::namespace(),
            Self::caam_010_001_02(d) => d.xmlns = iso_20022_caam::caam_010_001_02::namespace(),
            Self::caam_011_001_01(d) => d.xmlns = iso_20022_caam::caam_011_001_01::namespace(),
            Self::caam_012_001_01(d) => d.xmlns = iso_20022_caam::caam_012_001_01::namespace(),
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
            // caam
            "caam.001.001.03" => Document::caam_001_001_03(Default::default()),
            "caam.002.001.03" => Document::caam_002_001_03(Default::default()),
            "caam.003.001.03" => Document::caam_003_001_03(Default::default()),
            "caam.004.001.03" => Document::caam_004_001_03(Default::default()),
            "caam.005.001.02" => Document::caam_005_001_02(Default::default()),
            "caam.006.001.02" => Document::caam_006_001_02(Default::default()),
            "caam.007.001.01" => Document::caam_007_001_01(Default::default()),
            "caam.008.001.01" => Document::caam_008_001_01(Default::default()),
            "caam.009.001.02" => Document::caam_009_001_02(Default::default()),
            "caam.010.001.02" => Document::caam_010_001_02(Default::default()),
            "caam.011.001.01" => Document::caam_011_001_01(Default::default()),
            "caam.012.001.01" => Document::caam_012_001_01(Default::default()),
            _ => {
                return Err(crate::message::Error::UnsupportedDocumentType(
                    s.to_string(),
                ))
            }
        };

        Ok(doc.set_namespace())
    }
}
