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

// Re-export the iso 20022 tsrv module
pub use iso_20022_tsrv::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename = "Document")]
pub enum Document {
    // tsrv
    tsrv_001_001_01(iso_20022_tsrv::tsrv_001_001_01::Document<Dmkr, Dmkr>),
    tsrv_002_001_01(iso_20022_tsrv::tsrv_002_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    tsrv_003_001_01(iso_20022_tsrv::tsrv_003_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    tsrv_004_001_01(iso_20022_tsrv::tsrv_004_001_01::Document<Dmkr, Dmkr>),
    tsrv_005_001_01(iso_20022_tsrv::tsrv_005_001_01::Document<Dmkr, Dmkr>),
    tsrv_006_001_01(iso_20022_tsrv::tsrv_006_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    tsrv_007_001_01(iso_20022_tsrv::tsrv_007_001_01::Document<Dmkr, Dmkr, Dmkr>),
    tsrv_008_001_01(iso_20022_tsrv::tsrv_008_001_01::Document<Dmkr>),
    tsrv_009_001_01(iso_20022_tsrv::tsrv_009_001_01::Document<Dmkr, Dmkr>),
    tsrv_010_001_01(iso_20022_tsrv::tsrv_010_001_01::Document<Dmkr>),
    tsrv_011_001_01(iso_20022_tsrv::tsrv_011_001_01::Document<Dmkr>),
    tsrv_012_001_01(iso_20022_tsrv::tsrv_012_001_01::Document<Dmkr, Dmkr>),
    tsrv_013_001_01(iso_20022_tsrv::tsrv_013_001_01::Document<Dmkr, Dmkr>),
    tsrv_014_001_01(iso_20022_tsrv::tsrv_014_001_01::Document<Dmkr, Dmkr>),
    tsrv_015_001_01(iso_20022_tsrv::tsrv_015_001_01::Document<Dmkr>),
    tsrv_016_001_01(iso_20022_tsrv::tsrv_016_001_01::Document<Dmkr>),
    tsrv_017_001_01(iso_20022_tsrv::tsrv_017_001_01::Document<Dmkr>),
    tsrv_018_001_01(iso_20022_tsrv::tsrv_018_001_01::Document<Dmkr>),
    tsrv_019_001_01(iso_20022_tsrv::tsrv_019_001_01::Document<Dmkr, Dmkr>),
    #[default]
    Unknown,
}

impl Document {
    /// Set the namespace of the document
    pub fn set_namespace(self) -> Self {
        let mut doc = self;

        match &mut doc {
            Self::tsrv_001_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_001_001_01::namespace(),
            Self::tsrv_002_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_002_001_01::namespace(),
            Self::tsrv_003_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_003_001_01::namespace(),
            Self::tsrv_004_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_004_001_01::namespace(),
            Self::tsrv_005_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_005_001_01::namespace(),
            Self::tsrv_006_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_006_001_01::namespace(),
            Self::tsrv_007_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_007_001_01::namespace(),
            Self::tsrv_008_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_008_001_01::namespace(),
            Self::tsrv_009_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_009_001_01::namespace(),
            Self::tsrv_010_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_010_001_01::namespace(),
            Self::tsrv_011_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_011_001_01::namespace(),
            Self::tsrv_012_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_012_001_01::namespace(),
            Self::tsrv_013_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_013_001_01::namespace(),
            Self::tsrv_014_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_014_001_01::namespace(),
            Self::tsrv_015_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_015_001_01::namespace(),
            Self::tsrv_016_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_016_001_01::namespace(),
            Self::tsrv_017_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_017_001_01::namespace(),
            Self::tsrv_018_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_018_001_01::namespace(),
            Self::tsrv_019_001_01(d) => d.xmlns = iso_20022_tsrv::tsrv_019_001_01::namespace(),
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
            // tsrv
            "tsrv.001.001.01" => Document::tsrv_001_001_01(Default::default()),
            "tsrv.002.001.01" => Document::tsrv_002_001_01(Default::default()),
            "tsrv.003.001.01" => Document::tsrv_003_001_01(Default::default()),
            "tsrv.004.001.01" => Document::tsrv_004_001_01(Default::default()),
            "tsrv.005.001.01" => Document::tsrv_005_001_01(Default::default()),
            "tsrv.006.001.01" => Document::tsrv_006_001_01(Default::default()),
            "tsrv.007.001.01" => Document::tsrv_007_001_01(Default::default()),
            "tsrv.008.001.01" => Document::tsrv_008_001_01(Default::default()),
            "tsrv.009.001.01" => Document::tsrv_009_001_01(Default::default()),
            "tsrv.010.001.01" => Document::tsrv_010_001_01(Default::default()),
            "tsrv.011.001.01" => Document::tsrv_011_001_01(Default::default()),
            "tsrv.012.001.01" => Document::tsrv_012_001_01(Default::default()),
            "tsrv.013.001.01" => Document::tsrv_013_001_01(Default::default()),
            "tsrv.014.001.01" => Document::tsrv_014_001_01(Default::default()),
            "tsrv.015.001.01" => Document::tsrv_015_001_01(Default::default()),
            "tsrv.016.001.01" => Document::tsrv_016_001_01(Default::default()),
            "tsrv.017.001.01" => Document::tsrv_017_001_01(Default::default()),
            "tsrv.018.001.01" => Document::tsrv_018_001_01(Default::default()),
            "tsrv.019.001.01" => Document::tsrv_019_001_01(Default::default()),
            _ => {
                return Err(crate::message::Error::UnsupportedDocumentType(
                    s.to_string(),
                ))
            }
        };

        Ok(doc.set_namespace())
    }
}
