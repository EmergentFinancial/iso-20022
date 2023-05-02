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

// Re-export the iso 20022 tsin module
pub use iso_20022_tsin::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    // tsin
    tsin_001_001_01(iso_20022_tsin::tsin_001_001_01::Document),
    tsin_002_001_01(iso_20022_tsin::tsin_002_001_01::Document),
    tsin_003_001_01(iso_20022_tsin::tsin_003_001_01::Document),
    tsin_005_001_01(iso_20022_tsin::tsin_005_001_01::Document<Dmkr, Dmkr>),
    tsin_006_001_01(iso_20022_tsin::tsin_006_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    tsin_007_001_01(iso_20022_tsin::tsin_007_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    tsin_008_001_01(iso_20022_tsin::tsin_008_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    tsin_009_001_01(iso_20022_tsin::tsin_009_001_01::Document<Dmkr, Dmkr, Dmkr>),
    tsin_010_001_01(iso_20022_tsin::tsin_010_001_01::Document<Dmkr, Dmkr, Dmkr>),
    tsin_011_001_01(iso_20022_tsin::tsin_011_001_01::Document<Dmkr, Dmkr, Dmkr>),
    tsin_012_001_01(iso_20022_tsin::tsin_012_001_01::Document<Dmkr, Dmkr, Dmkr>),
    tsin_013_001_01(iso_20022_tsin::tsin_013_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // tsin
            "tsin.001.001.01" => Document::tsin_001_001_01(Default::default()),
            "tsin.002.001.01" => Document::tsin_002_001_01(Default::default()),
            "tsin.003.001.01" => Document::tsin_003_001_01(Default::default()),
            "tsin.005.001.01" => Document::tsin_005_001_01(Default::default()),
            "tsin.006.001.01" => Document::tsin_006_001_01(Default::default()),
            "tsin.007.001.01" => Document::tsin_007_001_01(Default::default()),
            "tsin.008.001.01" => Document::tsin_008_001_01(Default::default()),
            "tsin.009.001.01" => Document::tsin_009_001_01(Default::default()),
            "tsin.010.001.01" => Document::tsin_010_001_01(Default::default()),
            "tsin.011.001.01" => Document::tsin_011_001_01(Default::default()),
            "tsin.012.001.01" => Document::tsin_012_001_01(Default::default()),
            "tsin.013.001.01" => Document::tsin_013_001_01(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
