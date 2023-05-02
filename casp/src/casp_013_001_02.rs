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

// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// See ISO-20022 Intellectual Property Rights Policy at
// <https://www.iso20022.org/intellectual-property-rights>
// for more information.

use validator::Validate;

::lazy_static::lazy_static! {
    static ref MAX_100_K_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_2_MAX_3_ALPHA_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_10_K_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_140_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_35_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:casp.013.001.02".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Header41 {
    #[serde(rename = "MsgFctn")]
    pub msg_fctn: RetailerMessage1Code,
    #[validate]
    #[serde(rename = "PrtcolVrsn")]
    pub prtcol_vrsn: Max6Text,
    #[validate]
    #[serde(rename = "XchgId")]
    pub xchg_id: Max35Text,
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "InitgPty")]
    pub initg_pty: GenericIdentification177,
    #[serde(rename = "RcptPty", skip_serializing_if = "Option::is_none")]
    pub rcpt_pty: Option<GenericIdentification177>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tracblt", default)]
    pub tracblt: Vec<Traceability8>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RetailerMessage1Code {
    #[serde(rename = "SSAB")]
    Ssab,
    #[serde(rename = "SAAQ")]
    Saaq,
    #[serde(rename = "SAAP")]
    Saap,
    #[serde(rename = "SDDR")]
    Sddr,
    #[serde(rename = "SDDP")]
    Sddp,
    #[serde(rename = "SSEN")]
    Ssen,
    #[serde(rename = "SSMQ")]
    Ssmq,
    #[serde(rename = "SSMR")]
    Ssmr,
    #[serde(rename = "SSRJ")]
    Ssrj,
    #[serde(rename = "SARQ")]
    Sarq,
    #[serde(rename = "SARP")]
    Sarp,
    #[serde(rename = "SFRP")]
    Sfrp,
    #[serde(rename = "SFRQ")]
    Sfrq,
    #[serde(rename = "SFSQ")]
    Sfsq,
    #[serde(rename = "SFSP")]
    Sfsp,
    #[serde(rename = "SASQ")]
    Sasq,
    #[serde(rename = "SASP")]
    Sasp,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeolocationGeographicCoordinates1 {
    #[validate]
    #[serde(rename = "Lat")]
    pub lat: Max35Text,
    #[validate]
    #[serde(rename = "Long")]
    pub long: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification177 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PartyType33Code>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyType33Code>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Min2Max3AlphaText>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "RmotAccs", skip_serializing_if = "Option::is_none")]
    pub rmot_accs: Option<NetworkParameters7>,
    #[serde(rename = "Glctn", skip_serializing_if = "Option::is_none")]
    pub glctn: Option<Geolocation1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max500Text {
    #[validate(length(min = 1, max = 500,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max100KBinary {
    #[validate(length(min = 1, max = 102400,), regex = "MAX_100_K_BINARY_REGEX")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max35Text {
    #[validate(length(min = 1, max = 35,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
#[serde(rename = "Document")]
pub struct Document {
    #[validate]
    #[serde(rename = "SaleToPOIMsgRjctn")]
    pub sale_to_poi_msg_rjctn: SaleToPoiMessageRejectionV02,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Min2Max3AlphaText {
    #[validate(regex = "MIN_2_MAX_3_ALPHA_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max6Text {
    #[validate(length(min = 1, max = 6,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType33Code {
    #[serde(rename = "OPOI")]
    Opoi,
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ITAG")]
    Itag,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "DLIS")]
    Dlis,
    #[serde(rename = "MTMG")]
    Mtmg,
    #[serde(rename = "TAXH")]
    Taxh,
    #[serde(rename = "TMGT")]
    Tmgt,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcceptorRejection2 {
    #[serde(rename = "RjctRsn")]
    pub rjct_rsn: RejectReason1Code,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max500Text>,
    #[serde(rename = "MsgInErr", skip_serializing_if = "Option::is_none")]
    pub msg_in_err: Option<Max100KBinary>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NetworkParameters9 {
    #[serde(rename = "NtwkTp")]
    pub ntwk_tp: NetworkType1Code,
    #[validate]
    #[serde(rename = "AdrVal")]
    pub adr_val: Max500Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Geolocation1 {
    #[serde(rename = "GeogcCordints", skip_serializing_if = "Option::is_none")]
    pub geogc_cordints: Option<GeolocationGeographicCoordinates1>,
    #[serde(rename = "UTMCordints", skip_serializing_if = "Option::is_none")]
    pub utm_cordints: Option<GeolocationUtmCoordinates1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max10KBinary {
    #[validate(length(min = 1, max = 10240,), regex = "MAX_10_K_BINARY_REGEX")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NetworkType1Code {
    #[serde(rename = "IPNW")]
    Ipnw,
    #[serde(rename = "PSTN")]
    Pstn,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max140Binary {
    #[validate(length(min = 1, max = 140,), regex = "MAX_140_BINARY_REGEX")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NetworkParameters7 {
    #[validate(length(min = 1,))]
    #[serde(rename = "Adr", default)]
    pub adr: Vec<NetworkParameters9>,
    #[serde(rename = "UsrNm", skip_serializing_if = "Option::is_none")]
    pub usr_nm: Option<Max35Text>,
    #[serde(rename = "AccsCd", skip_serializing_if = "Option::is_none")]
    pub accs_cd: Option<Max35Binary>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SvrCert", default)]
    pub svr_cert: Vec<Max10KBinary>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SvrCertIdr", default)]
    pub svr_cert_idr: Vec<Max140Binary>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ClntCert", default)]
    pub clnt_cert: Vec<Max10KBinary>,
    #[serde(rename = "SctyPrfl", skip_serializing_if = "Option::is_none")]
    pub scty_prfl: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RejectReason1Code {
    #[serde(rename = "UNPR")]
    Unpr,
    #[serde(rename = "IMSG")]
    Imsg,
    #[serde(rename = "PARS")]
    Pars,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "INTP")]
    Intp,
    #[serde(rename = "RCPP")]
    Rcpp,
    #[serde(rename = "DPMG")]
    Dpmg,
    #[serde(rename = "VERS")]
    Vers,
    #[serde(rename = "MSGT")]
    Msgt,
    #[default]
    Unknown,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SaleToPoiMessageRejectionV02 {
    #[validate]
    #[serde(rename = "Hdr")]
    pub hdr: Header41,
    #[validate]
    #[serde(rename = "Rjct")]
    pub rjct: AcceptorRejection2,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeolocationUtmCoordinates1 {
    #[validate]
    #[serde(rename = "UTMZone")]
    pub utm_zone: Max35Text,
    #[validate]
    #[serde(rename = "UTMEstwrd")]
    pub utm_estwrd: Max35Text,
    #[validate]
    #[serde(rename = "UTMNrthwrd")]
    pub utm_nrthwrd: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IsoDateTime {
    #[serde(rename = "$text")]
    pub value: ::chrono::DateTime<::chrono::Utc>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max35Binary {
    #[validate(length(min = 1, max = 35,), regex = "MAX_35_BINARY_REGEX")]
    pub value: String,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Traceability8 {
    #[validate]
    #[serde(rename = "RlayId")]
    pub rlay_id: GenericIdentification177,
    #[serde(rename = "PrtcolNm", skip_serializing_if = "Option::is_none")]
    pub prtcol_nm: Option<Max35Text>,
    #[serde(rename = "PrtcolVrsn", skip_serializing_if = "Option::is_none")]
    pub prtcol_vrsn: Option<Max6Text>,
    #[validate]
    #[serde(rename = "TracDtTmIn")]
    pub trac_dt_tm_in: IsoDateTime,
    #[validate]
    #[serde(rename = "TracDtTmOut")]
    pub trac_dt_tm_out: IsoDateTime,
}
