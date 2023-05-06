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
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref NACE_DOMAIN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-U]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_RESTRICTED_YEAR_REGEX: ::regex::Regex = ::regex::Regex::new(r#"^-?\d{4}([+-]\d{2}:\d{2}|Z)?$"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.090.001.01".to_string()
}

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralisationType1Code {
    #[serde(rename = "FLCL")]
    Flcl,
    #[serde(rename = "OWCL")]
    Owcl,
    #[serde(rename = "PRCL")]
    Prcl,
    #[serde(rename = "UNCL")]
    Uncl,
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
pub struct SupplementaryDataEnvelope1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(flatten)]
    pub value: A,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DebtInstrumentSeniorityType2Code {
    #[serde(rename = "SBOD")]
    Sbod,
    #[serde(rename = "SNDB")]
    Sndb,
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
pub struct PositionSetValueAndNotional1 {
    #[serde(rename = "Ntnl", skip_serializing_if = "Option::is_none")]
    pub ntnl: Option<ActiveOrHistoricCurrencyAnd20Amount>,
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<ActiveOrHistoricCurrencyAnd20Amount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentification7ChoiceEnum {
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<OrganisationIdentification30>,
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
pub struct OrganisationIdentification7Choice {
    #[serde(flatten)]
    pub value: OrganisationIdentification7ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DerivativesTradePositionSetReportV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "AggtdPos")]
    pub aggtd_pos: PositionSetAggregated1Choice,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<A>>,
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
pub struct ActiveOrHistoricCurrencyCode {
    #[validate(regex = "ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX")]
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
pub struct PositionSetBuyerAndSeller1 {
    #[serde(rename = "Buyr", skip_serializing_if = "Option::is_none")]
    pub buyr: Option<PositionSetTotal1>,
    #[serde(rename = "Sellr", skip_serializing_if = "Option::is_none")]
    pub sellr: Option<PositionSetTotal1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AnyBicDec2014Identifier {
    #[validate(regex = "ANY_BIC_DEC_2014_IDENTIFIER_REGEX")]
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
pub struct PositionSetPostedAndReceived1 {
    #[serde(rename = "Pstd", skip_serializing_if = "Option::is_none")]
    pub pstd: Option<ActiveOrHistoricCurrencyAnd20Amount>,
    #[serde(rename = "Rcvd", skip_serializing_if = "Option::is_none")]
    pub rcvd: Option<ActiveOrHistoricCurrencyAnd20Amount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyAnd20Amount {
    #[serde(rename = "ActiveOrHistoricCurrencyAnd20Amount")]
    pub value: ActiveOrHistoricCurrencyAnd20AmountSimpleType,
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
pub struct SecurityIdentification34ChoiceEnum {
    #[serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none")]
    pub altrntv_instrm_id: Option<Max52Text>,
    #[serde(rename = "BsktCnsttnts", skip_serializing_if = "Option::is_none")]
    pub bskt_cnsttnts: Option<SecurityIdentification18Choice>,
    #[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
    pub indx: Option<SecurityIdentification35Choice>,
    #[serde(rename = "IdNotAvlbl", skip_serializing_if = "Option::is_none")]
    pub id_not_avlbl: Option<UnderlyingIdentification1Code>,
    #[serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none")]
    pub unq_pdct_idr: Option<Max52Text>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification34Choice {
    #[serde(flatten)]
    pub value: SecurityIdentification34ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionParty1Code {
    #[serde(rename = "SLLR")]
    Sllr,
    #[serde(rename = "BYER")]
    Byer,
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
pub struct TimeToMaturityPeriod1 {
    #[serde(rename = "Start", skip_serializing_if = "Option::is_none")]
    pub start: Option<MaturityTerm2>,
    #[serde(rename = "End", skip_serializing_if = "Option::is_none")]
    pub end: Option<MaturityTerm2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeCounterpartyReport9 {
    #[validate]
    #[serde(rename = "RptgCtrPty")]
    pub rptg_ctr_pty: Counterparty26,
    #[validate]
    #[serde(rename = "OthrCtrPty")]
    pub othr_ctr_pty: Counterparty29,
    #[serde(rename = "Brkr", skip_serializing_if = "Option::is_none")]
    pub brkr: Option<OrganisationIdentification9Choice>,
    #[serde(rename = "SubmitgAgt", skip_serializing_if = "Option::is_none")]
    pub submitg_agt: Option<OrganisationIdentification9Choice>,
    #[serde(rename = "ClrMmb", skip_serializing_if = "Option::is_none")]
    pub clr_mmb: Option<OrganisationIdentification9Choice>,
    #[serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none")]
    pub bnfcry: Option<OrganisationIdentification9Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PositionSetCollateralTotal1 {
    #[serde(rename = "NbOfRpts", skip_serializing_if = "Option::is_none")]
    pub nb_of_rpts: Option<Max20PositiveNumber>,
    #[serde(rename = "InitlMrgn", skip_serializing_if = "Option::is_none")]
    pub initl_mrgn: Option<PositionSetPostedAndReceived1>,
    #[serde(rename = "VartnMrgn", skip_serializing_if = "Option::is_none")]
    pub vartn_mrgn: Option<PositionSetPostedAndReceived1>,
    #[serde(rename = "XcssCsh", skip_serializing_if = "Option::is_none")]
    pub xcss_csh: Option<PositionSetPostedAndReceived1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max52Text {
    #[validate(length(min = 1, max = 52,))]
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
pub struct MasterAgreement2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max50Text>,
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<IsoRestrictedYear>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max20PositiveNumber {
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
pub struct ExchangeRateBasis1ChoiceEnum {
    #[serde(rename = "CcyPair", skip_serializing_if = "Option::is_none")]
    pub ccy_pair: Option<ExchangeRateBasis1>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max52Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExchangeRateBasis1Choice {
    #[serde(flatten)]
    pub value: ExchangeRateBasis1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PositionSet4 {
    #[validate]
    #[serde(rename = "Dmnsns")]
    pub dmnsns: PositionSetCollateralDimensions2,
    #[validate]
    #[serde(rename = "Mtrcs")]
    pub mtrcs: PositionSetCollateralMetrics1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Counterparty26 {
    #[serde(rename = "Id")]
    pub id: OrganisationIdentification7Choice,
    #[serde(rename = "Ntr", skip_serializing_if = "Option::is_none")]
    pub ntr: Option<CounterpartyTradeNature5Choice>,
    #[serde(rename = "TradgCpcty", skip_serializing_if = "Option::is_none")]
    pub tradg_cpcty: Option<TradingCapacity7Code>,
    #[serde(rename = "CtrPtySd", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_sd: Option<OptionParty1Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionType2Code {
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "PUTO")]
    Puto,
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
pub struct Max105Text {
    #[validate(length(min = 1, max = 105,))]
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
pub struct PositionSetCollateralDimensions2 {
    #[serde(rename = "CtrPtyId", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_id: Option<TradeCounterpartyReport9>,
    #[serde(rename = "Collstn", skip_serializing_if = "Option::is_none")]
    pub collstn: Option<CollateralisationType1Code>,
    #[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
    pub prtfl: Option<Max52Text>,
    #[serde(rename = "InitlMrgnPstdCcy", skip_serializing_if = "Option::is_none")]
    pub initl_mrgn_pstd_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "VartnMrgnPstdCcy", skip_serializing_if = "Option::is_none")]
    pub vartn_mrgn_pstd_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "InitlMrgnRcvdCcy", skip_serializing_if = "Option::is_none")]
    pub initl_mrgn_rcvd_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "VartnMrgnRcvdCcy", skip_serializing_if = "Option::is_none")]
    pub vartn_mrgn_rcvd_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "XcssCollPstdCcy", skip_serializing_if = "Option::is_none")]
    pub xcss_coll_pstd_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "XcssCollRcvdCcy", skip_serializing_if = "Option::is_none")]
    pub xcss_coll_rcvd_ccy: Option<ActiveOrHistoricCurrencyCode>,
}
#[derive(
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
pub struct SecurityIdentification18ChoiceEnum {
    #[serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none")]
    pub altrntv_instrm_id: Option<Max52Text>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification18Choice {
    #[serde(flatten)]
    pub value: SecurityIdentification18ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification35ChoiceEnum {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
    pub indx: Option<BenchmarkCurveName3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification35Choice {
    #[serde(flatten)]
    pub value: SecurityIdentification35ChoiceEnum,
}
#[derive(
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
pub struct OrganisationIdentification30 {
    #[serde(rename = "Id")]
    pub id: OrganisationIdentification8Choice,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max105Text>,
    #[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
    pub dmcl: Option<Max500Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradingCapacity7Code {
    #[serde(rename = "AGEN")]
    Agen,
    #[serde(rename = "PRIN")]
    Prin,
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
pub struct NaceDomainIdentifier {
    #[validate(regex = "NACE_DOMAIN_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProductType4Code {
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "CURR")]
    Curr,
    #[serde(rename = "EQUI")]
    Equi,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "COMM")]
    Comm,
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
pub struct ActiveOrHistoricCurrencyAnd20AmountSimpleType {
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
pub struct LeiIdentifier {
    #[validate(regex = "LEI_IDENTIFIER_REGEX")]
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
pub struct Max3Number {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnderlyingIdentification1Code {
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "BSKT")]
    Bskt,
    #[serde(rename = "INDX")]
    Indx,
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
pub struct PositionSetAggregated1ChoiceEnum {
    #[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
    pub data_set_actn: Option<ReportPeriodActivity1Code>,
    #[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
    pub rpt: Option<PositionSetAggregated3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PositionSetAggregated1Choice {
    #[serde(flatten)]
    pub value: PositionSetAggregated1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FinancialInstrumentContractType2Code {
    #[serde(rename = "CFDS")]
    Cfds,
    #[serde(rename = "FRAS")]
    Fras,
    #[serde(rename = "FUTR")]
    Futr,
    #[serde(rename = "FORW")]
    Forw,
    #[serde(rename = "OPTN")]
    Optn,
    #[serde(rename = "SPDB")]
    Spdb,
    #[serde(rename = "SWAP")]
    Swap,
    #[serde(rename = "SWPT")]
    Swpt,
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
pub struct NonFinancialInstitutionSector2 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Sctr", default)]
    pub sctr: Vec<NaceDomainIdentifier>,
    #[serde(rename = "ClrThrshld", skip_serializing_if = "Option::is_none")]
    pub clr_thrshld: Option<TrueFalseIndicator>,
    #[serde(rename = "DrctlyLkdActvty", skip_serializing_if = "Option::is_none")]
    pub drctly_lkd_actvty: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PositionSetDimensions3 {
    #[serde(rename = "CtrPtyId", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_id: Option<TradeCounterpartyReport9>,
    #[serde(rename = "ValCcy", skip_serializing_if = "Option::is_none")]
    pub val_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "Collstn", skip_serializing_if = "Option::is_none")]
    pub collstn: Option<CollateralisationType1Code>,
    #[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
    pub prtfl: Option<Max52Text>,
    #[serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none")]
    pub ctrct_tp: Option<FinancialInstrumentContractType2Code>,
    #[serde(rename = "AsstClss", skip_serializing_if = "Option::is_none")]
    pub asst_clss: Option<ProductType4Code>,
    #[serde(rename = "UndrlygInstrm", skip_serializing_if = "Option::is_none")]
    pub undrlyg_instrm: Option<SecurityIdentification34Choice>,
    #[serde(rename = "FrstLegNtnlCcy", skip_serializing_if = "Option::is_none")]
    pub frst_leg_ntnl_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "ScndLegNtnlCcy", skip_serializing_if = "Option::is_none")]
    pub scnd_leg_ntnl_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "DlvrblCcy", skip_serializing_if = "Option::is_none")]
    pub dlvrbl_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "DlvrblCrossCcy", skip_serializing_if = "Option::is_none")]
    pub dlvrbl_cross_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
    pub mstr_agrmt: Option<MasterAgreement2>,
    #[serde(rename = "ClrSts", skip_serializing_if = "Option::is_none")]
    pub clr_sts: Option<TrueFalseIndicator>,
    #[serde(rename = "IntraGrp", skip_serializing_if = "Option::is_none")]
    pub intra_grp: Option<TrueFalseIndicator>,
    #[serde(rename = "XchgRateBsis", skip_serializing_if = "Option::is_none")]
    pub xchg_rate_bsis: Option<ExchangeRateBasis1Choice>,
    #[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
    pub optn_tp: Option<OptionType2Code>,
    #[serde(rename = "TmToMtrty", skip_serializing_if = "Option::is_none")]
    pub tm_to_mtrty: Option<TimeToMaturity1Choice>,
    #[serde(rename = "IRSTp", skip_serializing_if = "Option::is_none")]
    pub irs_tp: Option<Max52Text>,
    #[serde(rename = "Snrty", skip_serializing_if = "Option::is_none")]
    pub snrty: Option<DebtInstrumentSeniorityType2Code>,
    #[serde(rename = "Trch", skip_serializing_if = "Option::is_none")]
    pub trch: Option<TrueFalseIndicator>,
    #[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
    pub cmmdty: Option<Max52Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExchangeRateBasis1 {
    #[serde(rename = "BaseCcy")]
    pub base_ccy: ActiveCurrencyCode,
    #[serde(rename = "QtdCcy")]
    pub qtd_ccy: ActiveCurrencyCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CounterpartyTradeNature5ChoiceEnum {
    #[serde(rename = "FI", skip_serializing_if = "Option::is_none")]
    pub fi: Option<FinancialPartySectorType1Code>,
    #[serde(rename = "NFI", skip_serializing_if = "Option::is_none")]
    pub nfi: Option<NonFinancialInstitutionSector2>,
    #[serde(rename = "CntrlCntrPty", skip_serializing_if = "Option::is_none")]
    pub cntrl_cntr_pty: Option<NoReasonCode>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CounterpartyTradeNature5Choice {
    #[serde(flatten)]
    pub value: CounterpartyTradeNature5ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BenchmarkCurveName3Code {
    #[serde(rename = "ESTR")]
    Estr,
    #[serde(rename = "BBSW")]
    Bbsw,
    #[serde(rename = "BUBO")]
    Bubo,
    #[serde(rename = "CDOR")]
    Cdor,
    #[serde(rename = "CIBO")]
    Cibo,
    #[serde(rename = "EONA")]
    Eona,
    #[serde(rename = "EONS")]
    Eons,
    #[serde(rename = "EURI")]
    Euri,
    #[serde(rename = "EUUS")]
    Euus,
    #[serde(rename = "EUCH")]
    Euch,
    #[serde(rename = "FUSW")]
    Fusw,
    #[serde(rename = "GCFR")]
    Gcfr,
    #[serde(rename = "ISDA")]
    Isda,
    #[serde(rename = "JIBA")]
    Jiba,
    #[serde(rename = "LIBI")]
    Libi,
    #[serde(rename = "LIBO")]
    Libo,
    #[serde(rename = "MOSP")]
    Mosp,
    #[serde(rename = "MAAA")]
    Maaa,
    #[serde(rename = "NIBO")]
    Nibo,
    #[serde(rename = "PFAN")]
    Pfan,
    #[serde(rename = "PRBO")]
    Prbo,
    #[serde(rename = "STBO")]
    Stbo,
    #[serde(rename = "SWAP")]
    Swap,
    #[serde(rename = "TLBO")]
    Tlbo,
    #[serde(rename = "TIBO")]
    Tibo,
    #[serde(rename = "TREA")]
    Trea,
    #[serde(rename = "WIBO")]
    Wibo,
    #[serde(rename = "SOFR")]
    Sofr,
    #[serde(rename = "SONA")]
    Sona,
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
pub struct Counterparty29 {
    #[serde(rename = "Id")]
    pub id: OrganisationIdentification9Choice,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FinancialPartySectorType1Code {
    #[serde(rename = "AIFD")]
    Aifd,
    #[serde(rename = "ASSU")]
    Assu,
    #[serde(rename = "CDTI")]
    Cdti,
    #[serde(rename = "INUN")]
    Inun,
    #[serde(rename = "INVF")]
    Invf,
    #[serde(rename = "ORPI")]
    Orpi,
    #[serde(rename = "REIN")]
    Rein,
    #[serde(rename = "UCIT")]
    Ucit,
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
pub struct PositionSetAggregated3 {
    #[validate]
    #[serde(rename = "RefDt")]
    pub ref_dt: IsoDate,
    #[validate(length(min = 0,))]
    #[serde(rename = "PosSet", default)]
    pub pos_set: Vec<PositionSet5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CcyPosSet", default)]
    pub ccy_pos_set: Vec<PositionSet5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CollPosSet", default)]
    pub coll_pos_set: Vec<PositionSet4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CcyCollPosSet", default)]
    pub ccy_coll_pos_set: Vec<PositionSet4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MaturityTerm2 {
    #[serde(rename = "Unit")]
    pub unit: RateBasis1Code,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max3Number,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PositionSetCollateralMetrics1 {
    #[serde(rename = "Ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<PositionSetCollateralTotal1>,
    #[serde(rename = "Clean", skip_serializing_if = "Option::is_none")]
    pub clean: Option<PositionSetCollateralTotal1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IsoRestrictedYear {
    #[validate(regex = "ISO_RESTRICTED_YEAR_REGEX")]
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
pub struct IsinOct2015Identifier {
    #[validate(regex = "ISIN_OCT_2015_IDENTIFIER_REGEX")]
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
pub struct OrganisationIdentification8ChoiceEnum {
    #[serde(rename = "ClntId", skip_serializing_if = "Option::is_none")]
    pub clnt_id: Option<Max50Text>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentification8Choice {
    #[serde(flatten)]
    pub value: OrganisationIdentification8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentification9ChoiceEnum {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "ClntId", skip_serializing_if = "Option::is_none")]
    pub clnt_id: Option<Max50Text>,
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
pub struct OrganisationIdentification9Choice {
    #[serde(flatten)]
    pub value: OrganisationIdentification9ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "DerivsTradPosSetRpt")]
    pub derivs_trad_pos_set_rpt: DerivativesTradePositionSetReportV01<A>,
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
pub struct PositionSetMetrics1 {
    #[serde(rename = "Ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<PositionSetBuyerAndSeller1>,
    #[serde(rename = "Clean", skip_serializing_if = "Option::is_none")]
    pub clean: Option<PositionSetBuyerAndSeller1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReportPeriodActivity1Code {
    #[serde(rename = "NOTX")]
    Notx,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SpecialPurpose2Code {
    #[serde(rename = "BLNK")]
    Blnk,
    #[serde(rename = "NTAV")]
    Ntav,
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
pub struct PositionSet5 {
    #[validate]
    #[serde(rename = "Dmnsns")]
    pub dmnsns: PositionSetDimensions3,
    #[validate]
    #[serde(rename = "Mtrcs")]
    pub mtrcs: PositionSetMetrics1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PositionSetTotal1 {
    #[serde(rename = "NbOfTrds", skip_serializing_if = "Option::is_none")]
    pub nb_of_trds: Option<Max20PositiveNumber>,
    #[serde(rename = "Postv", skip_serializing_if = "Option::is_none")]
    pub postv: Option<PositionSetValueAndNotional1>,
    #[serde(rename = "Neg", skip_serializing_if = "Option::is_none")]
    pub neg: Option<PositionSetValueAndNotional1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TimeToMaturity1ChoiceEnum {
    #[serde(rename = "Spcl", skip_serializing_if = "Option::is_none")]
    pub spcl: Option<SpecialPurpose2Code>,
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<TimeToMaturityPeriod1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TimeToMaturity1Choice {
    #[serde(flatten)]
    pub value: TimeToMaturity1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RateBasis1Code {
    #[serde(rename = "DAYS")]
    Days,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "YEAR")]
    Year,
    #[default]
    Unknown,
}
