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

// Re-export the iso 20022 colr module
pub use iso_20022_colr::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename = "Document")]
pub enum Document {
    // colr
    colr_001_001_01(iso_20022_colr::colr_001_001_01::Document<Dmkr>),
    colr_002_001_01(iso_20022_colr::colr_002_001_01::Document<Dmkr>),
    colr_003_001_05(iso_20022_colr::colr_003_001_05::Document<Dmkr>),
    colr_004_001_05(iso_20022_colr::colr_004_001_05::Document<Dmkr>),
    colr_005_001_06(iso_20022_colr::colr_005_001_06::Document<Dmkr>),
    colr_006_001_05(iso_20022_colr::colr_006_001_05::Document<Dmkr>),
    colr_007_001_06(iso_20022_colr::colr_007_001_06::Document<Dmkr>),
    colr_008_001_06(iso_20022_colr::colr_008_001_06::Document<Dmkr>),
    colr_009_001_05(iso_20022_colr::colr_009_001_05::Document<Dmkr>),
    colr_010_001_05(iso_20022_colr::colr_010_001_05::Document<Dmkr>),
    colr_011_001_05(iso_20022_colr::colr_011_001_05::Document<Dmkr>),
    colr_012_001_05(iso_20022_colr::colr_012_001_05::Document<Dmkr>),
    colr_013_001_05(iso_20022_colr::colr_013_001_05::Document<Dmkr>),
    colr_014_001_05(iso_20022_colr::colr_014_001_05::Document<Dmkr>),
    colr_015_001_05(iso_20022_colr::colr_015_001_05::Document<Dmkr>),
    colr_016_001_05(iso_20022_colr::colr_016_001_05::Document<Dmkr>),
    colr_019_001_01(iso_20022_colr::colr_019_001_01::Document<Dmkr>),
    colr_020_001_01(iso_20022_colr::colr_020_001_01::Document<Dmkr>),
    colr_021_001_01(iso_20022_colr::colr_021_001_01::Document<Dmkr>),
    colr_022_001_01(iso_20022_colr::colr_022_001_01::Document<Dmkr>),
    colr_023_001_01(iso_20022_colr::colr_023_001_01::Document<Dmkr>),
    colr_024_001_01(iso_20022_colr::colr_024_001_01::Document<Dmkr>),
    #[default]
    Unknown,
}

impl Document {
    /// Set the namespace of the document
    pub fn set_namespace(self) -> Self {
        let mut doc = self;

        match &mut doc {
            Self::colr_001_001_01(d) => d.xmlns = iso_20022_colr::colr_001_001_01::namespace(),
            Self::colr_002_001_01(d) => d.xmlns = iso_20022_colr::colr_002_001_01::namespace(),
            Self::colr_003_001_05(d) => d.xmlns = iso_20022_colr::colr_003_001_05::namespace(),
            Self::colr_004_001_05(d) => d.xmlns = iso_20022_colr::colr_004_001_05::namespace(),
            Self::colr_005_001_06(d) => d.xmlns = iso_20022_colr::colr_005_001_06::namespace(),
            Self::colr_006_001_05(d) => d.xmlns = iso_20022_colr::colr_006_001_05::namespace(),
            Self::colr_007_001_06(d) => d.xmlns = iso_20022_colr::colr_007_001_06::namespace(),
            Self::colr_008_001_06(d) => d.xmlns = iso_20022_colr::colr_008_001_06::namespace(),
            Self::colr_009_001_05(d) => d.xmlns = iso_20022_colr::colr_009_001_05::namespace(),
            Self::colr_010_001_05(d) => d.xmlns = iso_20022_colr::colr_010_001_05::namespace(),
            Self::colr_011_001_05(d) => d.xmlns = iso_20022_colr::colr_011_001_05::namespace(),
            Self::colr_012_001_05(d) => d.xmlns = iso_20022_colr::colr_012_001_05::namespace(),
            Self::colr_013_001_05(d) => d.xmlns = iso_20022_colr::colr_013_001_05::namespace(),
            Self::colr_014_001_05(d) => d.xmlns = iso_20022_colr::colr_014_001_05::namespace(),
            Self::colr_015_001_05(d) => d.xmlns = iso_20022_colr::colr_015_001_05::namespace(),
            Self::colr_016_001_05(d) => d.xmlns = iso_20022_colr::colr_016_001_05::namespace(),
            Self::colr_019_001_01(d) => d.xmlns = iso_20022_colr::colr_019_001_01::namespace(),
            Self::colr_020_001_01(d) => d.xmlns = iso_20022_colr::colr_020_001_01::namespace(),
            Self::colr_021_001_01(d) => d.xmlns = iso_20022_colr::colr_021_001_01::namespace(),
            Self::colr_022_001_01(d) => d.xmlns = iso_20022_colr::colr_022_001_01::namespace(),
            Self::colr_023_001_01(d) => d.xmlns = iso_20022_colr::colr_023_001_01::namespace(),
            Self::colr_024_001_01(d) => d.xmlns = iso_20022_colr::colr_024_001_01::namespace(),
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
            // colr
            "colr.001.001.01" => Document::colr_001_001_01(Default::default()),
            "colr.002.001.01" => Document::colr_002_001_01(Default::default()),
            "colr.003.001.05" => Document::colr_003_001_05(Default::default()),
            "colr.004.001.05" => Document::colr_004_001_05(Default::default()),
            "colr.005.001.06" => Document::colr_005_001_06(Default::default()),
            "colr.006.001.05" => Document::colr_006_001_05(Default::default()),
            "colr.007.001.06" => Document::colr_007_001_06(Default::default()),
            "colr.008.001.06" => Document::colr_008_001_06(Default::default()),
            "colr.009.001.05" => Document::colr_009_001_05(Default::default()),
            "colr.010.001.05" => Document::colr_010_001_05(Default::default()),
            "colr.011.001.05" => Document::colr_011_001_05(Default::default()),
            "colr.012.001.05" => Document::colr_012_001_05(Default::default()),
            "colr.013.001.05" => Document::colr_013_001_05(Default::default()),
            "colr.014.001.05" => Document::colr_014_001_05(Default::default()),
            "colr.015.001.05" => Document::colr_015_001_05(Default::default()),
            "colr.016.001.05" => Document::colr_016_001_05(Default::default()),
            "colr.019.001.01" => Document::colr_019_001_01(Default::default()),
            "colr.020.001.01" => Document::colr_020_001_01(Default::default()),
            "colr.021.001.01" => Document::colr_021_001_01(Default::default()),
            "colr.022.001.01" => Document::colr_022_001_01(Default::default()),
            "colr.023.001.01" => Document::colr_023_001_01(Default::default()),
            "colr.024.001.01" => Document::colr_024_001_01(Default::default()),
            _ => {
                return Err(crate::message::Error::UnsupportedDocumentType(
                    s.to_string(),
                ))
            }
        };

        Ok(doc.set_namespace())
    }
}
