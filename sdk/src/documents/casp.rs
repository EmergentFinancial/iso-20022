// Copyright 2023 Emergent Financial, LLC - All Rights Reserved
//
// This software is dual-licensed under the MIT License OR the Apache License, Version 2.0.
//
// MIT License
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the “Software”),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// Licensed under the Apache License, Version 2.0 (the “License”);
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an “AS IS” BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use super::Dmkr;

// Re-export the iso 20022 casp module
pub use iso_20022_casp::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    // casp
    casp_001_001_04(iso_20022_casp::casp_001_001_04::Document<Dmkr>),
    casp_002_001_04(iso_20022_casp::casp_002_001_04::Document<Dmkr>),
    casp_003_001_04(iso_20022_casp::casp_003_001_04::Document<Dmkr>),
    casp_004_001_04(iso_20022_casp::casp_004_001_04::Document<Dmkr>),
    casp_005_001_04(iso_20022_casp::casp_005_001_04::Document<Dmkr>),
    casp_006_001_04(iso_20022_casp::casp_006_001_04::Document<Dmkr, Dmkr>),
    casp_007_001_04(iso_20022_casp::casp_007_001_04::Document<Dmkr>),
    casp_008_001_04(iso_20022_casp::casp_008_001_04::Document<Dmkr>),
    casp_009_001_04(iso_20022_casp::casp_009_001_04::Document<Dmkr>),
    casp_010_001_04(iso_20022_casp::casp_010_001_04::Document<Dmkr>),
    casp_011_001_04(iso_20022_casp::casp_011_001_04::Document<Dmkr>),
    casp_012_001_04(iso_20022_casp::casp_012_001_04::Document<Dmkr>),
    casp_013_001_02(iso_20022_casp::casp_013_001_02::Document),
    casp_014_001_04(iso_20022_casp::casp_014_001_04::Document<Dmkr>),
    casp_015_001_04(iso_20022_casp::casp_015_001_04::Document<Dmkr>),
    casp_016_001_04(iso_20022_casp::casp_016_001_04::Document<Dmkr>),
    casp_017_001_04(iso_20022_casp::casp_017_001_04::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // casp
            "casp.001.001.04" => Document::casp_001_001_04(Default::default()),
            "casp.002.001.04" => Document::casp_002_001_04(Default::default()),
            "casp.003.001.04" => Document::casp_003_001_04(Default::default()),
            "casp.004.001.04" => Document::casp_004_001_04(Default::default()),
            "casp.005.001.04" => Document::casp_005_001_04(Default::default()),
            "casp.006.001.04" => Document::casp_006_001_04(Default::default()),
            "casp.007.001.04" => Document::casp_007_001_04(Default::default()),
            "casp.008.001.04" => Document::casp_008_001_04(Default::default()),
            "casp.009.001.04" => Document::casp_009_001_04(Default::default()),
            "casp.010.001.04" => Document::casp_010_001_04(Default::default()),
            "casp.011.001.04" => Document::casp_011_001_04(Default::default()),
            "casp.012.001.04" => Document::casp_012_001_04(Default::default()),
            "casp.013.001.02" => Document::casp_013_001_02(Default::default()),
            "casp.014.001.04" => Document::casp_014_001_04(Default::default()),
            "casp.015.001.04" => Document::casp_015_001_04(Default::default()),
            "casp.016.001.04" => Document::casp_016_001_04(Default::default()),
            "casp.017.001.04" => Document::casp_017_001_04(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
