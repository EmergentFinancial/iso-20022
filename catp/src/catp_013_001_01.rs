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
    static ref MIN_5_MAX_16_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5000_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BBAN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_2_MAX_3_ALPHA_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_2_MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{2,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_140_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_35_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_500_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIN_8_MAX_28_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{8,28}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_100_K_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_10000_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref UPIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{8,17}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:catp.013.001.01".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
#[derive(
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
pub struct DetailedAmount13 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "Labl", skip_serializing_if = "Option::is_none")]
    pub labl: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmTransactionAmounts8 {
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "MaxPssblAmt", skip_serializing_if = "Option::is_none")]
    pub max_pssbl_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "MinPssblAmt", skip_serializing_if = "Option::is_none")]
    pub min_pssbl_amt: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlAmt", default)]
    pub addtl_amt: Vec<AtmTransactionAmounts7>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DpstLmts", default)]
    pub dpst_lmts: Vec<AtmTransactionAmounts9>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm8Code {
    #[serde(rename = "MGF1")]
    Mgf1,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AtmMediaType2Code {
    #[serde(rename = "CARD")]
    Card,
    #[serde(rename = "COIN")]
    Coin,
    #[serde(rename = "CMDT")]
    Cmdt,
    #[serde(rename = "CPNS")]
    Cpns,
    #[serde(rename = "NOTE")]
    Note,
    #[serde(rename = "STMP")]
    Stmp,
    #[serde(rename = "UDTM")]
    Udtm,
    #[serde(rename = "CHCK")]
    Chck,
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
pub struct AtmEnvironment12 {
    #[serde(rename = "Acqrr", skip_serializing_if = "Option::is_none")]
    pub acqrr: Option<Acquirer7>,
    #[serde(rename = "ATMMgr", skip_serializing_if = "Option::is_none")]
    pub atm_mgr: Option<Acquirer8>,
    #[serde(rename = "HstgNtty", skip_serializing_if = "Option::is_none")]
    pub hstg_ntty: Option<TerminalHosting1>,
    #[validate]
    #[serde(rename = "ATM")]
    pub atm: AutomatedTellerMachine2,
    #[validate]
    #[serde(rename = "Cstmr")]
    pub cstmr: AtmCustomer5,
    #[serde(rename = "PrtctdCardData", skip_serializing_if = "Option::is_none")]
    pub prtctd_card_data: Option<ContentInformationType10>,
    #[serde(rename = "PlainCardData", skip_serializing_if = "Option::is_none")]
    pub plain_card_data: Option<PlainCardData19>,
}
#[derive(
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
    #[serde(rename = "KeyIdr", skip_serializing_if = "Option::is_none")]
    pub key_idr: Option<KekIdentifier2>,
    #[serde(rename = "IssrAndSrlNb", skip_serializing_if = "Option::is_none")]
    pub issr_and_srl_nb: Option<IssuerAndSerialNumber1>,
}
#[derive(
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
pub struct PostalAddress1 {
    #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<AddressType2Code>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "AdrLine", default)]
    pub adr_line: Vec<Max70Text>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max16Text>,
    #[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
    pub pst_cd: Option<Max16Text>,
    #[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
    pub twn_nm: Option<Max35Text>,
    #[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<Max35Text>,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmConfigurationParameter1 {
    #[serde(rename = "Tp")]
    pub tp: DataSetCategory7Code,
    #[validate]
    #[serde(rename = "Vrsn")]
    pub vrsn: Max35Text,
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
pub struct IsoDate {
    #[serde(rename = "$text")]
    pub value: ::chrono::NaiveDate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AtmDevice1Code {
    #[serde(rename = "CDIS")]
    Cdis,
    #[serde(rename = "DPRN")]
    Dprn,
    #[serde(rename = "JRNL")]
    Jrnl,
    #[serde(rename = "JPRN")]
    Jprn,
    #[serde(rename = "RPRN")]
    Rprn,
    #[serde(rename = "RWDR")]
    Rwdr,
    #[default]
    Unknown,
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
pub struct EncryptedContent3 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[validate]
    #[serde(rename = "CnttNcrptnAlgo")]
    pub cntt_ncrptn_algo: AlgorithmIdentification14,
    #[validate]
    #[serde(rename = "NcrptdData")]
    pub ncrptd_data: Max100KBinary,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PlainCardData19 {
    #[serde(rename = "PAN", skip_serializing_if = "Option::is_none")]
    pub pan: Option<Min8Max28NumericText>,
    #[serde(rename = "CardSeqNb", skip_serializing_if = "Option::is_none")]
    pub card_seq_nb: Option<Min2Max3NumericText>,
    #[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
    pub fctv_dt: Option<Max10Text>,
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<Max10Text>,
    #[serde(rename = "Trck1", skip_serializing_if = "Option::is_none")]
    pub trck_1: Option<Max76Text>,
    #[serde(rename = "Trck2", skip_serializing_if = "Option::is_none")]
    pub trck_2: Option<Max37Text>,
    #[serde(rename = "Trck3", skip_serializing_if = "Option::is_none")]
    pub trck_3: Option<Max104Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PlusOrMinusIndicator {
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
pub struct AtmCommandParameters1ChoiceEnum {
    #[serde(rename = "XpctdMsgFctn", skip_serializing_if = "Option::is_none")]
    pub xpctd_msg_fctn: Option<MessageFunction8Code>,
    #[serde(rename = "ReqrdCfgtnParam", skip_serializing_if = "Option::is_none")]
    pub reqrd_cfgtn_param: Option<AtmConfigurationParameter1>,
    #[serde(rename = "ATMReqrdGblSts", skip_serializing_if = "Option::is_none")]
    pub atm_reqrd_gbl_sts: Option<AtmStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmCommandParameters1Choice {
    #[serde(flatten)]
    pub value: AtmCommandParameters1ChoiceEnum,
}
#[derive(
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
pub struct AuthenticatedData4 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Rcpt", default)]
    pub rcpt: Vec<Recipient4Choice>,
    #[validate]
    #[serde(rename = "MACAlgo")]
    pub mac_algo: AlgorithmIdentification15,
    #[validate]
    #[serde(rename = "NcpsltdCntt")]
    pub ncpsltd_cntt: EncapsulatedContent3,
    #[validate]
    #[serde(rename = "MAC")]
    pub mac: Max140Binary,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlgorithmIdentification11 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm7Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter4>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Verification1Code {
    #[serde(rename = "FAIL")]
    Fail,
    #[serde(rename = "MISS")]
    Miss,
    #[serde(rename = "NOVF")]
    Novf,
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
pub enum PartyType12Code {
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "ATMG")]
    Atmg,
    #[serde(rename = "CISP")]
    Cisp,
    #[serde(rename = "DLIS")]
    Dlis,
    #[serde(rename = "HSTG")]
    Hstg,
    #[serde(rename = "ITAG")]
    Itag,
    #[serde(rename = "OATM")]
    Oatm,
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
pub struct CurrencyAndAmount {
    #[serde(rename = "CurrencyAndAmount")]
    pub value: CurrencyAndAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: CurrencyCode,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MessageFunction8Code {
    #[serde(rename = "BALN")]
    Baln,
    #[serde(rename = "GSTS")]
    Gsts,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "INQC")]
    Inqc,
    #[serde(rename = "KEYQ")]
    Keyq,
    #[serde(rename = "SSTS")]
    Ssts,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ResultDetail4Code {
    #[serde(rename = "ACTF")]
    Actf,
    #[serde(rename = "ACQS")]
    Acqs,
    #[serde(rename = "AMLV")]
    Amlv,
    #[serde(rename = "AMTA")]
    Amta,
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "BANK")]
    Bank,
    #[serde(rename = "CRDR")]
    Crdr,
    #[serde(rename = "CRDF")]
    Crdf,
    #[serde(rename = "ACTC")]
    Actc,
    #[serde(rename = "CTVG")]
    Ctvg,
    #[serde(rename = "DBER")]
    Dber,
    #[serde(rename = "FEES")]
    Fees,
    #[serde(rename = "TXNL")]
    Txnl,
    #[serde(rename = "AMTD")]
    Amtd,
    #[serde(rename = "NMBD")]
    Nmbd,
    #[serde(rename = "CRDX")]
    Crdx,
    #[serde(rename = "FDCL")]
    Fdcl,
    #[serde(rename = "FMTR")]
    Fmtr,
    #[serde(rename = "TXNG")]
    Txng,
    #[serde(rename = "FNDI")]
    Fndi,
    #[serde(rename = "ACPI")]
    Acpi,
    #[serde(rename = "AMTI")]
    Amti,
    #[serde(rename = "ADDI")]
    Addi,
    #[serde(rename = "BRHI")]
    Brhi,
    #[serde(rename = "CHDI")]
    Chdi,
    #[serde(rename = "CRDI")]
    Crdi,
    #[serde(rename = "CTFV")]
    Ctfv,
    #[serde(rename = "AMTO")]
    Amto,
    #[serde(rename = "PINV")]
    Pinv,
    #[serde(rename = "TKKO")]
    Tkko,
    #[serde(rename = "SGNI")]
    Sgni,
    #[serde(rename = "TKID")]
    Tkid,
    #[serde(rename = "TXNV")]
    Txnv,
    #[serde(rename = "DATI")]
    Dati,
    #[serde(rename = "ISSP")]
    Issp,
    #[serde(rename = "ISSF")]
    Issf,
    #[serde(rename = "ISSO")]
    Isso,
    #[serde(rename = "ISST")]
    Isst,
    #[serde(rename = "ISSU")]
    Issu,
    #[serde(rename = "KEYS")]
    Keys,
    #[serde(rename = "LBLA")]
    Lbla,
    #[serde(rename = "CRDL")]
    Crdl,
    #[serde(rename = "MACR")]
    Macr,
    #[serde(rename = "MACK")]
    Mack,
    #[serde(rename = "ICCM")]
    Iccm,
    #[serde(rename = "PINN")]
    Pinn,
    #[serde(rename = "CRDA")]
    Crda,
    #[serde(rename = "LBLU")]
    Lblu,
    #[serde(rename = "PINA")]
    Pina,
    #[serde(rename = "NPRA")]
    Npra,
    #[serde(rename = "OFFL")]
    Offl,
    #[serde(rename = "ONLP")]
    Onlp,
    #[serde(rename = "NPRC")]
    Nprc,
    #[serde(rename = "TXNM")]
    Txnm,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "BALO")]
    Balo,
    #[serde(rename = "SEQO")]
    Seqo,
    #[serde(rename = "PINC")]
    Pinc,
    #[serde(rename = "PIND")]
    Pind,
    #[serde(rename = "PINS")]
    Pins,
    #[serde(rename = "PINX")]
    Pinx,
    #[serde(rename = "PINE")]
    Pine,
    #[serde(rename = "QMAX")]
    Qmax,
    #[serde(rename = "RECD")]
    Recd,
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "SECV")]
    Secv,
    #[serde(rename = "SRVU")]
    Srvu,
    #[serde(rename = "SFWE")]
    Sfwe,
    #[serde(rename = "SPCC")]
    Spcc,
    #[serde(rename = "CRDS")]
    Crds,
    #[serde(rename = "SRCH")]
    Srch,
    #[serde(rename = "CNTC")]
    Cntc,
    #[serde(rename = "FRDS")]
    Frds,
    #[serde(rename = "SYSP")]
    Sysp,
    #[serde(rename = "SYSM")]
    Sysm,
    #[serde(rename = "TRMI")]
    Trmi,
    #[serde(rename = "ACTT")]
    Actt,
    #[serde(rename = "TTLV")]
    Ttlv,
    #[serde(rename = "TXNU")]
    Txnu,
    #[serde(rename = "TXND")]
    Txnd,
    #[serde(rename = "ORGF")]
    Orgf,
    #[serde(rename = "UNBO")]
    Unbo,
    #[serde(rename = "UNBP")]
    Unbp,
    #[serde(rename = "UNBC")]
    Unbc,
    #[serde(rename = "CMKY")]
    Cmky,
    #[serde(rename = "CRDU")]
    Crdu,
    #[serde(rename = "SVSU")]
    Svsu,
    #[serde(rename = "VNDR")]
    Vndr,
    #[serde(rename = "VNDF")]
    Vndf,
    #[serde(rename = "AMTW")]
    Amtw,
    #[serde(rename = "NMBW")]
    Nmbw,
    #[serde(rename = "CRDW")]
    Crdw,
    #[serde(rename = "MEDI")]
    Medi,
    #[serde(rename = "SRVI")]
    Srvi,
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
pub struct Max10Text {
    #[validate(length(min = 1, max = 10,))]
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
pub struct AlgorithmIdentification15 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm12Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter7>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType16Code {
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "DLIS")]
    Dlis,
    #[serde(rename = "ITAG")]
    Itag,
    #[serde(rename = "OTRM")]
    Otrm,
    #[serde(rename = "BKAF")]
    Bkaf,
    #[serde(rename = "BKAT")]
    Bkat,
    #[serde(rename = "ATMG")]
    Atmg,
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
pub struct AmountAndDirection43 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: CurrencyAndAmount,
    #[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
    pub sgn: Option<PlusOrMinusIndicator>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AtmCommand4Code {
    #[serde(rename = "ABAL")]
    Abal,
    #[serde(rename = "ASTS")]
    Asts,
    #[serde(rename = "CFGT")]
    Cfgt,
    #[serde(rename = "CCNT")]
    Ccnt,
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "SNDM")]
    Sndm,
    #[serde(rename = "RPTC")]
    Rptc,
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
pub struct AtmCustomerProfile2 {
    #[serde(rename = "PrflRef", skip_serializing_if = "Option::is_none")]
    pub prfl_ref: Option<Max35Text>,
    #[serde(rename = "CstmrId", skip_serializing_if = "Option::is_none")]
    pub cstmr_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Kek4 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate]
    #[serde(rename = "KEKId")]
    pub kek_id: KekIdentifier2,
    #[validate]
    #[serde(rename = "KeyNcrptnAlgo")]
    pub key_ncrptn_algo: AlgorithmIdentification13,
    #[validate]
    #[serde(rename = "NcrptdKey")]
    pub ncrptd_key: Max500Binary,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmDepositResponseV01 {
    #[validate]
    #[serde(rename = "Hdr")]
    pub hdr: Header31,
    #[serde(rename = "PrtctdATMDpstRspn", skip_serializing_if = "Option::is_none")]
    pub prtctd_atm_dpst_rspn: Option<ContentInformationType10>,
    #[serde(rename = "ATMDpstRspn", skip_serializing_if = "Option::is_none")]
    pub atm_dpst_rspn: Option<AtmDepositResponse1>,
    #[serde(rename = "SctyTrlr", skip_serializing_if = "Option::is_none")]
    pub scty_trlr: Option<ContentInformationType15>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CountryCode {
    #[validate(regex = "COUNTRY_CODE_REGEX")]
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
pub struct Acquirer8 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "ApplVrsn", skip_serializing_if = "Option::is_none")]
    pub appl_vrsn: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CurrencyCode {
    #[validate(regex = "CURRENCY_CODE_REGEX")]
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
pub struct AtmTransactionAmounts7 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "Labl", skip_serializing_if = "Option::is_none")]
    pub labl: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max37Text {
    #[validate(length(min = 1, max = 37,))]
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
pub struct GeographicCoordinates1 {
    #[validate]
    #[serde(rename = "Lat")]
    pub lat: Max16Text,
    #[validate]
    #[serde(rename = "Long")]
    pub long: Max16Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Response4Code {
    #[serde(rename = "APPR")]
    Appr,
    #[serde(rename = "DECL")]
    Decl,
    #[serde(rename = "PART")]
    Part,
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
pub struct Acquirer7 {
    #[serde(rename = "AcqrgInstn", skip_serializing_if = "Option::is_none")]
    pub acqrg_instn: Option<Max35Text>,
    #[serde(rename = "Brnch", skip_serializing_if = "Option::is_none")]
    pub brnch: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ActionType6Code {
    #[serde(rename = "DCCQ")]
    Dccq,
    #[serde(rename = "FEES")]
    Fees,
    #[serde(rename = "HAMT")]
    Hamt,
    #[serde(rename = "LAMT")]
    Lamt,
    #[serde(rename = "BUSY")]
    Busy,
    #[serde(rename = "CPTR")]
    Cptr,
    #[serde(rename = "DISP")]
    Disp,
    #[serde(rename = "CPNS")]
    Cpns,
    #[serde(rename = "RQST")]
    Rqst,
    #[serde(rename = "PINL")]
    Pinl,
    #[serde(rename = "PINR")]
    Pinr,
    #[serde(rename = "TRCK")]
    Trck,
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
pub struct AlgorithmIdentification14 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm15Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CurrencyAndAmountSimpleType {
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
pub struct Action7 {
    #[serde(rename = "ActnTp")]
    pub actn_tp: ActionType6Code,
    #[serde(rename = "MsgToPres", skip_serializing_if = "Option::is_none")]
    pub msg_to_pres: Option<ActionMessage4>,
    #[serde(rename = "ReqToPrfrm", skip_serializing_if = "Option::is_none")]
    pub req_to_prfrm: Option<MessageFunction11Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EncryptionFormat1Code {
    #[serde(rename = "TR31")]
    Tr31,
    #[serde(rename = "TR34")]
    Tr34,
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
pub struct AtmDepositResponse1 {
    #[validate]
    #[serde(rename = "Envt")]
    pub envt: AtmEnvironment12,
    #[validate]
    #[serde(rename = "Cntxt")]
    pub cntxt: AtmContext11,
    #[validate]
    #[serde(rename = "Tx")]
    pub tx: AtmTransaction16,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeographicLocation1ChoiceEnum {
    #[serde(rename = "GeogcCordints", skip_serializing_if = "Option::is_none")]
    pub geogc_cordints: Option<GeographicCoordinates1>,
    #[serde(rename = "UTMCordints", skip_serializing_if = "Option::is_none")]
    pub utm_cordints: Option<UtmCoordinates1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeographicLocation1Choice {
    #[serde(flatten)]
    pub value: GeographicLocation1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MessageFunction11Code {
    #[serde(rename = "BALN")]
    Baln,
    #[serde(rename = "CMPA")]
    Cmpa,
    #[serde(rename = "CMPD")]
    Cmpd,
    #[serde(rename = "ACMD")]
    Acmd,
    #[serde(rename = "DVCC")]
    Dvcc,
    #[serde(rename = "DIAQ")]
    Diaq,
    #[serde(rename = "DIAP")]
    Diap,
    #[serde(rename = "GSTS")]
    Gsts,
    #[serde(rename = "INQQ")]
    Inqq,
    #[serde(rename = "INQP")]
    Inqp,
    #[serde(rename = "KYAQ")]
    Kyaq,
    #[serde(rename = "KYAP")]
    Kyap,
    #[serde(rename = "PINQ")]
    Pinq,
    #[serde(rename = "PINP")]
    Pinp,
    #[serde(rename = "RJAQ")]
    Rjaq,
    #[serde(rename = "RJAP")]
    Rjap,
    #[serde(rename = "WITV")]
    Witv,
    #[serde(rename = "WITK")]
    Witk,
    #[serde(rename = "WITQ")]
    Witq,
    #[serde(rename = "WITP")]
    Witp,
    #[serde(rename = "INQC")]
    Inqc,
    #[serde(rename = "H2AP")]
    H2Ap,
    #[serde(rename = "H2AQ")]
    H2Aq,
    #[serde(rename = "TMOP")]
    Tmop,
    #[serde(rename = "CSEC")]
    Csec,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "SKSC")]
    Sksc,
    #[serde(rename = "SSTS")]
    Ssts,
    #[serde(rename = "DPSK")]
    Dpsk,
    #[serde(rename = "DPSV")]
    Dpsv,
    #[serde(rename = "DPSQ")]
    Dpsq,
    #[serde(rename = "DPSP")]
    Dpsp,
    #[serde(rename = "EXPK")]
    Expk,
    #[serde(rename = "EXPV")]
    Expv,
    #[serde(rename = "TRFQ")]
    Trfq,
    #[serde(rename = "TRFP")]
    Trfp,
    #[serde(rename = "RPTC")]
    Rptc,
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
pub struct AmountAndCurrency1 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveCurrencyCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveCurrencyCode {
    #[validate(regex = "ACTIVE_CURRENCY_CODE_REGEX")]
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
pub struct AtmTransactionAmounts9 {
    #[serde(rename = "MdiaTp")]
    pub mdia_tp: AtmMediaType2Code,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "MinNb", skip_serializing_if = "Option::is_none")]
    pub min_nb: Option<Number>,
    #[serde(rename = "MaxNb", skip_serializing_if = "Option::is_none")]
    pub max_nb: Option<Number>,
    #[serde(rename = "DispFlg", skip_serializing_if = "Option::is_none")]
    pub disp_flg: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TerminalHosting1 {
    #[serde(rename = "Ctgy", skip_serializing_if = "Option::is_none")]
    pub ctgy: Option<TransactionEnvironment3Code>,
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
pub struct ResponseType7 {
    #[serde(rename = "Rspn")]
    pub rspn: Response4Code,
    #[serde(rename = "RspnRsn", skip_serializing_if = "Option::is_none")]
    pub rspn_rsn: Option<ResultDetail4Code>,
    #[serde(rename = "AddtlRspnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rspn_inf: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification72ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification72Choice {
    #[serde(flatten)]
    pub value: PartyIdentification72ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm15Code {
    #[serde(rename = "EA2C")]
    Ea2C,
    #[serde(rename = "E3DC")]
    E3Dc,
    #[serde(rename = "EA9C")]
    Ea9C,
    #[serde(rename = "EA5C")]
    Ea5C,
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
#[serde(rename = "Document")]
pub struct Document {
    #[validate]
    #[serde(rename = "ATMDpstRspn")]
    pub atm_dpst_rspn: AtmDepositResponseV01,
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
pub struct ContentInformationType15 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[validate]
    #[serde(rename = "AuthntcdData")]
    pub authntcd_data: AuthenticatedData4,
}
#[derive(
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
pub struct Max76Text {
    #[validate(length(min = 1, max = 76,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AtmServiceType6Code {
    #[serde(rename = "MCHG")]
    Mchg,
    #[serde(rename = "DPSN")]
    Dpsn,
    #[serde(rename = "DPSV")]
    Dpsv,
    #[default]
    Unknown,
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
pub struct AtmCommand7 {
    #[serde(rename = "Tp")]
    pub tp: AtmCommand4Code,
    #[serde(rename = "Urgcy")]
    pub urgcy: TmsContactLevel2Code,
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<IsoDateTime>,
    #[serde(rename = "CmdId", skip_serializing_if = "Option::is_none")]
    pub cmd_id: Option<AtmCommandIdentification1>,
    #[serde(rename = "CmdParams", skip_serializing_if = "Option::is_none")]
    pub cmd_params: Option<AtmCommandParameters1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EncapsulatedContent3 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[serde(rename = "Cntt", skip_serializing_if = "Option::is_none")]
    pub cntt: Option<Max100KBinary>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionVerificationResult5 {
    #[serde(rename = "Mtd")]
    pub mtd: AuthenticationMethod7Code,
    #[serde(rename = "VrfctnNtty", skip_serializing_if = "Option::is_none")]
    pub vrfctn_ntty: Option<AuthenticationEntity2Code>,
    #[serde(rename = "Rslt", skip_serializing_if = "Option::is_none")]
    pub rslt: Option<Verification1Code>,
    #[serde(rename = "AddtlRslt", skip_serializing_if = "Option::is_none")]
    pub addtl_rslt: Option<Max500Text>,
    #[serde(rename = "AuthntcnTkn", skip_serializing_if = "Option::is_none")]
    pub authntcn_tkn: Option<Max140Binary>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PostalAddress17 {
    #[validate(length(min = 0, max = 2,))]
    #[serde(rename = "AdrLine", default)]
    pub adr_line: Vec<Max70Text>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max16Text>,
    #[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
    pub pst_cd: Option<Max16Text>,
    #[validate]
    #[serde(rename = "TwnNm")]
    pub twn_nm: Max35Text,
    #[validate(length(min = 0, max = 2,))]
    #[serde(rename = "CtrySubDvsn", default)]
    pub ctry_sub_dvsn: Vec<Max35Text>,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
    #[serde(rename = "GLctn", skip_serializing_if = "Option::is_none")]
    pub g_lctn: Option<GeographicLocation1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BbanIdentifier {
    #[validate(regex = "BBAN_IDENTIFIER_REGEX")]
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
pub struct AnyBicIdentifier {
    #[validate(regex = "ANY_BIC_IDENTIFIER_REGEX")]
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
pub struct IbanIdentifier {
    #[validate(regex = "IBAN_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmCommandIdentification1 {
    #[serde(rename = "Orgn", skip_serializing_if = "Option::is_none")]
    pub orgn: Option<Max35Text>,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<Max35Text>,
    #[serde(rename = "Prcr", skip_serializing_if = "Option::is_none")]
    pub prcr: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmCustomer5 {
    #[serde(rename = "Prfl", skip_serializing_if = "Option::is_none")]
    pub prfl: Option<AtmCustomerProfile2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AuthntcnRslt", default)]
    pub authntcn_rslt: Vec<TransactionVerificationResult5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmService12 {
    #[serde(rename = "SvcRef", skip_serializing_if = "Option::is_none")]
    pub svc_ref: Option<Max35Text>,
    #[serde(rename = "ATMSvcCd", skip_serializing_if = "Option::is_none")]
    pub atm_svc_cd: Option<Max35Text>,
    #[serde(rename = "HstSvcCd", skip_serializing_if = "Option::is_none")]
    pub hst_svc_cd: Option<Max35Text>,
    #[serde(rename = "SvcTp")]
    pub svc_tp: AtmServiceType6Code,
    #[validate(length(min = 0,))]
    #[serde(rename = "SvcVarntId", default)]
    pub svc_varnt_id: Vec<Max35Text>,
    #[serde(rename = "CshBck", skip_serializing_if = "Option::is_none")]
    pub csh_bck: Option<TrueFalseIndicator>,
    #[serde(rename = "MultiAcct", skip_serializing_if = "Option::is_none")]
    pub multi_acct: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContentInformationType10 {
    #[serde(rename = "CnttTp")]
    pub cntt_tp: ContentType2Code,
    #[validate]
    #[serde(rename = "EnvlpdData")]
    pub envlpd_data: EnvelopedData4,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NameAndAddress3 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max70Text,
    #[validate]
    #[serde(rename = "Adr")]
    pub adr: PostalAddress1,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm12Code {
    #[serde(rename = "MACC")]
    Macc,
    #[serde(rename = "MCCS")]
    Mccs,
    #[serde(rename = "CMA1")]
    Cma1,
    #[serde(rename = "MCC1")]
    Mcc1,
    #[serde(rename = "CMA9")]
    Cma9,
    #[serde(rename = "CMA5")]
    Cma5,
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
pub struct AutomatedTellerMachine2 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "AddtlId", skip_serializing_if = "Option::is_none")]
    pub addtl_id: Option<Max35Text>,
    #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<Max35Text>,
    #[serde(rename = "BaseCcy", skip_serializing_if = "Option::is_none")]
    pub base_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "Lctn", skip_serializing_if = "Option::is_none")]
    pub lctn: Option<PostalAddress17>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Parameter6 {
    #[serde(rename = "NcrptnFrmt", skip_serializing_if = "Option::is_none")]
    pub ncrptn_frmt: Option<EncryptionFormat1Code>,
    #[serde(rename = "InitlstnVctr", skip_serializing_if = "Option::is_none")]
    pub initlstn_vctr: Option<Max500Binary>,
    #[serde(rename = "BPddg", skip_serializing_if = "Option::is_none")]
    pub b_pddg: Option<BytePadding1Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AccountChoiceMethod1Code {
    #[serde(rename = "ACSL")]
    Acsl,
    #[serde(rename = "ENTR")]
    Entr,
    #[serde(rename = "IMAC")]
    Imac,
    #[serde(rename = "IMPL")]
    Impl,
    #[serde(rename = "NOSL")]
    Nosl,
    #[serde(rename = "TPSL")]
    Tpsl,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TmsContactLevel2Code {
    #[serde(rename = "ASAP")]
    Asap,
    #[serde(rename = "CRIT")]
    Crit,
    #[serde(rename = "DTIM")]
    Dtim,
    #[serde(rename = "ENCS")]
    Encs,
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
pub struct CardAccount10 {
    #[serde(rename = "AcctSeqNb", skip_serializing_if = "Option::is_none")]
    pub acct_seq_nb: Option<Number>,
    #[serde(rename = "SelctnMtd", skip_serializing_if = "Option::is_none")]
    pub selctn_mtd: Option<AccountChoiceMethod1Code>,
    #[serde(rename = "SelctdAcctTp", skip_serializing_if = "Option::is_none")]
    pub selctd_acct_tp: Option<CardAccountType3Code>,
    #[serde(rename = "AcctNm", skip_serializing_if = "Option::is_none")]
    pub acct_nm: Option<Max70Text>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<NameAndAddress3>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "AcctIdr", skip_serializing_if = "Option::is_none")]
    pub acct_idr: Option<AccountIdentification31Choice>,
    #[serde(rename = "CdtRef", skip_serializing_if = "Option::is_none")]
    pub cdt_ref: Option<Max35Text>,
    #[serde(rename = "Svcr", skip_serializing_if = "Option::is_none")]
    pub svcr: Option<PartyIdentification72Choice>,
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<AmountAndDirection43>,
    #[serde(rename = "BalDispFlg", skip_serializing_if = "Option::is_none")]
    pub bal_disp_flg: Option<TrueFalseIndicator>,
    #[serde(rename = "DfltAcctInd", skip_serializing_if = "Option::is_none")]
    pub dflt_acct_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "Lmts", skip_serializing_if = "Option::is_none")]
    pub lmts: Option<AtmTransactionAmounts8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification77 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp")]
    pub tp: PartyType12Code,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyType12Code>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<Min2Max3AlphaText>,
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
pub struct Header31 {
    #[validate]
    #[serde(rename = "MsgFctn")]
    pub msg_fctn: AtmMessageFunction2,
    #[validate]
    #[serde(rename = "PrtcolVrsn")]
    pub prtcol_vrsn: Max6Text,
    #[validate]
    #[serde(rename = "XchgId")]
    pub xchg_id: Max3NumericText,
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "InitgPty")]
    pub initg_pty: Max35Text,
    #[serde(rename = "RcptPty", skip_serializing_if = "Option::is_none")]
    pub rcpt_pty: Option<Max35Text>,
    #[serde(rename = "PrcStat", skip_serializing_if = "Option::is_none")]
    pub prc_stat: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tracblt", default)]
    pub tracblt: Vec<Traceability4>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Parameter5 {
    #[serde(rename = "DgstAlgo", skip_serializing_if = "Option::is_none")]
    pub dgst_algo: Option<Algorithm11Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Min6Max8Text {
    #[validate(length(min = 6, max = 8,))]
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
pub struct ResponseType8 {
    #[validate]
    #[serde(rename = "RspndrId")]
    pub rspndr_id: Max35Text,
    #[serde(rename = "Cdfctn", skip_serializing_if = "Option::is_none")]
    pub cdfctn: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Rspn")]
    pub rspn: Max35Text,
    #[serde(rename = "RspnRsn", skip_serializing_if = "Option::is_none")]
    pub rspn_rsn: Option<Max35Text>,
    #[serde(rename = "AddtlRspnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rspn_inf: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Parameter4 {
    #[serde(rename = "NcrptnFrmt", skip_serializing_if = "Option::is_none")]
    pub ncrptn_frmt: Option<EncryptionFormat1Code>,
    #[serde(rename = "DgstAlgo", skip_serializing_if = "Option::is_none")]
    pub dgst_algo: Option<Algorithm11Code>,
    #[serde(rename = "MskGnrtrAlgo", skip_serializing_if = "Option::is_none")]
    pub msk_gnrtr_algo: Option<AlgorithmIdentification12>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AuthenticationMethod7Code {
    #[serde(rename = "TOKA")]
    Toka,
    #[serde(rename = "BIOM")]
    Biom,
    #[serde(rename = "MOBL")]
    Mobl,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "FPIN")]
    Fpin,
    #[serde(rename = "NPIN")]
    Npin,
    #[serde(rename = "PSWD")]
    Pswd,
    #[serde(rename = "SCRT")]
    Scrt,
    #[serde(rename = "SCNL")]
    Scnl,
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
pub struct Min2Max3AlphaText {
    #[validate(regex = "MIN_2_MAX_3_ALPHA_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AddressType2Code {
    #[serde(rename = "ADDR")]
    Addr,
    #[serde(rename = "PBOX")]
    Pbox,
    #[serde(rename = "HOME")]
    Home,
    #[serde(rename = "BIZZ")]
    Bizz,
    #[serde(rename = "MLTO")]
    Mlto,
    #[serde(rename = "DLVY")]
    Dlvy,
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
pub struct UtmCoordinates1 {
    #[validate]
    #[serde(rename = "UTMZone")]
    pub utm_zone: Max16Text,
    #[validate]
    #[serde(rename = "UTMEstwrd")]
    pub utm_estwrd: Number,
    #[validate]
    #[serde(rename = "UTMNrthwrd")]
    pub utm_nrthwrd: Number,
}
#[derive(
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
pub struct SimpleIdentificationInformation4 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlgorithmIdentification12 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm8Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmContext11 {
    #[serde(rename = "SsnRef", skip_serializing_if = "Option::is_none")]
    pub ssn_ref: Option<Max35Text>,
    #[serde(rename = "Svc", skip_serializing_if = "Option::is_none")]
    pub svc: Option<AtmService12>,
}
#[derive(
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
pub struct Max500Binary {
    #[validate(length(min = 1, max = 500,), regex = "MAX_500_BINARY_REGEX")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransactionEnvironment3Code {
    #[serde(rename = "BRCH")]
    Brch,
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "OTHR")]
    Othr,
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
pub struct Recipient4ChoiceEnum {
    #[serde(rename = "KeyTrnsprt", skip_serializing_if = "Option::is_none")]
    pub key_trnsprt: Option<KeyTransport4>,
    #[serde(rename = "KEK", skip_serializing_if = "Option::is_none")]
    pub kek: Option<Kek4>,
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
pub struct Recipient4Choice {
    #[serde(flatten)]
    pub value: Recipient4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CardAccountType3Code {
    #[serde(rename = "CTDP")]
    Ctdp,
    #[serde(rename = "CHCK")]
    Chck,
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "CURR")]
    Curr,
    #[serde(rename = "CDBT")]
    Cdbt,
    #[serde(rename = "DFLT")]
    Dflt,
    #[serde(rename = "EPRS")]
    Eprs,
    #[serde(rename = "HEQL")]
    Heql,
    #[serde(rename = "ISTL")]
    Istl,
    #[serde(rename = "INVS")]
    Invs,
    #[serde(rename = "LCDT")]
    Lcdt,
    #[serde(rename = "MBNW")]
    Mbnw,
    #[serde(rename = "MNMK")]
    Mnmk,
    #[serde(rename = "MNMC")]
    Mnmc,
    #[serde(rename = "MTGL")]
    Mtgl,
    #[serde(rename = "RTRM")]
    Rtrm,
    #[serde(rename = "RVLV")]
    Rvlv,
    #[serde(rename = "SVNG")]
    Svng,
    #[serde(rename = "STBD")]
    Stbd,
    #[serde(rename = "UVRL")]
    Uvrl,
    #[serde(rename = "PRPD")]
    Prpd,
    #[serde(rename = "FLTC")]
    Fltc,
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
pub struct Max20000Text {
    #[validate(length(min = 1, max = 20000,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OutputFormat2Code {
    #[serde(rename = "MREF")]
    Mref,
    #[serde(rename = "SREF")]
    Sref,
    #[serde(rename = "TEXT")]
    Text,
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
pub struct ActionMessage4 {
    #[serde(rename = "Frmt", skip_serializing_if = "Option::is_none")]
    pub frmt: Option<OutputFormat2Code>,
    #[serde(rename = "Msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<Max20000Text>,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<Max35Text>,
    #[serde(rename = "Dvc", skip_serializing_if = "Option::is_none")]
    pub dvc: Option<AtmDevice1Code>,
    #[serde(rename = "MsgCnttSgntr", skip_serializing_if = "Option::is_none")]
    pub msg_cntt_sgntr: Option<Max35Binary>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlgorithmIdentification13 {
    #[serde(rename = "Algo")]
    pub algo: Algorithm13Code,
    #[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
    pub param: Option<Parameter6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmTransaction16 {
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: TransactionIdentifier1,
    #[serde(rename = "RcncltnId", skip_serializing_if = "Option::is_none")]
    pub rcncltn_id: Option<Max35Text>,
    #[serde(rename = "CmpltnReqrd", skip_serializing_if = "Option::is_none")]
    pub cmpltn_reqrd: Option<TrueFalseIndicator>,
    #[serde(rename = "AcctData", skip_serializing_if = "Option::is_none")]
    pub acct_data: Option<CardAccount10>,
    #[serde(rename = "PrtctdAcctData", skip_serializing_if = "Option::is_none")]
    pub prtctd_acct_data: Option<ContentInformationType10>,
    #[validate]
    #[serde(rename = "TtlAuthrsdAmt")]
    pub ttl_authrsd_amt: AmountAndCurrency1,
    #[serde(rename = "TtlReqdAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_reqd_amt: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DtldReqdAmt", default)]
    pub dtld_reqd_amt: Vec<DetailedAmount16>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlChrg", default)]
    pub addtl_chrg: Vec<DetailedAmount13>,
    #[serde(rename = "AuthstnRslt", skip_serializing_if = "Option::is_none")]
    pub authstn_rslt: Option<AuthorisationResult13>,
    #[serde(rename = "ICCRltdData", skip_serializing_if = "Option::is_none")]
    pub icc_rltd_data: Option<Max10000Binary>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Cmd", default)]
    pub cmd: Vec<AtmCommand7>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DetailedAmount16 {
    #[serde(rename = "AcctSeqNb", skip_serializing_if = "Option::is_none")]
    pub acct_seq_nb: Option<Number>,
    #[serde(rename = "AmtToDpst", skip_serializing_if = "Option::is_none")]
    pub amt_to_dpst: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "CshBckAmt", skip_serializing_if = "Option::is_none")]
    pub csh_bck_amt: Option<ImpliedCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Fees", default)]
    pub fees: Vec<DetailedAmount13>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Dontn", default)]
    pub dontn: Vec<DetailedAmount13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Min8Max28NumericText {
    #[validate(regex = "MIN_8_MAX_28_NUMERIC_TEXT_REGEX")]
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
pub struct EnvelopedData4 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Rcpt", default)]
    pub rcpt: Vec<Recipient4Choice>,
    #[serde(rename = "NcrptdCntt", skip_serializing_if = "Option::is_none")]
    pub ncrptd_cntt: Option<EncryptedContent3>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AtmStatus1Code {
    #[serde(rename = "INSV")]
    Insv,
    #[serde(rename = "OUTS")]
    Outs,
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
pub struct TransactionIdentifier1 {
    #[validate]
    #[serde(rename = "TxDtTm")]
    pub tx_dt_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "TxRef")]
    pub tx_ref: Max35Text,
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
pub enum AuthenticationEntity2Code {
    #[serde(rename = "ICCD")]
    Iccd,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "ISSR")]
    Issr,
    #[serde(rename = "TRML")]
    Trml,
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
pub struct Traceability4 {
    #[validate]
    #[serde(rename = "RlayId")]
    pub rlay_id: GenericIdentification77,
    #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<Max35Text>,
    #[validate]
    #[serde(rename = "TracDtTmIn")]
    pub trac_dt_tm_in: IsoDateTime,
    #[validate]
    #[serde(rename = "TracDtTmOut")]
    pub trac_dt_tm_out: IsoDateTime,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AtmMessageFunction2 {
    #[serde(rename = "Fctn")]
    pub fctn: MessageFunction11Code,
    #[serde(rename = "ATMSvcCd", skip_serializing_if = "Option::is_none")]
    pub atm_svc_cd: Option<Max35Text>,
    #[serde(rename = "HstSvcCd", skip_serializing_if = "Option::is_none")]
    pub hst_svc_cd: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Algorithm11Code {
    #[serde(rename = "HS25")]
    Hs25,
    #[serde(rename = "HS38")]
    Hs38,
    #[serde(rename = "HS51")]
    Hs51,
    #[serde(rename = "HS01")]
    Hs01,
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
pub struct AuthorisationResult13 {
    #[serde(rename = "AuthstnNtty", skip_serializing_if = "Option::is_none")]
    pub authstn_ntty: Option<PartyType16Code>,
    #[validate]
    #[serde(rename = "AuthstnRspn")]
    pub authstn_rspn: ResponseType7,
    #[validate(length(min = 0,))]
    #[serde(rename = "RspnTrac", default)]
    pub rspn_trac: Vec<ResponseType8>,
    #[serde(rename = "AuthstnCd", skip_serializing_if = "Option::is_none")]
    pub authstn_cd: Option<Min6Max8Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Actn", default)]
    pub actn: Vec<Action7>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DataSetCategory7Code {
    #[serde(rename = "ATMC")]
    Atmc,
    #[serde(rename = "ATMP")]
    Atmp,
    #[serde(rename = "APPR")]
    Appr,
    #[serde(rename = "CRAP")]
    Crap,
    #[serde(rename = "CPRC")]
    Cprc,
    #[serde(rename = "OEXR")]
    Oexr,
    #[serde(rename = "AMNT")]
    Amnt,
    #[serde(rename = "LOCC")]
    Locc,
    #[serde(rename = "MNOC")]
    Mnoc,
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
pub struct Max10000Binary {
    #[validate(length(min = 1, max = 10000,), regex = "MAX_10000_BINARY_REGEX")]
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
pub struct KeyTransport4 {
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Number>,
    #[serde(rename = "RcptId")]
    pub rcpt_id: Recipient5Choice,
    #[validate]
    #[serde(rename = "KeyNcrptnAlgo")]
    pub key_ncrptn_algo: AlgorithmIdentification11,
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
pub struct UpicIdentifier {
    #[validate(regex = "UPIC_IDENTIFIER_REGEX")]
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
pub struct IssuerAndSerialNumber1 {
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: CertificateIssuer1,
    #[validate]
    #[serde(rename = "SrlNb")]
    pub srl_nb: Max35Binary,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification31ChoiceEnum {
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<IbanIdentifier>,
    #[serde(rename = "UPIC", skip_serializing_if = "Option::is_none")]
    pub upic: Option<UpicIdentifier>,
    #[serde(rename = "BBAN", skip_serializing_if = "Option::is_none")]
    pub bban: Option<BbanIdentifier>,
    #[serde(rename = "DmstAcct", skip_serializing_if = "Option::is_none")]
    pub dmst_acct: Option<SimpleIdentificationInformation4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification31Choice {
    #[serde(flatten)]
    pub value: AccountIdentification31ChoiceEnum,
}
