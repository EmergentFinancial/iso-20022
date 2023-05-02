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
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_8_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_6_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9\s]{6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_MAX_3_A_COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_8_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_140_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_MAX_3_A_LANGUAGE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_15_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,15}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_8583_RESPONSE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_2_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_3_NUMERIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_1_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_32_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,32}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_12_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,12}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_8583_MESSAGE_REASON_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{4,4}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:canm.002.001.03".to_string()
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
pub struct Number {
    #[serde(rename = "$text")]
    pub value: f64,
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
pub struct Action13 {
    #[serde(rename = "Dstn", skip_serializing_if = "Option::is_none")]
    pub dstn: Option<PartyType20Code>,
    #[serde(rename = "ActnTp", skip_serializing_if = "Option::is_none")]
    pub actn_tp: Option<ActionType11Code>,
    #[serde(rename = "OthrActnTp", skip_serializing_if = "Option::is_none")]
    pub othr_actn_tp: Option<Max35Text>,
    #[serde(rename = "Ctct", skip_serializing_if = "Option::is_none")]
    pub ctct: Option<Contact6>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ActionType10Code {
    #[serde(rename = "ACTV")]
    Actv,
    #[serde(rename = "DEAC")]
    Deac,
    #[serde(rename = "DISP")]
    Disp,
    #[serde(rename = "FUPD")]
    Fupd,
    #[serde(rename = "PRNT")]
    Prnt,
    #[serde(rename = "SNDM")]
    Sndm,
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
pub struct Max4000Text {
    #[validate(length(min = 1, max = 4000,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MessageError1Code {
    #[serde(rename = "IDEF")]
    Idef,
    #[serde(rename = "IDEL")]
    Idel,
    #[serde(rename = "IDEV")]
    Idev,
    #[serde(rename = "INME")]
    Inme,
    #[serde(rename = "INMF")]
    Inmf,
    #[serde(rename = "MEPE")]
    Mepe,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PRVE")]
    Prve,
    #[serde(rename = "RDEM")]
    Rdem,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "UDFD")]
    Udfd,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "ITDE")]
    Itde,
    #[serde(rename = "DUME")]
    Dume,
    #[serde(rename = "IDWM")]
    Idwm,
    #[serde(rename = "IDRM")]
    Idrm,
    #[serde(rename = "IBAT")]
    Ibat,
    #[serde(rename = "ICOL")]
    Icol,
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
pub struct ContentInformationType20 {
    #[validate]
    #[serde(rename = "MACData")]
    pub mac_data: MacData1,
    #[validate]
    #[serde(rename = "MAC")]
    pub mac: Max8HexBinaryText,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType22Code {
    #[serde(rename = "CRDH")]
    Crdh,
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "AGNT")]
    Agnt,
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
pub struct PhoneNumber {
    #[validate(regex = "PHONE_NUMBER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Response8Code {
    #[serde(rename = "PRCS")]
    Prcs,
    #[serde(rename = "UNPR")]
    Unpr,
    #[serde(rename = "UNRV")]
    Unrv,
    #[serde(rename = "REJT")]
    Rejt,
    #[serde(rename = "TECH")]
    Tech,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
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
pub struct BatchManagementInformation1 {
    #[serde(rename = "ColltnId", skip_serializing_if = "Option::is_none")]
    pub colltn_id: Option<Max35Text>,
    #[validate]
    #[serde(rename = "BtchId")]
    pub btch_id: Max35Text,
    #[serde(rename = "MsgSeqNb", skip_serializing_if = "Option::is_none")]
    pub msg_seq_nb: Option<Max15NumericText>,
    #[serde(rename = "MsgChcksmInptVal", skip_serializing_if = "Option::is_none")]
    pub msg_chcksm_inpt_val: Option<Max140Binary>,
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
pub struct Max2048Text {
    #[validate(length(min = 1, max = 2048,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfAmount21Code {
    #[serde(rename = "INTC")]
    Intc,
    #[serde(rename = "FEEP")]
    Feep,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "FEEA")]
    Feea,
    #[serde(rename = "CSIF")]
    Csif,
    #[serde(rename = "MXIF")]
    Mxif,
    #[serde(rename = "MNIF")]
    Mnif,
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
pub struct BaseOne25Rate {
    #[serde(rename = "$text")]
    pub value: f64,
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
pub struct Max8HexBinaryText {
    #[validate(regex = "MAX_8_HEX_BINARY_TEXT_REGEX")]
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
pub struct CardProgrammeMode3 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlId", default)]
    pub addtl_id: Vec<AdditionalData1>,
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
pub struct Exact6AlphaNumericText {
    #[validate(regex = "EXACT_6_ALPHA_NUMERIC_TEXT_REGEX")]
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
pub struct Header63 {
    #[serde(rename = "MsgFctn")]
    pub msg_fctn: MessageFunction23Code,
    #[validate]
    #[serde(rename = "PrtcolVrsn")]
    pub prtcol_vrsn: Max2048Text,
    #[serde(rename = "XchgId", skip_serializing_if = "Option::is_none")]
    pub xchg_id: Option<Max35Text>,
    #[serde(rename = "ReTrnsmssnCntr", skip_serializing_if = "Option::is_none")]
    pub re_trnsmssn_cntr: Option<Max3NumericText>,
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
    #[serde(rename = "BtchMgmtInf", skip_serializing_if = "Option::is_none")]
    pub btch_mgmt_inf: Option<BatchManagementInformation1>,
    #[validate]
    #[serde(rename = "InitgPty")]
    pub initg_pty: GenericIdentification183,
    #[serde(rename = "RcptPty", skip_serializing_if = "Option::is_none")]
    pub rcpt_pty: Option<GenericIdentification183>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TracData", default)]
    pub trac_data: Vec<AdditionalData1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tracblt", default)]
    pub tracblt: Vec<Traceability9>,
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
pub struct IsoMax3ACountryCode {
    #[validate(regex = "ISO_MAX_3_A_COUNTRY_CODE_REGEX")]
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
pub struct Max256Text {
    #[validate(length(min = 1, max = 256,))]
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
pub struct IsoDate {
    #[serde(rename = "$text")]
    pub value: ::chrono::NaiveDate,
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
pub struct Exact12Text {
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
pub struct Max8NumericText {
    #[validate(regex = "MAX_8_NUMERIC_TEXT_REGEX")]
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
pub struct TransactionContext10 {
    #[serde(rename = "CardPrgrmmApld", skip_serializing_if = "Option::is_none")]
    pub card_prgrmm_apld: Option<CardProgrammeMode3>,
    #[serde(rename = "Jursdctn", skip_serializing_if = "Option::is_none")]
    pub jursdctn: Option<Jurisdiction2>,
    #[serde(rename = "SttlmSvc", skip_serializing_if = "Option::is_none")]
    pub sttlm_svc: Option<SettlementService4>,
    #[serde(rename = "Rcncltn", skip_serializing_if = "Option::is_none")]
    pub rcncltn: Option<Reconciliation3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
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
pub struct Max3NumericText {
    #[validate(regex = "MAX_3_NUMERIC_TEXT_REGEX")]
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
pub struct Content1 {
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max20KText,
    #[serde(rename = "Sgntr", skip_serializing_if = "Option::is_none")]
    pub sgntr: Option<Max140Binary>,
    #[serde(rename = "CertId", skip_serializing_if = "Option::is_none")]
    pub cert_id: Option<Max70Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CreditDebit3Code {
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "DBIT")]
    Dbit,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UserInterface6Code {
    #[serde(rename = "CDSP")]
    Cdsp,
    #[serde(rename = "CRCP")]
    Crcp,
    #[serde(rename = "MDSP")]
    Mdsp,
    #[serde(rename = "MRCP")]
    Mrcp,
    #[serde(rename = "CRDO")]
    Crdo,
    #[serde(rename = "FILE")]
    File,
    #[serde(rename = "CHAP")]
    Chap,
    #[serde(rename = "MRAP")]
    Mrap,
    #[serde(rename = "MRIN")]
    Mrin,
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
pub struct Max70Text {
    #[validate(length(min = 1, max = 70,))]
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
pub struct Contact6 {
    #[serde(rename = "HomePhneNb", skip_serializing_if = "Option::is_none")]
    pub home_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "BizPhneNb", skip_serializing_if = "Option::is_none")]
    pub biz_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "MobPhneNb", skip_serializing_if = "Option::is_none")]
    pub mob_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "OthrPhneNb", skip_serializing_if = "Option::is_none")]
    pub othr_phne_nb: Option<PhoneNumber>,
    #[serde(rename = "PrsnlEmailAdr", skip_serializing_if = "Option::is_none")]
    pub prsnl_email_adr: Option<Max256Text>,
    #[serde(rename = "BizEmailAdr", skip_serializing_if = "Option::is_none")]
    pub biz_email_adr: Option<Max256Text>,
    #[serde(rename = "OthrEmailAdr", skip_serializing_if = "Option::is_none")]
    pub othr_email_adr: Option<Max256Text>,
    #[serde(rename = "HomeFaxNb", skip_serializing_if = "Option::is_none")]
    pub home_fax_nb: Option<PhoneNumber>,
    #[serde(rename = "BizFaxNb", skip_serializing_if = "Option::is_none")]
    pub biz_fax_nb: Option<PhoneNumber>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max256Text>,
    #[serde(rename = "Lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageCode>,
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
pub struct IsoTime {
    #[serde(rename = "$value")]
    pub value: ::chrono::naive::NaiveTime,
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
pub struct MacData1 {
    #[validate]
    #[serde(rename = "Ctrl")]
    pub ctrl: Exact1HexBinaryText,
    #[validate]
    #[serde(rename = "KeySetIdr")]
    pub key_set_idr: Max8NumericText,
    #[serde(rename = "DrvdInf", skip_serializing_if = "Option::is_none")]
    pub drvd_inf: Option<Max32HexBinaryText>,
    #[validate]
    #[serde(rename = "Algo")]
    pub algo: Max2NumericText,
    #[serde(rename = "KeyLngth", skip_serializing_if = "Option::is_none")]
    pub key_lngth: Option<Max4NumericText>,
    #[serde(rename = "KeyPrtcn", skip_serializing_if = "Option::is_none")]
    pub key_prtcn: Option<Max2NumericText>,
    #[serde(rename = "KeyIndx", skip_serializing_if = "Option::is_none")]
    pub key_indx: Option<Max5NumericText>,
    #[serde(rename = "PddgMtd", skip_serializing_if = "Option::is_none")]
    pub pddg_mtd: Option<Max2NumericText>,
    #[serde(rename = "InitlstnVctr", skip_serializing_if = "Option::is_none")]
    pub initlstn_vctr: Option<Max32HexBinaryText>,
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
pub struct IsoMax3ALanguageCode {
    #[validate(regex = "ISO_MAX_3_A_LANGUAGE_CODE_REGEX")]
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
pub struct Max15NumericText {
    #[validate(regex = "MAX_15_NUMERIC_TEXT_REGEX")]
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
pub struct SettlementService4 {
    #[serde(rename = "SttlmSvcApld", skip_serializing_if = "Option::is_none")]
    pub sttlm_svc_apld: Option<SettlementServiceMode1>,
    #[serde(rename = "SttlmSvcDts", skip_serializing_if = "Option::is_none")]
    pub sttlm_svc_dts: Option<SettlementServiceDate2>,
    #[serde(rename = "SttlmRptgNtty", skip_serializing_if = "Option::is_none")]
    pub sttlm_rptg_ntty: Option<SettlementReportingEntity1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlSttlmInf", default)]
    pub addtl_sttlm_inf: Vec<AdditionalData1>,
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
pub struct Traceability9 {
    #[validate]
    #[serde(rename = "RlayId")]
    pub rlay_id: GenericIdentification183,
    #[serde(rename = "TracDtTmIn", skip_serializing_if = "Option::is_none")]
    pub trac_dt_tm_in: Option<IsoDateTime>,
    #[serde(rename = "TracDtTmOut", skip_serializing_if = "Option::is_none")]
    pub trac_dt_tm_out: Option<IsoDateTime>,
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
pub struct ImpliedCurrencyAndAmount {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
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
pub struct Iso8583ResponseCode {
    #[validate(regex = "ISO_8583_RESPONSE_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ActionDestination1Code {
    #[serde(rename = "FILE")]
    File,
    #[serde(rename = "MOBL")]
    Mobl,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PECR")]
    Pecr,
    #[serde(rename = "POFS")]
    Pofs,
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
pub struct ErrorDetails2 {
    #[serde(rename = "MsgErrTp")]
    pub msg_err_tp: MessageError1Code,
    #[serde(rename = "OthrMsgErrTp", skip_serializing_if = "Option::is_none")]
    pub othr_msg_err_tp: Option<Max35Text>,
    #[serde(rename = "ErrCd", skip_serializing_if = "Option::is_none")]
    pub err_cd: Option<Max35Text>,
    #[serde(rename = "ErrDesc", skip_serializing_if = "Option::is_none")]
    pub err_desc: Option<Max500Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DataElmtInErr", default)]
    pub data_elmt_in_err: Vec<Max4000Text>,
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
pub struct Max140Text {
    #[validate(length(min = 1, max = 140,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType18Code {
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "CSCH")]
    Csch,
    #[serde(rename = "AGNT")]
    Agnt,
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
pub struct Context17 {
    #[serde(rename = "TxCntxt", skip_serializing_if = "Option::is_none")]
    pub tx_cntxt: Option<TransactionContext10>,
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
pub struct SupplementaryData1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
    pub plc_and_nm: Option<Max350Text>,
    #[validate]
    #[serde(rename = "Envlp")]
    pub envlp: SupplementaryDataEnvelope1<A>,
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
pub struct TrueFalseIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
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
pub struct SupplementaryDataEnvelope1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(flatten)]
    pub value: A,
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
pub struct Transaction134 {
    #[serde(rename = "NtwkMgmtTp", skip_serializing_if = "Option::is_none")]
    pub ntwk_mgmt_tp: Option<NetworkManagementType1Code>,
    #[serde(rename = "OthrNtwkMgmtTp", skip_serializing_if = "Option::is_none")]
    pub othr_ntwk_mgmt_tp: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "MsgRsn", default)]
    pub msg_rsn: Vec<Iso8583MessageReasonCode>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AltrnMsgRsn", default)]
    pub altrn_msg_rsn: Vec<Max256Text>,
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: TransactionIdentification12,
    #[serde(rename = "NbOfMsgs", skip_serializing_if = "Option::is_none")]
    pub nb_of_msgs: Option<Number>,
    #[serde(rename = "MaxNbOfMsgs", skip_serializing_if = "Option::is_none")]
    pub max_nb_of_msgs: Option<Number>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlFee", default)]
    pub addtl_fee: Vec<AdditionalFee2>,
    #[serde(rename = "TxDesc", skip_serializing_if = "Option::is_none")]
    pub tx_desc: Option<Max1000Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
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
pub struct Max2NumericText {
    #[validate(regex = "MAX_2_NUMERIC_TEXT_REGEX")]
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
pub struct Max20KText {
    #[validate(length(min = 1, max = 20000,))]
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
pub struct Max1000Text {
    #[validate(length(min = 1, max = 1000,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType20Code {
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "CRDH")]
    Crdh,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "AGNT")]
    Agnt,
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
pub struct SettlementServiceMode1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "SttlmPrty", skip_serializing_if = "Option::is_none")]
    pub sttlm_prty: Option<Priority3Code>,
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
pub struct AdditionalData1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<Max2048Text>,
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
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "NtwkMgmtRspn")]
    pub ntwk_mgmt_rspn: NetworkManagementResponseV03<A>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NetworkManagementType1Code {
    #[serde(rename = "ECTS")]
    Ects,
    #[serde(rename = "ESFW")]
    Esfw,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "SGNF")]
    Sgnf,
    #[serde(rename = "SGNN")]
    Sgnn,
    #[serde(rename = "DSFW")]
    Dsfw,
    #[serde(rename = "TSUN")]
    Tsun,
    #[serde(rename = "MOSB")]
    Mosb,
    #[serde(rename = "SPIN")]
    Spin,
    #[serde(rename = "IART")]
    Iart,
    #[serde(rename = "SYCL")]
    Sycl,
    #[serde(rename = "DRBI")]
    Drbi,
    #[serde(rename = "ERBI")]
    Erbi,
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
pub struct Jurisdiction2 {
    #[serde(rename = "DmstInd", skip_serializing_if = "Option::is_none")]
    pub dmst_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "DmstQlfctn", skip_serializing_if = "Option::is_none")]
    pub dmst_qlfctn: Option<Max35Text>,
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
pub struct AdditionalInformation29 {
    #[serde(rename = "Rcpt", skip_serializing_if = "Option::is_none")]
    pub rcpt: Option<PartyType22Code>,
    #[serde(rename = "OthrRcpt", skip_serializing_if = "Option::is_none")]
    pub othr_rcpt: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Trgt", default)]
    pub trgt: Vec<UserInterface6Code>,
    #[serde(rename = "OthrTrgt", skip_serializing_if = "Option::is_none")]
    pub othr_trgt: Option<Max35Text>,
    #[serde(rename = "Frmt", skip_serializing_if = "Option::is_none")]
    pub frmt: Option<OutputFormat4Code>,
    #[serde(rename = "OthrFrmt", skip_serializing_if = "Option::is_none")]
    pub othr_frmt: Option<Max35Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Lang")]
    pub lang: IsoMax3ALanguageCode,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max20KText,
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
pub struct Max4NumericText {
    #[validate(regex = "MAX_4_NUMERIC_TEXT_REGEX")]
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
pub struct TransactionLifeCycleIdentification2 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Exact15Text,
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
pub struct Iso3NumericCurrencyCode {
    #[validate(regex = "ISO_3_NUMERIC_CURRENCY_CODE_REGEX")]
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
pub struct SettlementServiceDate2 {
    #[serde(rename = "ReqdSttlmDt", skip_serializing_if = "Option::is_none")]
    pub reqd_sttlm_dt: Option<IsoDate>,
    #[serde(rename = "DfrrdSttlmInd", skip_serializing_if = "Option::is_none")]
    pub dfrrd_sttlm_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "SttlmDt", skip_serializing_if = "Option::is_none")]
    pub sttlm_dt: Option<IsoDate>,
    #[serde(rename = "SttlmTm", skip_serializing_if = "Option::is_none")]
    pub sttlm_tm: Option<IsoTime>,
    #[serde(rename = "SttlmPrd", skip_serializing_if = "Option::is_none")]
    pub sttlm_prd: Option<Max35Text>,
    #[serde(rename = "SttlmCutOffTm", skip_serializing_if = "Option::is_none")]
    pub sttlm_cut_off_tm: Option<IsoDateTime>,
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
pub struct Exact1HexBinaryText {
    #[validate(regex = "EXACT_1_HEX_BINARY_TEXT_REGEX")]
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
pub struct Max5NumericText {
    #[validate(regex = "MAX_5_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType17Code {
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "ACQP")]
    Acqp,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "CISP")]
    Cisp,
    #[serde(rename = "AGNT")]
    Agnt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType21Code {
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "CRDH")]
    Crdh,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType26Code {
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "ICCA")]
    Icca,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "DLIS")]
    Dlis,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MessageFunction23Code {
    #[serde(rename = "ADVC")]
    Advc,
    #[serde(rename = "REQU")]
    Requ,
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
pub struct TransactionIdentification12 {
    #[validate]
    #[serde(rename = "SysTracAudtNb")]
    pub sys_trac_audt_nb: Max12NumericText,
    #[validate]
    #[serde(rename = "TrnsmssnDtTm")]
    pub trnsmssn_dt_tm: IsoDateTime,
    #[serde(rename = "RtrvlRefNb", skip_serializing_if = "Option::is_none")]
    pub rtrvl_ref_nb: Option<Exact12Text>,
    #[serde(rename = "LifeCyclTracIdData", skip_serializing_if = "Option::is_none")]
    pub life_cycl_trac_id_data: Option<TransactionLifeCycleIdentification2>,
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
pub struct Max350Text {
    #[validate(length(min = 1, max = 350,))]
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
pub struct FeeAmount3 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<Iso3NumericCurrencyCode>,
    #[serde(rename = "FctvXchgRate", skip_serializing_if = "Option::is_none")]
    pub fctv_xchg_rate: Option<BaseOne25Rate>,
    #[serde(rename = "ConvsDt", skip_serializing_if = "Option::is_none")]
    pub convs_dt: Option<IsoDate>,
    #[serde(rename = "ConvsTm", skip_serializing_if = "Option::is_none")]
    pub convs_tm: Option<IsoTime>,
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
pub struct Max32HexBinaryText {
    #[validate(regex = "MAX_32_HEX_BINARY_TEXT_REGEX")]
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
pub struct YesNoIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
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
pub struct NetworkManagementResponse2<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Cntxt", skip_serializing_if = "Option::is_none")]
    pub cntxt: Option<Context17>,
    #[validate]
    #[serde(rename = "Tx")]
    pub tx: Transaction134,
    #[validate]
    #[serde(rename = "PrcgRslt")]
    pub prcg_rslt: ProcessingResult9,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<A>>,
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
pub struct AdditionalAction1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ActionType10Code>,
    #[serde(rename = "Dstn", skip_serializing_if = "Option::is_none")]
    pub dstn: Option<PartyType21Code>,
    #[serde(rename = "OthrDstn", skip_serializing_if = "Option::is_none")]
    pub othr_dstn: Option<Max35Text>,
    #[serde(rename = "DstnTp", skip_serializing_if = "Option::is_none")]
    pub dstn_tp: Option<ActionDestination1Code>,
    #[serde(rename = "OthrDstnTp", skip_serializing_if = "Option::is_none")]
    pub othr_dstn_tp: Option<Max35Text>,
    #[serde(rename = "DstnAdr", skip_serializing_if = "Option::is_none")]
    pub dstn_adr: Option<Max70Text>,
    #[serde(rename = "Frmt", skip_serializing_if = "Option::is_none")]
    pub frmt: Option<OutputFormat4Code>,
    #[serde(rename = "OthrFrmt", skip_serializing_if = "Option::is_none")]
    pub othr_frmt: Option<Max35Text>,
    #[serde(rename = "Cntt", skip_serializing_if = "Option::is_none")]
    pub cntt: Option<Content1>,
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
pub struct Exact15Text {
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
pub struct LanguageCode {
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
pub struct NetworkManagementResponseV03<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Hdr")]
    pub hdr: Header63,
    #[validate]
    #[serde(rename = "Body")]
    pub body: NetworkManagementResponse2<A>,
    #[serde(rename = "SctyTrlr", skip_serializing_if = "Option::is_none")]
    pub scty_trlr: Option<ContentInformationType20>,
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
pub struct ResultData7 {
    #[serde(rename = "Rslt", skip_serializing_if = "Option::is_none")]
    pub rslt: Option<Response8Code>,
    #[serde(rename = "OthrRslt", skip_serializing_if = "Option::is_none")]
    pub othr_rslt: Option<Max35Text>,
    #[serde(rename = "RsltDtls")]
    pub rslt_dtls: Iso8583ResponseCode,
    #[serde(rename = "OthrRsltDtls", skip_serializing_if = "Option::is_none")]
    pub othr_rslt_dtls: Option<Max35Text>,
    #[serde(
        rename = "TempScrCardDataReusePrtd",
        skip_serializing_if = "Option::is_none"
    )]
    pub temp_scr_card_data_reuse_prtd: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlRsltInf", default)]
    pub addtl_rslt_inf: Vec<AdditionalData1>,
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
pub struct Reconciliation3 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "ChckptRef", skip_serializing_if = "Option::is_none")]
    pub chckpt_ref: Option<Max35Text>,
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
pub struct Max12NumericText {
    #[validate(regex = "MAX_12_NUMERIC_TEXT_REGEX")]
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
pub struct ProcessingResult9 {
    #[serde(rename = "RspnSrc", skip_serializing_if = "Option::is_none")]
    pub rspn_src: Option<ApprovalEntity2>,
    #[validate]
    #[serde(rename = "RsltData")]
    pub rslt_data: ResultData7,
    #[serde(rename = "ApprvlCd", skip_serializing_if = "Option::is_none")]
    pub apprvl_cd: Option<Exact6AlphaNumericText>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ErrDtl", default)]
    pub err_dtl: Vec<ErrorDetails2>,
    #[serde(rename = "OrgnlRsltData", skip_serializing_if = "Option::is_none")]
    pub orgnl_rslt_data: Option<ResultData7>,
    #[serde(rename = "ActnReqrd", skip_serializing_if = "Option::is_none")]
    pub actn_reqrd: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Actn", default)]
    pub actn: Vec<Action13>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlActn", default)]
    pub addtl_actn: Vec<AdditionalAction1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation29>,
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
pub struct ApprovalEntity2 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Tp")]
    pub tp: PartyType26Code,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<PartyType9Code>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<IsoMax3ACountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType9Code {
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "ACQP")]
    Acqp,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "CISP")]
    Cisp,
    #[serde(rename = "CSCH")]
    Csch,
    #[serde(rename = "SCHP")]
    Schp,
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
pub struct SettlementReportingEntity1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
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
pub struct AdditionalFee2 {
    #[serde(rename = "Tp")]
    pub tp: TypeOfAmount21Code,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "FeePrgm", skip_serializing_if = "Option::is_none")]
    pub fee_prgm: Option<Max35Text>,
    #[serde(rename = "FeeDscrptr", skip_serializing_if = "Option::is_none")]
    pub fee_dscrptr: Option<Max35Text>,
    #[validate]
    #[serde(rename = "FeeAmt")]
    pub fee_amt: FeeAmount3,
    #[serde(rename = "FeeRcncltnAmt", skip_serializing_if = "Option::is_none")]
    pub fee_rcncltn_amt: Option<FeeAmount3>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
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
pub struct GenericIdentification183 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PartyType17Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<PartyType18Code>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<IsoMax3ACountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ActionType11Code {
    #[serde(rename = "CNTI")]
    Cnti,
    #[serde(rename = "CNIS")]
    Cnis,
    #[serde(rename = "CNTA")]
    Cnta,
    #[serde(rename = "CNAS")]
    Cnas,
    #[serde(rename = "CPTR")]
    Cptr,
    #[serde(rename = "CHDV")]
    Chdv,
    #[serde(rename = "VIPM")]
    Vipm,
    #[serde(rename = "TRCK")]
    Trck,
    #[serde(rename = "TRXR")]
    Trxr,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "SIGN")]
    Sign,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OutputFormat4Code {
    #[serde(rename = "FLNM")]
    Flnm,
    #[serde(rename = "MREF")]
    Mref,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "SMSI")]
    Smsi,
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "URLI")]
    Urli,
    #[serde(rename = "HTML")]
    Html,
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
pub struct Iso8583MessageReasonCode {
    #[validate(regex = "ISO_8583_MESSAGE_REASON_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Priority3Code {
    #[serde(rename = "URGT")]
    Urgt,
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "NORM")]
    Norm,
    #[default]
    Unknown,
}
