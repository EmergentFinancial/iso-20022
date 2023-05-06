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
// See ISO-20022 Intellectual Property Rights Policy at
// <https://www.iso20022.org/intellectual-property-rights>
// for more information.

use validator::Validate;

::lazy_static::lazy_static! {
    static ref ISO_YEAR_MONTH_REGEX: ::regex::Regex = ::regex::Regex::new(r#"^-?\d{4}-(0[1-9]|1[0-2])([+-]\d{2}:\d{2}|Z)?$"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BLOOMBERG_2_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"(BBG)[BCDFGHJKLMNPQRSTVWXYZ\d]{8}\d"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:setr.016.001.04".to_string()
}

#[derive(
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
pub struct QuickIdentifier {
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
pub struct PartyIdentification113 {
    #[serde(rename = "Pty")]
    pub pty: PartyIdentification90Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
}
#[derive(
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
pub struct ActiveOrHistoricCurrencyCode {
    #[validate(regex = "ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CancelledStatusReason2Code {
    #[serde(rename = "CANH")]
    Canh,
    #[serde(rename = "CANP")]
    Canp,
    #[serde(rename = "CXLR")]
    Cxlr,
    #[serde(rename = "CANO")]
    Cano,
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
pub struct InRepairStatusReason4ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "RsnDtls", skip_serializing_if = "Option::is_none")]
    pub rsn_dtls: Option<InRepairStatusReason4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InRepairStatusReason4Choice {
    #[serde(flatten)]
    pub value: InRepairStatusReason4ChoiceEnum,
}
#[derive(
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
pub struct SicovamIdentifier {
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
pub struct ActiveCurrencyAndAmount {
    #[serde(rename = "ActiveCurrencyAndAmount")]
    pub value: ActiveCurrencyAndAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveCurrencyCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancelledReason12ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CancelledStatusReason2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancelledReason12Choice {
    #[serde(flatten)]
    pub value: CancelledReason12ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DeliveryReceiptType2Code {
    #[serde(rename = "FREE")]
    Free,
    #[serde(rename = "APMT")]
    Apmt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NoReasonCode {
    #[serde(rename = "NORE")]
    Nore,
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
pub struct TickerIdentifier {
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
pub struct AlternateSecurityIdentification7 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "IdSrc")]
    pub id_src: IdentificationSource1Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct HoldBackInformation3 {
    #[serde(rename = "Tp")]
    pub tp: GateHoldBack1Code,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "XpctdRlsDt", skip_serializing_if = "Option::is_none")]
    pub xpctd_rls_dt: Option<IsoDate>,
    #[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_id: Option<SecurityIdentification25Choice>,
    #[serde(rename = "FinInstrmNm", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_nm: Option<Max350Text>,
    #[serde(rename = "RedCmpltn", skip_serializing_if = "Option::is_none")]
    pub red_cmpltn: Option<RedemptionCompletion1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MessageIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
}
#[derive(
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
pub struct Extension1 {
    #[validate]
    #[serde(rename = "PlcAndNm")]
    pub plc_and_nm: Max350Text,
    #[validate]
    #[serde(rename = "Txt")]
    pub txt: Max350Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveCurrencyAndAmountSimpleType {
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
pub struct WertpapierIdentifier {
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
pub struct PartiallySettled21ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettledStatusReason2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartiallySettled21Choice {
    #[serde(flatten)]
    pub value: PartiallySettled21ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OrderStatus4Code {
    #[serde(rename = "PACK")]
    Pack,
    #[serde(rename = "COSE")]
    Cose,
    #[serde(rename = "STNP")]
    Stnp,
    #[serde(rename = "RECE")]
    Rece,
    #[serde(rename = "SETT")]
    Sett,
    #[serde(rename = "CPNP")]
    Cpnp,
    #[serde(rename = "CNFC")]
    Cnfc,
    #[serde(rename = "DONE")]
    Done,
    #[serde(rename = "DONF")]
    Donf,
    #[serde(rename = "OPOD")]
    Opod,
    #[serde(rename = "IACO")]
    Iaco,
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
pub struct ActiveOrHistoricCurrencyAndAmount {
    #[serde(rename = "ActiveOrHistoricCurrencyAndAmount")]
    pub value: ActiveOrHistoricCurrencyAndAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveOrHistoricCurrencyCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Exact4AlphaNumericText {
    #[validate(regex = "EXACT_4_ALPHA_NUMERIC_TEXT_REGEX")]
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
pub struct AdditionalAmount1ChoiceEnum {
    #[serde(rename = "AddtlCshIn", skip_serializing_if = "Option::is_none")]
    pub addtl_csh_in: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "RsltgCshOut", skip_serializing_if = "Option::is_none")]
    pub rsltg_csh_out: Option<ActiveOrHistoricCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalAmount1Choice {
    #[serde(flatten)]
    pub value: AdditionalAmount1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SuspendedStatusReason3Code {
    #[serde(rename = "PRIC")]
    Pric,
    #[serde(rename = "FLOW")]
    Flow,
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
pub struct LeiIdentifier {
    #[validate(regex = "LEI_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestmentFundFee1Code {
    #[serde(rename = "BEND")]
    Bend,
    #[serde(rename = "BRKF")]
    Brkf,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "CDPL")]
    Cdpl,
    #[serde(rename = "CDSC")]
    Cdsc,
    #[serde(rename = "CBCH")]
    Cbch,
    #[serde(rename = "DLEV")]
    Dlev,
    #[serde(rename = "FEND")]
    Fend,
    #[serde(rename = "INIT")]
    Init,
    #[serde(rename = "ADDF")]
    Addf,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PREM")]
    Prem,
    #[serde(rename = "CHAR")]
    Char,
    #[serde(rename = "SHIP")]
    Ship,
    #[serde(rename = "SWIT")]
    Swit,
    #[serde(rename = "UCIC")]
    Ucic,
    #[serde(rename = "REGF")]
    Regf,
    #[serde(rename = "PENA")]
    Pena,
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
pub struct OrderStatus3ChoiceEnum {
    #[serde(rename = "Canc", skip_serializing_if = "Option::is_none")]
    pub canc: Option<CancelledStatusReason16>,
    #[serde(rename = "CondlyAccptd", skip_serializing_if = "Option::is_none")]
    pub condly_accptd: Option<ConditionallyAcceptedStatus3Choice>,
    #[serde(rename = "Rjctd", skip_serializing_if = "Option::is_none")]
    pub rjctd: Option<RejectedStatus9>,
    #[serde(rename = "Sspd", skip_serializing_if = "Option::is_none")]
    pub sspd: Option<SuspendedStatusReason4Choice>,
    #[serde(rename = "PrtlySttld", skip_serializing_if = "Option::is_none")]
    pub prtly_sttld: Option<PartiallySettledStatus10>,
    #[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
    pub sts: Option<OrderStatus4Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrderStatus3Choice {
    #[serde(flatten)]
    pub value: OrderStatus3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification25ChoiceEnum {
    #[serde(rename = "SEDOL", skip_serializing_if = "Option::is_none")]
    pub sedol: Option<SedolIdentifier>,
    #[serde(rename = "CTA", skip_serializing_if = "Option::is_none")]
    pub cta: Option<ConsolidatedTapeAssociationIdentifier>,
    #[serde(rename = "QUICK", skip_serializing_if = "Option::is_none")]
    pub quick: Option<QuickIdentifier>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "Belgn", skip_serializing_if = "Option::is_none")]
    pub belgn: Option<BelgianIdentifier>,
    #[serde(rename = "RIC", skip_serializing_if = "Option::is_none")]
    pub ric: Option<RicIdentifier>,
    #[serde(rename = "Vlrn", skip_serializing_if = "Option::is_none")]
    pub vlrn: Option<ValorenIdentifier>,
    #[serde(rename = "Dtch", skip_serializing_if = "Option::is_none")]
    pub dtch: Option<DutchIdentifier>,
    #[serde(rename = "Cmon", skip_serializing_if = "Option::is_none")]
    pub cmon: Option<EuroclearClearstreamIdentifier>,
    #[serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none")]
    pub tckr_symb: Option<TickerIdentifier>,
    #[serde(rename = "CUSIP", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<CusipIdentifier>,
    #[serde(rename = "Wrtppr", skip_serializing_if = "Option::is_none")]
    pub wrtppr: Option<WertpapierIdentifier>,
    #[serde(rename = "SCVM", skip_serializing_if = "Option::is_none")]
    pub scvm: Option<SicovamIdentifier>,
    #[serde(rename = "OthrPrtryId", skip_serializing_if = "Option::is_none")]
    pub othr_prtry_id: Option<AlternateSecurityIdentification7>,
    #[serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none")]
    pub blmbrg: Option<Bloomberg2Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification25Choice {
    #[serde(flatten)]
    pub value: SecurityIdentification25ChoiceEnum,
}
#[derive(
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
pub struct RicIdentifier {
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
pub struct IndividualOrderStatusAndReason7 {
    #[serde(rename = "MstrRef", skip_serializing_if = "Option::is_none")]
    pub mstr_ref: Option<Max35Text>,
    #[validate]
    #[serde(rename = "OrdrRef")]
    pub ordr_ref: Max35Text,
    #[serde(rename = "ClntRef", skip_serializing_if = "Option::is_none")]
    pub clnt_ref: Option<Max35Text>,
    #[serde(rename = "DealRef", skip_serializing_if = "Option::is_none")]
    pub deal_ref: Option<Max35Text>,
    #[serde(rename = "CxlRef", skip_serializing_if = "Option::is_none")]
    pub cxl_ref: Option<Max35Text>,
    #[serde(rename = "OrdrSts")]
    pub ordr_sts: OrderStatus5Choice,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "RprdFee", default)]
    pub rprd_fee: Vec<Fee3>,
    #[serde(rename = "StsInitr", skip_serializing_if = "Option::is_none")]
    pub sts_initr: Option<PartyIdentification113>,
    #[serde(rename = "OrdrData", skip_serializing_if = "Option::is_none")]
    pub ordr_data: Option<FundOrderData5>,
    #[serde(rename = "NewDtls", skip_serializing_if = "Option::is_none")]
    pub new_dtls: Option<ExpectedExecutionDetails4>,
    #[serde(rename = "GtgOrHldBckDtls", skip_serializing_if = "Option::is_none")]
    pub gtg_or_hld_bck_dtls: Option<HoldBackInformation3>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RejectedStatusReason11Code {
    #[serde(rename = "BLCA")]
    Blca,
    #[serde(rename = "BLTR")]
    Bltr,
    #[serde(rename = "DOCC")]
    Docc,
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "ILLI")]
    Illi,
    #[serde(rename = "BMIN")]
    Bmin,
    #[serde(rename = "BMRA")]
    Bmra,
    #[serde(rename = "BMRV")]
    Bmrv,
    #[serde(rename = "CUTO")]
    Cuto,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "IDDB")]
    Iddb,
    #[serde(rename = "ORRF")]
    Orrf,
    #[serde(rename = "FEEE")]
    Feee,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "IDNA")]
    Idna,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "CLOS")]
    Clos,
    #[serde(rename = "IPAC")]
    Ipac,
    #[serde(rename = "INSU")]
    Insu,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "ICTR")]
    Ictr,
    #[serde(rename = "IOTP")]
    Iotp,
    #[serde(rename = "DFOR")]
    Dfor,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "LOCK")]
    Lock,
    #[serde(rename = "NRGM")]
    Nrgm,
    #[serde(rename = "NSLA")]
    Nsla,
    #[serde(rename = "MONY")]
    Mony,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "IPAY")]
    Ipay,
    #[serde(rename = "PRCT")]
    Prct,
    #[serde(rename = "DLVY")]
    Dlvy,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "PLCE")]
    Plce,
    #[serde(rename = "IVAG")]
    Ivag,
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "ISAF")]
    Isaf,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "SETR")]
    Setr,
    #[serde(rename = "IEXE")]
    Iexe,
    #[serde(rename = "SHIG")]
    Shig,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "SLOW")]
    Slow,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "UWAI")]
    Uwai,
    #[serde(rename = "UDCY")]
    Udcy,
    #[serde(rename = "UNAV")]
    Unav,
    #[serde(rename = "UPAY")]
    Upay,
    #[serde(rename = "URSC")]
    Ursc,
    #[serde(rename = "ULNK")]
    Ulnk,
    #[serde(rename = "UNSC")]
    Unsc,
    #[serde(rename = "POIN")]
    Poin,
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
pub struct DutchIdentifier {
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
pub struct CusipIdentifier {
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
pub struct InRepairStatusReason5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InRepairStatusReason1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InRepairStatusReason5Choice {
    #[serde(flatten)]
    pub value: InRepairStatusReason5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalReference8 {
    #[validate]
    #[serde(rename = "Ref")]
    pub r#ref: Max35Text,
    #[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
    pub ref_issr: Option<PartyIdentification113>,
    #[serde(rename = "MsgNm", skip_serializing_if = "Option::is_none")]
    pub msg_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FundOrderData5 {
    #[serde(rename = "InvstmtAcctDtls", skip_serializing_if = "Option::is_none")]
    pub invstmt_acct_dtls: Option<InvestmentAccount58>,
    #[serde(rename = "FinInstrmDtls", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_dtls: Option<FinancialInstrument57>,
    #[serde(rename = "UnitsNb", skip_serializing_if = "Option::is_none")]
    pub units_nb: Option<DecimalNumber>,
    #[serde(rename = "NetAmt", skip_serializing_if = "Option::is_none")]
    pub net_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "GrssAmt", skip_serializing_if = "Option::is_none")]
    pub grss_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "HldgsRedRate", skip_serializing_if = "Option::is_none")]
    pub hldgs_red_rate: Option<PercentageRate>,
    #[serde(rename = "SttlmAmt", skip_serializing_if = "Option::is_none")]
    pub sttlm_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none")]
    pub unit_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "QtdCcy", skip_serializing_if = "Option::is_none")]
    pub qtd_ccy: Option<ActiveCurrencyCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BelgianIdentifier {
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
pub struct DecimalNumber {
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
pub struct ExpectedExecutionDetails2 {
    #[serde(rename = "XpctdTradDtTm", skip_serializing_if = "Option::is_none")]
    pub xpctd_trad_dt_tm: Option<DateAndDateTimeChoice>,
    #[serde(rename = "XpctdCshSttlmDt", skip_serializing_if = "Option::is_none")]
    pub xpctd_csh_sttlm_dt: Option<IsoDate>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DistributionPolicy1Code {
    #[serde(rename = "DIST")]
    Dist,
    #[serde(rename = "ACCU")]
    Accu,
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
pub struct PartyIdentification90ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicIdentifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification90Choice {
    #[serde(flatten)]
    pub value: PartyIdentification90ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct References61ChoiceEnum {
    #[serde(rename = "OthrRef", skip_serializing_if = "Option::is_none")]
    pub othr_ref: Option<AdditionalReference8>,
    #[serde(rename = "RltdRef", skip_serializing_if = "Option::is_none")]
    pub rltd_ref: Option<AdditionalReference8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct References61Choice {
    #[serde(flatten)]
    pub value: References61ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "OrdrInstrStsRpt")]
    pub ordr_instr_sts_rpt: OrderInstructionStatusReportV04,
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
pub struct ChargeType5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestmentFundFee1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ChargeType5Choice {
    #[serde(flatten)]
    pub value: ChargeType5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SedolIdentifier {
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ConditionallyAcceptedStatusReason2Code {
    #[serde(rename = "DOCC")]
    Docc,
    #[serde(rename = "AWRM")]
    Awrm,
    #[serde(rename = "AWSM")]
    Awsm,
    #[serde(rename = "DUPL")]
    Dupl,
    #[serde(rename = "CRED")]
    Cred,
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
pub struct Fee3 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ChargeType5Choice>,
    #[serde(rename = "RprdStdAmt", skip_serializing_if = "Option::is_none")]
    pub rprd_std_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "RprdStdRate", skip_serializing_if = "Option::is_none")]
    pub rprd_std_rate: Option<PercentageRate>,
    #[serde(rename = "RprdDscntAmt", skip_serializing_if = "Option::is_none")]
    pub rprd_dscnt_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "RprdDscntRate", skip_serializing_if = "Option::is_none")]
    pub rprd_dscnt_rate: Option<PercentageRate>,
    #[serde(rename = "RprdReqdAmt", skip_serializing_if = "Option::is_none")]
    pub rprd_reqd_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "RprdReqdRate", skip_serializing_if = "Option::is_none")]
    pub rprd_reqd_rate: Option<PercentageRate>,
    #[serde(rename = "ComrclAgrmtRef", skip_serializing_if = "Option::is_none")]
    pub comrcl_agrmt_ref: Option<Max35Text>,
    #[serde(
        rename = "NewComrclAgrmtRefInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub new_comrcl_agrmt_ref_ind: Option<YesNoIndicator>,
}
#[derive(
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
pub enum OrderOriginatorEligibility1Code {
    #[serde(rename = "ELIG")]
    Elig,
    #[serde(rename = "RETL")]
    Retl,
    #[serde(rename = "PROF")]
    Prof,
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
pub struct OrderStatusAndReason10 {
    #[serde(rename = "MstrRef", skip_serializing_if = "Option::is_none")]
    pub mstr_ref: Option<Max35Text>,
    #[serde(rename = "OrdrSts")]
    pub ordr_sts: OrderStatus3Choice,
    #[serde(rename = "StsInitr", skip_serializing_if = "Option::is_none")]
    pub sts_initr: Option<PartyIdentification113>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ConsolidatedTapeAssociationIdentifier {
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
pub struct GenericIdentification47 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Exact4AlphaNumericText,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max4AlphaNumericText,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max4AlphaNumericText>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InRepairStatusReason1Code {
    #[serde(rename = "COMA")]
    Coma,
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
pub struct OrderInstructionStatusReportV04 {
    #[validate]
    #[serde(rename = "MsgId")]
    pub msg_id: MessageIdentification1,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<References61Choice>,
    #[serde(rename = "StsRpt")]
    pub sts_rpt: Status24Choice,
    #[validate(length(min = 0,))]
    #[serde(rename = "Xtnsn", default)]
    pub xtnsn: Vec<Extension1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartiallySettledStatus10 {
    #[serde(rename = "Rsn")]
    pub rsn: PartiallySettled21Choice,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max350Text>,
}
#[derive(
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
pub struct ConditionallyAcceptedStatusReason3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ConditionallyAcceptedStatusReason2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ConditionallyAcceptedStatusReason3Choice {
    #[serde(flatten)]
    pub value: ConditionallyAcceptedStatusReason3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IsinOct2015Identifier {
    #[validate(regex = "ISIN_OCT_2015_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettledStatusReason2Code {
    #[serde(rename = "CPST")]
    Cpst,
    #[serde(rename = "GATM")]
    Gatm,
    #[serde(rename = "GAT1")]
    Gat1,
    #[serde(rename = "UCPS")]
    Ucps,
    #[serde(rename = "UPST")]
    Upst,
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
pub struct Status24ChoiceEnum {
    #[serde(rename = "OrdrDtlsRpt", skip_serializing_if = "Option::is_none")]
    pub ordr_dtls_rpt: Option<OrderStatusAndReason10>,
    #[serde(rename = "IndvOrdrDtlsRpt", skip_serializing_if = "Option::is_none")]
    pub indv_ordr_dtls_rpt: Option<IndividualOrderStatusAndReason7>,
    #[serde(rename = "SwtchOrdrDtlsRpt", skip_serializing_if = "Option::is_none")]
    pub swtch_ordr_dtls_rpt: Option<SwitchOrderStatusAndReason2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Status24Choice {
    #[serde(flatten)]
    pub value: Status24ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExpectedExecutionDetails4 {
    #[serde(rename = "XpctdTradDtTm", skip_serializing_if = "Option::is_none")]
    pub xpctd_trad_dt_tm: Option<DateAndDateTimeChoice>,
    #[serde(rename = "XpctdCshSttlmDt", skip_serializing_if = "Option::is_none")]
    pub xpctd_csh_sttlm_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndDateTimeChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<IsoDateTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndDateTimeChoice {
    #[serde(flatten)]
    pub value: DateAndDateTimeChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationSource1ChoiceEnum {
    #[serde(rename = "Dmst", skip_serializing_if = "Option::is_none")]
    pub dmst: Option<CountryCode>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationSource1Choice {
    #[serde(flatten)]
    pub value: IdentificationSource1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SwitchLegReferences2 {
    #[serde(rename = "LegId")]
    pub leg_id: LegIdentification1Choice,
    #[serde(rename = "LegRjctnRsn", skip_serializing_if = "Option::is_none")]
    pub leg_rjctn_rsn: Option<Max350Text>,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "RprdFee", default)]
    pub rprd_fee: Vec<Fee3>,
    #[serde(rename = "InvstmtAcctDtls", skip_serializing_if = "Option::is_none")]
    pub invstmt_acct_dtls: Option<InvestmentAccount58>,
    #[serde(rename = "FinInstrmDtls", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_dtls: Option<FinancialInstrument57>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestmentAccount58 {
    #[validate]
    #[serde(rename = "AcctId")]
    pub acct_id: Max35Text,
    #[serde(rename = "AcctNm", skip_serializing_if = "Option::is_none")]
    pub acct_nm: Option<Max35Text>,
    #[serde(rename = "AcctDsgnt", skip_serializing_if = "Option::is_none")]
    pub acct_dsgnt: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OwnrId", default)]
    pub ownr_id: Vec<PartyIdentification113>,
    #[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
    pub acct_svcr: Option<PartyIdentification113>,
    #[serde(rename = "OrdrOrgtrElgblty", skip_serializing_if = "Option::is_none")]
    pub ordr_orgtr_elgblty: Option<OrderOriginatorEligibility1Code>,
    #[serde(rename = "SubAcctDtls", skip_serializing_if = "Option::is_none")]
    pub sub_acct_dtls: Option<SubAccount6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat42ChoiceEnum {
    #[serde(rename = "YrMnth", skip_serializing_if = "Option::is_none")]
    pub yr_mnth: Option<IsoYearMonth>,
    #[serde(rename = "YrMnthDay", skip_serializing_if = "Option::is_none")]
    pub yr_mnth_day: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat42Choice {
    #[serde(flatten)]
    pub value: DateFormat42ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectedReason20ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RejectedStatusReason11Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectedReason20Choice {
    #[serde(flatten)]
    pub value: RejectedReason20ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EuroclearClearstreamIdentifier {
    #[validate(length(min = 1, max = 12,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FormOfSecurity1Code {
    #[serde(rename = "BEAR")]
    Bear,
    #[serde(rename = "REGD")]
    Regd,
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
pub struct SuspendedStatusReason4 {
    #[serde(rename = "Rsn")]
    pub rsn: SuspendedStatusReason5Choice,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Bloomberg2Identifier {
    #[validate(regex = "BLOOMBERG_2_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GateHoldBack1Code {
    #[serde(rename = "GATE")]
    Gate,
    #[serde(rename = "HOLD")]
    Hold,
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
pub struct Max4AlphaNumericText {
    #[validate(length(min = 1, max = 4,), regex = "MAX_4_ALPHA_NUMERIC_TEXT_REGEX")]
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
pub struct Series1 {
    #[serde(rename = "SrsDt", skip_serializing_if = "Option::is_none")]
    pub srs_dt: Option<DateFormat42Choice>,
    #[serde(rename = "SrsNm", skip_serializing_if = "Option::is_none")]
    pub srs_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrument57 {
    #[serde(rename = "Id")]
    pub id: SecurityIdentification25Choice,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none")]
    pub splmtry_id: Option<Max35Text>,
    #[serde(rename = "ClssTp", skip_serializing_if = "Option::is_none")]
    pub clss_tp: Option<Max35Text>,
    #[serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none")]
    pub scties_form: Option<FormOfSecurity1Code>,
    #[serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none")]
    pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
    #[serde(rename = "PdctGrp", skip_serializing_if = "Option::is_none")]
    pub pdct_grp: Option<Max140Text>,
    #[serde(rename = "SrsId", skip_serializing_if = "Option::is_none")]
    pub srs_id: Option<Series1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrderStatus4ChoiceEnum {
    #[serde(rename = "Sspd", skip_serializing_if = "Option::is_none")]
    pub sspd: Option<SuspendedStatusReason4Choice>,
    #[serde(rename = "Rjctd", skip_serializing_if = "Option::is_none")]
    pub rjctd: Option<RejectedStatus9>,
    #[serde(rename = "CondlyAccptd", skip_serializing_if = "Option::is_none")]
    pub condly_accptd: Option<ConditionallyAcceptedStatus3Choice>,
    #[serde(rename = "InRpr", skip_serializing_if = "Option::is_none")]
    pub in_rpr: Option<InRepairStatusReason4Choice>,
    #[serde(rename = "Canc", skip_serializing_if = "Option::is_none")]
    pub canc: Option<CancelledStatusReason16>,
    #[serde(rename = "PrtlySttld", skip_serializing_if = "Option::is_none")]
    pub prtly_sttld: Option<PartiallySettledStatus10>,
    #[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
    pub sts: Option<OrderStatus4Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrderStatus4Choice {
    #[serde(flatten)]
    pub value: OrderStatus4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SwitchOrderStatusAndReason2 {
    #[serde(rename = "MstrRef", skip_serializing_if = "Option::is_none")]
    pub mstr_ref: Option<Max35Text>,
    #[validate]
    #[serde(rename = "OrdrRef")]
    pub ordr_ref: Max35Text,
    #[serde(rename = "ClntRef", skip_serializing_if = "Option::is_none")]
    pub clnt_ref: Option<Max35Text>,
    #[serde(rename = "DealRef", skip_serializing_if = "Option::is_none")]
    pub deal_ref: Option<Max35Text>,
    #[serde(rename = "CxlRef", skip_serializing_if = "Option::is_none")]
    pub cxl_ref: Option<Max35Text>,
    #[serde(rename = "OrdrSts")]
    pub ordr_sts: OrderStatus4Choice,
    #[validate(length(min = 0,))]
    #[serde(rename = "LegInf", default)]
    pub leg_inf: Vec<SwitchLegReferences2>,
    #[serde(rename = "StsInitr", skip_serializing_if = "Option::is_none")]
    pub sts_initr: Option<PartyIdentification113>,
    #[serde(rename = "OrdrData", skip_serializing_if = "Option::is_none")]
    pub ordr_data: Option<FundOrderData6>,
    #[serde(rename = "NewDtls", skip_serializing_if = "Option::is_none")]
    pub new_dtls: Option<ExpectedExecutionDetails2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ValorenIdentifier {
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RedemptionCompletion1Code {
    #[serde(rename = "RED0")]
    Red0,
    #[serde(rename = "RED1")]
    Red1,
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
pub struct ConditionallyAcceptedStatusReason3 {
    #[serde(rename = "Rsn")]
    pub rsn: ConditionallyAcceptedStatusReason3Choice,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SubAccount6 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[serde(rename = "Chrtc", skip_serializing_if = "Option::is_none")]
    pub chrtc: Option<Max35Text>,
    #[serde(rename = "AcctDsgnt", skip_serializing_if = "Option::is_none")]
    pub acct_dsgnt: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancelledStatusReason16 {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<CancelledReason12Choice>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ConditionallyAcceptedStatus3ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "RsnDtls", skip_serializing_if = "Option::is_none")]
    pub rsn_dtls: Option<ConditionallyAcceptedStatusReason3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ConditionallyAcceptedStatus3Choice {
    #[serde(flatten)]
    pub value: ConditionallyAcceptedStatus3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FundOrderData6 {
    #[serde(rename = "SttlmAmt", skip_serializing_if = "Option::is_none")]
    pub sttlm_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "SttlmMtd", skip_serializing_if = "Option::is_none")]
    pub sttlm_mtd: Option<DeliveryReceiptType2Code>,
    #[serde(rename = "AddtlAmt", skip_serializing_if = "Option::is_none")]
    pub addtl_amt: Option<AdditionalAmount1Choice>,
    #[serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none")]
    pub unit_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "QtdCcy", skip_serializing_if = "Option::is_none")]
    pub qtd_ccy: Option<ActiveCurrencyCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectedStatus9 {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<RejectedReason20Choice>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SuspendedStatusReason4ChoiceEnum {
    #[serde(rename = "RsnDtls", skip_serializing_if = "Option::is_none")]
    pub rsn_dtls: Option<SuspendedStatusReason4>,
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SuspendedStatusReason4Choice {
    #[serde(flatten)]
    pub value: SuspendedStatusReason4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SuspendedStatusReason5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SuspendedStatusReason3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SuspendedStatusReason5Choice {
    #[serde(flatten)]
    pub value: SuspendedStatusReason5ChoiceEnum,
}
#[derive(
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
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
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
pub struct NameAndAddress5 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<PostalAddress1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LegIdentification1ChoiceEnum {
    #[serde(rename = "SbcptLegId", skip_serializing_if = "Option::is_none")]
    pub sbcpt_leg_id: Option<Max35Text>,
    #[serde(rename = "RedLegId", skip_serializing_if = "Option::is_none")]
    pub red_leg_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LegIdentification1Choice {
    #[serde(flatten)]
    pub value: LegIdentification1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrderStatus5ChoiceEnum {
    #[serde(rename = "PrtlySttld", skip_serializing_if = "Option::is_none")]
    pub prtly_sttld: Option<PartiallySettledStatus10>,
    #[serde(rename = "Canc", skip_serializing_if = "Option::is_none")]
    pub canc: Option<CancelledStatusReason16>,
    #[serde(rename = "Rjctd", skip_serializing_if = "Option::is_none")]
    pub rjctd: Option<RejectedStatus9>,
    #[serde(rename = "CondlyAccptd", skip_serializing_if = "Option::is_none")]
    pub condly_accptd: Option<ConditionallyAcceptedStatus3Choice>,
    #[serde(rename = "InRpr", skip_serializing_if = "Option::is_none")]
    pub in_rpr: Option<InRepairStatusReason4Choice>,
    #[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
    pub sts: Option<OrderStatus4Code>,
    #[serde(rename = "Sspd", skip_serializing_if = "Option::is_none")]
    pub sspd: Option<SuspendedStatusReason4Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrderStatus5Choice {
    #[serde(flatten)]
    pub value: OrderStatus5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InRepairStatusReason4 {
    #[serde(rename = "Rsn")]
    pub rsn: InRepairStatusReason5Choice,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max350Text>,
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
