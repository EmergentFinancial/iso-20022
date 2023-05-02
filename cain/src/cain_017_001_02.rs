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
    static ref EXTERNAL_ENCRYPTED_ELEMENT_IDENTIFICATION_1_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_3_NUMERIC_COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_8583_TRANSACTION_TYPE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_15_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,15}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_11_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,11}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_140_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5000_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_8_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_9999_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,9999}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_23_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,23}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_8583_ACCOUNT_TYPE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_18245_MERCHANT_CATEGORY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_12_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,12}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_32_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,32}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_10_K_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,10000}  "#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_35_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_2_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_MAX_3_A_COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_5_MAX_16_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_COUNTRY_SUB_DIVISION_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_1_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_35_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,35}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_500_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_3_NUMERIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_8583_AMOUNT_TYPE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_100_K_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_8_HEX_BINARY_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9A-F][0-9A-F]){1,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_YEAR_MONTH_REGEX: ::regex::Regex = ::regex::Regex::new(r#"^-?\d{4}-(0[1-9]|1[0-2])([+-]\d{2}:\d{2}|Z)?$"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_2_A_LANGUAGE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_8583_RESPONSE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_19_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,19}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref GEOGRAPHIC_POINT_IN_DECIMAL_DEGREES_REGEX: ::regex::Regex = ::regex::Regex::new(r#"(\+|-)?[\d]{1,3}(\.[\d]{1,8})?/(\+|-)?[\d]{1,3}(\.[\d]{1,8})?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_MAX_3_A_LANGUAGE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_2_MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_2_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,4}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:cain.017.001.02".to_string()
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
pub struct Terminal6 {
    #[validate]
    #[serde(rename = "TermnlId")]
    pub termnl_id: TerminalIdentification3,
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
pub struct AccountDetails3 {
    #[serde(rename = "AcctNm", skip_serializing_if = "Option::is_none")]
    pub acct_nm: Option<Max70Text>,
    #[serde(rename = "AcctTp", skip_serializing_if = "Option::is_none")]
    pub acct_tp: Option<Iso8583AccountTypeCode>,
    #[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<Max70Text>,
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
pub struct ExternalEncryptedElementIdentification1Code {
    #[validate(regex = "EXTERNAL_ENCRYPTED_ELEMENT_IDENTIFICATION_1_CODE_REGEX")]
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
pub struct ExternalAuthenticationMethod1Code {
    #[validate(length(min = 1, max = 4,))]
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
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "NqryRspn")]
    pub nqry_rspn: InquiryResponseV02<A>,
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
pub struct RiskInputData1 {
    #[serde(rename = "Ntty", skip_serializing_if = "Option::is_none")]
    pub ntty: Option<RiskAssessmentDataEntityProvider1>,
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max10KText,
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
pub struct InstalmentAmountDetails2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<InstalmentAmountDetailsType2Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "SubTp", skip_serializing_if = "Option::is_none")]
    pub sub_tp: Option<Max35Text>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<Amount16>,
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<PercentageRate>,
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
pub struct Max100Text {
    #[validate(length(min = 1, max = 100,))]
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
pub struct Iso3NumericCountryCode {
    #[validate(regex = "ISO_3_NUMERIC_COUNTRY_CODE_REGEX")]
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
pub struct ImpliedCurrencyAndAmount {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PlanOwner1Code {
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "ISSR")]
    Issr,
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
pub struct Wallet2 {
    #[serde(rename = "Prvdr", skip_serializing_if = "Option::is_none")]
    pub prvdr: Option<PartyIdentification258>,
    #[serde(rename = "PANAge", skip_serializing_if = "Option::is_none")]
    pub pan_age: Option<Max5PositiveNumber>,
    #[serde(rename = "UsrAcctAge", skip_serializing_if = "Option::is_none")]
    pub usr_acct_age: Option<Max5PositiveNumber>,
    #[serde(rename = "WlltAcctAge", skip_serializing_if = "Option::is_none")]
    pub wllt_acct_age: Option<Max5PositiveNumber>,
    #[serde(rename = "DaysSncLastActvty", skip_serializing_if = "Option::is_none")]
    pub days_snc_last_actvty: Option<Max5PositiveNumber>,
    #[serde(rename = "Actvty", skip_serializing_if = "Option::is_none")]
    pub actvty: Option<Max10PositiveNumber>,
    #[serde(rename = "ActvtyIntrvl", skip_serializing_if = "Option::is_none")]
    pub actvty_intrvl: Option<Frequency12Code>,
    #[serde(rename = "LastWlltChng", skip_serializing_if = "Option::is_none")]
    pub last_wllt_chng: Option<Max5PositiveNumber>,
    #[serde(rename = "SspdCrds", skip_serializing_if = "Option::is_none")]
    pub sspd_crds: Option<Max5PositiveNumber>,
    #[serde(rename = "WlltAcctCtry", skip_serializing_if = "Option::is_none")]
    pub wllt_acct_ctry: Option<IsoMax3ACountryCode>,
    #[serde(rename = "CardDataNtryMd", skip_serializing_if = "Option::is_none")]
    pub card_data_ntry_md: Option<CardDataReading9Code>,
    #[serde(rename = "OthrCardDataNtryMd", skip_serializing_if = "Option::is_none")]
    pub othr_card_data_ntry_md: Option<Max35Text>,
    #[serde(rename = "WlltAcctEmailAge", skip_serializing_if = "Option::is_none")]
    pub wllt_acct_email_age: Option<Max5PositiveNumber>,
    #[serde(rename = "WlltPrvdrRskAssmnt", skip_serializing_if = "Option::is_none")]
    pub wllt_prvdr_rsk_assmnt: Option<RiskAssessment1Code>,
    #[serde(
        rename = "WlltPrvdrRskAssmntMdlVrsn",
        skip_serializing_if = "Option::is_none"
    )]
    pub wllt_prvdr_rsk_assmnt_mdl_vrsn: Option<Max35Text>,
    #[serde(rename = "WlltPrvdrPhneScore", skip_serializing_if = "Option::is_none")]
    pub wllt_prvdr_phne_score: Option<Max5PositiveNumber>,
    #[serde(rename = "WlltPrvdrDvcScore", skip_serializing_if = "Option::is_none")]
    pub wllt_prvdr_dvc_score: Option<Max5PositiveNumber>,
    #[serde(rename = "WlltPrvdrAcctScore", skip_serializing_if = "Option::is_none")]
    pub wllt_prvdr_acct_score: Option<Max5PositiveNumber>,
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
pub struct ECommerceData1 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max2048Text,
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
pub struct Iso8583TransactionTypeCode {
    #[validate(regex = "ISO_8583_TRANSACTION_TYPE_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct Instalment4 {
    #[serde(rename = "PmtSeqNb", skip_serializing_if = "Option::is_none")]
    pub pmt_seq_nb: Option<Number>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Plan", default)]
    pub plan: Vec<Plan2>,
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
pub struct Parameter13 {
    #[serde(rename = "DgstAlgo", skip_serializing_if = "Option::is_none")]
    pub dgst_algo: Option<Algorithm20Code>,
    #[serde(rename = "MskGnrtrAlgo", skip_serializing_if = "Option::is_none")]
    pub msk_gnrtr_algo: Option<AlgorithmIdentification26>,
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
pub struct LocalData4 {
    #[serde(rename = "Lang")]
    pub lang: IsoMax3ALanguageCode,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max70Text>,
    #[serde(rename = "LglCorpNm", skip_serializing_if = "Option::is_none")]
    pub lgl_corp_nm: Option<Max210Text>,
    #[serde(rename = "NmAndLctn", skip_serializing_if = "Option::is_none")]
    pub nm_and_lctn: Option<Max200Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address3>,
    #[serde(rename = "AddtlAdrInf", skip_serializing_if = "Option::is_none")]
    pub addtl_adr_inf: Option<Max512Text>,
    #[serde(rename = "AddtlCtctInf", skip_serializing_if = "Option::is_none")]
    pub addtl_ctct_inf: Option<Max512Text>,
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
pub struct Max11NumericText {
    #[validate(regex = "MAX_11_NUMERIC_TEXT_REGEX")]
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
pub struct Address3 {
    #[serde(rename = "AdrLine1", skip_serializing_if = "Option::is_none")]
    pub adr_line_1: Option<Max200Text>,
    #[serde(rename = "AdrLine2", skip_serializing_if = "Option::is_none")]
    pub adr_line_2: Option<Max200Text>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max200Text>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max35Text>,
    #[serde(rename = "PstlCd", skip_serializing_if = "Option::is_none")]
    pub pstl_cd: Option<Max35Text>,
    #[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
    pub twn_nm: Option<Max100Text>,
    #[serde(rename = "CtrySubDvsnMnrNm", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mnr_nm: Option<Max100Text>,
    #[serde(rename = "CtrySubDvsnMjrNm", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mjr_nm: Option<Max100Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<IsoMax3ACountryCode>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BytePadding1Code {
    #[serde(rename = "LNGT")]
    Lngt,
    #[serde(rename = "NUL8")]
    Nul8,
    #[serde(rename = "NULG")]
    Nulg,
    #[serde(rename = "NULL")]
    Null,
    #[serde(rename = "RAND")]
    Rand,
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
pub struct InquiryResponseV02<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Hdr")]
    pub hdr: Header60,
    #[validate]
    #[serde(rename = "Body")]
    pub body: InquiryResponse2<A>,
    #[serde(rename = "SctyTrlr", skip_serializing_if = "Option::is_none")]
    pub scty_trlr: Option<ContentInformationType20>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm8Code {
    #[serde(rename = "MGF1")]
    Mgf1,
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
pub struct SponsoredMerchant2 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: PartyIdentification262,
    #[serde(rename = "CmonNm", skip_serializing_if = "Option::is_none")]
    pub cmon_nm: Option<Max140Text>,
    #[serde(rename = "LglCorpNm", skip_serializing_if = "Option::is_none")]
    pub lgl_corp_nm: Option<Max99Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "AddtlAdrInf", skip_serializing_if = "Option::is_none")]
    pub addtl_adr_inf: Option<Max256Text>,
    #[serde(rename = "GeogcLctn", skip_serializing_if = "Option::is_none")]
    pub geogc_lctn: Option<GeographicPointInDecimalDegrees>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
    #[serde(rename = "LclData", skip_serializing_if = "Option::is_none")]
    pub lcl_data: Option<LocalData5>,
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
pub struct Max5000Binary {
    #[validate(length(min = 1, max = 5000,), regex = "MAX_5000_BINARY_REGEX")]
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
pub struct InquiryResponse2<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Envt")]
    pub envt: Environment24,
    #[validate]
    #[serde(rename = "Cntxt")]
    pub cntxt: Context13,
    #[validate]
    #[serde(rename = "Tx")]
    pub tx: Transaction132,
    #[serde(rename = "AdddmData", skip_serializing_if = "Option::is_none")]
    pub adddm_data: Option<AddendumData5>,
    #[validate]
    #[serde(rename = "PrcgRslt")]
    pub prcg_rslt: ProcessingResult17,
    #[serde(rename = "ICCRltdData", skip_serializing_if = "Option::is_none")]
    pub icc_rltd_data: Option<Max10KHexBinaryText>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrtctdData", default)]
    pub prtctd_data: Vec<ProtectedData1>,
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
pub struct AdditionalRiskData1 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max10KText,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm7Code {
    #[serde(rename = "ERSA")]
    Ersa,
    #[serde(rename = "RSAO")]
    Rsao,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EncryptionFormat3Code {
    #[serde(rename = "TR34")]
    Tr34,
    #[serde(rename = "TR31")]
    Tr31,
    #[serde(rename = "CTCE")]
    Ctce,
    #[serde(rename = "CBCE")]
    Cbce,
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
pub struct Max8NumericText {
    #[validate(regex = "MAX_8_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum QrCodePresentmentMode1Code {
    #[serde(rename = "CPMD")]
    Cpmd,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "MPMD")]
    Mpmd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GracePeriodUnitType1Code {
    #[serde(rename = "WEKS")]
    Weks,
    #[serde(rename = "PMTS")]
    Pmts,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "DAYS")]
    Days,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProtectionMethod1Code {
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "SELM")]
    Selm,
    #[serde(rename = "SNCL")]
    Sncl,
    #[serde(rename = "SOFT")]
    Soft,
    #[serde(rename = "TEEN")]
    Teen,
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
pub struct Recipient5ChoiceEnum {
    #[serde(rename = "IssrAndSrlNb", skip_serializing_if = "Option::is_none")]
    pub issr_and_srl_nb: Option<IssuerAndSerialNumber1>,
    #[serde(rename = "KeyIdr", skip_serializing_if = "Option::is_none")]
    pub key_idr: Option<KekIdentifier2>,
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
pub struct Recipient5Choice {
    #[serde(flatten)]
    pub value: Recipient5ChoiceEnum,
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
pub struct EnvelopedData6 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Rcpt", default)]
    pub rcpt: Vec<Recipient7Choice>,
    #[serde(rename = "NcrptdCntt", skip_serializing_if = "Option::is_none")]
    pub ncrptd_cntt: Option<EncryptedContent5>,
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
pub struct CardholderName2 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
    pub gvn_nm: Option<Max70Text>,
    #[serde(rename = "MddlNm", skip_serializing_if = "Option::is_none")]
    pub mddl_nm: Option<Max70Text>,
    #[serde(rename = "LastNm", skip_serializing_if = "Option::is_none")]
    pub last_nm: Option<Max70Text>,
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
pub struct Max500Text {
    #[validate(length(min = 1, max = 500,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm23Code {
    #[serde(rename = "EA2C")]
    Ea2C,
    #[serde(rename = "E3DC")]
    E3Dc,
    #[serde(rename = "EA9C")]
    Ea9C,
    #[serde(rename = "EA5C")]
    Ea5C,
    #[serde(rename = "EA2R")]
    Ea2R,
    #[serde(rename = "EA9R")]
    Ea9R,
    #[serde(rename = "EA5R")]
    Ea5R,
    #[serde(rename = "E3DR")]
    E3Dr,
    #[serde(rename = "E36C")]
    E36C,
    #[serde(rename = "E36R")]
    E36R,
    #[serde(rename = "SD5C")]
    Sd5C,
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
pub struct SpecialConditions1 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<Max35Text>,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum StorageLocation1Code {
    #[serde(rename = "CAWL")]
    Cawl,
    #[serde(rename = "DVCE")]
    Dvce,
    #[serde(rename = "ISWL")]
    Iswl,
    #[serde(rename = "ONFL")]
    Onfl,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "TPWL")]
    Tpwl,
    #[default]
    Unknown,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Frequency12Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "FRTN")]
    Frtn,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "QURT")]
    Qurt,
    #[serde(rename = "MIAN")]
    Mian,
    #[serde(rename = "TEND")]
    Tend,
    #[serde(rename = "WEEK")]
    Week,
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
pub struct SpecialProgrammeQualification1 {
    #[serde(rename = "Prgrmm", skip_serializing_if = "Option::is_none")]
    pub prgrmm: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Dtl", default)]
    pub dtl: Vec<SpecialProgrammeDetails1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ActionType8Code {
    #[serde(rename = "APPV")]
    Appv,
    #[serde(rename = "BLCK")]
    Blck,
    #[serde(rename = "CPTR")]
    Cptr,
    #[serde(rename = "DCLN")]
    Dcln,
    #[serde(rename = "RQID")]
    Rqid,
    #[serde(rename = "NDCL")]
    Ndcl,
    #[serde(rename = "RFRL")]
    Rfrl,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "STUA")]
    Stua,
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
pub struct Max9999HexBinaryText {
    #[validate(regex = "MAX_9999_HEX_BINARY_TEXT_REGEX")]
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
pub struct Verification4 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ExternalAuthenticationMethod1Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "SubTp", skip_serializing_if = "Option::is_none")]
    pub sub_tp: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "VrfctnRslt", default)]
    pub vrfctn_rslt: Vec<VerificationResult2>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FeeCollectionInitiator1Code {
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "CISS")]
    Ciss,
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
pub struct Max5PositiveNumber {
    #[validate(range(min = 1,))]
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
pub struct Max10KText {
    #[validate(length(min = 1, max = 10000,))]
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
pub struct PartyIdentification258 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Max35Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Iso3NumericCountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "LglCorpNm", skip_serializing_if = "Option::is_none")]
    pub lgl_corp_nm: Option<Max99Text>,
    #[serde(rename = "AddtlId", skip_serializing_if = "Option::is_none")]
    pub addtl_id: Option<AdditionalData1>,
    #[serde(rename = "LclData", skip_serializing_if = "Option::is_none")]
    pub lcl_data: Option<LocalData1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Frequency18Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "FRTN")]
    Frtn,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "QURT")]
    Qurt,
    #[serde(rename = "MIAN")]
    Mian,
    #[serde(rename = "TEND")]
    Tend,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "TWWK")]
    Twwk,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BalanceType15Code {
    #[serde(rename = "AMOH")]
    Amoh,
    #[serde(rename = "AMTO")]
    Amto,
    #[serde(rename = "AMTD")]
    Amtd,
    #[serde(rename = "CRDL")]
    Crdl,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "AVLB")]
    Avlb,
    #[serde(rename = "CLRI")]
    Clri,
    #[serde(rename = "LDGR")]
    Ldgr,
    #[serde(rename = "PNTS")]
    Pnts,
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
pub struct TerminalIdentification3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max16Text,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Max35Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<IsoMax3ACountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
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
pub struct KeyTransport6 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[serde(rename = "RcptId")]
    pub rcpt_id: Recipient5Choice,
    #[validate]
    #[serde(rename = "KeyNcrptnAlgo")]
    pub key_ncrptn_algo: AlgorithmIdentification27,
    #[validate]
    #[serde(rename = "NcrptdKey")]
    pub ncrptd_key: Max5000Binary,
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
pub struct Max23NumericText {
    #[validate(regex = "MAX_23_NUMERIC_TEXT_REGEX")]
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
pub struct Max210Text {
    #[validate(length(min = 1, max = 210,))]
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
pub struct Iso8583AccountTypeCode {
    #[validate(regex = "ISO_8583_ACCOUNT_TYPE_CODE_REGEX")]
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
pub struct Iso18245MerchantCategoryCode {
    #[validate(regex = "ISO_18245_MERCHANT_CATEGORY_CODE_REGEX")]
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
pub struct AdditionalService2 {
    #[serde(rename = "Tp")]
    pub tp: AdditionalServiceType2Code,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Rslt", skip_serializing_if = "Option::is_none")]
    pub rslt: Option<AdditionalServiceResult1Code>,
    #[serde(rename = "OthrRslt", skip_serializing_if = "Option::is_none")]
    pub othr_rslt: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SvcDtl", default)]
    pub svc_dtl: Vec<AdditionalData1>,
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
pub struct RelativeDistinguishedName1 {
    #[serde(rename = "AttrTp")]
    pub attr_tp: AttributeType1Code,
    #[validate]
    #[serde(rename = "AttrVal")]
    pub attr_val: Max140Text,
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
pub struct AdditionalAmounts3 {
    #[serde(rename = "Tp")]
    pub tp: Iso8583AmountTypeCode,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: Amount17,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max70Text>,
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
pub struct PointOfServiceContext4 {
    #[serde(rename = "CardPres", skip_serializing_if = "Option::is_none")]
    pub card_pres: Option<TrueFalseIndicator>,
    #[serde(rename = "CrdhldrPres", skip_serializing_if = "Option::is_none")]
    pub crdhldr_pres: Option<TrueFalseIndicator>,
    #[serde(rename = "CrdhldrActvtd", skip_serializing_if = "Option::is_none")]
    pub crdhldr_actvtd: Option<TrueFalseIndicator>,
    #[serde(rename = "TrnspndrInittd", skip_serializing_if = "Option::is_none")]
    pub trnspndr_inittd: Option<TrueFalseIndicator>,
    #[serde(rename = "AttnddInd", skip_serializing_if = "Option::is_none")]
    pub attndd_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "UattnddLvlCtgy", skip_serializing_if = "Option::is_none")]
    pub uattndd_lvl_ctgy: Option<Max35NumericText>,
    #[serde(rename = "EComrcInd", skip_serializing_if = "Option::is_none")]
    pub e_comrc_ind: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "EComrcData", default)]
    pub e_comrc_data: Vec<ECommerceData1>,
    #[serde(rename = "MOTOCd", skip_serializing_if = "Option::is_none")]
    pub moto_cd: Option<Moto1Code>,
    #[serde(rename = "TrnstInd", skip_serializing_if = "Option::is_none")]
    pub trnst_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "PrtlApprvlSpprtd", skip_serializing_if = "Option::is_none")]
    pub prtl_apprvl_spprtd: Option<TrueFalseIndicator>,
    #[serde(rename = "DelydAuthstnInd", skip_serializing_if = "Option::is_none")]
    pub delyd_authstn_ind: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SctyChrtcs", default)]
    pub scty_chrtcs: Vec<SecurityCharacteristics1Code>,
    #[serde(rename = "OthrSctyChrtcs", skip_serializing_if = "Option::is_none")]
    pub othr_scty_chrtcs: Option<Max35Text>,
    #[serde(rename = "CardDataNtryMd", skip_serializing_if = "Option::is_none")]
    pub card_data_ntry_md: Option<CardDataReading10Code>,
    #[serde(rename = "OthrCardDataNtryMd", skip_serializing_if = "Option::is_none")]
    pub othr_card_data_ntry_md: Option<Max35Text>,
    #[serde(rename = "QRCdPresntmntMd", skip_serializing_if = "Option::is_none")]
    pub qr_cd_presntmnt_md: Option<QrCodePresentmentMode1Code>,
    #[serde(
        rename = "OthrQRCdPresntmntMd",
        skip_serializing_if = "Option::is_none"
    )]
    pub othr_qr_cd_presntmnt_md: Option<Max35Text>,
    #[serde(
        rename = "TempScrCardDataReusd",
        skip_serializing_if = "Option::is_none"
    )]
    pub temp_scr_card_data_reusd: Option<TrueFalseIndicator>,
    #[serde(rename = "StorgLctn", skip_serializing_if = "Option::is_none")]
    pub storg_lctn: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SpclConds", default)]
    pub spcl_conds: Vec<SpecialConditions1>,
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
pub enum Verification3Code {
    #[serde(rename = "FAIL")]
    Fail,
    #[serde(rename = "FUTA")]
    Futa,
    #[serde(rename = "MISS")]
    Miss,
    #[serde(rename = "NOSP")]
    Nosp,
    #[serde(rename = "NOVF")]
    Novf,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "SUCC")]
    Succ,
    #[serde(rename = "ERRR")]
    Errr,
    #[default]
    Unknown,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum VerificationEntity2Code {
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "ISSR")]
    Issr,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "CDAD")]
    Cdad,
    #[serde(rename = "ICCA")]
    Icca,
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
pub struct EncryptedContent5 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[validate]
    #[serde(rename = "CnttNcrptnAlgo")]
    pub cntt_ncrptn_algo: AlgorithmIdentification25,
    #[validate(length(min = 1,))]
    #[serde(rename = "NcrptdDataElmt", default)]
    pub ncrptd_data_elmt: Vec<EncryptedDataElement1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RiskAssessment1Code {
    #[serde(rename = "APPC")]
    Appc,
    #[serde(rename = "APPH")]
    Apph,
    #[serde(rename = "APPU")]
    Appu,
    #[serde(rename = "DONT")]
    Dont,
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
pub struct Balance28 {
    #[serde(rename = "Tp")]
    pub tp: BalanceType15Code,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<Iso3NumericCurrencyCode>,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
    #[serde(rename = "CrdhldrCcyInd", skip_serializing_if = "Option::is_none")]
    pub crdhldr_ccy_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "BalDt", skip_serializing_if = "Option::is_none")]
    pub bal_dt: Option<IsoDate>,
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
pub struct Environment24 {
    #[validate]
    #[serde(rename = "Acqrr")]
    pub acqrr: PartyIdentification263,
    #[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
    pub orgtr: Option<PartyIdentification263>,
    #[serde(rename = "Sndr", skip_serializing_if = "Option::is_none")]
    pub sndr: Option<PartyIdentification263>,
    #[serde(rename = "Rcvr", skip_serializing_if = "Option::is_none")]
    pub rcvr: Option<PartyIdentification263>,
    #[validate]
    #[serde(rename = "Accptr")]
    pub accptr: PartyIdentification255,
    #[serde(rename = "Dstn", skip_serializing_if = "Option::is_none")]
    pub dstn: Option<PartyIdentification263>,
    #[serde(rename = "Termnl", skip_serializing_if = "Option::is_none")]
    pub termnl: Option<Terminal6>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyIdentification263>,
    #[validate]
    #[serde(rename = "Card")]
    pub card: CardData9,
    #[serde(rename = "Wllt", skip_serializing_if = "Option::is_none")]
    pub wllt: Option<Wallet2>,
    #[serde(rename = "Tkn", skip_serializing_if = "Option::is_none")]
    pub tkn: Option<Token2>,
    #[serde(rename = "Crdhldr", skip_serializing_if = "Option::is_none")]
    pub crdhldr: Option<Cardholder19>,
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
pub struct Cardholder19 {
    #[serde(rename = "CrdhldrNm", skip_serializing_if = "Option::is_none")]
    pub crdhldr_nm: Option<CardholderName3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Id", default)]
    pub id: Vec<Credentials2>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "CtctInf", skip_serializing_if = "Option::is_none")]
    pub ctct_inf: Option<Contact1>,
    #[serde(rename = "DtOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_of_birth: Option<IsoDate>,
    #[serde(rename = "HghValCstmrInd", skip_serializing_if = "Option::is_none")]
    pub hgh_val_cstmr_ind: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
    #[serde(rename = "LclData", skip_serializing_if = "Option::is_none")]
    pub lcl_data: Option<LocalData7>,
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
pub struct Max10KHexBinaryText {
    #[validate(regex = "MAX_10_K_HEX_BINARY_TEXT_REGEX")]
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
pub struct AlgorithmIdentification27 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm7Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter13>,
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
pub struct LocalData7 {
    #[serde(rename = "Lang")]
    pub lang: IsoMax3ALanguageCode,
    #[serde(rename = "CrdhldrNm", skip_serializing_if = "Option::is_none")]
    pub crdhldr_nm: Option<CardholderName2>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address3>,
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
pub struct Max50Text {
    #[validate(length(min = 1, max = 50,))]
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
pub struct BaseOne25Rate {
    #[serde(rename = "$text")]
    pub value: f64,
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
pub struct Exact2NumericText {
    #[validate(regex = "EXACT_2_NUMERIC_TEXT_REGEX")]
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
pub struct AccountBalance2 {
    #[serde(rename = "AcctTp")]
    pub acct_tp: Iso8583AccountTypeCode,
    #[validate(length(min = 0,))]
    #[serde(rename = "Bal", default)]
    pub bal: Vec<Balance28>,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InstalmentPeriod1Code {
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "ANNU")]
    Annu,
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
pub struct SpecialProgrammeDetails1 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<Max35Text>,
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
pub struct Min5Max16Binary {
    #[validate(length(min = 5, max = 16,), regex = "MIN_5_MAX_16_BINARY_REGEX")]
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm5Code {
    #[serde(rename = "HS25")]
    Hs25,
    #[serde(rename = "HS38")]
    Hs38,
    #[serde(rename = "HS51")]
    Hs51,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CardDataReading10Code {
    #[serde(rename = "ICPY")]
    Icpy,
    #[serde(rename = "MGST")]
    Mgst,
    #[serde(rename = "ICCY")]
    Iccy,
    #[serde(rename = "MICR")]
    Micr,
    #[serde(rename = "MLEY")]
    Mley,
    #[serde(rename = "OCRR")]
    Ocrr,
    #[serde(rename = "MSIP")]
    Msip,
    #[serde(rename = "OPTC")]
    Optc,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "RFID")]
    Rfid,
    #[serde(rename = "UNSP")]
    Unsp,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "KEEN")]
    Keen,
    #[serde(rename = "DFLE")]
    Dfle,
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
pub struct IsoCountrySubDivisionCode {
    #[validate(regex = "ISO_COUNTRY_SUB_DIVISION_CODE_REGEX")]
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
pub struct Token2 {
    #[serde(rename = "PmtTkn", skip_serializing_if = "Option::is_none")]
    pub pmt_tkn: Option<Max19NumericText>,
    #[serde(rename = "TknXpryDt", skip_serializing_if = "Option::is_none")]
    pub tkn_xpry_dt: Option<IsoYearMonth>,
    #[serde(rename = "TknRqstrId", skip_serializing_if = "Option::is_none")]
    pub tkn_rqstr_id: Option<Max11NumericText>,
    #[serde(rename = "TknAssrncData", skip_serializing_if = "Option::is_none")]
    pub tkn_assrnc_data: Option<Max140Text>,
    #[serde(rename = "TknAssrncMtd", skip_serializing_if = "Option::is_none")]
    pub tkn_assrnc_mtd: Option<Max2NumericText>,
    #[serde(rename = "TknInittdInd", skip_serializing_if = "Option::is_none")]
    pub tkn_inittd_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "StorgLctn", skip_serializing_if = "Option::is_none")]
    pub storg_lctn: Option<StorageLocation1Code>,
    #[serde(rename = "OthrStorgLctn", skip_serializing_if = "Option::is_none")]
    pub othr_storg_lctn: Option<Max35Text>,
    #[serde(rename = "PrtcnMtd", skip_serializing_if = "Option::is_none")]
    pub prtcn_mtd: Option<ProtectionMethod1Code>,
    #[serde(rename = "OthrPrtcnMtd", skip_serializing_if = "Option::is_none")]
    pub othr_prtcn_mtd: Option<Max35Text>,
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
pub struct EncryptedData1 {
    #[serde(rename = "Ctrl", skip_serializing_if = "Option::is_none")]
    pub ctrl: Option<Exact1HexBinaryText>,
    #[serde(rename = "KeySetIdr", skip_serializing_if = "Option::is_none")]
    pub key_set_idr: Option<Max8NumericText>,
    #[serde(rename = "DrvdInf", skip_serializing_if = "Option::is_none")]
    pub drvd_inf: Option<Max32HexBinaryText>,
    #[serde(rename = "Algo", skip_serializing_if = "Option::is_none")]
    pub algo: Option<Max2NumericText>,
    #[serde(rename = "KeyLngth", skip_serializing_if = "Option::is_none")]
    pub key_lngth: Option<Max4NumericText>,
    #[serde(rename = "KeyPrtcn", skip_serializing_if = "Option::is_none")]
    pub key_prtcn: Option<Max2NumericText>,
    #[serde(rename = "KeyIndx", skip_serializing_if = "Option::is_none")]
    pub key_indx: Option<Max5NumericText>,
    #[serde(rename = "PddgMtd", skip_serializing_if = "Option::is_none")]
    pub pddg_mtd: Option<Max2NumericText>,
    #[serde(rename = "NcrptdDataFrmt", skip_serializing_if = "Option::is_none")]
    pub ncrptd_data_frmt: Option<Max2NumericText>,
    #[validate(length(min = 1,))]
    #[serde(rename = "NcrptdDataElmt", default)]
    pub ncrptd_data_elmt: Vec<EncryptedDataElement1>,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransactionInitiator1Code {
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "CUST")]
    Cust,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm20Code {
    #[serde(rename = "HS25")]
    Hs25,
    #[serde(rename = "HS38")]
    Hs38,
    #[serde(rename = "HS51")]
    Hs51,
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
pub struct IssuerAndSerialNumber1 {
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: CertificateIssuer1,
    #[validate]
    #[serde(rename = "SrlNb")]
    pub srl_nb: Max35Binary,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ContentType3Code {
    #[serde(rename = "EVLP")]
    Evlp,
    #[serde(rename = "IFSE")]
    Ifse,
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
pub struct Max350Text {
    #[validate(length(min = 1, max = 350,))]
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
pub struct CertificateIssuer1 {
    #[validate(length(min = 1,))]
    #[serde(rename = "RltvDstngshdNm", default)]
    pub rltv_dstngshd_nm: Vec<RelativeDistinguishedName1>,
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
pub struct PartyIdentification255 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Max35Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Iso3NumericCountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "LglCorpNm", skip_serializing_if = "Option::is_none")]
    pub lgl_corp_nm: Option<Max99Text>,
    #[serde(rename = "AddtlId", skip_serializing_if = "Option::is_none")]
    pub addtl_id: Option<AdditionalData1>,
    #[validate]
    #[serde(rename = "NmAndLctn")]
    pub nm_and_lctn: Max99Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address2>,
    #[serde(rename = "AddtlAdrInf", skip_serializing_if = "Option::is_none")]
    pub addtl_adr_inf: Option<Max256Text>,
    #[serde(rename = "GeogcLctn", skip_serializing_if = "Option::is_none")]
    pub geogc_lctn: Option<GeographicPointInDecimalDegrees>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Max256Text>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max256Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<Max35Text>,
    #[serde(rename = "CstmrSvc", skip_serializing_if = "Option::is_none")]
    pub cstmr_svc: Option<Max35Text>,
    #[serde(rename = "AddtlCtctInf", skip_serializing_if = "Option::is_none")]
    pub addtl_ctct_inf: Option<Max256Text>,
    #[serde(rename = "TaxRegnId", skip_serializing_if = "Option::is_none")]
    pub tax_regn_id: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
    #[serde(rename = "LclData", skip_serializing_if = "Option::is_none")]
    pub lcl_data: Option<LocalData4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SpnsrdMrchnt", default)]
    pub spnsrd_mrchnt: Vec<SponsoredMerchant2>,
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
pub struct Address2 {
    #[serde(rename = "AdrLine1", skip_serializing_if = "Option::is_none")]
    pub adr_line_1: Option<Max99Text>,
    #[serde(rename = "AdrLine2", skip_serializing_if = "Option::is_none")]
    pub adr_line_2: Option<Max99Text>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max99Text>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max16Text>,
    #[serde(rename = "PstlCd", skip_serializing_if = "Option::is_none")]
    pub pstl_cd: Option<Max16Text>,
    #[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
    pub twn_nm: Option<Max50Text>,
    #[serde(rename = "CtrySubDvsnMnr", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mnr: Option<IsoCountrySubDivisionCode>,
    #[serde(rename = "CtrySubDvsnMjr", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mjr: Option<IsoCountrySubDivisionCode>,
    #[serde(rename = "CtrySubDvsnMjrNm", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mjr_nm: Option<Max50Text>,
    #[serde(rename = "CtrySubDvsnMnrNm", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mnr_nm: Option<Max50Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<IsoMax3ACountryCode>,
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
pub struct Max35NumericText {
    #[validate(regex = "MAX_35_NUMERIC_TEXT_REGEX")]
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
pub struct IsoTime {
    #[serde(rename = "$value")]
    pub value: ::chrono::naive::NaiveTime,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecurityCharacteristics1Code {
    #[serde(rename = "CETE")]
    Cete,
    #[serde(rename = "CPTE")]
    Cpte,
    #[serde(rename = "CENC")]
    Cenc,
    #[serde(rename = "CMAC")]
    Cmac,
    #[serde(rename = "ETEE")]
    Etee,
    #[serde(rename = "METE")]
    Mete,
    #[serde(rename = "MPTE")]
    Mpte,
    #[serde(rename = "OPNN")]
    Opnn,
    #[serde(rename = "PMAC")]
    Pmac,
    #[serde(rename = "PKIE")]
    Pkie,
    #[serde(rename = "PRAE")]
    Prae,
    #[serde(rename = "PRAM")]
    Pram,
    #[serde(rename = "PRVN")]
    Prvn,
    #[serde(rename = "STAM")]
    Stam,
    #[serde(rename = "APTE")]
    Apte,
    #[serde(rename = "AETE")]
    Aete,
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
pub struct Exact12Text {
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CardDataReading9Code {
    #[serde(rename = "UNKW")]
    Unkw,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "CAMR")]
    Camr,
    #[serde(rename = "KEEN")]
    Keen,
    #[serde(rename = "ICPY")]
    Icpy,
    #[serde(rename = "OPTC")]
    Optc,
    #[serde(rename = "CDFL")]
    Cdfl,
    #[serde(rename = "MBNK")]
    Mbnk,
    #[serde(rename = "TOKN")]
    Tokn,
    #[serde(rename = "ICCY")]
    Iccy,
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
pub struct Parameter7 {
    #[serde(rename = "InitlstnVctr", skip_serializing_if = "Option::is_none")]
    pub initlstn_vctr: Option<Max500Binary>,
    #[serde(rename = "BPddg", skip_serializing_if = "Option::is_none")]
    pub b_pddg: Option<BytePadding1Code>,
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
pub struct Max500Binary {
    #[validate(length(min = 1, max = 500,), regex = "MAX_500_BINARY_REGEX")]
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
pub struct Parameter14 {
    #[serde(rename = "NcrptnFrmt", skip_serializing_if = "Option::is_none")]
    pub ncrptn_frmt: Option<EncryptionFormat3Code>,
    #[serde(rename = "InitlstnVctr", skip_serializing_if = "Option::is_none")]
    pub initlstn_vctr: Option<Max500Binary>,
    #[serde(rename = "BPddg", skip_serializing_if = "Option::is_none")]
    pub b_pddg: Option<BytePadding1Code>,
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
pub struct AccountStatementData2 {
    #[serde(rename = "StmtDt", skip_serializing_if = "Option::is_none")]
    pub stmt_dt: Option<IsoDate>,
    #[serde(rename = "StmtTm", skip_serializing_if = "Option::is_none")]
    pub stmt_tm: Option<IsoTime>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AcctStmt", default)]
    pub acct_stmt: Vec<AccountStatementDetails2>,
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
pub struct RiskContext2 {
    #[validate(length(min = 0,))]
    #[serde(rename = "RskInptData", default)]
    pub rsk_inpt_data: Vec<RiskInputData1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RskAssmnt", default)]
    pub rsk_assmnt: Vec<RiskAssessment2>,
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
pub struct Max16Text {
    #[validate(length(min = 1, max = 16,))]
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
pub struct Iso8583AmountTypeCode {
    #[validate(regex = "ISO_8583_AMOUNT_TYPE_CODE_REGEX")]
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
pub struct BaseOneRate {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EncryptedDataFormat1Code {
    #[serde(rename = "ASCI")]
    Asci,
    #[serde(rename = "BINF")]
    Binf,
    #[serde(rename = "EBCD")]
    Ebcd,
    #[serde(rename = "HEXF")]
    Hexf,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[default]
    Unknown,
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
pub struct ProtectedData1 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType3Code,
    #[serde(rename = "EnvlpdData", skip_serializing_if = "Option::is_none")]
    pub envlpd_data: Option<EnvelopedData6>,
    #[serde(rename = "NcrptdData", skip_serializing_if = "Option::is_none")]
    pub ncrptd_data: Option<EncryptedData1>,
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
pub struct AlgorithmIdentification25 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm23Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter7>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AdditionalServiceType2Code {
    #[serde(rename = "CACT")]
    Cact,
    #[serde(rename = "CSHB")]
    Cshb,
    #[serde(rename = "DCCV")]
    Dccv,
    #[serde(rename = "INTP")]
    Intp,
    #[serde(rename = "INTT")]
    Intt,
    #[serde(rename = "LOYT")]
    Loyt,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PRST")]
    Prst,
    #[serde(rename = "BALC")]
    Balc,
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
pub struct InterestRateDetails2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<InterestRate1Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<InstalmentPeriod1Code>,
    #[validate]
    #[serde(rename = "Rate")]
    pub rate: BaseOneRate,
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
pub struct VerificationResult2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Ntty", skip_serializing_if = "Option::is_none")]
    pub ntty: Option<VerificationEntity2Code>,
    #[serde(rename = "OthrNtty", skip_serializing_if = "Option::is_none")]
    pub othr_ntty: Option<Max35Text>,
    #[serde(rename = "Rslt", skip_serializing_if = "Option::is_none")]
    pub rslt: Option<Verification3Code>,
    #[serde(rename = "OthrRslt", skip_serializing_if = "Option::is_none")]
    pub othr_rslt: Option<Max500Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RsltDtls", default)]
    pub rslt_dtls: Vec<AdditionalData1>,
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
pub struct AlgorithmIdentification28 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm13Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter14>,
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
pub struct PartyIdentification263 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max11NumericText,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Max35Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Iso3NumericCountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "LglCorpNm", skip_serializing_if = "Option::is_none")]
    pub lgl_corp_nm: Option<Max99Text>,
    #[serde(rename = "AddtlId", skip_serializing_if = "Option::is_none")]
    pub addtl_id: Option<AdditionalData1>,
    #[serde(rename = "LclData", skip_serializing_if = "Option::is_none")]
    pub lcl_data: Option<LocalData1>,
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
pub struct Max104Text {
    #[validate(length(min = 1, max = 104,))]
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
pub struct PartyIdentification262 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<Max35Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Iso3NumericCountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "AddtlId", skip_serializing_if = "Option::is_none")]
    pub addtl_id: Option<Max35Text>,
    #[serde(rename = "FrgnMrchnt", skip_serializing_if = "Option::is_none")]
    pub frgn_mrchnt: Option<TrueFalseIndicator>,
    #[serde(rename = "LclData", skip_serializing_if = "Option::is_none")]
    pub lcl_data: Option<LocalData1>,
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
pub struct TransactionIdentification18 {
    #[serde(rename = "LclDt", skip_serializing_if = "Option::is_none")]
    pub lcl_dt: Option<IsoDate>,
    #[serde(rename = "LclTm", skip_serializing_if = "Option::is_none")]
    pub lcl_tm: Option<IsoTime>,
    #[serde(rename = "TmZone", skip_serializing_if = "Option::is_none")]
    pub tm_zone: Option<Max70Text>,
    #[serde(rename = "TxRef", skip_serializing_if = "Option::is_none")]
    pub tx_ref: Option<Max35Text>,
    #[serde(rename = "TrnsmssnDtTm", skip_serializing_if = "Option::is_none")]
    pub trnsmssn_dt_tm: Option<IsoDateTime>,
    #[validate]
    #[serde(rename = "SysTracAudtNb")]
    pub sys_trac_audt_nb: Max12NumericText,
    #[validate]
    #[serde(rename = "RtrvlRefNb")]
    pub rtrvl_ref_nb: Exact12Text,
    #[serde(rename = "LifeCyclSpprtInd", skip_serializing_if = "Option::is_none")]
    pub life_cycl_spprt_ind: Option<Exact2NumericText>,
    #[serde(rename = "LifeCyclTracIdData", skip_serializing_if = "Option::is_none")]
    pub life_cycl_trac_id_data: Option<TransactionLifeCycleIdentification1>,
    #[serde(
        rename = "LifeCyclTracIdMssng",
        skip_serializing_if = "Option::is_none"
    )]
    pub life_cycl_trac_id_mssng: Option<Max70Text>,
    #[serde(rename = "AcqrrRefData", skip_serializing_if = "Option::is_none")]
    pub acqrr_ref_data: Option<Max140Text>,
    #[serde(rename = "AcqrrRefNb", skip_serializing_if = "Option::is_none")]
    pub acqrr_ref_nb: Option<Max23NumericText>,
    #[serde(rename = "CardIssrRefData", skip_serializing_if = "Option::is_none")]
    pub card_issr_ref_data: Option<Max1000Text>,
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
pub struct Kek6 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate]
    #[serde(rename = "KEKId")]
    pub kek_id: KekIdentifier6,
    #[validate]
    #[serde(rename = "KeyNcrptnAlgo")]
    pub key_ncrptn_algo: AlgorithmIdentification28,
    #[serde(rename = "NcrptdKey", skip_serializing_if = "Option::is_none")]
    pub ncrptd_key: Option<Max500Binary>,
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
pub struct Plan2 {
    #[serde(rename = "PlanId", skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<Max70Text>,
    #[serde(rename = "PlanOwnr", skip_serializing_if = "Option::is_none")]
    pub plan_ownr: Option<PlanOwner1Code>,
    #[serde(rename = "OthrPlanOwnr", skip_serializing_if = "Option::is_none")]
    pub othr_plan_ownr: Option<Max35Text>,
    #[serde(rename = "InstlmtPmtTp", skip_serializing_if = "Option::is_none")]
    pub instlmt_pmt_tp: Option<Max35Text>,
    #[serde(rename = "DfrrdInstlmtInd", skip_serializing_if = "Option::is_none")]
    pub dfrrd_instlmt_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "PrdUnit", skip_serializing_if = "Option::is_none")]
    pub prd_unit: Option<Frequency18Code>,
    #[serde(rename = "NbOfPrds", skip_serializing_if = "Option::is_none")]
    pub nb_of_prds: Option<Number>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IntrstRate", default)]
    pub intrst_rate: Vec<InterestRateDetails2>,
    #[serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none")]
    pub frst_pmt_dt: Option<IsoDate>,
    #[serde(rename = "FrstAmt", skip_serializing_if = "Option::is_none")]
    pub frst_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "NrmlPmtAmt", skip_serializing_if = "Option::is_none")]
    pub nrml_pmt_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "TtlNbOfPmts", skip_serializing_if = "Option::is_none")]
    pub ttl_nb_of_pmts: Option<Number>,
    #[serde(rename = "InstlmtCcy", skip_serializing_if = "Option::is_none")]
    pub instlmt_ccy: Option<Iso3NumericCurrencyCode>,
    #[serde(rename = "GracePrd", skip_serializing_if = "Option::is_none")]
    pub grace_prd: Option<GracePeriod2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AmtDtls", default)]
    pub amt_dtls: Vec<InstalmentAmountDetails2>,
    #[serde(rename = "GrdTtlAmt", skip_serializing_if = "Option::is_none")]
    pub grd_ttl_amt: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ContentType2Code {
    #[serde(rename = "DATA")]
    Data,
    #[serde(rename = "SIGN")]
    Sign,
    #[serde(rename = "EVLP")]
    Evlp,
    #[serde(rename = "DGST")]
    Dgst,
    #[serde(rename = "AUTH")]
    Auth,
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
pub struct RiskAssessment2 {
    #[serde(rename = "RskAssmntNtty", skip_serializing_if = "Option::is_none")]
    pub rsk_assmnt_ntty: Option<PartyIdentification200>,
    #[serde(rename = "RskAssmntTp", skip_serializing_if = "Option::is_none")]
    pub rsk_assmnt_tp: Option<Max35Text>,
    #[serde(rename = "HghRskTx", skip_serializing_if = "Option::is_none")]
    pub hgh_rsk_tx: Option<TrueFalseIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Rsn", default)]
    pub rsn: Vec<Max35Text>,
    #[serde(rename = "Rslt", skip_serializing_if = "Option::is_none")]
    pub rslt: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RskCond", default)]
    pub rsk_cond: Vec<AdditionalData1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlRskData", default)]
    pub addtl_rsk_data: Vec<AdditionalRiskData1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RcmmnddActn", default)]
    pub rcmmndd_actn: Vec<ActionType8Code>,
    #[serde(rename = "OthrRcmmnddActn", skip_serializing_if = "Option::is_none")]
    pub othr_rcmmndd_actn: Option<Max35Text>,
    #[serde(rename = "RcmmnddActnDtls", skip_serializing_if = "Option::is_none")]
    pub rcmmndd_actn_dtls: Option<Max256Text>,
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
pub struct KekIdentifier2 {
    #[validate]
    #[serde(rename = "KeyId")]
    pub key_id: Max140Text,
    #[validate]
    #[serde(rename = "KeyVrsn")]
    pub key_vrsn: Max140Text,
    #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<Number>,
    #[serde(rename = "DerivtnId", skip_serializing_if = "Option::is_none")]
    pub derivtn_id: Option<Min5Max16Binary>,
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
pub struct TransactionContext8 {
    #[serde(rename = "MrchntCtgyCd", skip_serializing_if = "Option::is_none")]
    pub mrchnt_ctgy_cd: Option<Iso18245MerchantCategoryCode>,
    #[serde(
        rename = "MrchntCtgySpcfcData",
        skip_serializing_if = "Option::is_none"
    )]
    pub mrchnt_ctgy_spcfc_data: Option<Max35Text>,
    #[serde(rename = "FeeColltnInitr", skip_serializing_if = "Option::is_none")]
    pub fee_colltn_initr: Option<FeeCollectionInitiator1Code>,
    #[serde(rename = "TxInitr", skip_serializing_if = "Option::is_none")]
    pub tx_initr: Option<TransactionInitiator1Code>,
    #[serde(rename = "PrtlShipmntInd", skip_serializing_if = "Option::is_none")]
    pub prtl_shipmnt_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "DelydChrgsInd", skip_serializing_if = "Option::is_none")]
    pub delyd_chrgs_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "NoShowInd", skip_serializing_if = "Option::is_none")]
    pub no_show_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "ReauthstnInd", skip_serializing_if = "Option::is_none")]
    pub reauthstn_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "ReSubmissnInd", skip_serializing_if = "Option::is_none")]
    pub re_submissn_ind: Option<TrueFalseIndicator>,
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
pub struct AlgorithmIdentification26 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm8Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Algorithm5Code>,
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
pub struct IsoYearMonth {
    #[validate(regex = "ISO_YEAR_MONTH_REGEX")]
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
pub struct LocalData5 {
    #[serde(rename = "Lang")]
    pub lang: IsoMax3ALanguageCode,
    #[serde(rename = "CmonNm", skip_serializing_if = "Option::is_none")]
    pub cmon_nm: Option<Max280Text>,
    #[serde(rename = "LglCorpNm", skip_serializing_if = "Option::is_none")]
    pub lgl_corp_nm: Option<Max210Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Address3>,
    #[serde(rename = "AddtlAdrInf", skip_serializing_if = "Option::is_none")]
    pub addtl_adr_inf: Option<Max512Text>,
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
pub struct Iso2ALanguageCode {
    #[validate(regex = "ISO_2_A_LANGUAGE_CODE_REGEX")]
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
pub struct ProcessingResult17 {
    #[serde(rename = "RspnSrc", skip_serializing_if = "Option::is_none")]
    pub rspn_src: Option<ApprovalEntity2>,
    #[validate]
    #[serde(rename = "RsltData")]
    pub rslt_data: ResultData7,
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
pub struct CardholderName3 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
    pub gvn_nm: Option<Max35Text>,
    #[serde(rename = "MddlNm", skip_serializing_if = "Option::is_none")]
    pub mddl_nm: Option<Max35Text>,
    #[serde(rename = "LastNm", skip_serializing_if = "Option::is_none")]
    pub last_nm: Option<Max35Text>,
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
pub struct Amount7 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<Iso3NumericCurrencyCode>,
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
pub struct Max19NumericText {
    #[validate(regex = "MAX_19_NUMERIC_TEXT_REGEX")]
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
pub struct GeographicPointInDecimalDegrees {
    #[validate(length(max = 27,), regex = "GEOGRAPHIC_POINT_IN_DECIMAL_DEGREES_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
pub struct EncryptedData1ChoiceEnum {
    #[serde(rename = "HexBinryVal", skip_serializing_if = "Option::is_none")]
    pub hex_binry_val: Option<Max9999HexBinaryText>,
    #[serde(rename = "BinryData", skip_serializing_if = "Option::is_none")]
    pub binry_data: Option<Max100KBinary>,
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
pub struct EncryptedData1Choice {
    #[serde(flatten)]
    pub value: EncryptedData1ChoiceEnum,
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
pub struct TransactionLifeCycleIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Exact15Text,
    #[serde(rename = "AuthstnSeqNb", skip_serializing_if = "Option::is_none")]
    pub authstn_seq_nb: Option<Exact2NumericText>,
    #[serde(rename = "PresntmntSeqNb", skip_serializing_if = "Option::is_none")]
    pub presntmnt_seq_nb: Option<Exact2NumericText>,
    #[serde(rename = "PresntmntSeqCnt", skip_serializing_if = "Option::is_none")]
    pub presntmnt_seq_cnt: Option<Exact2NumericText>,
    #[serde(rename = "AuthntcnTkn", skip_serializing_if = "Option::is_none")]
    pub authntcn_tkn: Option<Max35Text>,
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
pub struct LocalData1 {
    #[serde(rename = "Lang")]
    pub lang: IsoMax3ALanguageCode,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max70Text>,
    #[serde(rename = "LglCorpNm", skip_serializing_if = "Option::is_none")]
    pub lgl_corp_nm: Option<Max210Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlData", default)]
    pub addtl_data: Vec<AdditionalData1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Identification3Code {
    #[serde(rename = "DRID")]
    Drid,
    #[serde(rename = "NTID")]
    Ntid,
    #[serde(rename = "PASS")]
    Pass,
    #[serde(rename = "SSYN")]
    Ssyn,
    #[serde(rename = "ARNB")]
    Arnb,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "EMAL")]
    Emal,
    #[serde(rename = "PHNB")]
    Phnb,
    #[serde(rename = "CUID")]
    Cuid,
    #[serde(rename = "TXID")]
    Txid,
    #[serde(rename = "PRXY")]
    Prxy,
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
pub struct Transaction132 {
    #[serde(rename = "TxTp")]
    pub tx_tp: Iso8583TransactionTypeCode,
    #[serde(rename = "TxSubTp", skip_serializing_if = "Option::is_none")]
    pub tx_sub_tp: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlSvc", default)]
    pub addtl_svc: Vec<AdditionalService2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SpclPrgrmmQlfctn", default)]
    pub spcl_prgrmm_qlfctn: Vec<SpecialProgrammeQualification1>,
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: TransactionIdentification18,
    #[serde(rename = "RcncltnAmt", skip_serializing_if = "Option::is_none")]
    pub rcncltn_amt: Option<Amount15>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlAmt", default)]
    pub addtl_amt: Vec<AdditionalAmounts3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlFee", default)]
    pub addtl_fee: Vec<AdditionalFee2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AcctBal", default)]
    pub acct_bal: Vec<AccountBalance2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AcctStmtData", default)]
    pub acct_stmt_data: Vec<AccountStatementData2>,
    #[serde(rename = "AcctFr", skip_serializing_if = "Option::is_none")]
    pub acct_fr: Option<AccountDetails3>,
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
pub struct Traceability9 {
    #[validate]
    #[serde(rename = "RlayId")]
    pub rlay_id: GenericIdentification183,
    #[serde(rename = "TracDtTmIn", skip_serializing_if = "Option::is_none")]
    pub trac_dt_tm_in: Option<IsoDateTime>,
    #[serde(rename = "TracDtTmOut", skip_serializing_if = "Option::is_none")]
    pub trac_dt_tm_out: Option<IsoDateTime>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InstalmentAmountDetailsType2Code {
    #[serde(rename = "AFCO")]
    Afco,
    #[serde(rename = "EXPN")]
    Expn,
    #[serde(rename = "FEES")]
    Fees,
    #[serde(rename = "FUNA")]
    Funa,
    #[serde(rename = "INSU")]
    Insu,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "OTHC")]
    Othc,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PRNC")]
    Prnc,
    #[serde(rename = "RQST")]
    Rqst,
    #[serde(rename = "TAXX")]
    Taxx,
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
pub struct Min2Max3NumericText {
    #[validate(regex = "MIN_2_MAX_3_NUMERIC_TEXT_REGEX")]
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AttributeType1Code {
    #[serde(rename = "CNAT")]
    Cnat,
    #[serde(rename = "LATT")]
    Latt,
    #[serde(rename = "OATT")]
    Oatt,
    #[serde(rename = "OUAT")]
    Ouat,
    #[serde(rename = "CATT")]
    Catt,
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
pub struct PercentageRate {
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
pub struct Max200Text {
    #[validate(length(min = 1, max = 200,))]
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
pub struct KekIdentifier6 {
    #[validate]
    #[serde(rename = "KeyId")]
    pub key_id: Max140Text,
    #[serde(rename = "KeyVrsn", skip_serializing_if = "Option::is_none")]
    pub key_vrsn: Option<Max140Text>,
    #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<Number>,
    #[serde(rename = "DerivtnId", skip_serializing_if = "Option::is_none")]
    pub derivtn_id: Option<Min5Max16Binary>,
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
pub struct EncryptedDataElement1 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ExternalEncryptedElementIdentification1Code>,
    #[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
    pub othr_id: Option<Max35Text>,
    #[serde(rename = "NcrptdData")]
    pub ncrptd_data: EncryptedData1Choice,
    #[serde(rename = "ClearTxtDataFrmt", skip_serializing_if = "Option::is_none")]
    pub clear_txt_data_frmt: Option<EncryptedDataFormat1Code>,
    #[serde(
        rename = "OthrClearTxtDataFrmt",
        skip_serializing_if = "Option::is_none"
    )]
    pub othr_clear_txt_data_frmt: Option<Max35Text>,
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
pub struct PartyIdentification200 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PartyType28Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
    #[serde(rename = "Assgnr", skip_serializing_if = "Option::is_none")]
    pub assgnr: Option<PartyType18Code>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Iso3NumericCountryCode>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InterestRate1Code {
    #[serde(rename = "GSRT")]
    Gsrt,
    #[serde(rename = "NTRT")]
    Ntrt,
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
pub struct Amount16 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
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
pub struct Max512Text {
    #[validate(length(min = 1, max = 512,))]
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
pub struct RiskAssessmentDataEntityProvider1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PartyType28Code>,
    #[serde(rename = "OthrTp", skip_serializing_if = "Option::is_none")]
    pub othr_tp: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm13Code {
    #[serde(rename = "EA2C")]
    Ea2C,
    #[serde(rename = "E3DC")]
    E3Dc,
    #[serde(rename = "DKP9")]
    Dkp9,
    #[serde(rename = "UKPT")]
    Ukpt,
    #[serde(rename = "UKA1")]
    Uka1,
    #[serde(rename = "EA9C")]
    Ea9C,
    #[serde(rename = "EA5C")]
    Ea5C,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AdditionalServiceResult1Code {
    #[serde(rename = "NOPF")]
    Nopf,
    #[serde(rename = "NOSP")]
    Nosp,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "PERF")]
    Perf,
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
pub struct AccountStatementDetails2 {
    #[validate]
    #[serde(rename = "TxDt")]
    pub tx_dt: IsoDate,
    #[serde(rename = "PdgInd", skip_serializing_if = "Option::is_none")]
    pub pdg_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "TxPstngDt", skip_serializing_if = "Option::is_none")]
    pub tx_pstng_dt: Option<IsoDate>,
    #[serde(rename = "TxAmt", skip_serializing_if = "Option::is_none")]
    pub tx_amt: Option<Amount7>,
    #[serde(rename = "CrdhldrBllgAmt", skip_serializing_if = "Option::is_none")]
    pub crdhldr_bllg_amt: Option<Amount7>,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
    #[serde(rename = "AccptrNmAndLctn", skip_serializing_if = "Option::is_none")]
    pub accptr_nm_and_lctn: Option<Max99Text>,
    #[serde(rename = "ShrtTxDesc", skip_serializing_if = "Option::is_none")]
    pub shrt_tx_desc: Option<Max70Text>,
    #[serde(rename = "LngTxDesc", skip_serializing_if = "Option::is_none")]
    pub lng_tx_desc: Option<Max256Text>,
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
pub struct Max99Text {
    #[validate(length(min = 1, max = 99,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType28Code {
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "OTHN")]
    Othn,
    #[serde(rename = "OTHP")]
    Othp,
    #[serde(rename = "WLPR")]
    Wlpr,
    #[serde(rename = "ISUR")]
    Isur,
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
pub struct CardData9 {
    #[serde(rename = "PAN", skip_serializing_if = "Option::is_none")]
    pub pan: Option<Max19NumericText>,
    #[serde(rename = "PrtctdPANInd", skip_serializing_if = "Option::is_none")]
    pub prtctd_pan_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "CardSeqNb", skip_serializing_if = "Option::is_none")]
    pub card_seq_nb: Option<Min2Max3NumericText>,
    #[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
    pub fctv_dt: Option<IsoYearMonth>,
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<IsoYearMonth>,
    #[serde(rename = "Trck3", skip_serializing_if = "Option::is_none")]
    pub trck_3: Option<Max104Text>,
    #[serde(rename = "PmtAcctRef", skip_serializing_if = "Option::is_none")]
    pub pmt_acct_ref: Option<Max35Text>,
    #[serde(rename = "PANAcctRg", skip_serializing_if = "Option::is_none")]
    pub pan_acct_rg: Option<Max19NumericText>,
    #[serde(rename = "PANFourLastDgts", skip_serializing_if = "Option::is_none")]
    pub pan_four_last_dgts: Option<Max4NumericText>,
    #[serde(rename = "CardCtryCd", skip_serializing_if = "Option::is_none")]
    pub card_ctry_cd: Option<Iso3NumericCountryCode>,
    #[serde(rename = "CardCcyCd", skip_serializing_if = "Option::is_none")]
    pub card_ccy_cd: Option<Iso3NumericCurrencyCode>,
    #[serde(rename = "CardPdctTp", skip_serializing_if = "Option::is_none")]
    pub card_pdct_tp: Option<Max35Text>,
    #[serde(rename = "CardPdctSubTp", skip_serializing_if = "Option::is_none")]
    pub card_pdct_sub_tp: Option<Max35Text>,
    #[serde(rename = "CardPrtflIdr", skip_serializing_if = "Option::is_none")]
    pub card_prtfl_idr: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlCardData", default)]
    pub addtl_card_data: Vec<AdditionalData1>,
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
pub struct Header60 {
    #[serde(rename = "MsgFctn")]
    pub msg_fctn: MessageFunction17Code,
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
pub struct Amount15 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
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
pub struct Contact1 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
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
    #[serde(rename = "Lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<Iso2ALanguageCode>,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CreditDebit3Code {
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "DBIT")]
    Dbit,
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
pub struct Recipient7ChoiceEnum {
    #[serde(rename = "KeyTrnsprt", skip_serializing_if = "Option::is_none")]
    pub key_trnsprt: Option<KeyTransport6>,
    #[serde(rename = "KEK", skip_serializing_if = "Option::is_none")]
    pub kek: Option<Kek6>,
    #[serde(rename = "KeyIdr", skip_serializing_if = "Option::is_none")]
    pub key_idr: Option<KekIdentifier6>,
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
pub struct Recipient7Choice {
    #[serde(flatten)]
    pub value: Recipient7ChoiceEnum,
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
pub struct Credentials2 {
    #[serde(rename = "IdCd")]
    pub id_cd: Identification3Code,
    #[serde(rename = "OthrIdCd", skip_serializing_if = "Option::is_none")]
    pub othr_id_cd: Option<Max35Text>,
    #[validate]
    #[serde(rename = "IdVal")]
    pub id_val: Max70Text,
    #[serde(rename = "IdXpryDt", skip_serializing_if = "Option::is_none")]
    pub id_xpry_dt: Option<IsoYearMonth>,
    #[serde(rename = "AssgnrAuthrty", skip_serializing_if = "Option::is_none")]
    pub assgnr_authrty: Option<Authority1>,
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
pub struct Max280Text {
    #[validate(length(min = 1, max = 280,))]
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
pub struct Max10PositiveNumber {
    #[validate(range(min = 1,))]
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MessageFunction17Code {
    #[serde(rename = "NOTI")]
    Noti,
    #[serde(rename = "REQU")]
    Requ,
    #[serde(rename = "ADVC")]
    Advc,
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
pub struct Amount17 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<Iso3NumericCurrencyCode>,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
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
pub struct Authority1 {
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Iso3NumericCountryCode>,
    #[serde(rename = "CtrySubDvsnMjr", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mjr: Option<IsoCountrySubDivisionCode>,
    #[serde(rename = "CtrySubDvsnMnr", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mnr: Option<IsoCountrySubDivisionCode>,
    #[serde(rename = "CtrySubDvsnMjrNm", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mjr_nm: Option<Max50Text>,
    #[serde(rename = "CtrySubDvsnMnrNm", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn_mnr_nm: Option<Max50Text>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max50Text>,
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
pub struct AddendumData5 {
    #[serde(rename = "Instlmt", skip_serializing_if = "Option::is_none")]
    pub instlmt: Option<Instalment4>,
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
pub struct GracePeriod2 {
    #[serde(rename = "Tm", skip_serializing_if = "Option::is_none")]
    pub tm: Option<Max3NumericText>,
    #[serde(rename = "UnitTp", skip_serializing_if = "Option::is_none")]
    pub unit_tp: Option<GracePeriodUnitType1Code>,
    #[serde(rename = "OthrUnitTp", skip_serializing_if = "Option::is_none")]
    pub othr_unit_tp: Option<Max35Text>,
    #[serde(
        rename = "CstmrSelctdGracePrd",
        skip_serializing_if = "Option::is_none"
    )]
    pub cstmr_selctd_grace_prd: Option<bool>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Moto1Code {
    #[serde(rename = "MAOR")]
    Maor,
    #[serde(rename = "MOTO")]
    Moto,
    #[serde(rename = "TPOR")]
    Tpor,
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
pub struct Context13 {
    #[serde(rename = "PtOfSvcCntxt", skip_serializing_if = "Option::is_none")]
    pub pt_of_svc_cntxt: Option<PointOfServiceContext4>,
    #[serde(rename = "TxCntxt", skip_serializing_if = "Option::is_none")]
    pub tx_cntxt: Option<TransactionContext8>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Vrfctn", default)]
    pub vrfctn: Vec<Verification4>,
    #[serde(rename = "RskCntxt", skip_serializing_if = "Option::is_none")]
    pub rsk_cntxt: Option<RiskContext2>,
}
