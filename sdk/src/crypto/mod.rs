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

pub mod p256;

#[derive(Debug, Clone, Default, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SignatureType {
    P256(p256::P256Sig),
    #[default]
    Unknown,
}

#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::validator::Validate,
)]
#[serde(transparent)]
pub struct Signature {
    #[serde(rename = "$value")]
    pub value: SignatureType,
}
