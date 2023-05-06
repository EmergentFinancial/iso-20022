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
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_2021_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_YEAR_MONTH_REGEX: ::regex::Regex = ::regex::Regex::new(r#"^-?\d{4}-(0[1-9]|1[0-2])([+-]\d{2}:\d{2}|Z)?$"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_15_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,15}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:reda.006.001.01".to_string()
}

#[derive(
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
pub struct Max35Text {
    #[validate(length(min = 1, max = 35,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Appearance1Code {
    #[serde(rename = "DELI")]
    Deli,
    #[serde(rename = "NDEL")]
    Ndel,
    #[serde(rename = "LIMI")]
    Limi,
    #[serde(rename = "BENT")]
    Bent,
    #[serde(rename = "DFBE")]
    Dfbe,
    #[serde(rename = "DLBE")]
    Dlbe,
    #[serde(rename = "TMPG")]
    Tmpg,
    #[serde(rename = "GLOB")]
    Glob,
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
pub struct GlobalNote2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<GlobalNote1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GlobalNote2Choice {
    #[serde(flatten)]
    pub value: GlobalNote2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MaturityRedemptionType1Code {
    #[serde(rename = "FRED")]
    Fred,
    #[serde(rename = "PRNR")]
    Prnr,
    #[serde(rename = "PRWR")]
    Prwr,
    #[serde(rename = "RNDM")]
    Rndm,
    #[serde(rename = "PRRA")]
    Prra,
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "PUUT")]
    Puut,
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
pub struct OptionParty3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionParty1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionParty3Choice {
    #[serde(flatten)]
    pub value: OptionParty3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityCreationRequestV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "MsgHdr", skip_serializing_if = "Option::is_none")]
    pub msg_hdr: Option<MessageHeader1>,
    #[validate]
    #[serde(rename = "Scty")]
    pub scty: SecurityAttributes10,
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
pub struct Issuance5 {
    #[serde(rename = "IssePlc", skip_serializing_if = "Option::is_none")]
    pub isse_plc: Option<MicIdentifier>,
    #[serde(rename = "CtryOfIsse", skip_serializing_if = "Option::is_none")]
    pub ctry_of_isse: Option<CountryCode>,
    #[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
    pub isse_dt: Option<IsoDate>,
    #[serde(rename = "AnncmntDt", skip_serializing_if = "Option::is_none")]
    pub anncmnt_dt: Option<IsoDateTime>,
    #[serde(rename = "ISINVldFr", skip_serializing_if = "Option::is_none")]
    pub isin_vld_fr: Option<IsoDate>,
    #[serde(rename = "IssrOrg", skip_serializing_if = "Option::is_none")]
    pub issr_org: Option<Organisation38>,
    #[serde(rename = "IsseNmnlAmt", skip_serializing_if = "Option::is_none")]
    pub isse_nmnl_amt: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "FullIssdAmt", skip_serializing_if = "Option::is_none")]
    pub full_issd_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "IsseSz", skip_serializing_if = "Option::is_none")]
    pub isse_sz: Option<Number>,
    #[serde(rename = "IssePric", skip_serializing_if = "Option::is_none")]
    pub isse_pric: Option<PriceValue1>,
    #[serde(rename = "IssncDstrbtn", skip_serializing_if = "Option::is_none")]
    pub issnc_dstrbtn: Option<SecuritiesTransactionType31Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "GovngLaw", default)]
    pub govng_law: Vec<Jurisdiction1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementUnitType3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementUnitType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementUnitType3Choice {
    #[serde(flatten)]
    pub value: SettlementUnitType3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NameAndAddress4 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[validate]
    #[serde(rename = "Adr")]
    pub adr: PostalAddress1,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeTransactionCondition2Code {
    #[serde(rename = "SPCC")]
    Spcc,
    #[serde(rename = "SECN")]
    Secn,
    #[serde(rename = "SEBN")]
    Sebn,
    #[serde(rename = "SCBN")]
    Scbn,
    #[serde(rename = "SCRT")]
    Scrt,
    #[serde(rename = "SERT")]
    Sert,
    #[serde(rename = "SCCR")]
    Sccr,
    #[serde(rename = "SECR")]
    Secr,
    #[serde(rename = "CAST")]
    Cast,
    #[serde(rename = "SPPR")]
    Sppr,
    #[serde(rename = "SPCU")]
    Spcu,
    #[serde(rename = "SPEX")]
    Spex,
    #[serde(rename = "GTDL")]
    Gtdl,
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
pub struct DistributionPolicy2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DistributionPolicy1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DistributionPolicy2Choice {
    #[serde(flatten)]
    pub value: DistributionPolicy2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateTimePeriod2 {
    #[validate]
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm: IsoDateTime,
    #[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
    pub to_dt_tm: Option<IsoDateTime>,
}
#[derive(
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
pub struct OtherIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
    pub sfx: Option<Max16Text>,
    #[serde(rename = "Tp")]
    pub tp: IdentificationSource3Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InitialPhysicalForm2Code {
    #[serde(rename = "GPGP")]
    Gpgp,
    #[serde(rename = "DERN")]
    Dern,
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
pub struct UnitOfMeasure7ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<UnitOfMeasure9Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitOfMeasure7Choice {
    #[serde(flatten)]
    pub value: UnitOfMeasure7ChoiceEnum,
}
#[derive(
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
pub struct IdentificationSource3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
}
#[derive(
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
pub struct TradingParameters2 {
    #[serde(rename = "MktId", skip_serializing_if = "Option::is_none")]
    pub mkt_id: Option<MicIdentifier>,
    #[serde(rename = "RndLot", skip_serializing_if = "Option::is_none")]
    pub rnd_lot: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "TradLotSz", skip_serializing_if = "Option::is_none")]
    pub trad_lot_sz: Option<FinancialInstrumentQuantity1Choice>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "ScndryPlcOfListg", default)]
    pub scndry_plc_of_listg: Vec<MicIdentifier>,
    #[serde(rename = "MinTraddNmnlQty", skip_serializing_if = "Option::is_none")]
    pub min_tradd_nmnl_qty: Option<UnitOrFaceAmount1Choice>,
    #[serde(rename = "MaxTraddNmnlQty", skip_serializing_if = "Option::is_none")]
    pub max_tradd_nmnl_qty: Option<UnitOrFaceAmount1Choice>,
    #[serde(
        rename = "MinTradgPricgIncrmt",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_tradg_pricg_incrmt: Option<Number>,
    #[serde(rename = "PmryPlcOfListgId", skip_serializing_if = "Option::is_none")]
    pub pmry_plc_of_listg_id: Option<MicIdentifier>,
}
#[derive(
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
pub struct DateTimePeriod1ChoiceEnum {
    #[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
    pub to_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "FrDtTm", skip_serializing_if = "Option::is_none")]
    pub fr_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "DtTmRg", skip_serializing_if = "Option::is_none")]
    pub dt_tm_rg: Option<DateTimePeriod1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateTimePeriod1Choice {
    #[serde(flatten)]
    pub value: DateTimePeriod1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct YieldCalculation6 {
    #[validate]
    #[serde(rename = "Val")]
    pub val: PercentageRate,
    #[serde(rename = "ClctnTp", skip_serializing_if = "Option::is_none")]
    pub clctn_tp: Option<CalculationType3Choice>,
    #[serde(rename = "RedPric", skip_serializing_if = "Option::is_none")]
    pub red_pric: Option<Price8>,
    #[validate]
    #[serde(rename = "ValDt")]
    pub val_dt: IsoDate,
    #[serde(rename = "ValPrd")]
    pub val_prd: DateTimePeriod1Choice,
    #[validate]
    #[serde(rename = "ClctnDt")]
    pub clctn_dt: IsoDateTime,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PreferenceToIncome1Code {
    #[serde(rename = "ORDN")]
    Ordn,
    #[serde(rename = "PFRD")]
    Pfrd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettleStyle1Code {
    #[serde(rename = "SETC")]
    Setc,
    #[serde(rename = "SETO")]
    Seto,
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
pub struct FinancialInstrumentQuantity1ChoiceEnum {
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity1Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantity1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InitialPhysicalForm4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InitialPhysicalForm1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InitialPhysicalForm4Choice {
    #[serde(flatten)]
    pub value: InitialPhysicalForm4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestorRestrictionType3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestorRestrictionType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestorRestrictionType3Choice {
    #[serde(flatten)]
    pub value: InvestorRestrictionType3ChoiceEnum,
}
#[derive(
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
pub struct BenchmarkCurveName7ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BenchmarkCurveName1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BenchmarkCurveName7Choice {
    #[serde(flatten)]
    pub value: BenchmarkCurveName7ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Frequency5Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "QURT")]
    Qurt,
    #[serde(rename = "MIAN")]
    Mian,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "ADHO")]
    Adho,
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "OVNG")]
    Ovng,
    #[serde(rename = "TEND")]
    Tend,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InitialPhysicalForm1Code {
    #[serde(rename = "GTGT")]
    Gtgt,
    #[serde(rename = "GPGP")]
    Gpgp,
    #[serde(rename = "DERN")]
    Dern,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BenchmarkCurveName1Code {
    #[serde(rename = "MAAA")]
    Maaa,
    #[serde(rename = "FUSW")]
    Fusw,
    #[serde(rename = "LIBI")]
    Libi,
    #[serde(rename = "LIBO")]
    Libo,
    #[serde(rename = "SWAP")]
    Swap,
    #[serde(rename = "TREA")]
    Trea,
    #[serde(rename = "EURI")]
    Euri,
    #[serde(rename = "PFAN")]
    Pfan,
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
pub struct Equity3 {
    #[serde(rename = "PrefToIncm")]
    pub pref_to_incm: PreferenceToIncome5Choice,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<IsoDateTime>,
    #[serde(rename = "NonPdAmt", skip_serializing_if = "Option::is_none")]
    pub non_pd_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ParVal", skip_serializing_if = "Option::is_none")]
    pub par_val: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "VtngRghtsPerShr", skip_serializing_if = "Option::is_none")]
    pub vtng_rghts_per_shr: Option<Number>,
}
#[derive(
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
pub struct PartyIdentification120ChoiceEnum {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification120Choice {
    #[serde(flatten)]
    pub value: PartyIdentification120ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Option15 {
    #[serde(rename = "OptnSttlmStyle", skip_serializing_if = "Option::is_none")]
    pub optn_sttlm_style: Option<SettleStyle2Choice>,
    #[serde(rename = "ConvsDt", skip_serializing_if = "Option::is_none")]
    pub convs_dt: Option<IsoDateTime>,
    #[serde(rename = "StrkPric", skip_serializing_if = "Option::is_none")]
    pub strk_pric: Option<Price8>,
    #[serde(rename = "MinExrcblQty", skip_serializing_if = "Option::is_none")]
    pub min_exrcbl_qty: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "ConvsPrd", skip_serializing_if = "Option::is_none")]
    pub convs_prd: Option<DateTimePeriod1Choice>,
    #[serde(rename = "OptnStyle", skip_serializing_if = "Option::is_none")]
    pub optn_style: Option<OptionStyle1Choice>,
    #[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
    pub optn_tp: Option<OptionType8Choice>,
    #[serde(rename = "StrkVal", skip_serializing_if = "Option::is_none")]
    pub strk_val: Option<Number>,
    #[serde(rename = "StrkMltplr", skip_serializing_if = "Option::is_none")]
    pub strk_mltplr: Option<Number>,
    #[serde(rename = "InstrmAssgnmtMtd", skip_serializing_if = "Option::is_none")]
    pub instrm_assgnmt_mtd: Option<AssignmentMethod2Choice>,
    #[serde(rename = "VrsnNb", skip_serializing_if = "Option::is_none")]
    pub vrsn_nb: Option<Number>,
    #[serde(rename = "XpryLctn", skip_serializing_if = "Option::is_none")]
    pub xpry_lctn: Option<Max4AlphaNumericText>,
    #[serde(rename = "Stdstn", skip_serializing_if = "Option::is_none")]
    pub stdstn: Option<Standardisation3Choice>,
    #[serde(rename = "TradgPtyRole", skip_serializing_if = "Option::is_none")]
    pub tradg_pty_role: Option<OptionParty3Choice>,
    #[serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none")]
    pub ctrct_sz: Option<BaseOneRate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlUndrlygAttrbts", default)]
    pub addtl_undrlyg_attrbts: Vec<UnderlyingAttributes4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification177ChoiceEnum {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
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
pub struct PartyIdentification177Choice {
    #[serde(flatten)]
    pub value: PartyIdentification177ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PutType3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PutType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PutType3Choice {
    #[serde(flatten)]
    pub value: PutType3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TefraRules3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TefraRules1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TefraRules3Choice {
    #[serde(flatten)]
    pub value: TefraRules3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestorType3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestorType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestorType3Choice {
    #[serde(flatten)]
    pub value: InvestorType3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfPrice1Code {
    #[serde(rename = "AVER")]
    Aver,
    #[serde(rename = "AVOV")]
    Avov,
    #[serde(rename = "COMB")]
    Comb,
    #[serde(rename = "GREX")]
    Grex,
    #[serde(rename = "LIMI")]
    Limi,
    #[serde(rename = "NET2")]
    Net2,
    #[serde(rename = "NDIS")]
    Ndis,
    #[serde(rename = "NET1")]
    Net1,
    #[serde(rename = "NUND")]
    Nund,
    #[serde(rename = "NOGR")]
    Nogr,
    #[serde(rename = "PARV")]
    Parv,
    #[serde(rename = "RDAV")]
    Rdav,
    #[serde(rename = "STOP")]
    Stop,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitOrFaceAmount1ChoiceEnum {
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitOrFaceAmount1Choice {
    #[serde(flatten)]
    pub value: UnitOrFaceAmount1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PutType1Code {
    #[serde(rename = "MAND")]
    Mand,
    #[serde(rename = "OPTI")]
    Opti,
    #[serde(rename = "TWOS")]
    Twos,
    #[default]
    Unknown,
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
pub struct RateOrAbsoluteValue1ChoiceEnum {
    #[serde(rename = "AbsVal", skip_serializing_if = "Option::is_none")]
    pub abs_val: Option<Number>,
    #[serde(rename = "RateVal", skip_serializing_if = "Option::is_none")]
    pub rate_val: Option<PercentageRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateOrAbsoluteValue1Choice {
    #[serde(flatten)]
    pub value: RateOrAbsoluteValue1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Term1 {
    #[serde(rename = "Oprtr")]
    pub oprtr: Operator1Code,
    #[serde(rename = "Val")]
    pub val: RateOrAbsoluteValue1Choice,
}
#[derive(
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
pub struct FormOfSecurity8ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FormOfSecurity1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FormOfSecurity8Choice {
    #[serde(flatten)]
    pub value: FormOfSecurity8ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementType3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementType3Choice {
    #[serde(flatten)]
    pub value: SettlementType3ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Operator1Code {
    #[serde(rename = "SMAL")]
    Smal,
    #[serde(rename = "SMEQ")]
    Smeq,
    #[serde(rename = "GREA")]
    Grea,
    #[serde(rename = "GREQ")]
    Greq,
    #[serde(rename = "EQAL")]
    Eqal,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecurityStatus2Code {
    #[serde(rename = "ACTV")]
    Actv,
    #[serde(rename = "INAC")]
    Inac,
    #[serde(rename = "SUSP")]
    Susp,
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
pub struct SecurityStatus3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SecurityStatus2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityStatus3Choice {
    #[serde(flatten)]
    pub value: SecurityStatus3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CfiOct2015Identifier {
    #[validate(regex = "CFI_OCT_2015_IDENTIFIER_REGEX")]
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
    #[serde(rename = "SctyCreReq")]
    pub scty_cre_req: SecurityCreationRequestV01<A>,
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
pub struct Max140Text {
    #[validate(length(min = 1, max = 140,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestorType1Code {
    #[serde(rename = "RETL")]
    Retl,
    #[serde(rename = "PROF")]
    Prof,
    #[serde(rename = "STAF")]
    Staf,
    #[serde(rename = "PPER")]
    Pper,
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
pub struct SecurityWithHoldingTax1 {
    #[serde(rename = "WhldgTaxVal")]
    pub whldg_tax_val: RateAndAmountFormat1Choice,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TimeUnit1Code {
    #[serde(rename = "DAYC")]
    Dayc,
    #[serde(rename = "HOUR")]
    Hour,
    #[serde(rename = "MINU")]
    Minu,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "SECO")]
    Seco,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "YEAR")]
    Year,
    #[default]
    Unknown,
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
pub struct PartyIdentification136 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification120Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LegalRestrictions2Code {
    #[serde(rename = "JURO")]
    Juro,
    #[serde(rename = "PPLA")]
    Ppla,
    #[serde(rename = "ACRI")]
    Acri,
    #[serde(rename = "MARG")]
    Marg,
    #[serde(rename = "PRIV")]
    Priv,
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
pub struct MaturityRedemptionType3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MaturityRedemptionType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MaturityRedemptionType3Choice {
    #[serde(flatten)]
    pub value: MaturityRedemptionType3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionType1Code {
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "PUTO")]
    Puto,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityRestriction3 {
    #[serde(rename = "FctvPrd", skip_serializing_if = "Option::is_none")]
    pub fctv_prd: Option<DateTimePeriod2>,
    #[serde(rename = "RstrctnTp", skip_serializing_if = "Option::is_none")]
    pub rstrctn_tp: Option<SecurityRestrictionType2Choice>,
    #[serde(rename = "LglRstrctnTp", skip_serializing_if = "Option::is_none")]
    pub lgl_rstrctn_tp: Option<LegalRestrictions5Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InvstrRstrctnTp", default)]
    pub invstr_rstrctn_tp: Vec<InvestorRestrictionType3Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InvstrTp", default)]
    pub invstr_tp: Vec<InvestorType3Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveCurrencyAnd13DecimalAmount {
    #[serde(rename = "ActiveCurrencyAnd13DecimalAmount")]
    pub value: ActiveCurrencyAnd13DecimalAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveCurrencyCode,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnitOfMeasure9Code {
    #[serde(rename = "BAGG")]
    Bagg,
    #[serde(rename = "BALE")]
    Bale,
    #[serde(rename = "BOTL")]
    Botl,
    #[serde(rename = "BOXX")]
    Boxx,
    #[serde(rename = "CRTN")]
    Crtn,
    #[serde(rename = "CELI")]
    Celi,
    #[serde(rename = "CMET")]
    Cmet,
    #[serde(rename = "CNTR")]
    Cntr,
    #[serde(rename = "CRAT")]
    Crat,
    #[serde(rename = "CBIN")]
    Cbin,
    #[serde(rename = "CBME")]
    Cbme,
    #[serde(rename = "CBML")]
    Cbml,
    #[serde(rename = "PIEC")]
    Piec,
    #[serde(rename = "FOOT")]
    Foot,
    #[serde(rename = "GBFO")]
    Gbfo,
    #[serde(rename = "GBGA")]
    Gbga,
    #[serde(rename = "GBPI")]
    Gbpi,
    #[serde(rename = "GBQA")]
    Gbqa,
    #[serde(rename = "GBTN")]
    Gbtn,
    #[serde(rename = "GRAM")]
    Gram,
    #[serde(rename = "INCH")]
    Inch,
    #[serde(rename = "KILO")]
    Kilo,
    #[serde(rename = "KMET")]
    Kmet,
    #[serde(rename = "LITR")]
    Litr,
    #[serde(rename = "METR")]
    Metr,
    #[serde(rename = "TONE")]
    Tone,
    #[serde(rename = "MILE")]
    Mile,
    #[serde(rename = "MMET")]
    Mmet,
    #[serde(rename = "MILI")]
    Mili,
    #[serde(rename = "PUND")]
    Pund,
    #[serde(rename = "USOU")]
    Usou,
    #[serde(rename = "SCMT")]
    Scmt,
    #[serde(rename = "SQFO")]
    Sqfo,
    #[serde(rename = "SQIN")]
    Sqin,
    #[serde(rename = "SQKI")]
    Sqki,
    #[serde(rename = "SMET")]
    Smet,
    #[serde(rename = "SQMI")]
    Sqmi,
    #[serde(rename = "SMIL")]
    Smil,
    #[serde(rename = "SQYA")]
    Sqya,
    #[serde(rename = "USBA")]
    Usba,
    #[serde(rename = "USFO")]
    Usfo,
    #[serde(rename = "USGA")]
    Usga,
    #[serde(rename = "USPI")]
    Uspi,
    #[serde(rename = "USQA")]
    Usqa,
    #[serde(rename = "USTN")]
    Ustn,
    #[serde(rename = "YARD")]
    Yard,
    #[serde(rename = "GBOU")]
    Gbou,
    #[serde(rename = "ACRE")]
    Acre,
    #[serde(rename = "ARES")]
    Ares,
    #[serde(rename = "HECT")]
    Hect,
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
pub struct Price8 {
    #[serde(rename = "ValTp", skip_serializing_if = "Option::is_none")]
    pub val_tp: Option<PriceValueType3Code>,
    #[serde(rename = "Val")]
    pub val: PriceRateOrAmount3Choice,
    #[serde(rename = "PricTp", skip_serializing_if = "Option::is_none")]
    pub pric_tp: Option<TypeOfPrice1Code>,
}
#[derive(
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
pub struct SecurityAttributes10 {
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: SecurityIdentification39,
    #[validate(length(min = 0,))]
    #[serde(rename = "FinInstrmTp", default)]
    pub fin_instrm_tp: Vec<FinancialInstrument97>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FinInstrmAttrbts", default)]
    pub fin_instrm_attrbts: Vec<CommonFinancialInstrumentAttributes10>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AssignmentMethod1Code {
    #[serde(rename = "RAND")]
    Rand,
    #[serde(rename = "PROR")]
    Pror,
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
pub struct Standardisation3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Standardisation1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Standardisation3Choice {
    #[serde(flatten)]
    pub value: Standardisation3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Isin2021Identifier {
    #[validate(regex = "ISIN_2021_IDENTIFIER_REGEX")]
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
pub struct PercentageRate {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RateType12Code {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "NILP")]
    Nilp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Standardisation1Code {
    #[serde(rename = "FLEX")]
    Flex,
    #[serde(rename = "NSTA")]
    Nsta,
    #[serde(rename = "STAN")]
    Stan,
    #[default]
    Unknown,
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
pub struct BenchmarkCurve6 {
    #[serde(rename = "Sprd", skip_serializing_if = "Option::is_none")]
    pub sprd: Option<DecimalNumber>,
    #[serde(rename = "BchmkId", skip_serializing_if = "Option::is_none")]
    pub bchmk_id: Option<SecurityIdentification39>,
    #[serde(rename = "BchmkPric", skip_serializing_if = "Option::is_none")]
    pub bchmk_pric: Option<Price8>,
    #[serde(rename = "BchmkCrvCcy", skip_serializing_if = "Option::is_none")]
    pub bchmk_crv_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "BchmkCrvNm", skip_serializing_if = "Option::is_none")]
    pub bchmk_crv_nm: Option<BenchmarkCurveName7Choice>,
    #[serde(rename = "BchmkCrvPt", skip_serializing_if = "Option::is_none")]
    pub bchmk_crv_pt: Option<Max256Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CalculationType3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CalculationType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CalculationType3Choice {
    #[serde(flatten)]
    pub value: CalculationType3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommunicationAddress3 {
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Max256Text>,
    #[serde(rename = "Phne", skip_serializing_if = "Option::is_none")]
    pub phne: Option<PhoneNumber>,
    #[serde(rename = "Mob", skip_serializing_if = "Option::is_none")]
    pub mob: Option<PhoneNumber>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<PhoneNumber>,
    #[serde(rename = "TlxAdr", skip_serializing_if = "Option::is_none")]
    pub tlx_adr: Option<Max35Text>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max256Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification13 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max4AlphaNumericText,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionType31ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SecuritiesTransactionType11Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionType31Choice {
    #[serde(flatten)]
    pub value: SecuritiesTransactionType31ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentDirectionIndicator {
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
pub struct SecurityIdentification39 {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<Isin2021Identifier>,
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
pub struct RateType12FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RateType12Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateType12FormatChoice {
    #[serde(flatten)]
    pub value: RateType12FormatChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CalculationType1Code {
    #[serde(rename = "AFTX")]
    Aftx,
    #[serde(rename = "ANNU")]
    Annu,
    #[serde(rename = "ISSU")]
    Issu,
    #[serde(rename = "AVMA")]
    Avma,
    #[serde(rename = "BOOK")]
    Book,
    #[serde(rename = "YTNC")]
    Ytnc,
    #[serde(rename = "CHCL")]
    Chcl,
    #[serde(rename = "CLOS")]
    Clos,
    #[serde(rename = "CMPD")]
    Cmpd,
    #[serde(rename = "CUYI")]
    Cuyi,
    #[serde(rename = "TRGR")]
    Trgr,
    #[serde(rename = "GVEQ")]
    Gveq,
    #[serde(rename = "FLAS")]
    Flas,
    #[serde(rename = "NVFL")]
    Nvfl,
    #[serde(rename = "LSCL")]
    Lscl,
    #[serde(rename = "LSMT")]
    Lsmt,
    #[serde(rename = "LSQR")]
    Lsqr,
    #[serde(rename = "LSYR")]
    Lsyr,
    #[serde(rename = "LGAL")]
    Lgal,
    #[serde(rename = "MARK")]
    Mark,
    #[serde(rename = "YTMA")]
    Ytma,
    #[serde(rename = "NXRF")]
    Nxrf,
    #[serde(rename = "PNAV")]
    Pnav,
    #[serde(rename = "NXPT")]
    Nxpt,
    #[serde(rename = "PRCL")]
    Prcl,
    #[serde(rename = "PRYL")]
    Pryl,
    #[serde(rename = "SEMI")]
    Semi,
    #[serde(rename = "SHLF")]
    Shlf,
    #[serde(rename = "SPLL")]
    Spll,
    #[serde(rename = "TXQV")]
    Txqv,
    #[serde(rename = "TTDT")]
    Ttdt,
    #[serde(rename = "TRYL")]
    Tryl,
    #[serde(rename = "WRST")]
    Wrst,
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
pub struct MicIdentifier {
    #[validate(regex = "MIC_IDENTIFIER_REGEX")]
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
pub struct BaseOneRate {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecuritiesPaymentStatus1Code {
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "NILL")]
    Nill,
    #[serde(rename = "PART")]
    Part,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementType1Code {
    #[serde(rename = "PRIN")]
    Prin,
    #[serde(rename = "NETO")]
    Neto,
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
pub struct PriceValue1 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
}
#[derive(
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
pub enum TefraRules1Code {
    #[serde(rename = "RULC")]
    Rulc,
    #[serde(rename = "RULD")]
    Ruld,
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
pub struct Organisation38 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max140Text,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<PartyIdentification177Choice>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    pub purp: Option<Max35Text>,
    #[serde(rename = "TaxtnCtry", skip_serializing_if = "Option::is_none")]
    pub taxtn_ctry: Option<CountryCode>,
    #[serde(rename = "RegnCtry", skip_serializing_if = "Option::is_none")]
    pub regn_ctry: Option<CountryCode>,
    #[serde(rename = "RegnDt", skip_serializing_if = "Option::is_none")]
    pub regn_dt: Option<IsoDate>,
    #[serde(rename = "TaxIdNb", skip_serializing_if = "Option::is_none")]
    pub tax_id_nb: Option<Max35Text>,
    #[serde(rename = "NtlRegnNb", skip_serializing_if = "Option::is_none")]
    pub ntl_regn_nb: Option<Max35Text>,
    #[validate(length(min = 1, max = 5,))]
    #[serde(rename = "PstlAdr", default)]
    pub pstl_adr: Vec<PostalAddress3>,
    #[serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none")]
    pub pmry_com_adr: Option<CommunicationAddress3>,
    #[serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none")]
    pub scndry_com_adr: Option<CommunicationAddress3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeTransactionCondition7ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TradeTransactionCondition2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeTransactionCondition7Choice {
    #[serde(flatten)]
    pub value: TradeTransactionCondition7ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestorRestrictionType1Code {
    #[serde(rename = "LERE")]
    Lere,
    #[serde(rename = "CITI")]
    Citi,
    #[serde(rename = "INDV")]
    Indv,
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
pub struct LegalRestrictions4ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<LegalRestrictions1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LegalRestrictions4Choice {
    #[serde(flatten)]
    pub value: LegalRestrictions4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CallType1Code {
    #[serde(rename = "LOTT")]
    Lott,
    #[serde(rename = "PRTA")]
    Prta,
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
pub struct UnderlyingAttributes4 {
    #[serde(rename = "AllcnPctg", skip_serializing_if = "Option::is_none")]
    pub allcn_pctg: Option<PercentageRate>,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<UnitOrFaceAmount1Choice>,
    #[serde(rename = "SttlmTp", skip_serializing_if = "Option::is_none")]
    pub sttlm_tp: Option<SettlementType3Choice>,
    #[serde(rename = "CshAmt", skip_serializing_if = "Option::is_none")]
    pub csh_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "CshTp", skip_serializing_if = "Option::is_none")]
    pub csh_tp: Option<Max35Text>,
    #[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
    pub pric: Option<Price8>,
    #[serde(rename = "DrtyPric", skip_serializing_if = "Option::is_none")]
    pub drty_pric: Option<Price8>,
    #[serde(rename = "EndPric", skip_serializing_if = "Option::is_none")]
    pub end_pric: Option<Price8>,
    #[serde(rename = "StartVal", skip_serializing_if = "Option::is_none")]
    pub start_val: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "CurVal", skip_serializing_if = "Option::is_none")]
    pub cur_val: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "EndVal", skip_serializing_if = "Option::is_none")]
    pub end_val: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "AdjstdQty", skip_serializing_if = "Option::is_none")]
    pub adjstd_qty: Option<UnitOrFaceAmount1Choice>,
    #[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
    pub xchg_rate: Option<PercentageRate>,
    #[serde(rename = "CapVal", skip_serializing_if = "Option::is_none")]
    pub cap_val: Option<ActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentForm2 {
    #[serde(rename = "BookgApprnc", skip_serializing_if = "Option::is_none")]
    pub bookg_apprnc: Option<Appearance3Choice>,
    #[serde(rename = "LglForm", skip_serializing_if = "Option::is_none")]
    pub lgl_form: Option<FormOfSecurity8Choice>,
}
#[derive(
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
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
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
pub struct Future4 {
    #[serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none")]
    pub ctrct_sz: Option<BaseOneRate>,
    #[serde(rename = "ExrcPric", skip_serializing_if = "Option::is_none")]
    pub exrc_pric: Option<Price8>,
    #[serde(rename = "FutrDt", skip_serializing_if = "Option::is_none")]
    pub futr_dt: Option<IsoDateTime>,
    #[serde(rename = "MinSz", skip_serializing_if = "Option::is_none")]
    pub min_sz: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
    pub unit_of_measr: Option<UnitOfMeasure7Choice>,
    #[serde(rename = "TmUnit", skip_serializing_if = "Option::is_none")]
    pub tm_unit: Option<TimeUnit3Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlUndrlygAttrbts", default)]
    pub addtl_undrlyg_attrbts: Vec<UnderlyingAttributes4>,
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
pub struct Exact4AlphaNumericText {
    #[validate(regex = "EXACT_4_ALPHA_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RestrictionType1Code {
    #[serde(rename = "SELR")]
    Selr,
    #[serde(rename = "BUYR")]
    Buyr,
    #[serde(rename = "PLAR")]
    Plar,
    #[serde(rename = "HOLR")]
    Holr,
    #[serde(rename = "VOTR")]
    Votr,
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
pub struct InstrumentSubStructureType2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InstrumentSubStructureType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InstrumentSubStructureType2Choice {
    #[serde(flatten)]
    pub value: InstrumentSubStructureType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Jurisdiction1 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max70Text>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RateAndAmountFormat1ChoiceEnum {
    #[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rate: Option<RateType12FormatChoice>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
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
pub struct RateAndAmountFormat1Choice {
    #[serde(flatten)]
    pub value: RateAndAmountFormat1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Warrant4 {
    #[serde(rename = "Mltplr", skip_serializing_if = "Option::is_none")]
    pub mltplr: Option<BaseOneRate>,
    #[serde(rename = "SbcptPric", skip_serializing_if = "Option::is_none")]
    pub sbcpt_pric: Option<Price8>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<WarrantStyle3Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "WarrtAgt", default)]
    pub warrt_agt: Vec<Organisation38>,
}
#[derive(
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
pub struct ExternalFinancialInstrumentIdentificationType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Operation1Code {
    #[serde(rename = "TILL")]
    Till,
    #[serde(rename = "ORRR")]
    Orrr,
    #[serde(rename = "ANDD")]
    Andd,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AddressType1Code {
    #[serde(rename = "HOME")]
    Home,
    #[serde(rename = "BIZZ")]
    Bizz,
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
pub struct Derivative4 {
    #[serde(rename = "Futr", skip_serializing_if = "Option::is_none")]
    pub futr: Option<Future4>,
    #[serde(rename = "Optn", skip_serializing_if = "Option::is_none")]
    pub optn: Option<Option15>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementInformation17 {
    #[serde(rename = "SctiesQtyTp", skip_serializing_if = "Option::is_none")]
    pub scties_qty_tp: Option<SettlementUnitType3Choice>,
    #[serde(rename = "CtrctSttlmMnth", skip_serializing_if = "Option::is_none")]
    pub ctrct_sttlm_mnth: Option<IsoYearMonth>,
    #[serde(rename = "MinDnmtn", skip_serializing_if = "Option::is_none")]
    pub min_dnmtn: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "MinMltplQty", skip_serializing_if = "Option::is_none")]
    pub min_mltpl_qty: Option<FinancialInstrumentQuantity1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DevtgSttlmUnit", default)]
    pub devtg_sttlm_unit: Vec<FinancialInstrumentQuantity1Choice>,
}
#[derive(
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
pub struct CallType3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CallType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CallType3Choice {
    #[serde(flatten)]
    pub value: CallType3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LegalRestrictions1Code {
    #[serde(rename = "USLE")]
    Usle,
    #[serde(rename = "NORE")]
    Nore,
    #[serde(rename = "REST")]
    Rest,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementUnitType1Code {
    #[serde(rename = "FAMT")]
    Famt,
    #[serde(rename = "UNIT")]
    Unit,
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
pub struct PreferenceToIncome5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PreferenceToIncome1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PreferenceToIncome5Choice {
    #[serde(flatten)]
    pub value: PreferenceToIncome5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountOrPercentageRange1 {
    #[serde(rename = "Opr", skip_serializing_if = "Option::is_none")]
    pub opr: Option<Operation1Code>,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "Term", default)]
    pub term: Vec<Term1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GlobalNote1Code {
    #[serde(rename = "NGNO")]
    Ngno,
    #[serde(rename = "CGNO")]
    Cgno,
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
pub struct OptionType8ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionType8Choice {
    #[serde(flatten)]
    pub value: OptionType8ChoiceEnum,
}
#[derive(
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
pub struct OptionStyle1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OptionStyle1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OptionStyle1Choice {
    #[serde(flatten)]
    pub value: OptionStyle1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommonFinancialInstrumentAttributes10 {
    #[serde(rename = "SctySts", skip_serializing_if = "Option::is_none")]
    pub scty_sts: Option<SecurityStatus3Choice>,
    #[serde(rename = "ISOSctyLngNm", skip_serializing_if = "Option::is_none")]
    pub iso_scty_lng_nm: Option<Max350Text>,
    #[serde(rename = "ISOSctyShrtNm", skip_serializing_if = "Option::is_none")]
    pub iso_scty_shrt_nm: Option<Max35Text>,
    #[serde(rename = "NmVldFr", skip_serializing_if = "Option::is_none")]
    pub nm_vld_fr: Option<DateAndDateTime2Choice>,
    #[serde(rename = "DnmtnCcy")]
    pub dnmtn_ccy: ActiveOrHistoricCurrencyCode,
    #[serde(rename = "CertNb", skip_serializing_if = "Option::is_none")]
    pub cert_nb: Option<Max35Text>,
    #[serde(rename = "CtrctVrsnNb", skip_serializing_if = "Option::is_none")]
    pub ctrct_vrsn_nb: Option<Number>,
    #[serde(rename = "CpnAttchdNb", skip_serializing_if = "Option::is_none")]
    pub cpn_attchd_nb: Option<Max3NumericText>,
    #[serde(rename = "TaxLotNb", skip_serializing_if = "Option::is_none")]
    pub tax_lot_nb: Option<Max15NumericText>,
    #[serde(rename = "PoolNb", skip_serializing_if = "Option::is_none")]
    pub pool_nb: Option<Max15NumericText>,
    #[serde(rename = "CvrdInd", skip_serializing_if = "Option::is_none")]
    pub cvrd_ind: Option<YesNoIndicator>,
    #[serde(rename = "LglRstrctns", skip_serializing_if = "Option::is_none")]
    pub lgl_rstrctns: Option<LegalRestrictions4Choice>,
    #[serde(rename = "PosLmt", skip_serializing_if = "Option::is_none")]
    pub pos_lmt: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "NearTermPosLmt", skip_serializing_if = "Option::is_none")]
    pub near_term_pos_lmt: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "ListgDt", skip_serializing_if = "Option::is_none")]
    pub listg_dt: Option<IsoDate>,
    #[serde(rename = "RcrdDt", skip_serializing_if = "Option::is_none")]
    pub rcrd_dt: Option<IsoDateTime>,
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<IsoDate>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    pub purp: Option<Max256Text>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<ClassificationType2>,
    #[serde(rename = "Issnc", skip_serializing_if = "Option::is_none")]
    pub issnc: Option<Issuance5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TradgMkt", default)]
    pub tradg_mkt: Vec<TradingParameters2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SprdAndBchmkCrv", default)]
    pub sprd_and_bchmk_crv: Vec<BenchmarkCurve6>,
    #[serde(rename = "PutTp", skip_serializing_if = "Option::is_none")]
    pub put_tp: Option<PutType3Choice>,
    #[serde(rename = "CallTp", skip_serializing_if = "Option::is_none")]
    pub call_tp: Option<CallType3Choice>,
    #[serde(rename = "FngbInd", skip_serializing_if = "Option::is_none")]
    pub fngb_ind: Option<YesNoIndicator>,
    #[serde(rename = "Cnfdtl", skip_serializing_if = "Option::is_none")]
    pub cnfdtl: Option<YesNoIndicator>,
    #[serde(rename = "PrvtPlcmnt", skip_serializing_if = "Option::is_none")]
    pub prvt_plcmnt: Option<YesNoIndicator>,
    #[serde(rename = "ConvtblInd", skip_serializing_if = "Option::is_none")]
    pub convtbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "ConvsPrd", skip_serializing_if = "Option::is_none")]
    pub convs_prd: Option<DateTimePeriod1>,
    #[serde(rename = "ConvsRatioNmrtr", skip_serializing_if = "Option::is_none")]
    pub convs_ratio_nmrtr: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "ConvsRatioDnmtr", skip_serializing_if = "Option::is_none")]
    pub convs_ratio_dnmtr: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "PmryPlcOfDpst", skip_serializing_if = "Option::is_none")]
    pub pmry_plc_of_dpst: Option<PartyIdentification136>,
    #[serde(rename = "TradgMtd", skip_serializing_if = "Option::is_none")]
    pub tradg_mtd: Option<UnitOrFaceAmount1Choice>,
    #[serde(rename = "TEFRARule", skip_serializing_if = "Option::is_none")]
    pub tefra_rule: Option<TefraRules3Choice>,
    #[serde(rename = "SrNb", skip_serializing_if = "Option::is_none")]
    pub sr_nb: Option<Max16Text>,
    #[serde(rename = "Clss", skip_serializing_if = "Option::is_none")]
    pub clss: Option<Max16Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "WhldgTaxRgm", default)]
    pub whldg_tax_rgm: Vec<SecurityWithHoldingTax1>,
    #[serde(rename = "PmtSts", skip_serializing_if = "Option::is_none")]
    pub pmt_sts: Option<SecuritiesPaymentStatus5Choice>,
    #[serde(rename = "InitlPhysForm", skip_serializing_if = "Option::is_none")]
    pub initl_phys_form: Option<InitialPhysicalForm4Choice>,
    #[serde(rename = "AftrXchgPhysForm", skip_serializing_if = "Option::is_none")]
    pub aftr_xchg_phys_form: Option<InitialPhysicalForm3Choice>,
    #[serde(rename = "CmonSfkpr", skip_serializing_if = "Option::is_none")]
    pub cmon_sfkpr: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "RedTp", skip_serializing_if = "Option::is_none")]
    pub red_tp: Option<MaturityRedemptionType3Choice>,
    #[serde(rename = "RedPmtCcy", skip_serializing_if = "Option::is_none")]
    pub red_pmt_ccy: Option<ActiveCurrencyCode>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Rstrctn", default)]
    pub rstrctn: Vec<SecurityRestriction3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SttlmInf", default)]
    pub sttlm_inf: Vec<SettlementInformation17>,
    #[serde(rename = "FinInstrmForm", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_form: Option<FinancialInstrumentForm2>,
    #[serde(rename = "CtctNm", skip_serializing_if = "Option::is_none")]
    pub ctct_nm: Option<Organisation38>,
    #[serde(rename = "LeadMgr", skip_serializing_if = "Option::is_none")]
    pub lead_mgr: Option<Organisation38>,
    #[serde(rename = "PrncplPngAgt", skip_serializing_if = "Option::is_none")]
    pub prncpl_png_agt: Option<Organisation38>,
    #[serde(rename = "PngAgt", skip_serializing_if = "Option::is_none")]
    pub png_agt: Option<Organisation38>,
    #[serde(rename = "Dpstry", skip_serializing_if = "Option::is_none")]
    pub dpstry: Option<Organisation38>,
    #[serde(rename = "UndrlygRsk", skip_serializing_if = "Option::is_none")]
    pub undrlyg_rsk: Option<Organisation38>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InstrumentSubStructureType1Code {
    #[serde(rename = "ABSE")]
    Abse,
    #[serde(rename = "AIRT")]
    Airt,
    #[serde(rename = "AUTT")]
    Autt,
    #[serde(rename = "CBOB")]
    Cbob,
    #[serde(rename = "CDOB")]
    Cdob,
    #[serde(rename = "CLNO")]
    Clno,
    #[serde(rename = "CLOB")]
    Clob,
    #[serde(rename = "CMBS")]
    Cmbs,
    #[serde(rename = "CSMR")]
    Csmr,
    #[serde(rename = "CRCT")]
    Crct,
    #[serde(rename = "HELO")]
    Helo,
    #[serde(rename = "LPNO")]
    Lpno,
    #[serde(rename = "PFAB")]
    Pfab,
    #[serde(rename = "PYRT")]
    Pyrt,
    #[serde(rename = "REPK")]
    Repk,
    #[serde(rename = "RMBS")]
    Rmbs,
    #[serde(rename = "SCBO")]
    Scbo,
    #[serde(rename = "STRB")]
    Strb,
    #[serde(rename = "STUT")]
    Stut,
    #[serde(rename = "WBSE")]
    Wbse,
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
pub struct MessageHeader1 {
    #[validate]
    #[serde(rename = "MsgId")]
    pub msg_id: Max35Text,
    #[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm: Option<IsoDateTime>,
}
#[derive(
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
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
}
#[derive(
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
pub struct AssignmentMethod2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AssignmentMethod1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AssignmentMethod2Choice {
    #[serde(flatten)]
    pub value: AssignmentMethod2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateTimePeriod1 {
    #[validate]
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "ToDtTm")]
    pub to_dt_tm: IsoDateTime,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalFinancialInstrumentProductType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionStyle1Code {
    #[serde(rename = "AMER")]
    Amer,
    #[serde(rename = "EURO")]
    Euro,
    #[serde(rename = "BERM")]
    Berm,
    #[serde(rename = "ASIA")]
    Asia,
    #[serde(rename = "CANA")]
    Cana,
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
pub struct SecuritiesPaymentStatus5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SecuritiesPaymentStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesPaymentStatus5Choice {
    #[serde(flatten)]
    pub value: SecuritiesPaymentStatus5ChoiceEnum,
}
#[derive(
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
pub struct LegalRestrictions5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<LegalRestrictions2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LegalRestrictions5Choice {
    #[serde(flatten)]
    pub value: LegalRestrictions5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityRestrictionType2ChoiceEnum {
    #[serde(rename = "RstrctnTp", skip_serializing_if = "Option::is_none")]
    pub rstrctn_tp: Option<RestrictionType1Code>,
    #[serde(rename = "PrtryRstrctn", skip_serializing_if = "Option::is_none")]
    pub prtry_rstrctn: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityRestrictionType2Choice {
    #[serde(flatten)]
    pub value: SecurityRestrictionType2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InterestType3Code {
    #[serde(rename = "ZCPN")]
    Zcpn,
    #[serde(rename = "FIXD")]
    Fixd,
    #[serde(rename = "FLRN")]
    Flrn,
    #[serde(rename = "DUAL")]
    Dual,
    #[serde(rename = "INDE")]
    Inde,
    #[serde(rename = "DSCO")]
    Dsco,
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
pub struct FinancialInstrument97 {
    #[serde(rename = "Eqty", skip_serializing_if = "Option::is_none")]
    pub eqty: Option<Equity3>,
    #[serde(rename = "Warrt", skip_serializing_if = "Option::is_none")]
    pub warrt: Option<Warrant4>,
    #[serde(rename = "Debt", skip_serializing_if = "Option::is_none")]
    pub debt: Option<Debt5>,
    #[serde(rename = "Deriv", skip_serializing_if = "Option::is_none")]
    pub deriv: Option<Derivative4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TimeUnit3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TimeUnit1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TimeUnit3Choice {
    #[serde(flatten)]
    pub value: TimeUnit3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct WarrantStyle3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<WarrantStyle1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct WarrantStyle3Choice {
    #[serde(flatten)]
    pub value: WarrantStyle3ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum WarrantStyle1Code {
    #[serde(rename = "AMER")]
    Amer,
    #[serde(rename = "EURO")]
    Euro,
    #[serde(rename = "BERM")]
    Berm,
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
pub struct InitialPhysicalForm3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InitialPhysicalForm2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InitialPhysicalForm3Choice {
    #[serde(flatten)]
    pub value: InitialPhysicalForm3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Appearance3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Appearance1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Appearance3Choice {
    #[serde(flatten)]
    pub value: Appearance3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettleStyle2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettleStyle1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettleStyle2Choice {
    #[serde(flatten)]
    pub value: SettleStyle2ChoiceEnum,
}
#[derive(
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
pub struct ClassificationType2 {
    #[serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none")]
    pub clssfctn_fin_instrm: Option<CfiOct2015Identifier>,
    #[serde(rename = "FinInstrmPdctTpCd", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_pdct_tp_cd: Option<ExternalFinancialInstrumentProductType1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AltrnClssfctn", default)]
    pub altrn_clssfctn: Vec<GenericIdentification36>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Debt5 {
    #[serde(rename = "PmtCcy", skip_serializing_if = "Option::is_none")]
    pub pmt_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none")]
    pub pmt_frqcy: Option<Frequency35Choice>,
    #[serde(rename = "IntrstFxgDt", skip_serializing_if = "Option::is_none")]
    pub intrst_fxg_dt: Option<IsoDateTime>,
    #[serde(rename = "DtdDt", skip_serializing_if = "Option::is_none")]
    pub dtd_dt: Option<IsoDateTime>,
    #[serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none")]
    pub frst_pmt_dt: Option<IsoDateTime>,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<IsoDateTime>,
    #[serde(rename = "NxtCpnDt", skip_serializing_if = "Option::is_none")]
    pub nxt_cpn_dt: Option<IsoDateTime>,
    #[serde(rename = "PutblDt", skip_serializing_if = "Option::is_none")]
    pub putbl_dt: Option<IsoDateTime>,
    #[serde(rename = "NxtCllblDt", skip_serializing_if = "Option::is_none")]
    pub nxt_cllbl_dt: Option<IsoDateTime>,
    #[serde(rename = "NxtFctrDt", skip_serializing_if = "Option::is_none")]
    pub nxt_fctr_dt: Option<IsoDateTime>,
    #[serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none")]
    pub xprtn_dt: Option<IsoDateTime>,
    #[serde(rename = "PmtDrctnInd", skip_serializing_if = "Option::is_none")]
    pub pmt_drctn_ind: Option<PaymentDirectionIndicator>,
    #[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
    pub intrst_rate: Option<PercentageRate>,
    #[serde(rename = "NxtIntrstRate", skip_serializing_if = "Option::is_none")]
    pub nxt_intrst_rate: Option<PercentageRate>,
    #[serde(rename = "OddCpnInd", skip_serializing_if = "Option::is_none")]
    pub odd_cpn_ind: Option<YesNoIndicator>,
    #[serde(rename = "CllblInd", skip_serializing_if = "Option::is_none")]
    pub cllbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "CPPrgm", skip_serializing_if = "Option::is_none")]
    pub cp_prgm: Option<Number>,
    #[serde(rename = "CPRegnTp", skip_serializing_if = "Option::is_none")]
    pub cp_regn_tp: Option<Max350Text>,
    #[serde(rename = "IntrstAcrlDt", skip_serializing_if = "Option::is_none")]
    pub intrst_acrl_dt: Option<IsoDateTime>,
    #[serde(rename = "PutblInd", skip_serializing_if = "Option::is_none")]
    pub putbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "PreFnddInd", skip_serializing_if = "Option::is_none")]
    pub pre_fndd_ind: Option<YesNoIndicator>,
    #[serde(rename = "EscrwdInd", skip_serializing_if = "Option::is_none")]
    pub escrwd_ind: Option<YesNoIndicator>,
    #[serde(rename = "PerptlInd", skip_serializing_if = "Option::is_none")]
    pub perptl_ind: Option<YesNoIndicator>,
    #[serde(rename = "SubrdntdInd", skip_serializing_if = "Option::is_none")]
    pub subrdntd_ind: Option<YesNoIndicator>,
    #[serde(rename = "XtndblInd", skip_serializing_if = "Option::is_none")]
    pub xtndbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "XtndblPrd", skip_serializing_if = "Option::is_none")]
    pub xtndbl_prd: Option<DateTimePeriod1Choice>,
    #[serde(rename = "VarblRateInd", skip_serializing_if = "Option::is_none")]
    pub varbl_rate_ind: Option<YesNoIndicator>,
    #[serde(rename = "OverAlltmtAmt", skip_serializing_if = "Option::is_none")]
    pub over_alltmt_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "OverAlltmtRate", skip_serializing_if = "Option::is_none")]
    pub over_alltmt_rate: Option<PercentageRate>,
    #[serde(rename = "AmtsblInd", skip_serializing_if = "Option::is_none")]
    pub amtsbl_ind: Option<YesNoIndicator>,
    #[serde(rename = "IntrstClctnMtd", skip_serializing_if = "Option::is_none")]
    pub intrst_clctn_mtd: Option<Max70Text>,
    #[serde(rename = "CptlsdIntrst", skip_serializing_if = "Option::is_none")]
    pub cptlsd_intrst: Option<DistributionPolicy2Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ActlDnmtnAmt", default)]
    pub actl_dnmtn_amt: Vec<ActiveCurrencyAndAmount>,
    #[serde(rename = "CurFctr", skip_serializing_if = "Option::is_none")]
    pub cur_fctr: Option<PercentageRate>,
    #[serde(rename = "NxtFctr", skip_serializing_if = "Option::is_none")]
    pub nxt_fctr: Option<PercentageRate>,
    #[serde(rename = "PrvsFctr", skip_serializing_if = "Option::is_none")]
    pub prvs_fctr: Option<PercentageRate>,
    #[serde(rename = "Pcs", skip_serializing_if = "Option::is_none")]
    pub pcs: Option<DecimalNumber>,
    #[serde(rename = "PlsMax", skip_serializing_if = "Option::is_none")]
    pub pls_max: Option<DecimalNumber>,
    #[serde(rename = "PlsPerMln", skip_serializing_if = "Option::is_none")]
    pub pls_per_mln: Option<DecimalNumber>,
    #[serde(rename = "PlsPerLot", skip_serializing_if = "Option::is_none")]
    pub pls_per_lot: Option<DecimalNumber>,
    #[serde(rename = "PlsPerTrad", skip_serializing_if = "Option::is_none")]
    pub pls_per_trad: Option<DecimalNumber>,
    #[serde(rename = "CstPrePmtPnltyInd", skip_serializing_if = "Option::is_none")]
    pub cst_pre_pmt_pnlty_ind: Option<YesNoIndicator>,
    #[serde(rename = "LotId", skip_serializing_if = "Option::is_none")]
    pub lot_id: Option<Max35Text>,
    #[serde(rename = "CstPrePmtYld", skip_serializing_if = "Option::is_none")]
    pub cst_pre_pmt_yld: Option<PercentageRate>,
    #[serde(rename = "WghtdAvrgCpn", skip_serializing_if = "Option::is_none")]
    pub wghtd_avrg_cpn: Option<PercentageRate>,
    #[serde(rename = "WghtdAvrgLife", skip_serializing_if = "Option::is_none")]
    pub wghtd_avrg_life: Option<DecimalNumber>,
    #[serde(rename = "WghtdAvrgLn", skip_serializing_if = "Option::is_none")]
    pub wghtd_avrg_ln: Option<DecimalNumber>,
    #[serde(rename = "WghtdAvrgMtrty", skip_serializing_if = "Option::is_none")]
    pub wghtd_avrg_mtrty: Option<DecimalNumber>,
    #[serde(rename = "InsrdInd", skip_serializing_if = "Option::is_none")]
    pub insrd_ind: Option<YesNoIndicator>,
    #[serde(rename = "BkQlfdInd", skip_serializing_if = "Option::is_none")]
    pub bk_qlfd_ind: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "YldClctn", default)]
    pub yld_clctn: Vec<YieldCalculation6>,
    #[serde(rename = "IntrstTp", skip_serializing_if = "Option::is_none")]
    pub intrst_tp: Option<InterestType3Code>,
    #[serde(rename = "InstrmStrTp", skip_serializing_if = "Option::is_none")]
    pub instrm_str_tp: Option<InstrumentSubStructureType2Choice>,
    #[serde(rename = "GblTp", skip_serializing_if = "Option::is_none")]
    pub gbl_tp: Option<GlobalNote2Choice>,
    #[serde(
        rename = "PotntlEuroSysElgblty",
        skip_serializing_if = "Option::is_none"
    )]
    pub potntl_euro_sys_elgblty: Option<YesNoIndicator>,
    #[serde(rename = "Geogcs", skip_serializing_if = "Option::is_none")]
    pub geogcs: Option<Max35Text>,
    #[serde(rename = "YldRg", skip_serializing_if = "Option::is_none")]
    pub yld_rg: Option<AmountOrPercentageRange1>,
    #[serde(rename = "CpnRg", skip_serializing_if = "Option::is_none")]
    pub cpn_rg: Option<AmountOrPercentageRange1>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    pub purp: Option<Max256Text>,
    #[serde(rename = "AltrntvMinTaxInd", skip_serializing_if = "Option::is_none")]
    pub altrntv_min_tax_ind: Option<YesNoIndicator>,
    #[serde(rename = "AutoRinvstmt", skip_serializing_if = "Option::is_none")]
    pub auto_rinvstmt: Option<PercentageRate>,
    #[serde(rename = "Hrcut", skip_serializing_if = "Option::is_none")]
    pub hrcut: Option<PercentageRate>,
    #[serde(rename = "TxConds", skip_serializing_if = "Option::is_none")]
    pub tx_conds: Option<TradeTransactionCondition7Choice>,
    #[serde(rename = "LookBck", skip_serializing_if = "Option::is_none")]
    pub look_bck: Option<Number>,
    #[serde(rename = "MaxSbstitn", skip_serializing_if = "Option::is_none")]
    pub max_sbstitn: Option<Number>,
    #[serde(rename = "MinIncrmt", skip_serializing_if = "Option::is_none")]
    pub min_incrmt: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "MinQty", skip_serializing_if = "Option::is_none")]
    pub min_qty: Option<FinancialInstrumentQuantity1Choice>,
    #[serde(rename = "Pdctn", skip_serializing_if = "Option::is_none")]
    pub pdctn: Option<Max35Text>,
    #[serde(rename = "RstrctdInd", skip_serializing_if = "Option::is_none")]
    pub rstrctd_ind: Option<YesNoIndicator>,
    #[serde(rename = "PricFrqcy", skip_serializing_if = "Option::is_none")]
    pub pric_frqcy: Option<Frequency35Choice>,
    #[serde(rename = "Sctr", skip_serializing_if = "Option::is_none")]
    pub sctr: Option<Max35Text>,
    #[serde(rename = "SbstitnFrqcy", skip_serializing_if = "Option::is_none")]
    pub sbstitn_frqcy: Option<Frequency35Choice>,
    #[serde(rename = "SbstitnLft", skip_serializing_if = "Option::is_none")]
    pub sbstitn_lft: Option<Number>,
    #[serde(rename = "WhlPoolInd", skip_serializing_if = "Option::is_none")]
    pub whl_pool_ind: Option<YesNoIndicator>,
    #[serde(rename = "PricSrc", skip_serializing_if = "Option::is_none")]
    pub pric_src: Option<Max35Text>,
    #[serde(rename = "PricRg", skip_serializing_if = "Option::is_none")]
    pub pric_rg: Option<AmountOrPercentageRange1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecuritiesTransactionType11Code {
    #[serde(rename = "NSYN")]
    Nsyn,
    #[serde(rename = "SYND")]
    Synd,
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
pub struct Frequency35ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Frequency5Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Frequency35Choice {
    #[serde(flatten)]
    pub value: Frequency35ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PostalAddress3 {
    #[serde(rename = "AdrTp")]
    pub adr_tp: AddressType1Code,
    #[validate]
    #[serde(rename = "MlngInd")]
    pub mlng_ind: YesNoIndicator,
    #[validate]
    #[serde(rename = "RegnAdrInd")]
    pub regn_adr_ind: YesNoIndicator,
    #[validate]
    #[serde(rename = "NmAndAdr")]
    pub nm_and_adr: NameAndAddress4,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceValueType3Code {
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "PREM")]
    Prem,
    #[serde(rename = "PARV")]
    Parv,
    #[serde(rename = "YIEL")]
    Yiel,
    #[serde(rename = "SPRE")]
    Spre,
    #[serde(rename = "PEUN")]
    Peun,
    #[serde(rename = "ABSO")]
    Abso,
    #[serde(rename = "TEDP")]
    Tedp,
    #[serde(rename = "TEDY")]
    Tedy,
    #[serde(rename = "FICT")]
    Fict,
    #[serde(rename = "VACT")]
    Vact,
    #[default]
    Unknown,
}
