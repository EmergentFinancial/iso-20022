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
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_15_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,15}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.105.001.01".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmount {
    #[serde(rename = "ActiveOrHistoricCurrencyAnd20DecimalAmount")]
    pub value: ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType,
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
pub struct PositionSetDimensions12 {
    #[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
    pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "CollData", skip_serializing_if = "Option::is_none")]
    pub coll_data: Option<CollateralData33>,
    #[serde(rename = "OtlrsIncl", skip_serializing_if = "Option::is_none")]
    pub otlrs_incl: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PostedMarginOrCollateral4 {
    #[serde(rename = "InitlMrgnPstd", skip_serializing_if = "Option::is_none")]
    pub initl_mrgn_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "VartnMrgnPstd", skip_serializing_if = "Option::is_none")]
    pub vartn_mrgn_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "XcssCollPstd", skip_serializing_if = "Option::is_none")]
    pub xcss_coll_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice19ChoiceEnum {
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<LongFraction19DecimalNumber>,
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<PercentageRate>,
    #[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
    pub mntry_val: Option<AmountAndDirection107>,
    #[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
    pub yld: Option<PercentageRate>,
    #[serde(rename = "PdgPric", skip_serializing_if = "Option::is_none")]
    pub pdg_pric: Option<PriceStatus1Code>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<SecuritiesTransactionPrice5>,
    #[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
    pub dcml: Option<BaseOneRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice19Choice {
    #[serde(flatten)]
    pub value: SecuritiesTransactionPrice19ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification175 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max72Text,
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
pub struct IssuerJurisdiction1ChoiceEnum {
    #[serde(rename = "CtryCd", skip_serializing_if = "Option::is_none")]
    pub ctry_cd: Option<CountryCode>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IssuerJurisdiction1Choice {
    #[serde(flatten)]
    pub value: IssuerJurisdiction1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIssuer4 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "JursdctnCtry")]
    pub jursdctn_ctry: CountryCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PositionSet20 {
    #[validate]
    #[serde(rename = "Dmnsns")]
    pub dmnsns: PositionSetDimensions15,
    #[validate]
    #[serde(rename = "Mtrcs")]
    pub mtrcs: PositionSetMetrics10,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesLendingType3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalSecuritiesLendingType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesLendingType3Choice {
    #[serde(flatten)]
    pub value: SecuritiesLendingType3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NamedPosition3 {
    #[validate]
    #[serde(rename = "RefDt")]
    pub ref_dt: IsoDate,
    #[validate(length(min = 0,))]
    #[serde(rename = "GnlInf", default)]
    pub gnl_inf: Vec<PositionSet16>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Ln", default)]
    pub ln: Vec<PositionSet17>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Coll", default)]
    pub coll: Vec<PositionSet18>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Mrgn", default)]
    pub mrgn: Vec<PositionSet20>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Reuse", default)]
    pub reuse: Vec<PositionSet19>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TimeToMaturity2ChoiceEnum {
    #[serde(rename = "Spcl", skip_serializing_if = "Option::is_none")]
    pub spcl: Option<SpecialPurpose2Code>,
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<TimeToMaturityPeriod2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TimeToMaturity2Choice {
    #[serde(flatten)]
    pub value: TimeToMaturity2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceMetrics3 {
    #[serde(rename = "Rates", skip_serializing_if = "Option::is_none")]
    pub rates: Option<Rates3>,
    #[serde(rename = "LndgFee", skip_serializing_if = "Option::is_none")]
    pub lndg_fee: Option<PercentageRate>,
}
#[derive(
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
pub struct PositionSetMetrics11 {
    #[serde(rename = "VolMtrcs", skip_serializing_if = "Option::is_none")]
    pub vol_mtrcs: Option<VolumeMetrics4>,
    #[serde(rename = "CshRinvstmtRate", skip_serializing_if = "Option::is_none")]
    pub csh_rinvstmt_rate: Option<PercentageRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QuantityNominalValue2ChoiceEnum {
    #[serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none")]
    pub nmnl_val: Option<AmountAndDirection53>,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<DecimalNumber>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QuantityNominalValue2Choice {
    #[serde(flatten)]
    pub value: QuantityNominalValue2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Rates3 {
    #[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
    pub fxd: Option<PercentageRate>,
    #[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
    pub fltg: Option<PercentageRate>,
    #[serde(rename = "BuySellBck", skip_serializing_if = "Option::is_none")]
    pub buy_sell_bck: Option<SecuritiesTransactionPrice18Choice>,
}
#[derive(
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
pub struct Rates1ChoiceEnum {
    #[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
    pub fxd: Option<NoReasonCode>,
    #[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
    pub fltg: Option<ExternalRatesAndTenors1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Rates1Choice {
    #[serde(flatten)]
    pub value: Rates1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PositionSet18 {
    #[validate]
    #[serde(rename = "Dmnsns")]
    pub dmnsns: PositionSetDimensions14,
    #[validate]
    #[serde(rename = "Mtrcs")]
    pub mtrcs: PositionSetMetrics12,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LongFraction19DecimalNumber {
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
pub struct OrganisationIdentification15ChoiceEnum {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<OrganisationIdentification38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentification15Choice {
    #[serde(flatten)]
    pub value: OrganisationIdentification15ChoiceEnum,
}
#[derive(
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
pub struct PositionSet17 {
    #[validate]
    #[serde(rename = "Dmnsns")]
    pub dmnsns: PositionSetDimensions14,
    #[validate]
    #[serde(rename = "Mtrcs")]
    pub mtrcs: PositionSetMetrics13,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceStatus1Code {
    #[serde(rename = "PNDG")]
    Pndg,
    #[serde(rename = "NOAP")]
    Noap,
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
pub struct ReuseValue1ChoiceEnum {
    #[serde(rename = "Estmtd", skip_serializing_if = "Option::is_none")]
    pub estmtd: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Actl", skip_serializing_if = "Option::is_none")]
    pub actl: Option<ActiveOrHistoricCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReuseValue1Choice {
    #[serde(flatten)]
    pub value: ReuseValue1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndDirection53 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
    pub sgn: Option<PlusOrMinusIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
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
pub struct Max500Text {
    #[validate(length(min = 1, max = 500,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReinvestmentType1Code {
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "OCMP")]
    Ocmp,
    #[serde(rename = "MMFT")]
    Mmft,
    #[serde(rename = "REPM")]
    Repm,
    #[serde(rename = "SDPU")]
    Sdpu,
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
pub struct Max3Number {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeRepositoryReportingType1Code {
    #[serde(rename = "SWOS")]
    Swos,
    #[serde(rename = "TWOS")]
    Twos,
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
pub struct SecuritiesTransactionPrice18ChoiceEnum {
    #[serde(rename = "BsisPts", skip_serializing_if = "Option::is_none")]
    pub bsis_pts: Option<DecimalNumber>,
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<PercentageRate>,
    #[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
    pub dcml: Option<BaseOneRate>,
    #[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
    pub mntry_val: Option<AmountAndDirection107>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice18Choice {
    #[serde(flatten)]
    pub value: SecuritiesTransactionPrice18ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PositionSetMetrics13 {
    #[validate]
    #[serde(rename = "VolMtrcs")]
    pub vol_mtrcs: VolumeMetrics5,
    #[serde(rename = "PricMtrcs", skip_serializing_if = "Option::is_none")]
    pub pric_mtrcs: Option<PriceMetrics3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TimeToMaturityPeriod2 {
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
pub struct ExternalAgreementType1Code {
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
pub struct VolumeMetrics5 {
    #[serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none")]
    pub nb_of_txs: Option<Max15NumericText>,
    #[serde(rename = "Xpsr", skip_serializing_if = "Option::is_none")]
    pub xpsr: Option<ExposureMetrics4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PositionSet19 {
    #[validate]
    #[serde(rename = "Dmnsns")]
    pub dmnsns: PositionSetDimensions12,
    #[validate]
    #[serde(rename = "Mtrcs")]
    pub mtrcs: PositionSetMetrics11,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ExposureType10Code {
    #[serde(rename = "SBSC")]
    Sbsc,
    #[serde(rename = "MGLD")]
    Mgld,
    #[serde(rename = "SLEB")]
    Sleb,
    #[serde(rename = "REPO")]
    Repo,
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
pub struct ContractTerm6ChoiceEnum {
    #[serde(rename = "Opn", skip_serializing_if = "Option::is_none")]
    pub opn: Option<TrueFalseIndicator>,
    #[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
    pub fxd: Option<TimeToMaturity2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContractTerm6Choice {
    #[serde(flatten)]
    pub value: ContractTerm6ChoiceEnum,
}
#[derive(
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
pub struct VolumeMetrics6 {
    #[serde(rename = "Postv", skip_serializing_if = "Option::is_none")]
    pub postv: Option<ExposureMetrics5>,
    #[serde(rename = "Neg", skip_serializing_if = "Option::is_none")]
    pub neg: Option<ExposureMetrics5>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralRole1Code {
    #[serde(rename = "GIVE")]
    Give,
    #[serde(rename = "TAKE")]
    Take,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralQualityType1Code {
    #[serde(rename = "INVG")]
    Invg,
    #[serde(rename = "NIVG")]
    Nivg,
    #[serde(rename = "NOTR")]
    Notr,
    #[serde(rename = "NOAP")]
    Noap,
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
pub struct PositionSetDimensions15 {
    #[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
    pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
    pub othr_ctr_pty: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "CollPrtflId", skip_serializing_if = "Option::is_none")]
    pub coll_prtfl_id: Option<Max52Text>,
    #[serde(rename = "OtlrsIncl", skip_serializing_if = "Option::is_none")]
    pub otlrs_incl: Option<TrueFalseIndicator>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralType6Code {
    #[serde(rename = "GBBK")]
    Gbbk,
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "INSU")]
    Insu,
    #[serde(rename = "LCRE")]
    Lcre,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "STCF")]
    Stcf,
    #[default]
    Unknown,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReportPeriodActivity1Code {
    #[serde(rename = "NOTX")]
    Notx,
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
pub struct TradingVenueType1ChoiceEnum {
    #[serde(rename = "OnVn", skip_serializing_if = "Option::is_none")]
    pub on_vn: Option<TradeMarket2Code>,
    #[serde(rename = "OffVn", skip_serializing_if = "Option::is_none")]
    pub off_vn: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradingVenueType1Choice {
    #[serde(flatten)]
    pub value: TradingVenueType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExposureMetrics6 {
    #[serde(rename = "PstdMrgnOrColl", skip_serializing_if = "Option::is_none")]
    pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral4>,
}
#[derive(
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
pub struct LoanData134 {
    #[serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none")]
    pub ctrct_tp: Option<ExposureType10Code>,
    #[serde(rename = "Clrd", skip_serializing_if = "Option::is_none")]
    pub clrd: Option<TrueFalseIndicator>,
    #[serde(rename = "PrtflCd", skip_serializing_if = "Option::is_none")]
    pub prtfl_cd: Option<Max52Text>,
    #[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
    pub tradg_vn: Option<TradingVenueType1Choice>,
    #[serde(rename = "MstrAgrmtTp", skip_serializing_if = "Option::is_none")]
    pub mstr_agrmt_tp: Option<ExternalAgreementType1Code>,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<IsoDate>,
    #[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
    pub gnl_coll: Option<SpecialCollateral1Code>,
    #[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
    pub term: Option<ContractTerm6Choice>,
    #[serde(rename = "Rates", skip_serializing_if = "Option::is_none")]
    pub rates: Option<Rates1Choice>,
    #[serde(rename = "PrncplAmtCcy", skip_serializing_if = "Option::is_none")]
    pub prncpl_amt_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "PricCcy", skip_serializing_if = "Option::is_none")]
    pub pric_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
    pub scty: Option<Security49>,
    #[serde(rename = "OutsdngMrgnLnCcy", skip_serializing_if = "Option::is_none")]
    pub outsdng_mrgn_ln_ccy: Option<ActiveOrHistoricCurrencyCode>,
}
#[derive(
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
pub struct PositionSetDimensions14 {
    #[serde(rename = "CtrPtyData", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_data: Option<CounterpartyData86>,
    #[serde(rename = "LnData", skip_serializing_if = "Option::is_none")]
    pub ln_data: Option<LoanData134>,
    #[serde(rename = "CollData", skip_serializing_if = "Option::is_none")]
    pub coll_data: Option<CollateralData33>,
    #[serde(rename = "OtlrsIncl", skip_serializing_if = "Option::is_none")]
    pub otlrs_incl: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CounterpartyData86 {
    #[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
    pub rptg_ctr_pty: Option<CounterpartyIdentification10>,
    #[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
    pub othr_ctr_pty: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
    pub trpty_agt: Option<TrueFalseIndicator>,
    #[serde(rename = "AgtLndr", skip_serializing_if = "Option::is_none")]
    pub agt_lndr: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExposureMetrics5 {
    #[serde(rename = "CshCollAmt", skip_serializing_if = "Option::is_none")]
    pub csh_coll_amt: Option<AmountAndDirection53>,
    #[serde(rename = "CollMktVal", skip_serializing_if = "Option::is_none")]
    pub coll_mkt_val: Option<AmountAndDirection53>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalRatesAndTenors1Code {
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
pub struct ExternalSecuritiesLendingType1Code {
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
pub struct CounterpartyIdentification10 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "Sd", skip_serializing_if = "Option::is_none")]
    pub sd: Option<CollateralRole1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct VolumeMetrics4 {
    #[serde(rename = "ReuseVal", skip_serializing_if = "Option::is_none")]
    pub reuse_val: Option<ReuseValue1Choice>,
    #[serde(rename = "RinvstdCshAmt", skip_serializing_if = "Option::is_none")]
    pub rinvstd_csh_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Security49 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<IsinOct2015Identifier>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<CfiOct2015Identifier>,
    #[serde(rename = "QtyOrNmnlVal", skip_serializing_if = "Option::is_none")]
    pub qty_or_nmnl_val: Option<QuantityNominalValue2Choice>,
    #[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
    #[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
    pub mkt_val: Option<AmountAndDirection53>,
    #[serde(rename = "Qlty", skip_serializing_if = "Option::is_none")]
    pub qlty: Option<CollateralQualityType1Code>,
    #[serde(rename = "Mtrty", skip_serializing_if = "Option::is_none")]
    pub mtrty: Option<IsoDate>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<SecurityIssuer4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Tp", default)]
    pub tp: Vec<SecuritiesLendingType3Choice>,
    #[serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none")]
    pub exclsv_arrgmnt: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralData33 {
    #[serde(rename = "NetXpsrCollstnInd", skip_serializing_if = "Option::is_none")]
    pub net_xpsr_collstn_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "CmpntTp", skip_serializing_if = "Option::is_none")]
    pub cmpnt_tp: Option<CollateralType6Code>,
    #[serde(rename = "CshCollCcy", skip_serializing_if = "Option::is_none")]
    pub csh_coll_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "PricCcy", skip_serializing_if = "Option::is_none")]
    pub pric_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "Qlty", skip_serializing_if = "Option::is_none")]
    pub qlty: Option<CollateralQualityType1Code>,
    #[serde(rename = "Mtrty", skip_serializing_if = "Option::is_none")]
    pub mtrty: Option<ContractTerm6Choice>,
    #[serde(rename = "IssrJursdctn", skip_serializing_if = "Option::is_none")]
    pub issr_jursdctn: Option<IssuerJurisdiction1Choice>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<SecuritiesLendingType3Choice>,
    #[serde(rename = "TradRpstry", skip_serializing_if = "Option::is_none")]
    pub trad_rpstry: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "RcncltnFlg", skip_serializing_if = "Option::is_none")]
    pub rcncltn_flg: Option<ReconciliationFlag2>,
    #[serde(rename = "RinvstdCsh", skip_serializing_if = "Option::is_none")]
    pub rinvstd_csh: Option<ReinvestedCashTypeAndAmount2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTransactionPrice5 {
    #[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
    pub val: Option<LongFraction19DecimalNumber>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeMarket2Code {
    #[serde(rename = "DMST")]
    Dmst,
    #[serde(rename = "FRGN")]
    Frgn,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SpecialCollateral1Code {
    #[serde(rename = "GENE")]
    Gene,
    #[serde(rename = "SPEC")]
    Spec,
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
pub struct PositionSetMetrics10 {
    #[serde(rename = "VolMtrcs", skip_serializing_if = "Option::is_none")]
    pub vol_mtrcs: Option<ExposureMetrics6>,
}
#[derive(
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
pub struct PositionSetMetrics7 {
    #[validate]
    #[serde(rename = "VolMtrcs")]
    pub vol_mtrcs: VolumeMetrics5,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesFinancingReportingPositionSetReportV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "AggtdPoss")]
    pub aggtd_poss: PositionSetReport3Choice,
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
pub struct Max72Text {
    #[validate(length(min = 1, max = 72,))]
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
pub struct OrganisationIdentification38 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: GenericIdentification175,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max105Text>,
    #[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
    pub dmcl: Option<Max500Text>,
}
#[derive(
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
    #[serde(rename = "SctiesFincgRptgPosSetRpt")]
    pub scties_fincg_rptg_pos_set_rpt: SecuritiesFinancingReportingPositionSetReportV01<A>,
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
pub struct PositionSet16 {
    #[validate]
    #[serde(rename = "Dmnsns")]
    pub dmnsns: PositionSetDimensions14,
    #[validate]
    #[serde(rename = "Mtrcs")]
    pub mtrcs: PositionSetMetrics7,
}
#[derive(
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
pub struct PrincipalAmount3 {
    #[serde(rename = "ValDtAmt", skip_serializing_if = "Option::is_none")]
    pub val_dt_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "MtrtyDtAmt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ReconciliationFlag2 {
    #[serde(rename = "RptTp", skip_serializing_if = "Option::is_none")]
    pub rpt_tp: Option<TradeRepositoryReportingType1Code>,
    #[serde(rename = "BothCtrPtiesRptg", skip_serializing_if = "Option::is_none")]
    pub both_ctr_pties_rptg: Option<TrueFalseIndicator>,
    #[serde(rename = "PairdSts", skip_serializing_if = "Option::is_none")]
    pub paird_sts: Option<TrueFalseIndicator>,
    #[serde(rename = "LnRcncltnSts", skip_serializing_if = "Option::is_none")]
    pub ln_rcncltn_sts: Option<TrueFalseIndicator>,
    #[serde(rename = "CollRcncltnSts", skip_serializing_if = "Option::is_none")]
    pub coll_rcncltn_sts: Option<TrueFalseIndicator>,
    #[serde(rename = "ModSts", skip_serializing_if = "Option::is_none")]
    pub mod_sts: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndDirection107 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAnd20DecimalAmount,
    #[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
    pub sgn: Option<PlusOrMinusIndicator>,
}
#[derive(
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
pub struct ReinvestedCashTypeAndAmount2 {
    #[serde(rename = "Tp")]
    pub tp: ReinvestmentType1Code,
    #[serde(rename = "RinvstdCshCcy")]
    pub rinvstd_csh_ccy: ActiveOrHistoricCurrencyCode,
}
#[derive(
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
pub struct ExposureMetrics4 {
    #[serde(rename = "PrncplAmt", skip_serializing_if = "Option::is_none")]
    pub prncpl_amt: Option<PrincipalAmount3>,
    #[serde(rename = "LnVal", skip_serializing_if = "Option::is_none")]
    pub ln_val: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
    pub mkt_val: Option<AmountAndDirection53>,
    #[serde(rename = "OutsdngMrgnLnAmt", skip_serializing_if = "Option::is_none")]
    pub outsdng_mrgn_ln_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "ShrtMktValAmt", skip_serializing_if = "Option::is_none")]
    pub shrt_mkt_val_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "MrgnLn", skip_serializing_if = "Option::is_none")]
    pub mrgn_ln: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "CshCollAmt", skip_serializing_if = "Option::is_none")]
    pub csh_coll_amt: Option<AmountAndDirection53>,
    #[serde(rename = "CollMktVal", skip_serializing_if = "Option::is_none")]
    pub coll_mkt_val: Option<AmountAndDirection53>,
}
#[derive(
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
pub struct PositionSetReport3ChoiceEnum {
    #[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
    pub data_set_actn: Option<ReportPeriodActivity1Code>,
    #[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
    pub rpt: Option<NamedPosition3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PositionSetReport3Choice {
    #[serde(flatten)]
    pub value: PositionSetReport3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PositionSetMetrics12 {
    #[serde(rename = "VolMtrcs", skip_serializing_if = "Option::is_none")]
    pub vol_mtrcs: Option<VolumeMetrics6>,
    #[serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none")]
    pub hrcut_or_mrgn: Option<PercentageRate>,
    #[serde(rename = "QtyOrNmnlAmt", skip_serializing_if = "Option::is_none")]
    pub qty_or_nmnl_amt: Option<QuantityNominalValue2Choice>,
}
