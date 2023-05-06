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

// Re-export the iso 20022 cain module
pub use iso_20022_cain::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    // cain
    cain_001_001_03(iso_20022_cain::cain_001_001_03::Document<Dmkr>),
    cain_002_001_03(iso_20022_cain::cain_002_001_03::Document<Dmkr>),
    cain_003_001_03(iso_20022_cain::cain_003_001_03::Document<Dmkr>),
    cain_004_001_03(iso_20022_cain::cain_004_001_03::Document<Dmkr>),
    cain_005_001_03(iso_20022_cain::cain_005_001_03::Document<Dmkr>),
    cain_006_001_03(iso_20022_cain::cain_006_001_03::Document<Dmkr>),
    cain_014_001_02(iso_20022_cain::cain_014_001_02::Document<Dmkr>),
    cain_015_001_02(iso_20022_cain::cain_015_001_02::Document<Dmkr>),
    cain_016_001_02(iso_20022_cain::cain_016_001_02::Document<Dmkr>),
    cain_017_001_02(iso_20022_cain::cain_017_001_02::Document<Dmkr>),
    cain_018_001_02(iso_20022_cain::cain_018_001_02::Document<Dmkr>),
    cain_019_001_02(iso_20022_cain::cain_019_001_02::Document<Dmkr>),
    cain_020_001_02(iso_20022_cain::cain_020_001_02::Document<Dmkr>),
    cain_021_001_02(iso_20022_cain::cain_021_001_02::Document<Dmkr>),
    cain_022_001_02(iso_20022_cain::cain_022_001_02::Document<Dmkr>),
    cain_023_001_02(iso_20022_cain::cain_023_001_02::Document<Dmkr>),
    cain_024_001_02(iso_20022_cain::cain_024_001_02::Document<Dmkr>),
    cain_025_001_02(iso_20022_cain::cain_025_001_02::Document<Dmkr>),
    cain_026_001_02(iso_20022_cain::cain_026_001_02::Document<Dmkr>),
    cain_027_001_02(iso_20022_cain::cain_027_001_02::Document<Dmkr>),
    cain_028_001_02(iso_20022_cain::cain_028_001_02::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // cain
            "cain.001.001.03" => Document::cain_001_001_03(Default::default()),
            "cain.002.001.03" => Document::cain_002_001_03(Default::default()),
            "cain.003.001.03" => Document::cain_003_001_03(Default::default()),
            "cain.004.001.03" => Document::cain_004_001_03(Default::default()),
            "cain.005.001.03" => Document::cain_005_001_03(Default::default()),
            "cain.006.001.03" => Document::cain_006_001_03(Default::default()),
            "cain.014.001.02" => Document::cain_014_001_02(Default::default()),
            "cain.015.001.02" => Document::cain_015_001_02(Default::default()),
            "cain.016.001.02" => Document::cain_016_001_02(Default::default()),
            "cain.017.001.02" => Document::cain_017_001_02(Default::default()),
            "cain.018.001.02" => Document::cain_018_001_02(Default::default()),
            "cain.019.001.02" => Document::cain_019_001_02(Default::default()),
            "cain.020.001.02" => Document::cain_020_001_02(Default::default()),
            "cain.021.001.02" => Document::cain_021_001_02(Default::default()),
            "cain.022.001.02" => Document::cain_022_001_02(Default::default()),
            "cain.023.001.02" => Document::cain_023_001_02(Default::default()),
            "cain.024.001.02" => Document::cain_024_001_02(Default::default()),
            "cain.025.001.02" => Document::cain_025_001_02(Default::default()),
            "cain.026.001.02" => Document::cain_026_001_02(Default::default()),
            "cain.027.001.02" => Document::cain_027_001_02(Default::default()),
            "cain.028.001.02" => Document::cain_028_001_02(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
