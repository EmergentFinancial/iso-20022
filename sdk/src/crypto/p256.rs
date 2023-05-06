// Copyright 2023 Emergent Financial, LLC - All Rights Reserved
//
//
// This software is licensed under the Emergent Financial Limited Public License Version 1.0
// (EF-LPLv1). You may use, copy, modify, and distribute this software under the terms and
// conditions of the EF-LPL. For more information, please refer to the full text of the license
// at https://github.com/emergentfinancial/ef-lpl.
//
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use iso_20022_dsig::dsig::{Signature, SignatureBuilder};
use iso_20022_head::head_001_001_03::*;
use iso_20022_nvlp::nvlp_001_001_01::*;
use p256::ecdsa::{signature::Signer, Signature as P256Signature, SigningKey};
use quick_xml::de::from_str;
use quick_xml::se::to_string;

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    // ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct P256Sig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p256() {
        // let sig = SignatureBuilder::p256();

        // let sig = sig.build().ok();

        // println!("sig: {:#?}", sig);
        // println!("sig: {:?}", to_string(&sig));
    }
}
