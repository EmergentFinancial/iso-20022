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
