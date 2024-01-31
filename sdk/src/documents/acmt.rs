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

// Re-export the iso 20022 acmt module
pub use iso_20022_acmt::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    acmt_001_001_08(iso_20022_acmt::acmt_001_001_08::Document),
    acmt_002_001_08(iso_20022_acmt::acmt_002_001_08::Document),
    acmt_003_001_08(iso_20022_acmt::acmt_003_001_08::Document),
    acmt_005_001_06(iso_20022_acmt::acmt_005_001_06::Document),
    acmt_006_001_07(iso_20022_acmt::acmt_006_001_07::Document),
    acmt_007_001_04(iso_20022_acmt::acmt_007_001_04::Document<Dmkr, Dmkr>),
    acmt_008_001_04(iso_20022_acmt::acmt_008_001_04::Document<Dmkr, Dmkr>),
    acmt_009_001_03(iso_20022_acmt::acmt_009_001_03::Document<Dmkr, Dmkr>),
    acmt_010_001_03(iso_20022_acmt::acmt_010_001_03::Document<Dmkr, Dmkr>),
    acmt_011_001_03(iso_20022_acmt::acmt_011_001_03::Document<Dmkr, Dmkr>),
    acmt_012_001_03(iso_20022_acmt::acmt_012_001_03::Document<Dmkr, Dmkr>),
    acmt_013_001_03(iso_20022_acmt::acmt_013_001_03::Document<Dmkr, Dmkr>),
    acmt_014_001_04(iso_20022_acmt::acmt_014_001_04::Document<Dmkr, Dmkr>),
    acmt_015_001_03(iso_20022_acmt::acmt_015_001_03::Document<Dmkr, Dmkr>),
    acmt_016_001_03(iso_20022_acmt::acmt_016_001_03::Document<Dmkr, Dmkr>),
    acmt_017_001_03_0(iso_20022_acmt::acmt_017_001_03_0::Document<Dmkr, Dmkr>),
    acmt_018_001_03(iso_20022_acmt::acmt_018_001_03::Document<Dmkr, Dmkr>),
    acmt_019_001_03(iso_20022_acmt::acmt_019_001_03::Document<Dmkr, Dmkr>),
    acmt_020_001_03(iso_20022_acmt::acmt_020_001_03::Document<Dmkr, Dmkr>),
    acmt_021_001_03(iso_20022_acmt::acmt_021_001_03::Document<Dmkr, Dmkr>),
    acmt_022_001_03(iso_20022_acmt::acmt_022_001_03::Document<Dmkr>),
    acmt_023_001_03(iso_20022_acmt::acmt_023_001_03::Document<Dmkr>),
    acmt_024_001_03(iso_20022_acmt::acmt_024_001_03::Document<Dmkr>),
    acmt_027_001_04(iso_20022_acmt::acmt_027_001_04::Document<Dmkr>),
    acmt_028_001_04(iso_20022_acmt::acmt_028_001_04::Document<Dmkr>),
    acmt_029_001_04(iso_20022_acmt::acmt_029_001_04::Document<Dmkr>),
    acmt_030_001_03(iso_20022_acmt::acmt_030_001_03::Document<Dmkr>),
    acmt_031_001_04(iso_20022_acmt::acmt_031_001_04::Document<Dmkr>),
    acmt_032_001_04(iso_20022_acmt::acmt_032_001_04::Document<Dmkr>),
    acmt_033_001_02(iso_20022_acmt::acmt_033_001_02::Document<Dmkr>),
    acmt_034_001_04(iso_20022_acmt::acmt_034_001_04::Document<Dmkr>),
    acmt_035_001_02(iso_20022_acmt::acmt_035_001_02::Document<Dmkr>),
    acmt_036_001_01(iso_20022_acmt::acmt_036_001_01::Document<Dmkr>),
    acmt_037_001_02(iso_20022_acmt::acmt_037_001_02::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = crate::message::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // acmt
            "acmt.001.001.08" => Document::acmt_001_001_08(Default::default()),
            "acmt.002.001.08" => Document::acmt_002_001_08(Default::default()),
            "acmt.003.001.08" => Document::acmt_003_001_08(Default::default()),
            "acmt.005.001.06" => Document::acmt_005_001_06(Default::default()),
            "acmt.006.001.07" => Document::acmt_006_001_07(Default::default()),
            "acmt.007.001.04" => Document::acmt_007_001_04(Default::default()),
            "acmt.008.001.04" => Document::acmt_008_001_04(Default::default()),
            "acmt.009.001.03" => Document::acmt_009_001_03(Default::default()),
            "acmt.010.001.03" => Document::acmt_010_001_03(Default::default()),
            "acmt.011.001.03" => Document::acmt_011_001_03(Default::default()),
            "acmt.012.001.03" => Document::acmt_012_001_03(Default::default()),
            "acmt.013.001.03" => Document::acmt_013_001_03(Default::default()),
            "acmt.014.001.04" => Document::acmt_014_001_04(Default::default()),
            "acmt.015.001.03" => Document::acmt_015_001_03(Default::default()),
            "acmt.016.001.03" => Document::acmt_016_001_03(Default::default()),
            "acmt.017.001.03_0" => Document::acmt_017_001_03_0(Default::default()),
            "acmt.018.001.03" => Document::acmt_018_001_03(Default::default()),
            "acmt.019.001.03" => Document::acmt_019_001_03(Default::default()),
            "acmt.020.001.03" => Document::acmt_020_001_03(Default::default()),
            "acmt.021.001.03" => Document::acmt_021_001_03(Default::default()),
            "acmt.022.001.03" => Document::acmt_022_001_03(Default::default()),
            "acmt.023.001.03" => Document::acmt_023_001_03(Default::default()),
            "acmt.024.001.03" => Document::acmt_024_001_03(Default::default()),
            "acmt.027.001.04" => Document::acmt_027_001_04(Default::default()),
            "acmt.028.001.04" => Document::acmt_028_001_04(Default::default()),
            "acmt.029.001.04" => Document::acmt_029_001_04(Default::default()),
            "acmt.030.001.03" => Document::acmt_030_001_03(Default::default()),
            "acmt.031.001.04" => Document::acmt_031_001_04(Default::default()),
            "acmt.032.001.04" => Document::acmt_032_001_04(Default::default()),
            "acmt.033.001.02" => Document::acmt_033_001_02(Default::default()),
            "acmt.034.001.04" => Document::acmt_034_001_04(Default::default()),
            "acmt.035.001.02" => Document::acmt_035_001_02(Default::default()),
            "acmt.036.001.01" => Document::acmt_036_001_01(Default::default()),
            "acmt.037.001.02" => Document::acmt_037_001_02(Default::default()),
            _ => {
                return Err(crate::message::Error::UnsupportedDocumentType(
                    s.to_string(),
                ))
            }
        };

        Ok(doc)
    }
}
