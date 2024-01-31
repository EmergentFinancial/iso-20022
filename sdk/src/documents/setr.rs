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

// Re-export the iso 20022 setr module
pub use iso_20022_setr::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename = "Document")]
pub enum Document {
    // setr
    setr_001_001_04(iso_20022_setr::setr_001_001_04::Document),
    setr_002_001_04(iso_20022_setr::setr_002_001_04::Document),
    setr_003_001_04(iso_20022_setr::setr_003_001_04::Document),
    setr_004_001_04(iso_20022_setr::setr_004_001_04::Document),
    setr_005_001_04(iso_20022_setr::setr_005_001_04::Document),
    setr_006_001_04(iso_20022_setr::setr_006_001_04::Document),
    setr_007_001_04(iso_20022_setr::setr_007_001_04::Document),
    setr_008_001_04(iso_20022_setr::setr_008_001_04::Document),
    setr_009_001_04(iso_20022_setr::setr_009_001_04::Document),
    setr_010_001_04(iso_20022_setr::setr_010_001_04::Document),
    setr_011_001_04(iso_20022_setr::setr_011_001_04::Document),
    setr_012_001_04(iso_20022_setr::setr_012_001_04::Document),
    setr_013_001_04(iso_20022_setr::setr_013_001_04::Document),
    setr_014_001_04(iso_20022_setr::setr_014_001_04::Document),
    setr_015_001_04(iso_20022_setr::setr_015_001_04::Document),
    setr_016_001_04(iso_20022_setr::setr_016_001_04::Document),
    setr_017_001_04(iso_20022_setr::setr_017_001_04::Document),
    setr_018_001_04(iso_20022_setr::setr_018_001_04::Document),
    setr_027_001_03(iso_20022_setr::setr_027_001_03::Document<Dmkr>),
    setr_029_001_01(iso_20022_setr::setr_029_001_01::Document<Dmkr>),
    setr_030_001_01(iso_20022_setr::setr_030_001_01::Document<Dmkr>),
    setr_044_001_02(iso_20022_setr::setr_044_001_02::Document<Dmkr>),
    setr_047_001_02(iso_20022_setr::setr_047_001_02::Document),
    setr_049_001_02(iso_20022_setr::setr_049_001_02::Document),
    setr_051_001_02(iso_20022_setr::setr_051_001_02::Document),
    setr_053_001_02(iso_20022_setr::setr_053_001_02::Document),
    setr_055_001_02(iso_20022_setr::setr_055_001_02::Document),
    setr_057_001_02(iso_20022_setr::setr_057_001_02::Document),
    setr_058_001_02(iso_20022_setr::setr_058_001_02::Document),
    #[default]
    Unknown,
}

impl Document {
    /// Set the namespace of the document
    pub fn set_namespace(self) -> Self {
        let mut doc = self;

        match &mut doc {
            Self::setr_001_001_04(d) => d.xmlns = iso_20022_setr::setr_001_001_04::namespace(),
            Self::setr_002_001_04(d) => d.xmlns = iso_20022_setr::setr_002_001_04::namespace(),
            Self::setr_003_001_04(d) => d.xmlns = iso_20022_setr::setr_003_001_04::namespace(),
            Self::setr_004_001_04(d) => d.xmlns = iso_20022_setr::setr_004_001_04::namespace(),
            Self::setr_005_001_04(d) => d.xmlns = iso_20022_setr::setr_005_001_04::namespace(),
            Self::setr_006_001_04(d) => d.xmlns = iso_20022_setr::setr_006_001_04::namespace(),
            Self::setr_007_001_04(d) => d.xmlns = iso_20022_setr::setr_007_001_04::namespace(),
            Self::setr_008_001_04(d) => d.xmlns = iso_20022_setr::setr_008_001_04::namespace(),
            Self::setr_009_001_04(d) => d.xmlns = iso_20022_setr::setr_009_001_04::namespace(),
            Self::setr_010_001_04(d) => d.xmlns = iso_20022_setr::setr_010_001_04::namespace(),
            Self::setr_011_001_04(d) => d.xmlns = iso_20022_setr::setr_011_001_04::namespace(),
            Self::setr_012_001_04(d) => d.xmlns = iso_20022_setr::setr_012_001_04::namespace(),
            Self::setr_013_001_04(d) => d.xmlns = iso_20022_setr::setr_013_001_04::namespace(),
            Self::setr_014_001_04(d) => d.xmlns = iso_20022_setr::setr_014_001_04::namespace(),
            Self::setr_015_001_04(d) => d.xmlns = iso_20022_setr::setr_015_001_04::namespace(),
            Self::setr_016_001_04(d) => d.xmlns = iso_20022_setr::setr_016_001_04::namespace(),
            Self::setr_017_001_04(d) => d.xmlns = iso_20022_setr::setr_017_001_04::namespace(),
            Self::setr_018_001_04(d) => d.xmlns = iso_20022_setr::setr_018_001_04::namespace(),
            Self::setr_027_001_03(d) => d.xmlns = iso_20022_setr::setr_027_001_03::namespace(),
            Self::setr_029_001_01(d) => d.xmlns = iso_20022_setr::setr_029_001_01::namespace(),
            Self::setr_030_001_01(d) => d.xmlns = iso_20022_setr::setr_030_001_01::namespace(),
            Self::setr_044_001_02(d) => d.xmlns = iso_20022_setr::setr_044_001_02::namespace(),
            Self::setr_047_001_02(d) => d.xmlns = iso_20022_setr::setr_047_001_02::namespace(),
            Self::setr_049_001_02(d) => d.xmlns = iso_20022_setr::setr_049_001_02::namespace(),
            Self::setr_051_001_02(d) => d.xmlns = iso_20022_setr::setr_051_001_02::namespace(),
            Self::setr_053_001_02(d) => d.xmlns = iso_20022_setr::setr_053_001_02::namespace(),
            Self::setr_055_001_02(d) => d.xmlns = iso_20022_setr::setr_055_001_02::namespace(),
            Self::setr_057_001_02(d) => d.xmlns = iso_20022_setr::setr_057_001_02::namespace(),
            Self::setr_058_001_02(d) => d.xmlns = iso_20022_setr::setr_058_001_02::namespace(),
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
            // setr
            "setr.001.001.04" => Document::setr_001_001_04(Default::default()),
            "setr.002.001.04" => Document::setr_002_001_04(Default::default()),
            "setr.003.001.04" => Document::setr_003_001_04(Default::default()),
            "setr.004.001.04" => Document::setr_004_001_04(Default::default()),
            "setr.005.001.04" => Document::setr_005_001_04(Default::default()),
            "setr.006.001.04" => Document::setr_006_001_04(Default::default()),
            "setr.007.001.04" => Document::setr_007_001_04(Default::default()),
            "setr.008.001.04" => Document::setr_008_001_04(Default::default()),
            "setr.009.001.04" => Document::setr_009_001_04(Default::default()),
            "setr.010.001.04" => Document::setr_010_001_04(Default::default()),
            "setr.011.001.04" => Document::setr_011_001_04(Default::default()),
            "setr.012.001.04" => Document::setr_012_001_04(Default::default()),
            "setr.013.001.04" => Document::setr_013_001_04(Default::default()),
            "setr.014.001.04" => Document::setr_014_001_04(Default::default()),
            "setr.015.001.04" => Document::setr_015_001_04(Default::default()),
            "setr.016.001.04" => Document::setr_016_001_04(Default::default()),
            "setr.017.001.04" => Document::setr_017_001_04(Default::default()),
            "setr.018.001.04" => Document::setr_018_001_04(Default::default()),
            "setr.027.001.03" => Document::setr_027_001_03(Default::default()),
            "setr.029.001.01" => Document::setr_029_001_01(Default::default()),
            "setr.030.001.01" => Document::setr_030_001_01(Default::default()),
            "setr.044.001.02" => Document::setr_044_001_02(Default::default()),
            "setr.047.001.02" => Document::setr_047_001_02(Default::default()),
            "setr.049.001.02" => Document::setr_049_001_02(Default::default()),
            "setr.051.001.02" => Document::setr_051_001_02(Default::default()),
            "setr.053.001.02" => Document::setr_053_001_02(Default::default()),
            "setr.055.001.02" => Document::setr_055_001_02(Default::default()),
            "setr.057.001.02" => Document::setr_057_001_02(Default::default()),
            "setr.058.001.02" => Document::setr_058_001_02(Default::default()),
            _ => {
                return Err(crate::message::Error::UnsupportedDocumentType(
                    s.to_string(),
                ))
            }
        };

        Ok(doc.set_namespace())
    }
}
