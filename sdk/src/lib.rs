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
//
//!
//! # ISO 20022 Software Developer Kit (SDK)
//!
//! The `iso-20022-sdk` is a Rust library for working with the <a href="https://iso20022.org" target="_blank">ISO 20022</a> Universal financial industry message scheme.
//!
//!
//! > #
//! > Need ISO-20022 Integrations? [Contact us](mailto:ryan.tate@emergent.financial) to learn about our software systems and development services.
//! > #
//!
//! ## Install the SDK Library
//!
//! Add `iso-20022-sdk` to your `Cargo.toml` dependencies:
//!
//! ```toml
//!
//! [dependencies]
//! iso-20022-sdk = { version = "0.1.0" }
//!
//! ```
//!
//!
//! ## Features
//!
//! By default, `iso-20022-sdk` includes `nvlp`, `head` and `dsig` features, which imports `iso-20022-nvlp`, `iso-20022-head` and `iso-20022-dsig` respectively.
//!
//! Documents, e.g. `remt.001.001.01`, are conditionally compiled and need to be added individually, either as a business domain or message set feature, e.g.
//!
//! ```toml
//!
//! [dependencies]
//! iso-20022-sdk = { version = "0.1.0", features = ["remt"] }
//! ```
//!
//! Now you can create a `Document` from the `remt.001.001.01` namespace:
//!
//! ```rust
//! use iso_20022_sdk::Document;
//!
//! let mut doc = Document::from_namespace("remt.001.001.01")?;
//!
//! ```
//!
//! ### Business Domains
//!
//! To include messages relevant only to the `payments` business domain, add the `payments` feature to your `Cargo.toml`:
//!
//! ```toml
//!
//! [dependencies]
//! iso-20022-sdk = { version = "0.1.0", features = ["payments"] }
//!
//! ```
//!
//! > *Using the payments features will include all message sets in the payments business domain.*
//! >
//! ```toml
//! payments = ["acmt", "auth", "acmt", "admi", "camt", "pacs", "pain", "reda", "remt"]
//! ```
//! >
//! > Available `business domain` features
//! >
//! > - `payments`
//! > - `securities`
//! > - `trade`
//! > - `cards`
//! > - `fx`
//!
//!
//! ### Message Sets
//!
//! Each message set, e.g. `acmt`, has its own Rust library, e.g. `iso-20022-acmt`, which can be conditionally compiled using the `Cargo.toml` **features** flag corresponding to the message set.
//!
//!
//! ```toml
//!
//! [dependencies]
//! iso-20022-sdk = { version = "0.1.0", features = ["acmt", "admi"] }
//!
//! ```
//!
//!
//! > Available `message set` features
//! >
//! > - `acmt`
//! > - `admi`
//! > - `auth`
//! > - `caaa`
//! > - `caad`
//! > - `caam`
//! > - `cafc`
//! > - `cafm`
//! > - `cafr`
//! > - `cain`
//! > - `camt`
//! > - `canm`
//! > - `casp`
//! > - `casr`
//! > - `catm`
//! > - `catp`
//! > - `colr`
//! > - `fxtr`
//! > - `pacs`
//! > - `pain`
//! > - `reda`
//! > - `remt`
//! > - `secl`
//! > - `seev`
//! > - `semt`
//! > - `sese`
//! > - `setr`
//! > - `tsin`
//! > - `tsmt`
//! > - `tsrv`
//!
#[cfg(feature = "crypto")]
pub mod crypto;
#[allow(non_camel_case_types)]
pub mod documents;
pub mod external_codes;
#[cfg(feature = "msg")]
pub mod message;

// Re-exports
#[cfg(feature = "nvlp")]
pub use iso_20022_nvlp::{self as nvlp};

#[cfg(feature = "head")]
pub use iso_20022_head::{self as head};

#[cfg(feature = "dsig")]
pub use iso_20022_dsig::{self as dsig};

// Prelude
pub mod prelude {
    #[cfg(feature = "nvlp")]
    pub use super::nvlp;

    #[cfg(feature = "head")]
    pub use super::head;

    #[cfg(feature = "dsig")]
    pub use super::dsig;

    #[cfg(feature = "msg")]
    pub use super::message::*;

    pub use super::documents::*;

    pub use super::crypto::*;
}
