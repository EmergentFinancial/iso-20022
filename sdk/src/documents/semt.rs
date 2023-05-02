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

// Re-export the iso 20022 semt module
pub use iso_20022_semt::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    // semt
    semt_001_001_03(iso_20022_semt::semt_001_001_03::Document),
    semt_002_001_11(iso_20022_semt::semt_002_001_11::Document<Dmkr, Dmkr>),
    semt_002_002_11(iso_20022_semt::semt_002_002_11::Document<Dmkr, Dmkr>),
    semt_003_001_11(iso_20022_semt::semt_003_001_11::Document<Dmkr, Dmkr>),
    semt_003_002_11(iso_20022_semt::semt_003_002_11::Document<Dmkr, Dmkr>),
    semt_004_001_02(iso_20022_semt::semt_004_001_02::Document),
    semt_005_001_02(iso_20022_semt::semt_005_001_02::Document),
    semt_006_001_03(iso_20022_semt::semt_006_001_03::Document),
    semt_007_001_03(iso_20022_semt::semt_007_001_03::Document),
    semt_013_001_06(iso_20022_semt::semt_013_001_06::Document<Dmkr>),
    semt_013_002_06(iso_20022_semt::semt_013_002_06::Document<Dmkr>),
    semt_014_001_07(iso_20022_semt::semt_014_001_07::Document<Dmkr>),
    semt_014_002_07(iso_20022_semt::semt_014_002_07::Document<Dmkr>),
    semt_015_001_09(iso_20022_semt::semt_015_001_09::Document<Dmkr>),
    semt_015_002_09(iso_20022_semt::semt_015_002_09::Document<Dmkr>),
    semt_016_001_09(iso_20022_semt::semt_016_001_09::Document<Dmkr>),
    semt_016_002_09(iso_20022_semt::semt_016_002_09::Document<Dmkr>),
    semt_017_001_12(iso_20022_semt::semt_017_001_12::Document<Dmkr, Dmkr>),
    semt_017_002_12(iso_20022_semt::semt_017_002_12::Document<Dmkr, Dmkr>),
    semt_018_001_13(iso_20022_semt::semt_018_001_13::Document<Dmkr, Dmkr>),
    semt_018_002_13(iso_20022_semt::semt_018_002_13::Document<Dmkr, Dmkr>),
    semt_019_001_10(iso_20022_semt::semt_019_001_10::Document<Dmkr>),
    semt_019_002_10(iso_20022_semt::semt_019_002_10::Document<Dmkr>),
    semt_020_001_07(iso_20022_semt::semt_020_001_07::Document<Dmkr>),
    semt_020_002_07(iso_20022_semt::semt_020_002_07::Document<Dmkr>),
    semt_021_001_08(iso_20022_semt::semt_021_001_08::Document<Dmkr>),
    semt_021_002_08(iso_20022_semt::semt_021_002_08::Document<Dmkr>),
    semt_022_001_05(iso_20022_semt::semt_022_001_05::Document<Dmkr>),
    semt_022_002_05(iso_20022_semt::semt_022_002_05::Document<Dmkr>),
    semt_023_001_01(iso_20022_semt::semt_023_001_01::Document<Dmkr>),
    semt_041_001_02(
        iso_20022_semt::semt_041_001_02::Document<
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
            Dmkr,
        >,
    ),
    semt_042_001_01(iso_20022_semt::semt_042_001_01::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // semt
            "semt.001.001.03" => Document::semt_001_001_03(Default::default()),
            "semt.002.001.11" => Document::semt_002_001_11(Default::default()),
            "semt.002.002.11" => Document::semt_002_002_11(Default::default()),
            "semt.003.001.11" => Document::semt_003_001_11(Default::default()),
            "semt.003.002.11" => Document::semt_003_002_11(Default::default()),
            "semt.004.001.02" => Document::semt_004_001_02(Default::default()),
            "semt.005.001.02" => Document::semt_005_001_02(Default::default()),
            "semt.006.001.03" => Document::semt_006_001_03(Default::default()),
            "semt.007.001.03" => Document::semt_007_001_03(Default::default()),
            "semt.013.001.06" => Document::semt_013_001_06(Default::default()),
            "semt.013.002.06" => Document::semt_013_002_06(Default::default()),
            "semt.014.001.07" => Document::semt_014_001_07(Default::default()),
            "semt.014.002.07" => Document::semt_014_002_07(Default::default()),
            "semt.015.001.09" => Document::semt_015_001_09(Default::default()),
            "semt.015.002.09" => Document::semt_015_002_09(Default::default()),
            "semt.016.001.09" => Document::semt_016_001_09(Default::default()),
            "semt.016.002.09" => Document::semt_016_002_09(Default::default()),
            "semt.017.001.12" => Document::semt_017_001_12(Default::default()),
            "semt.017.002.12" => Document::semt_017_002_12(Default::default()),
            "semt.018.001.13" => Document::semt_018_001_13(Default::default()),
            "semt.018.002.13" => Document::semt_018_002_13(Default::default()),
            "semt.019.001.10" => Document::semt_019_001_10(Default::default()),
            "semt.019.002.10" => Document::semt_019_002_10(Default::default()),
            "semt.020.001.07" => Document::semt_020_001_07(Default::default()),
            "semt.020.002.07" => Document::semt_020_002_07(Default::default()),
            "semt.021.001.08" => Document::semt_021_001_08(Default::default()),
            "semt.021.002.08" => Document::semt_021_002_08(Default::default()),
            "semt.022.001.05" => Document::semt_022_001_05(Default::default()),
            "semt.022.002.05" => Document::semt_022_002_05(Default::default()),
            "semt.023.001.01" => Document::semt_023_001_01(Default::default()),
            "semt.041.001.02" => Document::semt_041_001_02(Default::default()),
            "semt.042.001.01" => Document::semt_042_001_01(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
