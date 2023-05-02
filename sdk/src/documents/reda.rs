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

// Re-export the iso 20022 reda module
pub use iso_20022_reda::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    // reda
    reda_001_001_04(iso_20022_reda::reda_001_001_04::Document),
    reda_002_001_04(iso_20022_reda::reda_002_001_04::Document),
    reda_004_001_06(iso_20022_reda::reda_004_001_06::Document),
    reda_005_001_03(iso_20022_reda::reda_005_001_03::Document),
    reda_006_001_01(iso_20022_reda::reda_006_001_01::Document<Dmkr>),
    reda_007_001_01(iso_20022_reda::reda_007_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr, Dmkr>),
    reda_008_001_01(iso_20022_reda::reda_008_001_01::Document<Dmkr>),
    reda_009_001_01(iso_20022_reda::reda_009_001_01::Document<Dmkr>),
    reda_010_001_01(iso_20022_reda::reda_010_001_01::Document<Dmkr>),
    reda_012_001_01(iso_20022_reda::reda_012_001_01::Document<Dmkr>),
    reda_013_001_01(iso_20022_reda::reda_013_001_01::Document<Dmkr>),
    reda_014_001_01(iso_20022_reda::reda_014_001_01::Document<Dmkr>),
    reda_015_001_01(iso_20022_reda::reda_015_001_01::Document<Dmkr>),
    reda_016_001_01(iso_20022_reda::reda_016_001_01::Document<Dmkr>),
    reda_017_001_01(iso_20022_reda::reda_017_001_01::Document<Dmkr>),
    reda_018_001_01(iso_20022_reda::reda_018_001_01::Document<Dmkr>),
    reda_019_001_01(iso_20022_reda::reda_019_001_01::Document<Dmkr>),
    reda_020_001_01(iso_20022_reda::reda_020_001_01::Document<Dmkr>),
    reda_021_001_01(iso_20022_reda::reda_021_001_01::Document<Dmkr>),
    reda_022_001_01(iso_20022_reda::reda_022_001_01::Document<Dmkr>),
    reda_023_001_01(iso_20022_reda::reda_023_001_01::Document<Dmkr>),
    reda_029_001_01(iso_20022_reda::reda_029_001_01::Document<Dmkr>),
    reda_030_001_01(iso_20022_reda::reda_030_001_01::Document<Dmkr>),
    reda_031_001_01(iso_20022_reda::reda_031_001_01::Document<Dmkr>),
    reda_032_001_01(iso_20022_reda::reda_032_001_01::Document<Dmkr>),
    reda_033_001_01(iso_20022_reda::reda_033_001_01::Document<Dmkr>),
    reda_034_001_01(iso_20022_reda::reda_034_001_01::Document<Dmkr>),
    reda_035_001_01(iso_20022_reda::reda_035_001_01::Document<Dmkr>),
    reda_036_001_01(iso_20022_reda::reda_036_001_01::Document<Dmkr>),
    reda_037_001_01(iso_20022_reda::reda_037_001_01::Document<Dmkr>),
    reda_041_001_01(iso_20022_reda::reda_041_001_01::Document<Dmkr>),
    reda_042_001_01(iso_20022_reda::reda_042_001_01::Document<Dmkr>),
    reda_043_001_01(iso_20022_reda::reda_043_001_01::Document<Dmkr>),
    reda_056_001_01(iso_20022_reda::reda_056_001_01::Document<Dmkr>),
    reda_057_001_01(iso_20022_reda::reda_057_001_01::Document<Dmkr>),
    reda_058_001_01(iso_20022_reda::reda_058_001_01::Document<Dmkr>),
    reda_059_001_01(iso_20022_reda::reda_059_001_01::Document<Dmkr>),
    reda_060_001_01(iso_20022_reda::reda_060_001_01::Document<Dmkr>),
    reda_061_001_01(iso_20022_reda::reda_061_001_01::Document<Dmkr>),
    reda_064_001_01(iso_20022_reda::reda_064_001_01::Document<Dmkr>),
    reda_065_001_01(iso_20022_reda::reda_065_001_01::Document<Dmkr>),
    reda_074_001_01(iso_20022_reda::reda_074_001_01::Document<Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // reda
            "reda.001.001.04" => Document::reda_001_001_04(Default::default()),
            "reda.002.001.04" => Document::reda_002_001_04(Default::default()),
            "reda.004.001.06" => Document::reda_004_001_06(Default::default()),
            "reda.005.001.03" => Document::reda_005_001_03(Default::default()),
            "reda.006.001.01" => Document::reda_006_001_01(Default::default()),
            "reda.007.001.01" => Document::reda_007_001_01(Default::default()),
            "reda.008.001.01" => Document::reda_008_001_01(Default::default()),
            "reda.009.001.01" => Document::reda_009_001_01(Default::default()),
            "reda.010.001.01" => Document::reda_010_001_01(Default::default()),
            "reda.012.001.01" => Document::reda_012_001_01(Default::default()),
            "reda.013.001.01" => Document::reda_013_001_01(Default::default()),
            "reda.014.001.01" => Document::reda_014_001_01(Default::default()),
            "reda.015.001.01" => Document::reda_015_001_01(Default::default()),
            "reda.016.001.01" => Document::reda_016_001_01(Default::default()),
            "reda.017.001.01" => Document::reda_017_001_01(Default::default()),
            "reda.018.001.01" => Document::reda_018_001_01(Default::default()),
            "reda.019.001.01" => Document::reda_019_001_01(Default::default()),
            "reda.020.001.01" => Document::reda_020_001_01(Default::default()),
            "reda.021.001.01" => Document::reda_021_001_01(Default::default()),
            "reda.022.001.01" => Document::reda_022_001_01(Default::default()),
            "reda.023.001.01" => Document::reda_023_001_01(Default::default()),
            "reda.029.001.01" => Document::reda_029_001_01(Default::default()),
            "reda.030.001.01" => Document::reda_030_001_01(Default::default()),
            "reda.031.001.01" => Document::reda_031_001_01(Default::default()),
            "reda.032.001.01" => Document::reda_032_001_01(Default::default()),
            "reda.033.001.01" => Document::reda_033_001_01(Default::default()),
            "reda.034.001.01" => Document::reda_034_001_01(Default::default()),
            "reda.035.001.01" => Document::reda_035_001_01(Default::default()),
            "reda.036.001.01" => Document::reda_036_001_01(Default::default()),
            "reda.037.001.01" => Document::reda_037_001_01(Default::default()),
            "reda.041.001.01" => Document::reda_041_001_01(Default::default()),
            "reda.042.001.01" => Document::reda_042_001_01(Default::default()),
            "reda.043.001.01" => Document::reda_043_001_01(Default::default()),
            "reda.056.001.01" => Document::reda_056_001_01(Default::default()),
            "reda.057.001.01" => Document::reda_057_001_01(Default::default()),
            "reda.058.001.01" => Document::reda_058_001_01(Default::default()),
            "reda.059.001.01" => Document::reda_059_001_01(Default::default()),
            "reda.060.001.01" => Document::reda_060_001_01(Default::default()),
            "reda.061.001.01" => Document::reda_061_001_01(Default::default()),
            "reda.064.001.01" => Document::reda_064_001_01(Default::default()),
            "reda.065.001.01" => Document::reda_065_001_01(Default::default()),
            "reda.074.001.01" => Document::reda_074_001_01(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
