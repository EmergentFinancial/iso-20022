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

// Re-export the iso 20022 caaa module
pub use iso_20022_caaa::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    // caaa
    caaa_001_001_11(iso_20022_caaa::caaa_001_001_11::Document<Dmkr>),
    caaa_002_001_11(iso_20022_caaa::caaa_002_001_11::Document<Dmkr>),
    caaa_003_001_11(iso_20022_caaa::caaa_003_001_11::Document<Dmkr>),
    caaa_004_001_10(iso_20022_caaa::caaa_004_001_10::Document<Dmkr>),
    caaa_005_001_11(iso_20022_caaa::caaa_005_001_11::Document),
    caaa_006_001_10(iso_20022_caaa::caaa_006_001_10::Document),
    caaa_007_001_11(iso_20022_caaa::caaa_007_001_11::Document),
    caaa_008_001_10(iso_20022_caaa::caaa_008_001_10::Document),
    caaa_009_001_10(iso_20022_caaa::caaa_009_001_10::Document),
    caaa_010_001_09(iso_20022_caaa::caaa_010_001_09::Document),
    caaa_011_001_11(iso_20022_caaa::caaa_011_001_11::Document),
    caaa_012_001_10(iso_20022_caaa::caaa_012_001_10::Document),
    caaa_013_001_10(iso_20022_caaa::caaa_013_001_10::Document),
    caaa_014_001_09(iso_20022_caaa::caaa_014_001_09::Document),
    caaa_015_001_06(iso_20022_caaa::caaa_015_001_06::Document),
    caaa_016_001_09(iso_20022_caaa::caaa_016_001_09::Document),
    caaa_017_001_09(iso_20022_caaa::caaa_017_001_09::Document),
    caaa_018_001_06(iso_20022_caaa::caaa_018_001_06::Document),
    caaa_019_001_05(iso_20022_caaa::caaa_019_001_05::Document),
    caaa_020_001_03(iso_20022_caaa::caaa_020_001_03::Document<Dmkr>),
    caaa_021_001_03(iso_20022_caaa::caaa_021_001_03::Document<Dmkr>),
    caaa_022_001_02(iso_20022_caaa::caaa_022_001_02::Document<Dmkr>),
    caaa_023_001_02(iso_20022_caaa::caaa_023_001_02::Document<Dmkr>),
    caaa_024_001_02(iso_20022_caaa::caaa_024_001_02::Document<Dmkr>),
    caaa_025_001_02(iso_20022_caaa::caaa_025_001_02::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // caaa
            "caaa.001.001.11" => Document::caaa_001_001_11(Default::default()),
            "caaa.002.001.11" => Document::caaa_002_001_11(Default::default()),
            "caaa.003.001.11" => Document::caaa_003_001_11(Default::default()),
            "caaa.004.001.10" => Document::caaa_004_001_10(Default::default()),
            "caaa.005.001.11" => Document::caaa_005_001_11(Default::default()),
            "caaa.006.001.10" => Document::caaa_006_001_10(Default::default()),
            "caaa.007.001.11" => Document::caaa_007_001_11(Default::default()),
            "caaa.008.001.10" => Document::caaa_008_001_10(Default::default()),
            "caaa.009.001.10" => Document::caaa_009_001_10(Default::default()),
            "caaa.010.001.09" => Document::caaa_010_001_09(Default::default()),
            "caaa.011.001.11" => Document::caaa_011_001_11(Default::default()),
            "caaa.012.001.10" => Document::caaa_012_001_10(Default::default()),
            "caaa.013.001.10" => Document::caaa_013_001_10(Default::default()),
            "caaa.014.001.09" => Document::caaa_014_001_09(Default::default()),
            "caaa.015.001.06" => Document::caaa_015_001_06(Default::default()),
            "caaa.016.001.09" => Document::caaa_016_001_09(Default::default()),
            "caaa.017.001.09" => Document::caaa_017_001_09(Default::default()),
            "caaa.018.001.06" => Document::caaa_018_001_06(Default::default()),
            "caaa.019.001.05" => Document::caaa_019_001_05(Default::default()),
            "caaa.020.001.03" => Document::caaa_020_001_03(Default::default()),
            "caaa.021.001.03" => Document::caaa_021_001_03(Default::default()),
            "caaa.022.001.02" => Document::caaa_022_001_02(Default::default()),
            "caaa.023.001.02" => Document::caaa_023_001_02(Default::default()),
            "caaa.024.001.02" => Document::caaa_024_001_02(Default::default()),
            "caaa.025.001.02" => Document::caaa_025_001_02(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
