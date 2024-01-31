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

// Re-export the iso 20022 admi module
pub use iso_20022_admi::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    // admi
    admi_002_001_01(iso_20022_admi::admi_002_001_01::Document),
    admi_004_001_02(iso_20022_admi::admi_004_001_02::Document),
    admi_005_001_01(iso_20022_admi::admi_005_001_01::Document<Dmkr>),
    admi_006_001_01(iso_20022_admi::admi_006_001_01::Document<Dmkr>),
    admi_007_001_01(iso_20022_admi::admi_007_001_01::Document<Dmkr>),
    admi_009_001_02(iso_20022_admi::admi_009_001_02::Document<Dmkr>),
    admi_010_001_02(iso_20022_admi::admi_010_001_02::Document<Dmkr>),
    admi_011_001_01(iso_20022_admi::admi_011_001_01::Document<Dmkr>),
    admi_017_001_01(iso_20022_admi::admi_017_001_01::Document),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = crate::message::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // admi
            "admi.002.001.01" => Document::admi_002_001_01(Default::default()),
            "admi.004.001.02" => Document::admi_004_001_02(Default::default()),
            "admi.005.001.01" => Document::admi_005_001_01(Default::default()),
            "admi.006.001.01" => Document::admi_006_001_01(Default::default()),
            "admi.007.001.01" => Document::admi_007_001_01(Default::default()),
            "admi.009.001.02" => Document::admi_009_001_02(Default::default()),
            "admi.010.001.02" => Document::admi_010_001_02(Default::default()),
            "admi.011.001.01" => Document::admi_011_001_01(Default::default()),
            "admi.017.001.01" => Document::admi_017_001_01(Default::default()),
            _ => {
                return Err(crate::message::Error::UnsupportedDocumentType(
                    s.to_string(),
                ))
            }
        };

        Ok(doc)
    }
}
