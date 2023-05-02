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

// Re-export the iso 20022 pain module
pub use iso_20022_pain::*;

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Document {
    // pain
    pain_001_001_11(iso_20022_pain::pain_001_001_11::Document<Dmkr, Dmkr>),
    pain_002_001_12(iso_20022_pain::pain_002_001_12::Document<Dmkr, Dmkr>),
    pain_007_001_11(iso_20022_pain::pain_007_001_11::Document<Dmkr, Dmkr>),
    pain_008_001_10(iso_20022_pain::pain_008_001_10::Document<Dmkr, Dmkr>),
    pain_009_001_07(iso_20022_pain::pain_009_001_07::Document<Dmkr, Dmkr>),
    pain_010_001_07(iso_20022_pain::pain_010_001_07::Document<Dmkr, Dmkr>),
    pain_011_001_07(iso_20022_pain::pain_011_001_07::Document<Dmkr, Dmkr>),
    pain_012_001_07(iso_20022_pain::pain_012_001_07::Document<Dmkr, Dmkr>),
    pain_013_001_10(iso_20022_pain::pain_013_001_10::Document<Dmkr, Dmkr, Dmkr>),
    pain_014_001_10(iso_20022_pain::pain_014_001_10::Document<Dmkr, Dmkr, Dmkr, Dmkr>),
    pain_017_001_03(iso_20022_pain::pain_017_001_03::Document<Dmkr, Dmkr>),
    pain_018_001_03(iso_20022_pain::pain_018_001_03::Document<Dmkr, Dmkr>),
    #[default]
    Unknown,
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let doc = match s {
            // pain
            "pain.001.001.11" => Document::pain_001_001_11(Default::default()),
            "pain.002.001.12" => Document::pain_002_001_12(Default::default()),
            "pain.007.001.11" => Document::pain_007_001_11(Default::default()),
            "pain.008.001.10" => Document::pain_008_001_10(Default::default()),
            "pain.009.001.07" => Document::pain_009_001_07(Default::default()),
            "pain.010.001.07" => Document::pain_010_001_07(Default::default()),
            "pain.011.001.07" => Document::pain_011_001_07(Default::default()),
            "pain.012.001.07" => Document::pain_012_001_07(Default::default()),
            "pain.013.001.10" => Document::pain_013_001_10(Default::default()),
            "pain.014.001.10" => Document::pain_014_001_10(Default::default()),
            "pain.017.001.03" => Document::pain_017_001_03(Default::default()),
            "pain.018.001.03" => Document::pain_018_001_03(Default::default()),
            _ => return Err(s.to_string()),
        };

        Ok(doc)
    }
}
