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

// Re-export the iso 20022 casr module
pub use iso_20022_casr::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    // casr
    casr_001_001_02(iso_20022_casr::casr_001_001_02::Document<Dmkr>),
    casr_002_001_02(iso_20022_casr::casr_002_001_02::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = crate::message::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // casr
            "casr.001.001.02" => Document::casr_001_001_02(Default::default()),
            "casr.002.001.02" => Document::casr_002_001_02(Default::default()),
            _ => {
                return Err(crate::message::Error::UnsupportedDocumentType(
                    s.to_string(),
                ))
            }
        };

        Ok(doc)
    }
}
