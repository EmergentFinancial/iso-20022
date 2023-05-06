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
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:colr.016.001.05".to_string()
}

#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralAppliedExcess1Code {
    #[serde(rename = "APLD")]
    Apld,
    #[serde(rename = "EXCS")]
    Excs,
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
pub struct Number {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InterestComputationMethod2Code {
    #[serde(rename = "A001")]
    A001,
    #[serde(rename = "A002")]
    A002,
    #[serde(rename = "A003")]
    A003,
    #[serde(rename = "A004")]
    A004,
    #[serde(rename = "A005")]
    A005,
    #[serde(rename = "A006")]
    A006,
    #[serde(rename = "A007")]
    A007,
    #[serde(rename = "A008")]
    A008,
    #[serde(rename = "A009")]
    A009,
    #[serde(rename = "A010")]
    A010,
    #[serde(rename = "A011")]
    A011,
    #[serde(rename = "A012")]
    A012,
    #[serde(rename = "A013")]
    A013,
    #[serde(rename = "A014")]
    A014,
    #[serde(rename = "NARR")]
    Narr,
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
pub struct ReportParameters6 {
    #[validate]
    #[serde(rename = "RptId")]
    pub rpt_id: Max35Text,
    #[serde(rename = "RptDtAndTm")]
    pub rpt_dt_and_tm: DateAndDateTime2Choice,
    #[serde(rename = "Frqcy")]
    pub frqcy: EventFrequency6Code,
    #[serde(rename = "RptCcy")]
    pub rpt_ccy: ActiveCurrencyCode,
    #[serde(rename = "ClctnDt", skip_serializing_if = "Option::is_none")]
    pub clctn_dt: Option<IsoDateTime>,
}
#[derive(
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
pub struct CollateralAccountIdentificationType3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CollateralAccountType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralAccountIdentificationType3Choice {
    #[serde(flatten)]
    pub value: CollateralAccountIdentificationType3ChoiceEnum,
}
#[derive(
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
pub struct CollateralAndExposureReportV05<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "RptParams")]
    pub rpt_params: ReportParameters6,
    #[serde(rename = "Pgntn", skip_serializing_if = "Option::is_none")]
    pub pgntn: Option<Pagination1>,
    #[validate]
    #[serde(rename = "Oblgtn")]
    pub oblgtn: Obligation11,
    #[serde(rename = "Agrmt", skip_serializing_if = "Option::is_none")]
    pub agrmt: Option<Agreement4>,
    #[validate(length(min = 1,))]
    #[serde(rename = "CollRpt", default)]
    pub coll_rpt: Vec<Collateral53>,
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
pub struct FinancialInstrumentQuantity33ChoiceEnum {
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "DgtlTknUnit", skip_serializing_if = "Option::is_none")]
    pub dgtl_tkn_unit: Option<Max30DecimalNumber>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity33Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantity33ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification78 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: GenericIdentification30,
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
pub struct ReturnExcessCash1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ReturnExcessCash1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReturnExcessCash1Choice {
    #[serde(flatten)]
    pub value: ReturnExcessCash1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Collateral53 {
    #[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<CollateralAccount3>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet5>,
    #[validate]
    #[serde(rename = "RptSummry")]
    pub rpt_summry: Summary3,
    #[validate(length(min = 0,))]
    #[serde(rename = "CollValtn", default)]
    pub coll_valtn: Vec<CollateralValuation13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateCode9ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateCode9Choice {
    #[serde(flatten)]
    pub value: DateCode9ChoiceEnum,
}
#[derive(
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
pub struct Obligation11 {
    #[validate]
    #[serde(rename = "PtyA")]
    pub pty_a: PartyIdentification242,
    #[serde(rename = "SvcgPtyA", skip_serializing_if = "Option::is_none")]
    pub svcg_pty_a: Option<PartyIdentification178Choice>,
    #[validate]
    #[serde(rename = "PtyB")]
    pub pty_b: PartyIdentification242,
    #[serde(rename = "SvcgPtyB", skip_serializing_if = "Option::is_none")]
    pub svcg_pty_b: Option<PartyIdentification178Choice>,
    #[serde(rename = "CollAcctId", skip_serializing_if = "Option::is_none")]
    pub coll_acct_id: Option<CollateralAccount3>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet5>,
    #[serde(rename = "XpsrTp", skip_serializing_if = "Option::is_none")]
    pub xpsr_tp: Option<ExposureType11Code>,
    #[serde(rename = "ValtnDt")]
    pub valtn_dt: DateAndDateTime2Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PostalAddress2 {
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[validate]
    #[serde(rename = "PstCdId")]
    pub pst_cd_id: Max16Text,
    #[validate]
    #[serde(rename = "TwnNm")]
    pub twn_nm: Max35Text,
    #[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<Max35Text>,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementStatus3Code {
    #[serde(rename = "ASTL")]
    Astl,
    #[serde(rename = "AAUT")]
    Aaut,
    #[serde(rename = "ACCF")]
    Accf,
    #[serde(rename = "ARCF")]
    Arcf,
    #[serde(rename = "MTCH")]
    Mtch,
    #[serde(rename = "PSTL")]
    Pstl,
    #[serde(rename = "RJCT")]
    Rjct,
    #[serde(rename = "STLD")]
    Stld,
    #[serde(rename = "STCR")]
    Stcr,
    #[serde(rename = "SPLT")]
    Splt,
    #[serde(rename = "NMAT")]
    Nmat,
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
pub struct SupplementaryDataEnvelope1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(flatten)]
    pub value: A,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExposureType13Code {
    #[serde(rename = "CCIR")]
    Ccir,
    #[serde(rename = "CRPR")]
    Crpr,
    #[serde(rename = "EQUI")]
    Equi,
    #[serde(rename = "EQPT")]
    Eqpt,
    #[serde(rename = "EQUS")]
    Equs,
    #[serde(rename = "EXTD")]
    Extd,
    #[serde(rename = "EXPT")]
    Expt,
    #[serde(rename = "FIXI")]
    Fixi,
    #[serde(rename = "FORX")]
    Forx,
    #[serde(rename = "FORW")]
    Forw,
    #[serde(rename = "FUTR")]
    Futr,
    #[serde(rename = "OPTN")]
    Optn,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "MGLD")]
    Mgld,
    #[serde(rename = "OTCD")]
    Otcd,
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "RVPO")]
    Rvpo,
    #[serde(rename = "SLOA")]
    Sloa,
    #[serde(rename = "SBSC")]
    Sbsc,
    #[serde(rename = "SCRP")]
    Scrp,
    #[serde(rename = "SLEB")]
    Sleb,
    #[serde(rename = "SHSL")]
    Shsl,
    #[serde(rename = "SCIR")]
    Scir,
    #[serde(rename = "SCIE")]
    Scie,
    #[serde(rename = "ESCL")]
    Escl,
    #[serde(rename = "SWPT")]
    Swpt,
    #[serde(rename = "TBAS")]
    Tbas,
    #[serde(rename = "ECRT")]
    Ecrt,
    #[serde(rename = "ECFR")]
    Ecfr,
    #[serde(rename = "EMLO")]
    Emlo,
    #[serde(rename = "EMLI")]
    Emli,
    #[serde(rename = "EOIM")]
    Eoim,
    #[serde(rename = "EOMI")]
    Eomi,
    #[serde(rename = "TRBD")]
    Trbd,
    #[serde(rename = "BFWD")]
    Bfwd,
    #[serde(rename = "PAYM")]
    Paym,
    #[serde(rename = "CCPC")]
    Ccpc,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "CRDS")]
    Crds,
    #[serde(rename = "CRTL")]
    Crtl,
    #[serde(rename = "CRSP")]
    Crsp,
    #[serde(rename = "EOMO")]
    Eomo,
    #[serde(rename = "CBCO")]
    Cbco,
    #[serde(rename = "TRCP")]
    Trcp,
    #[serde(rename = "UDMS")]
    Udms,
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
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
    #[serde(rename = "ActiveOrHistoricCurrencyAnd13DecimalAmount")]
    pub value: ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType,
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
pub struct BlockChainAddressWallet3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max140Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification30>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DepositType1Code {
    #[serde(rename = "FITE")]
    Fite,
    #[serde(rename = "CALL")]
    Call,
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
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
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
pub struct GenericIdentification30 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Exact4AlphaNumericText,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndDateTime2ChoiceEnum {
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<IsoDateTime>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndDateTime2Choice {
    #[serde(flatten)]
    pub value: DateAndDateTime2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralAccountType1Code {
    #[serde(rename = "HOUS")]
    Hous,
    #[serde(rename = "CLIE")]
    Clie,
    #[serde(rename = "LIPR")]
    Lipr,
    #[serde(rename = "MGIN")]
    Mgin,
    #[serde(rename = "DFLT")]
    Dflt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralDirection1Code {
    #[serde(rename = "CDPA")]
    Cdpa,
    #[serde(rename = "CDPB")]
    Cdpb,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CcpMemberType1Code {
    #[serde(rename = "ACMB")]
    Acmb,
    #[serde(rename = "CCPX")]
    Ccpx,
    #[serde(rename = "DCMB")]
    Dcmb,
    #[serde(rename = "FCMC")]
    Fcmc,
    #[serde(rename = "GCMB")]
    Gcmb,
    #[serde(rename = "SCMB")]
    Scmb,
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
pub struct SafekeepingPlaceTypeAndText8 {
    #[serde(rename = "SfkpgPlcTp")]
    pub sfkpg_plc_tp: SafekeepingPlace3Code,
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
pub struct IdentificationSource3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
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
pub struct IdentificationSource3Choice {
    #[serde(flatten)]
    pub value: IdentificationSource3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Summary3 {
    #[serde(rename = "XpsdAmtPtyA", skip_serializing_if = "Option::is_none")]
    pub xpsd_amt_pty_a: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "XpsdAmtPtyB", skip_serializing_if = "Option::is_none")]
    pub xpsd_amt_pty_b: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "XpsrTp")]
    pub xpsr_tp: ExposureType13Code,
    #[validate]
    #[serde(rename = "TtlValOfColl")]
    pub ttl_val_of_coll: ActiveCurrencyAndAmount,
    #[serde(rename = "NetXcssDfcit", skip_serializing_if = "Option::is_none")]
    pub net_xcss_dfcit: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "NetXcssDfcitInd", skip_serializing_if = "Option::is_none")]
    pub net_xcss_dfcit_ind: Option<ShortLong1Code>,
    #[validate]
    #[serde(rename = "ValtnDtTm")]
    pub valtn_dt_tm: IsoDateTime,
    #[serde(rename = "ReqdSttlmDt", skip_serializing_if = "Option::is_none")]
    pub reqd_sttlm_dt: Option<IsoDate>,
    #[serde(rename = "SummryDtls", skip_serializing_if = "Option::is_none")]
    pub summry_dtls: Option<SummaryAmounts2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SummaryAmounts2 {
    #[serde(rename = "ThrshldAmt", skip_serializing_if = "Option::is_none")]
    pub thrshld_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ThrshldTp", skip_serializing_if = "Option::is_none")]
    pub thrshld_tp: Option<ThresholdType1Code>,
    #[serde(rename = "PreHrcutCollVal", skip_serializing_if = "Option::is_none")]
    pub pre_hrcut_coll_val: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "AdjstdXpsr", skip_serializing_if = "Option::is_none")]
    pub adjstd_xpsr: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "CollReqrd", skip_serializing_if = "Option::is_none")]
    pub coll_reqrd: Option<ActiveCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RtrXcssCshAndCollCcy", default)]
    pub rtr_xcss_csh_and_coll_ccy: Vec<ReturnExcessCash1>,
    #[serde(rename = "MinTrfAmt", skip_serializing_if = "Option::is_none")]
    pub min_trf_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "RndgAmt", skip_serializing_if = "Option::is_none")]
    pub rndg_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "PrvsXpsrVal", skip_serializing_if = "Option::is_none")]
    pub prvs_xpsr_val: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "PrvsCollVal", skip_serializing_if = "Option::is_none")]
    pub prvs_coll_val: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TtlPdgIncmgColl", skip_serializing_if = "Option::is_none")]
    pub ttl_pdg_incmg_coll: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TtlPdgOutgngColl", skip_serializing_if = "Option::is_none")]
    pub ttl_pdg_outgng_coll: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TtlAcrdIntrstAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_acrd_intrst_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TtlFees", skip_serializing_if = "Option::is_none")]
    pub ttl_fees: Option<ActiveCurrencyAndAmount>,
}
#[derive(
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
#[derive(
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
pub struct OtherCollateral10 {
    #[serde(rename = "AsstNb", skip_serializing_if = "Option::is_none")]
    pub asst_nb: Option<Max35Text>,
    #[serde(rename = "LttrOfCdtId", skip_serializing_if = "Option::is_none")]
    pub lttr_of_cdt_id: Option<Max35Text>,
    #[serde(rename = "LttrOfCdtAmt", skip_serializing_if = "Option::is_none")]
    pub lttr_of_cdt_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "GrntAmt", skip_serializing_if = "Option::is_none")]
    pub grnt_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "OthrTpOfColl", skip_serializing_if = "Option::is_none")]
    pub othr_tp_of_coll: Option<OtherTypeOfCollateral3>,
    #[serde(rename = "CollOwnrsh", skip_serializing_if = "Option::is_none")]
    pub coll_ownrsh: Option<CollateralOwnership3>,
    #[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
    pub isse_dt: Option<DateFormat14Choice>,
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<DateFormat14Choice>,
    #[serde(rename = "LtdCvrgInd", skip_serializing_if = "Option::is_none")]
    pub ltd_cvrg_ind: Option<YesNoIndicator>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyIdentification178Choice>,
    #[serde(rename = "BlckdQty", skip_serializing_if = "Option::is_none")]
    pub blckd_qty: Option<FinancialInstrumentQuantity33Choice>,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<IsoDate>,
    #[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
    pub xchg_rate: Option<BaseOneRate>,
    #[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
    pub mkt_val: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "Hrcut", skip_serializing_if = "Option::is_none")]
    pub hrcut: Option<PercentageRate>,
    #[validate]
    #[serde(rename = "CollVal")]
    pub coll_val: ActiveCurrencyAndAmount,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<SafekeepingPlaceFormat29Choice>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount19>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat14ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<DateCode9Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat14Choice {
    #[serde(flatten)]
    pub value: DateFormat14ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherTypeOfCollateral3 {
    #[validate]
    #[serde(rename = "Desc")]
    pub desc: Max140Text,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<FinancialInstrumentQuantity33Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExposureType11Code {
    #[serde(rename = "BFWD")]
    Bfwd,
    #[serde(rename = "PAYM")]
    Paym,
    #[serde(rename = "CBCO")]
    Cbco,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "CRDS")]
    Crds,
    #[serde(rename = "CRTL")]
    Crtl,
    #[serde(rename = "CRSP")]
    Crsp,
    #[serde(rename = "CCIR")]
    Ccir,
    #[serde(rename = "CRPR")]
    Crpr,
    #[serde(rename = "EQPT")]
    Eqpt,
    #[serde(rename = "EQUS")]
    Equs,
    #[serde(rename = "EXTD")]
    Extd,
    #[serde(rename = "EXPT")]
    Expt,
    #[serde(rename = "FIXI")]
    Fixi,
    #[serde(rename = "FORX")]
    Forx,
    #[serde(rename = "FORW")]
    Forw,
    #[serde(rename = "FUTR")]
    Futr,
    #[serde(rename = "OPTN")]
    Optn,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "OTCD")]
    Otcd,
    #[serde(rename = "RVPO")]
    Rvpo,
    #[serde(rename = "SLOA")]
    Sloa,
    #[serde(rename = "SBSC")]
    Sbsc,
    #[serde(rename = "SCRP")]
    Scrp,
    #[serde(rename = "SLEB")]
    Sleb,
    #[serde(rename = "SCIR")]
    Scir,
    #[serde(rename = "SCIE")]
    Scie,
    #[serde(rename = "SWPT")]
    Swpt,
    #[serde(rename = "TBAS")]
    Tbas,
    #[serde(rename = "TRCP")]
    Trcp,
    #[serde(rename = "UDMS")]
    Udms,
    #[serde(rename = "CCPC")]
    Ccpc,
    #[serde(rename = "EQUI")]
    Equi,
    #[serde(rename = "TRBD")]
    Trbd,
    #[serde(rename = "REPO")]
    Repo,
    #[serde(rename = "SHSL")]
    Shsl,
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
pub struct PartyIdentification178ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress6>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
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
pub struct PartyIdentification178Choice {
    #[serde(flatten)]
    pub value: PartyIdentification178ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Price7 {
    #[serde(rename = "Tp")]
    pub tp: YieldedOrValueType1Choice,
    #[serde(rename = "Val")]
    pub val: PriceRateOrAmount3Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceFormat29ChoiceEnum {
    #[serde(rename = "TpAndId", skip_serializing_if = "Option::is_none")]
    pub tp_and_id: Option<SafekeepingPlaceTypeAndIdentification1>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<SafekeepingPlaceTypeAndText8>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification78>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceFormat29Choice {
    #[serde(flatten)]
    pub value: SafekeepingPlaceFormat29ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
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
pub struct Pagination1 {
    #[validate]
    #[serde(rename = "PgNb")]
    pub pg_nb: Max5NumericText,
    #[validate]
    #[serde(rename = "LastPgInd")]
    pub last_pg_ind: YesNoIndicator,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceTypeAndIdentification1 {
    #[serde(rename = "SfkpgPlcTp")]
    pub sfkpg_plc_tp: SafekeepingPlace1Code,
    #[validate]
    #[serde(rename = "Id")]
    pub id: AnyBicDec2014Identifier,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashCollateral4 {
    #[serde(rename = "AsstNb", skip_serializing_if = "Option::is_none")]
    pub asst_nb: Option<Max35Text>,
    #[serde(rename = "DpstAmt", skip_serializing_if = "Option::is_none")]
    pub dpst_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "DpstTp", skip_serializing_if = "Option::is_none")]
    pub dpst_tp: Option<DepositType1Code>,
    #[serde(rename = "BlckdAmt", skip_serializing_if = "Option::is_none")]
    pub blckd_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<IsoDate>,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<IsoDate>,
    #[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
    pub xchg_rate: Option<BaseOneRate>,
    #[validate]
    #[serde(rename = "CollVal")]
    pub coll_val: ActiveCurrencyAndAmount,
    #[serde(rename = "Hrcut", skip_serializing_if = "Option::is_none")]
    pub hrcut: Option<PercentageRate>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReturnExcessCash1Code {
    #[serde(rename = "RTND")]
    Rtnd,
    #[serde(rename = "RTDN")]
    Rtdn,
    #[serde(rename = "SSPD")]
    Sspd,
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
pub enum ThresholdType1Code {
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "UNSE")]
    Unse,
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
pub struct CollateralAmount1 {
    #[validate]
    #[serde(rename = "CollAmt")]
    pub coll_amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "RptdCcyAndAmt")]
    pub rptd_ccy_and_amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "MktValAmt")]
    pub mkt_val_amt: ActiveCurrencyAndAmount,
    #[serde(rename = "AcrdIntrstAmt", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "FeesAndComssns", skip_serializing_if = "Option::is_none")]
    pub fees_and_comssns: Option<ActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification36 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
}
#[derive(
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
pub struct Agreement4 {
    #[validate]
    #[serde(rename = "AgrmtDtls")]
    pub agrmt_dtls: Max140Text,
    #[serde(rename = "AgrmtId", skip_serializing_if = "Option::is_none")]
    pub agrmt_id: Option<Max140Text>,
    #[validate]
    #[serde(rename = "AgrmtDt")]
    pub agrmt_dt: IsoDate,
    #[serde(rename = "BaseCcy")]
    pub base_ccy: ActiveCurrencyCode,
    #[serde(rename = "AgrmtFrmwk", skip_serializing_if = "Option::is_none")]
    pub agrmt_frmwk: Option<AgreementFramework1Choice>,
}
#[derive(
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
pub struct NameAndAddress6 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max70Text,
    #[validate]
    #[serde(rename = "Adr")]
    pub adr: PostalAddress2,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ShortLong1Code {
    #[serde(rename = "SHOR")]
    Shor,
    #[serde(rename = "LONG")]
    Long,
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
pub struct OtherIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
    pub sfx: Option<Max16Text>,
    #[serde(rename = "Tp")]
    pub tp: IdentificationSource3Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesCollateral13 {
    #[serde(rename = "AsstNb", skip_serializing_if = "Option::is_none")]
    pub asst_nb: Option<Max35Text>,
    #[validate]
    #[serde(rename = "SctyId")]
    pub scty_id: SecurityIdentification19,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "CollOwnrsh", skip_serializing_if = "Option::is_none")]
    pub coll_ownrsh: Option<CollateralOwnership3>,
    #[serde(rename = "LtdCvrgInd", skip_serializing_if = "Option::is_none")]
    pub ltd_cvrg_ind: Option<YesNoIndicator>,
    #[serde(rename = "Qty")]
    pub qty: FinancialInstrumentQuantity33Choice,
    #[serde(rename = "BlckdQty", skip_serializing_if = "Option::is_none")]
    pub blckd_qty: Option<FinancialInstrumentQuantity33Choice>,
    #[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
    pub pric: Option<Price7>,
    #[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
    pub mkt_val: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "Hrcut", skip_serializing_if = "Option::is_none")]
    pub hrcut: Option<PercentageRate>,
    #[serde(rename = "CollVal", skip_serializing_if = "Option::is_none")]
    pub coll_val: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<IsoDate>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount19>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet3>,
    #[serde(rename = "SfkpgPlc")]
    pub sfkpg_plc: SafekeepingPlaceFormat29Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReturnExcessCash1 {
    #[serde(rename = "RtrXcssCshTp")]
    pub rtr_xcss_csh_tp: ReturnExcessCash1Choice,
    #[serde(rename = "CshCollCcy")]
    pub csh_coll_ccy: ActiveOrHistoricCurrencyCode,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralType8Code {
    #[serde(rename = "COMO")]
    Como,
    #[serde(rename = "CCCL")]
    Cccl,
    #[serde(rename = "CEMC")]
    Cemc,
    #[serde(rename = "CXCC")]
    Cxcc,
    #[serde(rename = "CFTD")]
    Cftd,
    #[serde(rename = "CFTI")]
    Cfti,
    #[serde(rename = "CTRC")]
    Ctrc,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "LCRE")]
    Lcre,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "CTCO")]
    Ctco,
    #[serde(rename = "CCVR")]
    Ccvr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AgreementFramework1Code {
    #[serde(rename = "FBAA")]
    Fbaa,
    #[serde(rename = "BBAA")]
    Bbaa,
    #[serde(rename = "DERV")]
    Derv,
    #[serde(rename = "ISDA")]
    Isda,
    #[serde(rename = "NONR")]
    Nonr,
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
pub struct IsoDateTime {
    #[serde(rename = "$text")]
    pub value: ::chrono::DateTime<::chrono::Utc>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DateType2Code {
    #[serde(rename = "OPEN")]
    Open,
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "CollAndXpsrRpt")]
    pub coll_and_xpsr_rpt: CollateralAndExposureReportV05<A>,
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
pub struct PercentageRate {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceValueType1Code {
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "PREM")]
    Prem,
    #[serde(rename = "PARV")]
    Parv,
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
pub struct CollateralOwnership3 {
    #[validate]
    #[serde(rename = "Prtry")]
    pub prtry: YesNoIndicator,
    #[serde(rename = "ClntNm", skip_serializing_if = "Option::is_none")]
    pub clnt_nm: Option<PartyIdentification178Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesAccount19 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification30>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct YieldedOrValueType1ChoiceEnum {
    #[serde(rename = "ValTp", skip_serializing_if = "Option::is_none")]
    pub val_tp: Option<PriceValueType1Code>,
    #[serde(rename = "Yldd", skip_serializing_if = "Option::is_none")]
    pub yldd: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct YieldedOrValueType1Choice {
    #[serde(flatten)]
    pub value: YieldedOrValueType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralAccount3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CollateralAccountIdentificationType3Choice>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgreementFramework1ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification30>,
    #[serde(rename = "AgrmtFrmwk", skip_serializing_if = "Option::is_none")]
    pub agrmt_frmwk: Option<AgreementFramework1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgreementFramework1Choice {
    #[serde(flatten)]
    pub value: AgreementFramework1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceRateOrAmount3ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceRateOrAmount3Choice {
    #[serde(flatten)]
    pub value: PriceRateOrAmount3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification19 {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrId", default)]
    pub othr_id: Vec<OtherIdentification1>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BlockChainAddressWallet5 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max140Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<CollateralAccountIdentificationType3Choice>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EventFrequency6Code {
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "ONDE")]
    Onde,
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
pub struct ImpliedCurrencyAndAmount {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SafekeepingPlace3Code {
    #[serde(rename = "SHHE")]
    Shhe,
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
pub struct PartyIdentification242 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification178Choice,
    #[serde(rename = "CCPMmbTp", skip_serializing_if = "Option::is_none")]
    pub ccp_mmb_tp: Option<CcpMemberType1Code>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SafekeepingPlace1Code {
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "ICSD")]
    Icsd,
    #[serde(rename = "NCSD")]
    Ncsd,
    #[serde(rename = "SHHE")]
    Shhe,
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
pub struct Max30DecimalNumber {
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
pub struct CollateralValuation13 {
    #[serde(rename = "CollId", skip_serializing_if = "Option::is_none")]
    pub coll_id: Option<Max35Text>,
    #[serde(rename = "CollTp")]
    pub coll_tp: CollateralType8Code,
    #[serde(rename = "CollDrctn", skip_serializing_if = "Option::is_none")]
    pub coll_drctn: Option<CollateralDirection1Code>,
    #[serde(rename = "SttlmSts")]
    pub sttlm_sts: SettlementStatus3Code,
    #[serde(rename = "ApldXcssInd", skip_serializing_if = "Option::is_none")]
    pub apld_xcss_ind: Option<CollateralAppliedExcess1Code>,
    #[serde(rename = "NbOfDaysAcrd", skip_serializing_if = "Option::is_none")]
    pub nb_of_days_acrd: Option<Number>,
    #[validate]
    #[serde(rename = "ValtnAmts")]
    pub valtn_amts: CollateralAmount1,
    #[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
    pub day_cnt_bsis: Option<InterestComputationMethod2Code>,
    #[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
    pub xchg_rate: Option<BaseOneRate>,
    #[serde(rename = "CcyHrcut", skip_serializing_if = "Option::is_none")]
    pub ccy_hrcut: Option<BaseOneRate>,
    #[serde(rename = "AdjstdRate", skip_serializing_if = "Option::is_none")]
    pub adjstd_rate: Option<BaseOneRate>,
    #[serde(rename = "SctiesColl", skip_serializing_if = "Option::is_none")]
    pub scties_coll: Option<SecuritiesCollateral13>,
    #[serde(rename = "CshColl", skip_serializing_if = "Option::is_none")]
    pub csh_coll: Option<CashCollateral4>,
    #[serde(rename = "OthrColl", skip_serializing_if = "Option::is_none")]
    pub othr_coll: Option<OtherCollateral10>,
}
