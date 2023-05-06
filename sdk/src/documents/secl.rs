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

// Re-export the iso 20022 secl module
pub use iso_20022_secl::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    // secl
    secl_001_001_03(iso_20022_secl::secl_001_001_03::Document<Dmkr>),
    secl_002_001_03(iso_20022_secl::secl_002_001_03::Document<Dmkr>),
    secl_003_001_03(iso_20022_secl::secl_003_001_03::Document<Dmkr>),
    secl_004_001_03(iso_20022_secl::secl_004_001_03::Document<Dmkr>),
    secl_005_001_02(iso_20022_secl::secl_005_001_02::Document<Dmkr>),
    secl_006_001_02(iso_20022_secl::secl_006_001_02::Document<Dmkr>),
    secl_007_001_03(iso_20022_secl::secl_007_001_03::Document<Dmkr>),
    secl_008_001_03(iso_20022_secl::secl_008_001_03::Document<Dmkr>),
    secl_009_001_03(iso_20022_secl::secl_009_001_03::Document<Dmkr>),
    secl_010_001_03(iso_20022_secl::secl_010_001_03::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // secl
            "secl.001.001.03" => Document::secl_001_001_03(Default::default()),
            "secl.002.001.03" => Document::secl_002_001_03(Default::default()),
            "secl.003.001.03" => Document::secl_003_001_03(Default::default()),
            "secl.004.001.03" => Document::secl_004_001_03(Default::default()),
            "secl.005.001.02" => Document::secl_005_001_02(Default::default()),
            "secl.006.001.02" => Document::secl_006_001_02(Default::default()),
            "secl.007.001.03" => Document::secl_007_001_03(Default::default()),
            "secl.008.001.03" => Document::secl_008_001_03(Default::default()),
            "secl.009.001.03" => Document::secl_009_001_03(Default::default()),
            "secl.010.001.03" => Document::secl_010_001_03(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
