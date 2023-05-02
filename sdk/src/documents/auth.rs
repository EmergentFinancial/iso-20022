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

// Re-export the iso 20022 auth module
pub use iso_20022_auth::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    // auth
    auth_001_001_01(iso_20022_auth::auth_001_001_01::Document<Dmkr>),
    auth_002_001_01(iso_20022_auth::auth_002_001_01::Document<Dmkr, Dmkr>),
    auth_003_001_01(iso_20022_auth::auth_003_001_01::Document<Dmkr>),
    auth_012_001_02(iso_20022_auth::auth_012_001_02::Document<Dmkr, Dmkr>),
    auth_013_001_02(iso_20022_auth::auth_013_001_02::Document<Dmkr, Dmkr>),
    auth_014_001_02(iso_20022_auth::auth_014_001_02::Document<Dmkr, Dmkr>),
    auth_015_001_02(iso_20022_auth::auth_015_001_02::Document<Dmkr, Dmkr>),
    auth_016_001_02(iso_20022_auth::auth_016_001_02::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    auth_017_001_02(iso_20022_auth::auth_017_001_02::Document<Dmkr>),
    auth_018_001_03(iso_20022_auth::auth_018_001_03::Document<Dmkr, Dmkr, Dmkr, Dmkr, Dmkr, Dmkr>),
    auth_019_001_03(iso_20022_auth::auth_019_001_03::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    auth_020_001_03(iso_20022_auth::auth_020_001_03::Document<Dmkr, Dmkr, Dmkr>),
    auth_021_001_03(iso_20022_auth::auth_021_001_03::Document<Dmkr, Dmkr, Dmkr, Dmkr, Dmkr, Dmkr>),
    auth_022_001_03(
        iso_20022_auth::auth_022_001_03::Document<
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
    auth_023_001_03(iso_20022_auth::auth_023_001_03::Document<Dmkr, Dmkr>),
    auth_024_001_03(iso_20022_auth::auth_024_001_03::Document<Dmkr, Dmkr, Dmkr>),
    auth_025_001_03(iso_20022_auth::auth_025_001_03::Document<Dmkr, Dmkr, Dmkr>),
    auth_026_001_03(iso_20022_auth::auth_026_001_03::Document<Dmkr, Dmkr, Dmkr>),
    auth_027_001_03(iso_20022_auth::auth_027_001_03::Document<Dmkr>),
    auth_028_001_01(iso_20022_auth::auth_028_001_01::Document<Dmkr, Dmkr>),
    auth_029_001_03(iso_20022_auth::auth_029_001_03::Document<Dmkr>),
    auth_030_001_03(
        iso_20022_auth::auth_030_001_03::Document<
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
    auth_031_001_01(iso_20022_auth::auth_031_001_01::Document<Dmkr, Dmkr, Dmkr>),
    auth_032_001_01(iso_20022_auth::auth_032_001_01::Document<Dmkr>),
    auth_033_001_02(iso_20022_auth::auth_033_001_02::Document<Dmkr>),
    auth_034_001_01(iso_20022_auth::auth_034_001_01::Document<Dmkr, Dmkr>),
    auth_035_001_01(iso_20022_auth::auth_035_001_01::Document<Dmkr>),
    auth_036_001_02(iso_20022_auth::auth_036_001_02::Document<Dmkr>),
    auth_038_001_01(iso_20022_auth::auth_038_001_01::Document<Dmkr, Dmkr>),
    auth_039_001_01(iso_20022_auth::auth_039_001_01::Document<Dmkr>),
    auth_040_001_01(iso_20022_auth::auth_040_001_01::Document<Dmkr>),
    auth_041_001_01(iso_20022_auth::auth_041_001_01::Document<Dmkr>),
    auth_042_001_02(iso_20022_auth::auth_042_001_02::Document<Dmkr, Dmkr>),
    auth_043_001_01(iso_20022_auth::auth_043_001_01::Document<Dmkr>),
    auth_044_001_02(iso_20022_auth::auth_044_001_02::Document<Dmkr>),
    auth_045_001_02(iso_20022_auth::auth_045_001_02::Document<Dmkr>),
    auth_047_001_01(iso_20022_auth::auth_047_001_01::Document<Dmkr>),
    auth_048_001_01(iso_20022_auth::auth_048_001_01::Document<Dmkr>),
    auth_049_001_02(iso_20022_auth::auth_049_001_02::Document<Dmkr>),
    auth_050_001_01(iso_20022_auth::auth_050_001_01::Document<Dmkr>),
    auth_052_001_02(
        iso_20022_auth::auth_052_001_02::Document<
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
    auth_053_001_01(iso_20022_auth::auth_053_001_01::Document<Dmkr>),
    auth_054_001_01(iso_20022_auth::auth_054_001_01::Document<Dmkr>),
    auth_055_001_01(iso_20022_auth::auth_055_001_01::Document<Dmkr>),
    auth_056_001_01(iso_20022_auth::auth_056_001_01::Document<Dmkr>),
    auth_057_001_01(iso_20022_auth::auth_057_001_01::Document<Dmkr>),
    auth_058_001_01(iso_20022_auth::auth_058_001_01::Document<Dmkr>),
    auth_059_001_01(iso_20022_auth::auth_059_001_01::Document<Dmkr>),
    auth_060_001_01(iso_20022_auth::auth_060_001_01::Document<Dmkr>),
    auth_061_001_01(iso_20022_auth::auth_061_001_01::Document<Dmkr>),
    auth_062_001_01(iso_20022_auth::auth_062_001_01::Document<Dmkr>),
    auth_063_001_01(iso_20022_auth::auth_063_001_01::Document<Dmkr>),
    auth_064_001_01(iso_20022_auth::auth_064_001_01::Document<Dmkr>),
    auth_065_001_01(iso_20022_auth::auth_065_001_01::Document<Dmkr>),
    auth_066_001_01(iso_20022_auth::auth_066_001_01::Document<Dmkr>),
    auth_067_001_01(iso_20022_auth::auth_067_001_01::Document<Dmkr>),
    auth_068_001_01(iso_20022_auth::auth_068_001_01::Document<Dmkr>),
    auth_069_001_01(iso_20022_auth::auth_069_001_01::Document<Dmkr>),
    auth_070_001_02(iso_20022_auth::auth_070_001_02::Document<Dmkr, Dmkr, Dmkr, Dmkr, Dmkr>),
    auth_071_001_02(iso_20022_auth::auth_071_001_02::Document<Dmkr, Dmkr, Dmkr, Dmkr, Dmkr>),
    auth_072_001_01(iso_20022_auth::auth_072_001_01::Document<Dmkr>),
    auth_076_001_01(iso_20022_auth::auth_076_001_01::Document<Dmkr, Dmkr, Dmkr>),
    auth_077_001_01(iso_20022_auth::auth_077_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    auth_078_001_02(iso_20022_auth::auth_078_001_02::Document<Dmkr>),
    auth_079_001_02(iso_20022_auth::auth_079_001_02::Document<Dmkr, Dmkr>),
    auth_080_001_02(iso_20022_auth::auth_080_001_02::Document<Dmkr, Dmkr>),
    auth_083_001_02(iso_20022_auth::auth_083_001_02::Document<Dmkr>),
    auth_084_001_02(iso_20022_auth::auth_084_001_02::Document<Dmkr, Dmkr>),
    auth_085_001_02(iso_20022_auth::auth_085_001_02::Document<Dmkr, Dmkr>),
    auth_086_001_02(iso_20022_auth::auth_086_001_02::Document<Dmkr, Dmkr>),
    auth_090_001_01(iso_20022_auth::auth_090_001_01::Document<Dmkr>),
    auth_091_001_02(iso_20022_auth::auth_091_001_02::Document<Dmkr>),
    auth_092_001_03(iso_20022_auth::auth_092_001_03::Document<Dmkr>),
    auth_094_001_02(iso_20022_auth::auth_094_001_02::Document<Dmkr>),
    auth_100_001_01(iso_20022_auth::auth_100_001_01::Document<Dmkr>),
    auth_101_001_01(iso_20022_auth::auth_101_001_01::Document<Dmkr>),
    auth_102_001_01(iso_20022_auth::auth_102_001_01::Document<Dmkr>),
    auth_105_001_01(iso_20022_auth::auth_105_001_01::Document<Dmkr>),
    auth_106_001_01(iso_20022_auth::auth_106_001_01::Document<Dmkr>),
    auth_107_001_01(iso_20022_auth::auth_107_001_01::Document<Dmkr, Dmkr>),
    auth_108_001_01(iso_20022_auth::auth_108_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr, Dmkr>),
    auth_109_001_01(iso_20022_auth::auth_109_001_01::Document<Dmkr, Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // auth
            "auth.001.001.01" => Document::auth_001_001_01(Default::default()),
            "auth.002.001.01" => Document::auth_002_001_01(Default::default()),
            "auth.003.001.01" => Document::auth_003_001_01(Default::default()),
            "auth.012.001.02" => Document::auth_012_001_02(Default::default()),
            "auth.013.001.02" => Document::auth_013_001_02(Default::default()),
            "auth.014.001.02" => Document::auth_014_001_02(Default::default()),
            "auth.015.001.02" => Document::auth_015_001_02(Default::default()),
            "auth.016.001.02" => Document::auth_016_001_02(Default::default()),
            "auth.017.001.02" => Document::auth_017_001_02(Default::default()),
            "auth.018.001.03" => Document::auth_018_001_03(Default::default()),
            "auth.019.001.03" => Document::auth_019_001_03(Default::default()),
            "auth.020.001.03" => Document::auth_020_001_03(Default::default()),
            "auth.021.001.03" => Document::auth_021_001_03(Default::default()),
            "auth.022.001.03" => Document::auth_022_001_03(Default::default()),
            "auth.023.001.03" => Document::auth_023_001_03(Default::default()),
            "auth.024.001.03" => Document::auth_024_001_03(Default::default()),
            "auth.025.001.03" => Document::auth_025_001_03(Default::default()),
            "auth.026.001.03" => Document::auth_026_001_03(Default::default()),
            "auth.027.001.03" => Document::auth_027_001_03(Default::default()),
            "auth.028.001.01" => Document::auth_028_001_01(Default::default()),
            "auth.029.001.03" => Document::auth_029_001_03(Default::default()),
            "auth.030.001.03" => Document::auth_030_001_03(Default::default()),
            "auth.031.001.01" => Document::auth_031_001_01(Default::default()),
            "auth.032.001.01" => Document::auth_032_001_01(Default::default()),
            "auth.033.001.02" => Document::auth_033_001_02(Default::default()),
            "auth.034.001.01" => Document::auth_034_001_01(Default::default()),
            "auth.035.001.01" => Document::auth_035_001_01(Default::default()),
            "auth.036.001.02" => Document::auth_036_001_02(Default::default()),
            "auth.038.001.01" => Document::auth_038_001_01(Default::default()),
            "auth.039.001.01" => Document::auth_039_001_01(Default::default()),
            "auth.040.001.01" => Document::auth_040_001_01(Default::default()),
            "auth.041.001.01" => Document::auth_041_001_01(Default::default()),
            "auth.042.001.02" => Document::auth_042_001_02(Default::default()),
            "auth.043.001.01" => Document::auth_043_001_01(Default::default()),
            "auth.044.001.02" => Document::auth_044_001_02(Default::default()),
            "auth.045.001.02" => Document::auth_045_001_02(Default::default()),
            "auth.047.001.01" => Document::auth_047_001_01(Default::default()),
            "auth.048.001.01" => Document::auth_048_001_01(Default::default()),
            "auth.049.001.02" => Document::auth_049_001_02(Default::default()),
            "auth.050.001.01" => Document::auth_050_001_01(Default::default()),
            "auth.052.001.02" => Document::auth_052_001_02(Default::default()),
            "auth.053.001.01" => Document::auth_053_001_01(Default::default()),
            "auth.054.001.01" => Document::auth_054_001_01(Default::default()),
            "auth.055.001.01" => Document::auth_055_001_01(Default::default()),
            "auth.056.001.01" => Document::auth_056_001_01(Default::default()),
            "auth.057.001.01" => Document::auth_057_001_01(Default::default()),
            "auth.058.001.01" => Document::auth_058_001_01(Default::default()),
            "auth.059.001.01" => Document::auth_059_001_01(Default::default()),
            "auth.060.001.01" => Document::auth_060_001_01(Default::default()),
            "auth.061.001.01" => Document::auth_061_001_01(Default::default()),
            "auth.062.001.01" => Document::auth_062_001_01(Default::default()),
            "auth.063.001.01" => Document::auth_063_001_01(Default::default()),
            "auth.064.001.01" => Document::auth_064_001_01(Default::default()),
            "auth.065.001.01" => Document::auth_065_001_01(Default::default()),
            "auth.066.001.01" => Document::auth_066_001_01(Default::default()),
            "auth.067.001.01" => Document::auth_067_001_01(Default::default()),
            "auth.068.001.01" => Document::auth_068_001_01(Default::default()),
            "auth.069.001.01" => Document::auth_069_001_01(Default::default()),
            "auth.070.001.02" => Document::auth_070_001_02(Default::default()),
            "auth.071.001.02" => Document::auth_071_001_02(Default::default()),
            "auth.072.001.01" => Document::auth_072_001_01(Default::default()),
            "auth.076.001.01" => Document::auth_076_001_01(Default::default()),
            "auth.077.001.01" => Document::auth_077_001_01(Default::default()),
            "auth.078.001.02" => Document::auth_078_001_02(Default::default()),
            "auth.079.001.02" => Document::auth_079_001_02(Default::default()),
            "auth.080.001.02" => Document::auth_080_001_02(Default::default()),
            "auth.083.001.02" => Document::auth_083_001_02(Default::default()),
            "auth.084.001.02" => Document::auth_084_001_02(Default::default()),
            "auth.085.001.02" => Document::auth_085_001_02(Default::default()),
            "auth.086.001.02" => Document::auth_086_001_02(Default::default()),
            "auth.090.001.01" => Document::auth_090_001_01(Default::default()),
            "auth.091.001.02" => Document::auth_091_001_02(Default::default()),
            "auth.092.001.03" => Document::auth_092_001_03(Default::default()),
            "auth.094.001.02" => Document::auth_094_001_02(Default::default()),
            "auth.100.001.01" => Document::auth_100_001_01(Default::default()),
            "auth.101.001.01" => Document::auth_101_001_01(Default::default()),
            "auth.102.001.01" => Document::auth_102_001_01(Default::default()),
            "auth.105.001.01" => Document::auth_105_001_01(Default::default()),
            "auth.106.001.01" => Document::auth_106_001_01(Default::default()),
            "auth.107.001.01" => Document::auth_107_001_01(Default::default()),
            "auth.108.001.01" => Document::auth_108_001_01(Default::default()),
            "auth.109.001.01" => Document::auth_109_001_01(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
