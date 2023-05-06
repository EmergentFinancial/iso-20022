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

// Re-export the iso 20022 tsmt module
pub use iso_20022_tsmt::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    // tsmt
    tsmt_001_001_03(iso_20022_tsmt::tsmt_001_001_03::Document),
    tsmt_002_001_04(iso_20022_tsmt::tsmt_002_001_04::Document),
    tsmt_003_001_03(iso_20022_tsmt::tsmt_003_001_03::Document),
    tsmt_004_001_02(iso_20022_tsmt::tsmt_004_001_02::Document),
    tsmt_005_001_02(iso_20022_tsmt::tsmt_005_001_02::Document),
    tsmt_006_001_03(iso_20022_tsmt::tsmt_006_001_03::Document),
    tsmt_007_001_02(iso_20022_tsmt::tsmt_007_001_02::Document),
    tsmt_008_001_03(iso_20022_tsmt::tsmt_008_001_03::Document),
    tsmt_009_001_05(iso_20022_tsmt::tsmt_009_001_05::Document),
    tsmt_010_001_03(iso_20022_tsmt::tsmt_010_001_03::Document),
    tsmt_011_001_04(iso_20022_tsmt::tsmt_011_001_04::Document),
    tsmt_012_001_05(iso_20022_tsmt::tsmt_012_001_05::Document),
    tsmt_013_001_03(iso_20022_tsmt::tsmt_013_001_03::Document),
    tsmt_014_001_05(iso_20022_tsmt::tsmt_014_001_05::Document),
    tsmt_015_001_03(iso_20022_tsmt::tsmt_015_001_03::Document),
    tsmt_016_001_03(iso_20022_tsmt::tsmt_016_001_03::Document),
    tsmt_017_001_05(iso_20022_tsmt::tsmt_017_001_05::Document),
    tsmt_018_001_05(iso_20022_tsmt::tsmt_018_001_05::Document),
    tsmt_019_001_05(iso_20022_tsmt::tsmt_019_001_05::Document),
    tsmt_020_001_02(iso_20022_tsmt::tsmt_020_001_02::Document),
    tsmt_021_001_03(iso_20022_tsmt::tsmt_021_001_03::Document),
    tsmt_022_001_02(iso_20022_tsmt::tsmt_022_001_02::Document),
    tsmt_023_001_03(iso_20022_tsmt::tsmt_023_001_03::Document),
    tsmt_024_001_03(iso_20022_tsmt::tsmt_024_001_03::Document),
    tsmt_025_001_03(iso_20022_tsmt::tsmt_025_001_03::Document),
    tsmt_026_001_02(iso_20022_tsmt::tsmt_026_001_02::Document),
    tsmt_027_001_02(iso_20022_tsmt::tsmt_027_001_02::Document),
    tsmt_028_001_03(iso_20022_tsmt::tsmt_028_001_03::Document),
    tsmt_029_001_02(iso_20022_tsmt::tsmt_029_001_02::Document),
    tsmt_030_001_03(iso_20022_tsmt::tsmt_030_001_03::Document),
    tsmt_031_001_03(iso_20022_tsmt::tsmt_031_001_03::Document),
    tsmt_032_001_03(iso_20022_tsmt::tsmt_032_001_03::Document),
    tsmt_033_001_03(iso_20022_tsmt::tsmt_033_001_03::Document),
    tsmt_034_001_03(iso_20022_tsmt::tsmt_034_001_03::Document),
    tsmt_035_001_03(iso_20022_tsmt::tsmt_035_001_03::Document),
    tsmt_036_001_03(iso_20022_tsmt::tsmt_036_001_03::Document),
    tsmt_038_001_03(iso_20022_tsmt::tsmt_038_001_03::Document),
    tsmt_040_001_03(iso_20022_tsmt::tsmt_040_001_03::Document),
    tsmt_041_001_03(iso_20022_tsmt::tsmt_041_001_03::Document),
    tsmt_042_001_03(iso_20022_tsmt::tsmt_042_001_03::Document),
    tsmt_044_001_02(iso_20022_tsmt::tsmt_044_001_02::Document),
    tsmt_045_001_02(iso_20022_tsmt::tsmt_045_001_02::Document),
    tsmt_046_001_01(iso_20022_tsmt::tsmt_046_001_01::Document),
    tsmt_047_001_01(iso_20022_tsmt::tsmt_047_001_01::Document),
    tsmt_048_001_01(iso_20022_tsmt::tsmt_048_001_01::Document),
    tsmt_049_001_01(iso_20022_tsmt::tsmt_049_001_01::Document),
    tsmt_050_001_01(iso_20022_tsmt::tsmt_050_001_01::Document),
    tsmt_051_001_01(iso_20022_tsmt::tsmt_051_001_01::Document),
    tsmt_052_001_01(iso_20022_tsmt::tsmt_052_001_01::Document),
    tsmt_053_001_01(iso_20022_tsmt::tsmt_053_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    tsmt_054_001_01(iso_20022_tsmt::tsmt_054_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    tsmt_055_001_01(iso_20022_tsmt::tsmt_055_001_01::Document<Dmkr, Dmkr, Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // tsmt
            "tsmt.001.001.03" => Document::tsmt_001_001_03(Default::default()),
            "tsmt.002.001.04" => Document::tsmt_002_001_04(Default::default()),
            "tsmt.003.001.03" => Document::tsmt_003_001_03(Default::default()),
            "tsmt.004.001.02" => Document::tsmt_004_001_02(Default::default()),
            "tsmt.005.001.02" => Document::tsmt_005_001_02(Default::default()),
            "tsmt.006.001.03" => Document::tsmt_006_001_03(Default::default()),
            "tsmt.007.001.02" => Document::tsmt_007_001_02(Default::default()),
            "tsmt.008.001.03" => Document::tsmt_008_001_03(Default::default()),
            "tsmt.009.001.05" => Document::tsmt_009_001_05(Default::default()),
            "tsmt.010.001.03" => Document::tsmt_010_001_03(Default::default()),
            "tsmt.011.001.04" => Document::tsmt_011_001_04(Default::default()),
            "tsmt.012.001.05" => Document::tsmt_012_001_05(Default::default()),
            "tsmt.013.001.03" => Document::tsmt_013_001_03(Default::default()),
            "tsmt.014.001.05" => Document::tsmt_014_001_05(Default::default()),
            "tsmt.015.001.03" => Document::tsmt_015_001_03(Default::default()),
            "tsmt.016.001.03" => Document::tsmt_016_001_03(Default::default()),
            "tsmt.017.001.05" => Document::tsmt_017_001_05(Default::default()),
            "tsmt.018.001.05" => Document::tsmt_018_001_05(Default::default()),
            "tsmt.019.001.05" => Document::tsmt_019_001_05(Default::default()),
            "tsmt.020.001.02" => Document::tsmt_020_001_02(Default::default()),
            "tsmt.021.001.03" => Document::tsmt_021_001_03(Default::default()),
            "tsmt.022.001.02" => Document::tsmt_022_001_02(Default::default()),
            "tsmt.023.001.03" => Document::tsmt_023_001_03(Default::default()),
            "tsmt.024.001.03" => Document::tsmt_024_001_03(Default::default()),
            "tsmt.025.001.03" => Document::tsmt_025_001_03(Default::default()),
            "tsmt.026.001.02" => Document::tsmt_026_001_02(Default::default()),
            "tsmt.027.001.02" => Document::tsmt_027_001_02(Default::default()),
            "tsmt.028.001.03" => Document::tsmt_028_001_03(Default::default()),
            "tsmt.029.001.02" => Document::tsmt_029_001_02(Default::default()),
            "tsmt.030.001.03" => Document::tsmt_030_001_03(Default::default()),
            "tsmt.031.001.03" => Document::tsmt_031_001_03(Default::default()),
            "tsmt.032.001.03" => Document::tsmt_032_001_03(Default::default()),
            "tsmt.033.001.03" => Document::tsmt_033_001_03(Default::default()),
            "tsmt.034.001.03" => Document::tsmt_034_001_03(Default::default()),
            "tsmt.035.001.03" => Document::tsmt_035_001_03(Default::default()),
            "tsmt.036.001.03" => Document::tsmt_036_001_03(Default::default()),
            "tsmt.038.001.03" => Document::tsmt_038_001_03(Default::default()),
            "tsmt.040.001.03" => Document::tsmt_040_001_03(Default::default()),
            "tsmt.041.001.03" => Document::tsmt_041_001_03(Default::default()),
            "tsmt.042.001.03" => Document::tsmt_042_001_03(Default::default()),
            "tsmt.044.001.02" => Document::tsmt_044_001_02(Default::default()),
            "tsmt.045.001.02" => Document::tsmt_045_001_02(Default::default()),
            "tsmt.046.001.01" => Document::tsmt_046_001_01(Default::default()),
            "tsmt.047.001.01" => Document::tsmt_047_001_01(Default::default()),
            "tsmt.048.001.01" => Document::tsmt_048_001_01(Default::default()),
            "tsmt.049.001.01" => Document::tsmt_049_001_01(Default::default()),
            "tsmt.050.001.01" => Document::tsmt_050_001_01(Default::default()),
            "tsmt.051.001.01" => Document::tsmt_051_001_01(Default::default()),
            "tsmt.052.001.01" => Document::tsmt_052_001_01(Default::default()),
            "tsmt.053.001.01" => Document::tsmt_053_001_01(Default::default()),
            "tsmt.054.001.01" => Document::tsmt_054_001_01(Default::default()),
            "tsmt.055.001.01" => Document::tsmt_055_001_01(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
