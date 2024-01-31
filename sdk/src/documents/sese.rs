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

// Re-export the iso 20022 sese module
pub use iso_20022_sese::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    // sese
    sese_001_001_09(iso_20022_sese::sese_001_001_09::Document),
    sese_002_001_09(iso_20022_sese::sese_002_001_09::Document),
    sese_003_001_09(iso_20022_sese::sese_003_001_09::Document),
    sese_004_001_09(iso_20022_sese::sese_004_001_09::Document),
    sese_005_001_09(iso_20022_sese::sese_005_001_09::Document),
    sese_006_001_09(iso_20022_sese::sese_006_001_09::Document),
    sese_007_001_09(iso_20022_sese::sese_007_001_09::Document),
    sese_008_001_09(iso_20022_sese::sese_008_001_09::Document),
    sese_009_001_08(iso_20022_sese::sese_009_001_08::Document),
    sese_010_001_07(iso_20022_sese::sese_010_001_07::Document),
    sese_011_001_09(iso_20022_sese::sese_011_001_09::Document),
    sese_012_001_11(iso_20022_sese::sese_012_001_11::Document),
    sese_013_001_11(iso_20022_sese::sese_013_001_11::Document),
    sese_014_001_09(iso_20022_sese::sese_014_001_09::Document),
    sese_018_001_09(iso_20022_sese::sese_018_001_09::Document),
    sese_019_001_08(iso_20022_sese::sese_019_001_08::Document),
    sese_020_001_07(iso_20022_sese::sese_020_001_07::Document<Dmkr>),
    sese_020_002_07(iso_20022_sese::sese_020_002_07::Document<Dmkr>),
    sese_021_001_06(iso_20022_sese::sese_021_001_06::Document<Dmkr>),
    sese_021_002_06(iso_20022_sese::sese_021_002_06::Document<Dmkr>),
    sese_022_001_06(iso_20022_sese::sese_022_001_06::Document<Dmkr>),
    sese_022_002_06(iso_20022_sese::sese_022_002_06::Document<Dmkr>),
    sese_023_001_11(iso_20022_sese::sese_023_001_11::Document<Dmkr>),
    sese_023_002_11(iso_20022_sese::sese_023_002_11::Document<Dmkr>),
    sese_024_001_12(iso_20022_sese::sese_024_001_12::Document<Dmkr>),
    sese_024_002_12(iso_20022_sese::sese_024_002_12::Document<Dmkr>),
    sese_025_001_11(iso_20022_sese::sese_025_001_11::Document<Dmkr>),
    sese_025_002_11(iso_20022_sese::sese_025_002_11::Document<Dmkr>),
    sese_026_001_10(iso_20022_sese::sese_026_001_10::Document<Dmkr>),
    sese_026_002_10(iso_20022_sese::sese_026_002_10::Document<Dmkr>),
    sese_027_001_07(iso_20022_sese::sese_027_001_07::Document<Dmkr>),
    sese_027_002_07(iso_20022_sese::sese_027_002_07::Document<Dmkr>),
    sese_028_001_10(iso_20022_sese::sese_028_001_10::Document<Dmkr>),
    sese_028_002_10(iso_20022_sese::sese_028_002_10::Document<Dmkr>),
    sese_029_001_06(iso_20022_sese::sese_029_001_06::Document<Dmkr>),
    sese_029_002_06(iso_20022_sese::sese_029_002_06::Document<Dmkr>),
    sese_030_001_09(iso_20022_sese::sese_030_001_09::Document<Dmkr>),
    sese_030_002_09(iso_20022_sese::sese_030_002_09::Document<Dmkr>),
    sese_031_001_09(iso_20022_sese::sese_031_001_09::Document<Dmkr>),
    sese_031_002_09(iso_20022_sese::sese_031_002_09::Document<Dmkr>),
    sese_032_001_11(iso_20022_sese::sese_032_001_11::Document<Dmkr>),
    sese_032_002_11(iso_20022_sese::sese_032_002_11::Document<Dmkr>),
    sese_033_001_11(iso_20022_sese::sese_033_001_11::Document<Dmkr>),
    sese_033_002_11(iso_20022_sese::sese_033_002_11::Document<Dmkr>),
    sese_034_001_09(iso_20022_sese::sese_034_001_09::Document<Dmkr>),
    sese_034_002_09(iso_20022_sese::sese_034_002_09::Document<Dmkr>),
    sese_035_001_11(iso_20022_sese::sese_035_001_11::Document<Dmkr>),
    sese_035_002_11(iso_20022_sese::sese_035_002_11::Document<Dmkr>),
    sese_036_001_08(iso_20022_sese::sese_036_001_08::Document<Dmkr>),
    sese_036_002_08(iso_20022_sese::sese_036_002_08::Document<Dmkr>),
    sese_037_001_07(iso_20022_sese::sese_037_001_07::Document<Dmkr>),
    sese_037_002_07(iso_20022_sese::sese_037_002_07::Document<Dmkr>),
    sese_038_001_09(iso_20022_sese::sese_038_001_09::Document<Dmkr, Dmkr, Dmkr>),
    sese_038_002_09(iso_20022_sese::sese_038_002_09::Document<Dmkr, Dmkr, Dmkr>),
    sese_039_001_06(iso_20022_sese::sese_039_001_06::Document<Dmkr>),
    sese_039_002_06(iso_20022_sese::sese_039_002_06::Document<Dmkr>),
    sese_040_001_04(iso_20022_sese::sese_040_001_04::Document<Dmkr>),
    sese_040_002_04(iso_20022_sese::sese_040_002_04::Document<Dmkr>),
    sese_041_001_01(iso_20022_sese::sese_041_001_01::Document<Dmkr>),
    sese_042_001_01(iso_20022_sese::sese_042_001_01::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = crate::message::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // sese
            "sese.001.001.09" => Document::sese_001_001_09(Default::default()),
            "sese.002.001.09" => Document::sese_002_001_09(Default::default()),
            "sese.003.001.09" => Document::sese_003_001_09(Default::default()),
            "sese.004.001.09" => Document::sese_004_001_09(Default::default()),
            "sese.005.001.09" => Document::sese_005_001_09(Default::default()),
            "sese.006.001.09" => Document::sese_006_001_09(Default::default()),
            "sese.007.001.09" => Document::sese_007_001_09(Default::default()),
            "sese.008.001.09" => Document::sese_008_001_09(Default::default()),
            "sese.009.001.08" => Document::sese_009_001_08(Default::default()),
            "sese.010.001.07" => Document::sese_010_001_07(Default::default()),
            "sese.011.001.09" => Document::sese_011_001_09(Default::default()),
            "sese.012.001.11" => Document::sese_012_001_11(Default::default()),
            "sese.013.001.11" => Document::sese_013_001_11(Default::default()),
            "sese.014.001.09" => Document::sese_014_001_09(Default::default()),
            "sese.018.001.09" => Document::sese_018_001_09(Default::default()),
            "sese.019.001.08" => Document::sese_019_001_08(Default::default()),
            "sese.020.001.07" => Document::sese_020_001_07(Default::default()),
            "sese.020.002.07" => Document::sese_020_002_07(Default::default()),
            "sese.021.001.06" => Document::sese_021_001_06(Default::default()),
            "sese.021.002.06" => Document::sese_021_002_06(Default::default()),
            "sese.022.001.06" => Document::sese_022_001_06(Default::default()),
            "sese.022.002.06" => Document::sese_022_002_06(Default::default()),
            "sese.023.001.11" => Document::sese_023_001_11(Default::default()),
            "sese.023.002.11" => Document::sese_023_002_11(Default::default()),
            "sese.024.001.12" => Document::sese_024_001_12(Default::default()),
            "sese.024.002.12" => Document::sese_024_002_12(Default::default()),
            "sese.025.001.11" => Document::sese_025_001_11(Default::default()),
            "sese.025.002.11" => Document::sese_025_002_11(Default::default()),
            "sese.026.001.10" => Document::sese_026_001_10(Default::default()),
            "sese.026.002.10" => Document::sese_026_002_10(Default::default()),
            "sese.027.001.07" => Document::sese_027_001_07(Default::default()),
            "sese.027.002.07" => Document::sese_027_002_07(Default::default()),
            "sese.028.001.10" => Document::sese_028_001_10(Default::default()),
            "sese.028.002.10" => Document::sese_028_002_10(Default::default()),
            "sese.029.001.06" => Document::sese_029_001_06(Default::default()),
            "sese.029.002.06" => Document::sese_029_002_06(Default::default()),
            "sese.030.001.09" => Document::sese_030_001_09(Default::default()),
            "sese.030.002.09" => Document::sese_030_002_09(Default::default()),
            "sese.031.001.09" => Document::sese_031_001_09(Default::default()),
            "sese.031.002.09" => Document::sese_031_002_09(Default::default()),
            "sese.032.001.11" => Document::sese_032_001_11(Default::default()),
            "sese.032.002.11" => Document::sese_032_002_11(Default::default()),
            "sese.033.001.11" => Document::sese_033_001_11(Default::default()),
            "sese.033.002.11" => Document::sese_033_002_11(Default::default()),
            "sese.034.001.09" => Document::sese_034_001_09(Default::default()),
            "sese.034.002.09" => Document::sese_034_002_09(Default::default()),
            "sese.035.001.11" => Document::sese_035_001_11(Default::default()),
            "sese.035.002.11" => Document::sese_035_002_11(Default::default()),
            "sese.036.001.08" => Document::sese_036_001_08(Default::default()),
            "sese.036.002.08" => Document::sese_036_002_08(Default::default()),
            "sese.037.001.07" => Document::sese_037_001_07(Default::default()),
            "sese.037.002.07" => Document::sese_037_002_07(Default::default()),
            "sese.038.001.09" => Document::sese_038_001_09(Default::default()),
            "sese.038.002.09" => Document::sese_038_002_09(Default::default()),
            "sese.039.001.06" => Document::sese_039_001_06(Default::default()),
            "sese.039.002.06" => Document::sese_039_002_06(Default::default()),
            "sese.040.001.04" => Document::sese_040_001_04(Default::default()),
            "sese.040.002.04" => Document::sese_040_002_04(Default::default()),
            "sese.041.001.01" => Document::sese_041_001_01(Default::default()),
            "sese.042.001.01" => Document::sese_042_001_01(Default::default()),
            _ => {
                return Err(crate::message::Error::UnsupportedDocumentType(
                    s.to_string(),
                ))
            }
        };

        Ok(doc)
    }
}
