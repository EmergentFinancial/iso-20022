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

// Re-export the iso 20022 catp module
pub use iso_20022_catp::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    // catp
    catp_001_001_02(iso_20022_catp::catp_001_001_02::Document),
    catp_002_001_02(iso_20022_catp::catp_002_001_02::Document),
    catp_003_001_02(iso_20022_catp::catp_003_001_02::Document),
    catp_004_001_02(iso_20022_catp::catp_004_001_02::Document),
    catp_005_001_02(iso_20022_catp::catp_005_001_02::Document),
    catp_006_001_02(iso_20022_catp::catp_006_001_02::Document),
    catp_007_001_02(iso_20022_catp::catp_007_001_02::Document),
    catp_008_001_02(iso_20022_catp::catp_008_001_02::Document),
    catp_009_001_02(iso_20022_catp::catp_009_001_02::Document),
    catp_010_001_02(iso_20022_catp::catp_010_001_02::Document),
    catp_011_001_02(iso_20022_catp::catp_011_001_02::Document),
    catp_012_001_01(iso_20022_catp::catp_012_001_01::Document),
    catp_013_001_01(iso_20022_catp::catp_013_001_01::Document),
    catp_014_001_01(iso_20022_catp::catp_014_001_01::Document),
    catp_015_001_01(iso_20022_catp::catp_015_001_01::Document),
    catp_016_001_01(iso_20022_catp::catp_016_001_01::Document),
    catp_017_001_01(iso_20022_catp::catp_017_001_01::Document),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // catp
            "catp.001.001.02" => Document::catp_001_001_02(Default::default()),
            "catp.002.001.02" => Document::catp_002_001_02(Default::default()),
            "catp.003.001.02" => Document::catp_003_001_02(Default::default()),
            "catp.004.001.02" => Document::catp_004_001_02(Default::default()),
            "catp.005.001.02" => Document::catp_005_001_02(Default::default()),
            "catp.006.001.02" => Document::catp_006_001_02(Default::default()),
            "catp.007.001.02" => Document::catp_007_001_02(Default::default()),
            "catp.008.001.02" => Document::catp_008_001_02(Default::default()),
            "catp.009.001.02" => Document::catp_009_001_02(Default::default()),
            "catp.010.001.02" => Document::catp_010_001_02(Default::default()),
            "catp.011.001.02" => Document::catp_011_001_02(Default::default()),
            "catp.012.001.01" => Document::catp_012_001_01(Default::default()),
            "catp.013.001.01" => Document::catp_013_001_01(Default::default()),
            "catp.014.001.01" => Document::catp_014_001_01(Default::default()),
            "catp.015.001.01" => Document::catp_015_001_01(Default::default()),
            "catp.016.001.01" => Document::catp_016_001_01(Default::default()),
            "catp.017.001.01" => Document::catp_017_001_01(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
