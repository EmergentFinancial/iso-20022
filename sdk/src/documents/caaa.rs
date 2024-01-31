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

// Re-export the iso 20022 caaa module
pub use iso_20022_caaa::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename = "Document")]
pub enum Document {
    // caaa
    caaa_001_001_11(iso_20022_caaa::caaa_001_001_11::Document<Dmkr>),
    caaa_002_001_11(iso_20022_caaa::caaa_002_001_11::Document<Dmkr>),
    caaa_003_001_11(iso_20022_caaa::caaa_003_001_11::Document<Dmkr>),
    caaa_004_001_10(iso_20022_caaa::caaa_004_001_10::Document<Dmkr>),
    caaa_005_001_11(iso_20022_caaa::caaa_005_001_11::Document),
    caaa_006_001_10(iso_20022_caaa::caaa_006_001_10::Document),
    caaa_007_001_11(iso_20022_caaa::caaa_007_001_11::Document),
    caaa_008_001_10(iso_20022_caaa::caaa_008_001_10::Document),
    caaa_009_001_10(iso_20022_caaa::caaa_009_001_10::Document),
    caaa_010_001_09(iso_20022_caaa::caaa_010_001_09::Document),
    caaa_011_001_11(iso_20022_caaa::caaa_011_001_11::Document),
    caaa_012_001_10(iso_20022_caaa::caaa_012_001_10::Document),
    caaa_013_001_10(iso_20022_caaa::caaa_013_001_10::Document),
    caaa_014_001_09(iso_20022_caaa::caaa_014_001_09::Document),
    caaa_015_001_06(iso_20022_caaa::caaa_015_001_06::Document),
    caaa_016_001_09(iso_20022_caaa::caaa_016_001_09::Document),
    caaa_017_001_09(iso_20022_caaa::caaa_017_001_09::Document),
    caaa_018_001_06(iso_20022_caaa::caaa_018_001_06::Document),
    caaa_019_001_05(iso_20022_caaa::caaa_019_001_05::Document),
    caaa_020_001_03(iso_20022_caaa::caaa_020_001_03::Document<Dmkr>),
    caaa_021_001_03(iso_20022_caaa::caaa_021_001_03::Document<Dmkr>),
    caaa_022_001_02(iso_20022_caaa::caaa_022_001_02::Document<Dmkr>),
    caaa_023_001_02(iso_20022_caaa::caaa_023_001_02::Document<Dmkr>),
    caaa_024_001_02(iso_20022_caaa::caaa_024_001_02::Document<Dmkr>),
    caaa_025_001_02(iso_20022_caaa::caaa_025_001_02::Document<Dmkr>),
    #[default]
    Unknown,
}

impl Document {
    /// Set the namespace of the document
    pub fn set_namespace(self) -> Self {
        let mut doc = self;

        match &mut doc {
            Self::caaa_001_001_11(d) => d.xmlns = iso_20022_caaa::caaa_001_001_11::namespace(),
            Self::caaa_002_001_11(d) => d.xmlns = iso_20022_caaa::caaa_002_001_11::namespace(),
            Self::caaa_003_001_11(d) => d.xmlns = iso_20022_caaa::caaa_003_001_11::namespace(),
            Self::caaa_004_001_10(d) => d.xmlns = iso_20022_caaa::caaa_004_001_10::namespace(),
            Self::caaa_005_001_11(d) => d.xmlns = iso_20022_caaa::caaa_005_001_11::namespace(),
            Self::caaa_006_001_10(d) => d.xmlns = iso_20022_caaa::caaa_006_001_10::namespace(),
            Self::caaa_007_001_11(d) => d.xmlns = iso_20022_caaa::caaa_007_001_11::namespace(),
            Self::caaa_008_001_10(d) => d.xmlns = iso_20022_caaa::caaa_008_001_10::namespace(),
            Self::caaa_009_001_10(d) => d.xmlns = iso_20022_caaa::caaa_009_001_10::namespace(),
            Self::caaa_010_001_09(d) => d.xmlns = iso_20022_caaa::caaa_010_001_09::namespace(),
            Self::caaa_011_001_11(d) => d.xmlns = iso_20022_caaa::caaa_011_001_11::namespace(),
            Self::caaa_012_001_10(d) => d.xmlns = iso_20022_caaa::caaa_012_001_10::namespace(),
            Self::caaa_013_001_10(d) => d.xmlns = iso_20022_caaa::caaa_013_001_10::namespace(),
            Self::caaa_014_001_09(d) => d.xmlns = iso_20022_caaa::caaa_014_001_09::namespace(),
            Self::caaa_015_001_06(d) => d.xmlns = iso_20022_caaa::caaa_015_001_06::namespace(),
            Self::caaa_016_001_09(d) => d.xmlns = iso_20022_caaa::caaa_016_001_09::namespace(),
            Self::caaa_017_001_09(d) => d.xmlns = iso_20022_caaa::caaa_017_001_09::namespace(),
            Self::caaa_018_001_06(d) => d.xmlns = iso_20022_caaa::caaa_018_001_06::namespace(),
            Self::caaa_019_001_05(d) => d.xmlns = iso_20022_caaa::caaa_019_001_05::namespace(),
            Self::caaa_020_001_03(d) => d.xmlns = iso_20022_caaa::caaa_020_001_03::namespace(),
            Self::caaa_021_001_03(d) => d.xmlns = iso_20022_caaa::caaa_021_001_03::namespace(),
            Self::caaa_022_001_02(d) => d.xmlns = iso_20022_caaa::caaa_022_001_02::namespace(),
            Self::caaa_023_001_02(d) => d.xmlns = iso_20022_caaa::caaa_023_001_02::namespace(),
            Self::caaa_024_001_02(d) => d.xmlns = iso_20022_caaa::caaa_024_001_02::namespace(),
            Self::caaa_025_001_02(d) => d.xmlns = iso_20022_caaa::caaa_025_001_02::namespace(),
            _ => {
                unimplemented!()
            }
        };

        doc
    }
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // caaa
            "caaa.001.001.11" => Document::caaa_001_001_11(Default::default()),
            "caaa.002.001.11" => Document::caaa_002_001_11(Default::default()),
            "caaa.003.001.11" => Document::caaa_003_001_11(Default::default()),
            "caaa.004.001.10" => Document::caaa_004_001_10(Default::default()),
            "caaa.005.001.11" => Document::caaa_005_001_11(Default::default()),
            "caaa.006.001.10" => Document::caaa_006_001_10(Default::default()),
            "caaa.007.001.11" => Document::caaa_007_001_11(Default::default()),
            "caaa.008.001.10" => Document::caaa_008_001_10(Default::default()),
            "caaa.009.001.10" => Document::caaa_009_001_10(Default::default()),
            "caaa.010.001.09" => Document::caaa_010_001_09(Default::default()),
            "caaa.011.001.11" => Document::caaa_011_001_11(Default::default()),
            "caaa.012.001.10" => Document::caaa_012_001_10(Default::default()),
            "caaa.013.001.10" => Document::caaa_013_001_10(Default::default()),
            "caaa.014.001.09" => Document::caaa_014_001_09(Default::default()),
            "caaa.015.001.06" => Document::caaa_015_001_06(Default::default()),
            "caaa.016.001.09" => Document::caaa_016_001_09(Default::default()),
            "caaa.017.001.09" => Document::caaa_017_001_09(Default::default()),
            "caaa.018.001.06" => Document::caaa_018_001_06(Default::default()),
            "caaa.019.001.05" => Document::caaa_019_001_05(Default::default()),
            "caaa.020.001.03" => Document::caaa_020_001_03(Default::default()),
            "caaa.021.001.03" => Document::caaa_021_001_03(Default::default()),
            "caaa.022.001.02" => Document::caaa_022_001_02(Default::default()),
            "caaa.023.001.02" => Document::caaa_023_001_02(Default::default()),
            "caaa.024.001.02" => Document::caaa_024_001_02(Default::default()),
            "caaa.025.001.02" => Document::caaa_025_001_02(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc.set_namespace())
    }
}
