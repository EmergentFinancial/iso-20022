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

// Re-export the iso 20022 casp module
pub use iso_20022_casp::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename = "Document")]
pub enum Document {
    // casp
    casp_001_001_04(iso_20022_casp::casp_001_001_04::Document<Dmkr>),
    casp_002_001_04(iso_20022_casp::casp_002_001_04::Document<Dmkr>),
    casp_003_001_04(iso_20022_casp::casp_003_001_04::Document<Dmkr>),
    casp_004_001_04(iso_20022_casp::casp_004_001_04::Document<Dmkr>),
    casp_005_001_04(iso_20022_casp::casp_005_001_04::Document<Dmkr>),
    casp_006_001_04(iso_20022_casp::casp_006_001_04::Document<Dmkr, Dmkr>),
    casp_007_001_04(iso_20022_casp::casp_007_001_04::Document<Dmkr>),
    casp_008_001_04(iso_20022_casp::casp_008_001_04::Document<Dmkr>),
    casp_009_001_04(iso_20022_casp::casp_009_001_04::Document<Dmkr>),
    casp_010_001_04(iso_20022_casp::casp_010_001_04::Document<Dmkr>),
    casp_011_001_04(iso_20022_casp::casp_011_001_04::Document<Dmkr>),
    casp_012_001_04(iso_20022_casp::casp_012_001_04::Document<Dmkr>),
    casp_013_001_02(iso_20022_casp::casp_013_001_02::Document),
    casp_014_001_04(iso_20022_casp::casp_014_001_04::Document<Dmkr>),
    casp_015_001_04(iso_20022_casp::casp_015_001_04::Document<Dmkr>),
    casp_016_001_04(iso_20022_casp::casp_016_001_04::Document<Dmkr>),
    casp_017_001_04(iso_20022_casp::casp_017_001_04::Document<Dmkr>),
    #[default]
    Unknown,
}

impl Document {
    /// Set the namespace of the document
    pub fn set_namespace(self) -> Self {
        let mut doc = self;

        match &mut doc {
            Self::casp_001_001_04(d) => d.xmlns = iso_20022_casp::casp_001_001_04::namespace(),
            Self::casp_002_001_04(d) => d.xmlns = iso_20022_casp::casp_002_001_04::namespace(),
            Self::casp_003_001_04(d) => d.xmlns = iso_20022_casp::casp_003_001_04::namespace(),
            Self::casp_004_001_04(d) => d.xmlns = iso_20022_casp::casp_004_001_04::namespace(),
            Self::casp_005_001_04(d) => d.xmlns = iso_20022_casp::casp_005_001_04::namespace(),
            Self::casp_006_001_04(d) => d.xmlns = iso_20022_casp::casp_006_001_04::namespace(),
            Self::casp_007_001_04(d) => d.xmlns = iso_20022_casp::casp_007_001_04::namespace(),
            Self::casp_008_001_04(d) => d.xmlns = iso_20022_casp::casp_008_001_04::namespace(),
            Self::casp_009_001_04(d) => d.xmlns = iso_20022_casp::casp_009_001_04::namespace(),
            Self::casp_010_001_04(d) => d.xmlns = iso_20022_casp::casp_010_001_04::namespace(),
            Self::casp_011_001_04(d) => d.xmlns = iso_20022_casp::casp_011_001_04::namespace(),
            Self::casp_012_001_04(d) => d.xmlns = iso_20022_casp::casp_012_001_04::namespace(),
            Self::casp_013_001_02(d) => d.xmlns = iso_20022_casp::casp_013_001_02::namespace(),
            Self::casp_014_001_04(d) => d.xmlns = iso_20022_casp::casp_014_001_04::namespace(),
            Self::casp_015_001_04(d) => d.xmlns = iso_20022_casp::casp_015_001_04::namespace(),
            Self::casp_016_001_04(d) => d.xmlns = iso_20022_casp::casp_016_001_04::namespace(),
            Self::casp_017_001_04(d) => d.xmlns = iso_20022_casp::casp_017_001_04::namespace(),
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
            // casp
            "casp.001.001.04" => Document::casp_001_001_04(Default::default()),
            "casp.002.001.04" => Document::casp_002_001_04(Default::default()),
            "casp.003.001.04" => Document::casp_003_001_04(Default::default()),
            "casp.004.001.04" => Document::casp_004_001_04(Default::default()),
            "casp.005.001.04" => Document::casp_005_001_04(Default::default()),
            "casp.006.001.04" => Document::casp_006_001_04(Default::default()),
            "casp.007.001.04" => Document::casp_007_001_04(Default::default()),
            "casp.008.001.04" => Document::casp_008_001_04(Default::default()),
            "casp.009.001.04" => Document::casp_009_001_04(Default::default()),
            "casp.010.001.04" => Document::casp_010_001_04(Default::default()),
            "casp.011.001.04" => Document::casp_011_001_04(Default::default()),
            "casp.012.001.04" => Document::casp_012_001_04(Default::default()),
            "casp.013.001.02" => Document::casp_013_001_02(Default::default()),
            "casp.014.001.04" => Document::casp_014_001_04(Default::default()),
            "casp.015.001.04" => Document::casp_015_001_04(Default::default()),
            "casp.016.001.04" => Document::casp_016_001_04(Default::default()),
            "casp.017.001.04" => Document::casp_017_001_04(Default::default()),
            _ => {
                return Err(crate::message::Error::UnsupportedDocumentType(
                    s.to_string(),
                ))
            }
        };

        Ok(doc.set_namespace())
    }
}
