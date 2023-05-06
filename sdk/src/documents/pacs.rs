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

// Re-export the iso 20022 pacs module
pub use iso_20022_pacs::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    // pacs
    pacs_002_001_12(iso_20022_pacs::pacs_002_001_12::Document<Dmkr, Dmkr>),
    pacs_003_001_09(iso_20022_pacs::pacs_003_001_09::Document<Dmkr, Dmkr>),
    pacs_004_001_11(iso_20022_pacs::pacs_004_001_11::Document<Dmkr, Dmkr>),
    pacs_007_001_11(iso_20022_pacs::pacs_007_001_11::Document<Dmkr, Dmkr>),
    pacs_008_001_10(iso_20022_pacs::pacs_008_001_10::Document<Dmkr, Dmkr>),
    pacs_009_001_10(iso_20022_pacs::pacs_009_001_10::Document<Dmkr, Dmkr>),
    pacs_010_001_05(iso_20022_pacs::pacs_010_001_05::Document<Dmkr, Dmkr>),
    pacs_028_001_05(iso_20022_pacs::pacs_028_001_05::Document<Dmkr, Dmkr>),
    pacs_029_001_01(iso_20022_pacs::pacs_029_001_01::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // pacs
            "pacs.002.001.12" => Document::pacs_002_001_12(Default::default()),
            "pacs.003.001.09" => Document::pacs_003_001_09(Default::default()),
            "pacs.004.001.11" => Document::pacs_004_001_11(Default::default()),
            "pacs.007.001.11" => Document::pacs_007_001_11(Default::default()),
            "pacs.008.001.10" => Document::pacs_008_001_10(Default::default()),
            "pacs.009.001.10" => Document::pacs_009_001_10(Default::default()),
            "pacs.010.001.05" => Document::pacs_010_001_05(Default::default()),
            "pacs.028.001.05" => Document::pacs_028_001_05(Default::default()),
            "pacs.029.001.01" => Document::pacs_029_001_01(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
