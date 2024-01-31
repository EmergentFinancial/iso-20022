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

// Re-export the iso 20022 seev module
pub use iso_20022_seev::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
#[serde(rename = "Document")]
pub enum Document {
    // seev
    seev_001_001_09(iso_20022_seev::seev_001_001_09::Document<Dmkr>),
    seev_002_001_08(iso_20022_seev::seev_002_001_08::Document<Dmkr>),
    seev_003_001_08(iso_20022_seev::seev_003_001_08::Document<Dmkr>),
    seev_004_001_08(iso_20022_seev::seev_004_001_08::Document<Dmkr>),
    seev_005_001_08(iso_20022_seev::seev_005_001_08::Document<Dmkr>),
    seev_006_001_08(iso_20022_seev::seev_006_001_08::Document<Dmkr>),
    seev_007_001_08(iso_20022_seev::seev_007_001_08::Document<Dmkr>),
    seev_008_001_07(iso_20022_seev::seev_008_001_07::Document<Dmkr>),
    seev_009_001_01(iso_20022_seev::seev_009_001_01::Document),
    seev_010_001_01(iso_20022_seev::seev_010_001_01::Document),
    seev_011_001_01(iso_20022_seev::seev_011_001_01::Document),
    seev_012_001_01(iso_20022_seev::seev_012_001_01::Document),
    seev_013_001_01(iso_20022_seev::seev_013_001_01::Document),
    seev_014_001_01(iso_20022_seev::seev_014_001_01::Document),
    seev_015_001_01(iso_20022_seev::seev_015_001_01::Document),
    seev_016_001_01(iso_20022_seev::seev_016_001_01::Document),
    seev_017_001_01(iso_20022_seev::seev_017_001_01::Document),
    seev_018_001_01(iso_20022_seev::seev_018_001_01::Document),
    seev_019_001_01(iso_20022_seev::seev_019_001_01::Document),
    seev_020_001_01(iso_20022_seev::seev_020_001_01::Document),
    seev_021_001_01(iso_20022_seev::seev_021_001_01::Document),
    seev_022_001_01(iso_20022_seev::seev_022_001_01::Document),
    seev_023_001_01(iso_20022_seev::seev_023_001_01::Document),
    seev_024_001_01(iso_20022_seev::seev_024_001_01::Document),
    seev_025_001_01(iso_20022_seev::seev_025_001_01::Document),
    seev_026_001_01(iso_20022_seev::seev_026_001_01::Document),
    seev_027_001_01(iso_20022_seev::seev_027_001_01::Document),
    seev_028_001_01(iso_20022_seev::seev_028_001_01::Document),
    seev_029_001_01(iso_20022_seev::seev_029_001_01::Document),
    seev_030_001_01(iso_20022_seev::seev_030_001_01::Document),
    seev_031_001_12(iso_20022_seev::seev_031_001_12::Document<Dmkr>),
    seev_031_002_12(iso_20022_seev::seev_031_002_12::Document<Dmkr>),
    seev_032_001_08(iso_20022_seev::seev_032_001_08::Document<Dmkr>),
    seev_032_002_08(iso_20022_seev::seev_032_002_08::Document<Dmkr>),
    seev_033_001_12(iso_20022_seev::seev_033_001_12::Document<Dmkr>),
    seev_033_002_12(iso_20022_seev::seev_033_002_12::Document<Dmkr>),
    seev_034_001_13(iso_20022_seev::seev_034_001_13::Document<Dmkr>),
    seev_034_002_13(iso_20022_seev::seev_034_002_13::Document<Dmkr>),
    seev_035_001_13(iso_20022_seev::seev_035_001_13::Document<Dmkr>),
    seev_035_002_13(iso_20022_seev::seev_035_002_13::Document<Dmkr>),
    seev_036_001_13(iso_20022_seev::seev_036_001_13::Document<Dmkr>),
    seev_036_002_13(iso_20022_seev::seev_036_002_13::Document<Dmkr>),
    seev_037_001_13_0(iso_20022_seev::seev_037_001_13_0::Document<Dmkr>),
    seev_037_002_13(iso_20022_seev::seev_037_002_13::Document<Dmkr>),
    seev_038_001_07(iso_20022_seev::seev_038_001_07::Document<Dmkr>),
    seev_038_002_07(iso_20022_seev::seev_038_002_07::Document<Dmkr>),
    seev_039_001_11(iso_20022_seev::seev_039_001_11::Document<Dmkr>),
    seev_039_002_11(iso_20022_seev::seev_039_002_11::Document<Dmkr>),
    seev_040_001_11(iso_20022_seev::seev_040_001_11::Document<Dmkr>),
    seev_040_002_11(iso_20022_seev::seev_040_002_11::Document<Dmkr>),
    seev_041_001_12(iso_20022_seev::seev_041_001_12::Document<Dmkr>),
    seev_041_002_12(iso_20022_seev::seev_041_002_12::Document<Dmkr>),
    seev_042_001_11(iso_20022_seev::seev_042_001_11::Document<Dmkr, Dmkr>),
    seev_042_002_11(iso_20022_seev::seev_042_002_11::Document<Dmkr, Dmkr>),
    seev_044_001_11(iso_20022_seev::seev_044_001_11::Document<Dmkr>),
    seev_044_002_11(iso_20022_seev::seev_044_002_11::Document<Dmkr>),
    seev_045_001_03(iso_20022_seev::seev_045_001_03::Document<Dmkr>),
    seev_046_001_01(iso_20022_seev::seev_046_001_01::Document<Dmkr>),
    seev_047_001_02(iso_20022_seev::seev_047_001_02::Document<Dmkr, Dmkr>),
    seev_048_001_01(iso_20022_seev::seev_048_001_01::Document<Dmkr>),
    seev_049_001_01(iso_20022_seev::seev_049_001_01::Document<Dmkr>),
    seev_050_001_01(iso_20022_seev::seev_050_001_01::Document<Dmkr>),
    seev_051_001_01(iso_20022_seev::seev_051_001_01::Document<Dmkr>),
    seev_052_001_01(iso_20022_seev::seev_052_001_01::Document<Dmkr>),
    seev_053_001_01(iso_20022_seev::seev_053_001_01::Document<Dmkr>),
    #[default]
    Unknown,
}

impl Document {
    /// Set the namespace of the document
    pub fn set_namespace(self) -> Self {
        let mut doc = self;

        match &mut doc {
            Self::seev_001_001_09(d) => d.xmlns = iso_20022_seev::seev_001_001_09::namespace(),
            Self::seev_002_001_08(d) => d.xmlns = iso_20022_seev::seev_002_001_08::namespace(),
            Self::seev_003_001_08(d) => d.xmlns = iso_20022_seev::seev_003_001_08::namespace(),
            Self::seev_004_001_08(d) => d.xmlns = iso_20022_seev::seev_004_001_08::namespace(),
            Self::seev_005_001_08(d) => d.xmlns = iso_20022_seev::seev_005_001_08::namespace(),
            Self::seev_006_001_08(d) => d.xmlns = iso_20022_seev::seev_006_001_08::namespace(),
            Self::seev_007_001_08(d) => d.xmlns = iso_20022_seev::seev_007_001_08::namespace(),
            Self::seev_008_001_07(d) => d.xmlns = iso_20022_seev::seev_008_001_07::namespace(),
            Self::seev_009_001_01(d) => d.xmlns = iso_20022_seev::seev_009_001_01::namespace(),
            Self::seev_010_001_01(d) => d.xmlns = iso_20022_seev::seev_010_001_01::namespace(),
            Self::seev_011_001_01(d) => d.xmlns = iso_20022_seev::seev_011_001_01::namespace(),
            Self::seev_012_001_01(d) => d.xmlns = iso_20022_seev::seev_012_001_01::namespace(),
            Self::seev_013_001_01(d) => d.xmlns = iso_20022_seev::seev_013_001_01::namespace(),
            Self::seev_014_001_01(d) => d.xmlns = iso_20022_seev::seev_014_001_01::namespace(),
            Self::seev_015_001_01(d) => d.xmlns = iso_20022_seev::seev_015_001_01::namespace(),
            Self::seev_016_001_01(d) => d.xmlns = iso_20022_seev::seev_016_001_01::namespace(),
            Self::seev_017_001_01(d) => d.xmlns = iso_20022_seev::seev_017_001_01::namespace(),
            Self::seev_018_001_01(d) => d.xmlns = iso_20022_seev::seev_018_001_01::namespace(),
            Self::seev_019_001_01(d) => d.xmlns = iso_20022_seev::seev_019_001_01::namespace(),
            Self::seev_020_001_01(d) => d.xmlns = iso_20022_seev::seev_020_001_01::namespace(),
            Self::seev_021_001_01(d) => d.xmlns = iso_20022_seev::seev_021_001_01::namespace(),
            Self::seev_022_001_01(d) => d.xmlns = iso_20022_seev::seev_022_001_01::namespace(),
            Self::seev_023_001_01(d) => d.xmlns = iso_20022_seev::seev_023_001_01::namespace(),
            Self::seev_024_001_01(d) => d.xmlns = iso_20022_seev::seev_024_001_01::namespace(),
            Self::seev_025_001_01(d) => d.xmlns = iso_20022_seev::seev_025_001_01::namespace(),
            Self::seev_026_001_01(d) => d.xmlns = iso_20022_seev::seev_026_001_01::namespace(),
            Self::seev_027_001_01(d) => d.xmlns = iso_20022_seev::seev_027_001_01::namespace(),
            Self::seev_028_001_01(d) => d.xmlns = iso_20022_seev::seev_028_001_01::namespace(),
            Self::seev_029_001_01(d) => d.xmlns = iso_20022_seev::seev_029_001_01::namespace(),
            Self::seev_030_001_01(d) => d.xmlns = iso_20022_seev::seev_030_001_01::namespace(),
            Self::seev_031_001_12(d) => d.xmlns = iso_20022_seev::seev_031_001_12::namespace(),
            Self::seev_031_002_12(d) => d.xmlns = iso_20022_seev::seev_031_002_12::namespace(),
            Self::seev_032_001_08(d) => d.xmlns = iso_20022_seev::seev_032_001_08::namespace(),
            Self::seev_032_002_08(d) => d.xmlns = iso_20022_seev::seev_032_002_08::namespace(),
            Self::seev_033_001_12(d) => d.xmlns = iso_20022_seev::seev_033_001_12::namespace(),
            Self::seev_033_002_12(d) => d.xmlns = iso_20022_seev::seev_033_002_12::namespace(),
            Self::seev_034_001_13(d) => d.xmlns = iso_20022_seev::seev_034_001_13::namespace(),
            Self::seev_034_002_13(d) => d.xmlns = iso_20022_seev::seev_034_002_13::namespace(),
            Self::seev_035_001_13(d) => d.xmlns = iso_20022_seev::seev_035_001_13::namespace(),
            Self::seev_035_002_13(d) => d.xmlns = iso_20022_seev::seev_035_002_13::namespace(),
            Self::seev_036_001_13(d) => d.xmlns = iso_20022_seev::seev_036_001_13::namespace(),
            Self::seev_036_002_13(d) => d.xmlns = iso_20022_seev::seev_036_002_13::namespace(),
            Self::seev_037_001_13_0(d) => d.xmlns = iso_20022_seev::seev_037_001_13_0::namespace(),
            Self::seev_037_002_13(d) => d.xmlns = iso_20022_seev::seev_037_002_13::namespace(),
            Self::seev_038_001_07(d) => d.xmlns = iso_20022_seev::seev_038_001_07::namespace(),
            Self::seev_038_002_07(d) => d.xmlns = iso_20022_seev::seev_038_002_07::namespace(),
            Self::seev_039_001_11(d) => d.xmlns = iso_20022_seev::seev_039_001_11::namespace(),
            Self::seev_039_002_11(d) => d.xmlns = iso_20022_seev::seev_039_002_11::namespace(),
            Self::seev_040_001_11(d) => d.xmlns = iso_20022_seev::seev_040_001_11::namespace(),
            Self::seev_040_002_11(d) => d.xmlns = iso_20022_seev::seev_040_002_11::namespace(),
            Self::seev_041_001_12(d) => d.xmlns = iso_20022_seev::seev_041_001_12::namespace(),
            Self::seev_041_002_12(d) => d.xmlns = iso_20022_seev::seev_041_002_12::namespace(),
            Self::seev_042_001_11(d) => d.xmlns = iso_20022_seev::seev_042_001_11::namespace(),
            Self::seev_042_002_11(d) => d.xmlns = iso_20022_seev::seev_042_002_11::namespace(),
            Self::seev_044_001_11(d) => d.xmlns = iso_20022_seev::seev_044_001_11::namespace(),
            Self::seev_044_002_11(d) => d.xmlns = iso_20022_seev::seev_044_002_11::namespace(),
            Self::seev_045_001_03(d) => d.xmlns = iso_20022_seev::seev_045_001_03::namespace(),
            Self::seev_046_001_01(d) => d.xmlns = iso_20022_seev::seev_046_001_01::namespace(),
            Self::seev_047_001_02(d) => d.xmlns = iso_20022_seev::seev_047_001_02::namespace(),
            Self::seev_048_001_01(d) => d.xmlns = iso_20022_seev::seev_048_001_01::namespace(),
            Self::seev_049_001_01(d) => d.xmlns = iso_20022_seev::seev_049_001_01::namespace(),
            Self::seev_050_001_01(d) => d.xmlns = iso_20022_seev::seev_050_001_01::namespace(),
            Self::seev_051_001_01(d) => d.xmlns = iso_20022_seev::seev_051_001_01::namespace(),
            Self::seev_052_001_01(d) => d.xmlns = iso_20022_seev::seev_052_001_01::namespace(),
            Self::seev_053_001_01(d) => d.xmlns = iso_20022_seev::seev_053_001_01::namespace(),
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
            // seev
            "seev.001.001.09" => Document::seev_001_001_09(Default::default()),
            "seev.002.001.08" => Document::seev_002_001_08(Default::default()),
            "seev.003.001.08" => Document::seev_003_001_08(Default::default()),
            "seev.004.001.08" => Document::seev_004_001_08(Default::default()),
            "seev.005.001.08" => Document::seev_005_001_08(Default::default()),
            "seev.006.001.08" => Document::seev_006_001_08(Default::default()),
            "seev.007.001.08" => Document::seev_007_001_08(Default::default()),
            "seev.008.001.07" => Document::seev_008_001_07(Default::default()),
            "seev.009.001.01" => Document::seev_009_001_01(Default::default()),
            "seev.010.001.01" => Document::seev_010_001_01(Default::default()),
            "seev.011.001.01" => Document::seev_011_001_01(Default::default()),
            "seev.012.001.01" => Document::seev_012_001_01(Default::default()),
            "seev.013.001.01" => Document::seev_013_001_01(Default::default()),
            "seev.014.001.01" => Document::seev_014_001_01(Default::default()),
            "seev.015.001.01" => Document::seev_015_001_01(Default::default()),
            "seev.016.001.01" => Document::seev_016_001_01(Default::default()),
            "seev.017.001.01" => Document::seev_017_001_01(Default::default()),
            "seev.018.001.01" => Document::seev_018_001_01(Default::default()),
            "seev.019.001.01" => Document::seev_019_001_01(Default::default()),
            "seev.020.001.01" => Document::seev_020_001_01(Default::default()),
            "seev.021.001.01" => Document::seev_021_001_01(Default::default()),
            "seev.022.001.01" => Document::seev_022_001_01(Default::default()),
            "seev.023.001.01" => Document::seev_023_001_01(Default::default()),
            "seev.024.001.01" => Document::seev_024_001_01(Default::default()),
            "seev.025.001.01" => Document::seev_025_001_01(Default::default()),
            "seev.026.001.01" => Document::seev_026_001_01(Default::default()),
            "seev.027.001.01" => Document::seev_027_001_01(Default::default()),
            "seev.028.001.01" => Document::seev_028_001_01(Default::default()),
            "seev.029.001.01" => Document::seev_029_001_01(Default::default()),
            "seev.030.001.01" => Document::seev_030_001_01(Default::default()),
            "seev.031.001.12" => Document::seev_031_001_12(Default::default()),
            "seev.031.002.12" => Document::seev_031_002_12(Default::default()),
            "seev.032.001.08" => Document::seev_032_001_08(Default::default()),
            "seev.032.002.08" => Document::seev_032_002_08(Default::default()),
            "seev.033.001.12" => Document::seev_033_001_12(Default::default()),
            "seev.033.002.12" => Document::seev_033_002_12(Default::default()),
            "seev.034.001.13" => Document::seev_034_001_13(Default::default()),
            "seev.034.002.13" => Document::seev_034_002_13(Default::default()),
            "seev.035.001.13" => Document::seev_035_001_13(Default::default()),
            "seev.035.002.13" => Document::seev_035_002_13(Default::default()),
            "seev.036.001.13" => Document::seev_036_001_13(Default::default()),
            "seev.036.002.13" => Document::seev_036_002_13(Default::default()),
            "seev.037.001.13_0" => Document::seev_037_001_13_0(Default::default()),
            "seev.037.002.13" => Document::seev_037_002_13(Default::default()),
            "seev.038.001.07" => Document::seev_038_001_07(Default::default()),
            "seev.038.002.07" => Document::seev_038_002_07(Default::default()),
            "seev.039.001.11" => Document::seev_039_001_11(Default::default()),
            "seev.039.002.11" => Document::seev_039_002_11(Default::default()),
            "seev.040.001.11" => Document::seev_040_001_11(Default::default()),
            "seev.040.002.11" => Document::seev_040_002_11(Default::default()),
            "seev.041.001.12" => Document::seev_041_001_12(Default::default()),
            "seev.041.002.12" => Document::seev_041_002_12(Default::default()),
            "seev.042.001.11" => Document::seev_042_001_11(Default::default()),
            "seev.042.002.11" => Document::seev_042_002_11(Default::default()),
            "seev.044.001.11" => Document::seev_044_001_11(Default::default()),
            "seev.044.002.11" => Document::seev_044_002_11(Default::default()),
            "seev.045.001.03" => Document::seev_045_001_03(Default::default()),
            "seev.046.001.01" => Document::seev_046_001_01(Default::default()),
            "seev.047.001.02" => Document::seev_047_001_02(Default::default()),
            "seev.048.001.01" => Document::seev_048_001_01(Default::default()),
            "seev.049.001.01" => Document::seev_049_001_01(Default::default()),
            "seev.050.001.01" => Document::seev_050_001_01(Default::default()),
            "seev.051.001.01" => Document::seev_051_001_01(Default::default()),
            "seev.052.001.01" => Document::seev_052_001_01(Default::default()),
            "seev.053.001.01" => Document::seev_053_001_01(Default::default()),
            _ => {
                return Err(crate::message::Error::UnsupportedDocumentType(
                    s.to_string(),
                ))
            }
        };

        Ok(doc.set_namespace())
    }
}
