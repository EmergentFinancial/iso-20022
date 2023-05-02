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

// Re-export the iso 20022 tsrv module
pub use iso_20022_tsrv::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    // tsrv
    tsrv_001_001_01(iso_20022_tsrv::tsrv_001_001_01::Document<Dmkr, Dmkr>),
    tsrv_002_001_01(iso_20022_tsrv::tsrv_002_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    tsrv_003_001_01(iso_20022_tsrv::tsrv_003_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    tsrv_004_001_01(iso_20022_tsrv::tsrv_004_001_01::Document<Dmkr, Dmkr>),
    tsrv_005_001_01(iso_20022_tsrv::tsrv_005_001_01::Document<Dmkr, Dmkr>),
    tsrv_006_001_01(iso_20022_tsrv::tsrv_006_001_01::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    tsrv_007_001_01(iso_20022_tsrv::tsrv_007_001_01::Document<Dmkr, Dmkr, Dmkr>),
    tsrv_008_001_01(iso_20022_tsrv::tsrv_008_001_01::Document<Dmkr>),
    tsrv_009_001_01(iso_20022_tsrv::tsrv_009_001_01::Document<Dmkr, Dmkr>),
    tsrv_010_001_01(iso_20022_tsrv::tsrv_010_001_01::Document<Dmkr>),
    tsrv_011_001_01(iso_20022_tsrv::tsrv_011_001_01::Document<Dmkr>),
    tsrv_012_001_01(iso_20022_tsrv::tsrv_012_001_01::Document<Dmkr, Dmkr>),
    tsrv_013_001_01(iso_20022_tsrv::tsrv_013_001_01::Document<Dmkr, Dmkr>),
    tsrv_014_001_01(iso_20022_tsrv::tsrv_014_001_01::Document<Dmkr, Dmkr>),
    tsrv_015_001_01(iso_20022_tsrv::tsrv_015_001_01::Document<Dmkr>),
    tsrv_016_001_01(iso_20022_tsrv::tsrv_016_001_01::Document<Dmkr>),
    tsrv_017_001_01(iso_20022_tsrv::tsrv_017_001_01::Document<Dmkr>),
    tsrv_018_001_01(iso_20022_tsrv::tsrv_018_001_01::Document<Dmkr>),
    tsrv_019_001_01(iso_20022_tsrv::tsrv_019_001_01::Document<Dmkr, Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // tsrv
            "tsrv.001.001.01" => Document::tsrv_001_001_01(Default::default()),
            "tsrv.002.001.01" => Document::tsrv_002_001_01(Default::default()),
            "tsrv.003.001.01" => Document::tsrv_003_001_01(Default::default()),
            "tsrv.004.001.01" => Document::tsrv_004_001_01(Default::default()),
            "tsrv.005.001.01" => Document::tsrv_005_001_01(Default::default()),
            "tsrv.006.001.01" => Document::tsrv_006_001_01(Default::default()),
            "tsrv.007.001.01" => Document::tsrv_007_001_01(Default::default()),
            "tsrv.008.001.01" => Document::tsrv_008_001_01(Default::default()),
            "tsrv.009.001.01" => Document::tsrv_009_001_01(Default::default()),
            "tsrv.010.001.01" => Document::tsrv_010_001_01(Default::default()),
            "tsrv.011.001.01" => Document::tsrv_011_001_01(Default::default()),
            "tsrv.012.001.01" => Document::tsrv_012_001_01(Default::default()),
            "tsrv.013.001.01" => Document::tsrv_013_001_01(Default::default()),
            "tsrv.014.001.01" => Document::tsrv_014_001_01(Default::default()),
            "tsrv.015.001.01" => Document::tsrv_015_001_01(Default::default()),
            "tsrv.016.001.01" => Document::tsrv_016_001_01(Default::default()),
            "tsrv.017.001.01" => Document::tsrv_017_001_01(Default::default()),
            "tsrv.018.001.01" => Document::tsrv_018_001_01(Default::default()),
            "tsrv.019.001.01" => Document::tsrv_019_001_01(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
