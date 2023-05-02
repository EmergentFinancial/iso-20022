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
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.012.001.02".to_string()
}

#[derive(
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
pub struct SupplementaryData1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
    pub plc_and_nm: Option<Max350Text>,
    #[validate]
    #[serde(rename = "Envlp")]
    pub envlp: SupplementaryDataEnvelope1<A>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransactionOperationType1Code {
    #[serde(rename = "AMND")]
    Amnd,
    #[serde(rename = "CANC")]
    Canc,
    #[serde(rename = "CORR")]
    Corr,
    #[serde(rename = "NEWT")]
    Newt,
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
pub struct CollateralValuation6 {
    #[serde(rename = "NmnlAmt", skip_serializing_if = "Option::is_none")]
    pub nmnl_amt: Option<ActiveCurrencyAndAmount>,
    #[validate]
    #[serde(rename = "ISIN")]
    pub isin: IsinOct2015Identifier,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuredMarketReport4ChoiceEnum<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
    pub data_set_actn: Option<ReportPeriodActivity3Code>,
    #[serde(rename = "Tx", skip_serializing_if = "Option::is_none")]
    pub tx: Option<SecuredMarketTransaction4<A>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuredMarketReport4Choice<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(flatten)]
    pub value: SecuredMarketReport4ChoiceEnum<A>,
}
#[derive(
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
pub enum CollateralPool1Code {
    #[serde(rename = "NOPL")]
    Nopl,
    #[serde(rename = "POOL")]
    Pool,
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
pub struct Sna2008SectorIdentifier {
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BrokeredDeal1Code {
    #[serde(rename = "BILA")]
    Bila,
    #[serde(rename = "BROK")]
    Brok,
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
pub struct MoneyMarketReportHeader1 {
    #[validate]
    #[serde(rename = "RptgAgt")]
    pub rptg_agt: LeiIdentifier,
    #[validate]
    #[serde(rename = "RefPrd")]
    pub ref_prd: DateTimePeriod1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NameAndLocation1 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max70Text,
    #[serde(rename = "Lctn")]
    pub lctn: CountryCode,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MoneyMarketTransactionType1Code {
    #[serde(rename = "BORR")]
    Borr,
    #[serde(rename = "LEND")]
    Lend,
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
pub struct FloatingRateNote2 {
    #[validate]
    #[serde(rename = "RefRateIndx")]
    pub ref_rate_indx: IsinOct2015Identifier,
    #[validate]
    #[serde(rename = "BsisPtSprd")]
    pub bsis_pt_sprd: Number,
}
#[derive(
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
pub struct MoneyMarketSecuredMarketStatisticalReportV02<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "RptHdr")]
    pub rpt_hdr: MoneyMarketReportHeader1,
    #[serde(rename = "ScrdMktRpt")]
    pub scrd_mkt_rpt: SecuredMarketReport4Choice<A>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<B>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralValuation7 {
    #[serde(rename = "PoolSts")]
    pub pool_sts: CollateralPool1Code,
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: CfiOct2015Identifier,
    #[validate]
    #[serde(rename = "Sctr")]
    pub sctr: Sna2008SectorIdentifier,
    #[serde(rename = "NmnlAmt", skip_serializing_if = "Option::is_none")]
    pub nmnl_amt: Option<ActiveCurrencyAndAmount>,
}
#[derive(
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
pub struct DateAndDateTimeChoiceEnum {
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
pub struct DateAndDateTimeChoice {
    #[serde(flatten)]
    pub value: DateAndDateTimeChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReportPeriodActivity3Code {
    #[serde(rename = "NOTX")]
    Notx,
    #[serde(rename = "NORA")]
    Nora,
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
pub struct ActiveCurrencyAndAmountSimpleType {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InterestRateType1Code {
    #[serde(rename = "FIXE")]
    Fixe,
    #[serde(rename = "VARI")]
    Vari,
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
pub enum NovationStatus1Code {
    #[serde(rename = "NONO")]
    Nono,
    #[serde(rename = "NOVA")]
    Nova,
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
pub struct Collateral18 {
    #[serde(rename = "Valtn")]
    pub valtn: SecuredCollateral2Choice,
    #[serde(rename = "Hrcut", skip_serializing_if = "Option::is_none")]
    pub hrcut: Option<PercentageRate>,
    #[serde(rename = "SpclCollInd", skip_serializing_if = "Option::is_none")]
    pub spcl_coll_ind: Option<SpecialCollateral2Code>,
}
#[derive(
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "MnyMktScrdMktSttstclRpt")]
    pub mny_mkt_scrd_mkt_sttstcl_rpt: MoneyMarketSecuredMarketStatisticalReportV02<A, B>,
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
pub struct SecuredCollateral2ChoiceEnum {
    #[serde(rename = "SnglColl", skip_serializing_if = "Option::is_none")]
    pub sngl_coll: Option<CollateralValuation6>,
    #[serde(rename = "OthrColl", skip_serializing_if = "Option::is_none")]
    pub othr_coll: Option<CollateralValuation7>,
    #[serde(rename = "MltplColl", skip_serializing_if = "Option::is_none")]
    pub mltpl_coll: Option<CollateralValuation6>,
    #[serde(rename = "PoolColl", skip_serializing_if = "Option::is_none")]
    pub pool_coll: Option<CollateralValuation6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuredCollateral2Choice {
    #[serde(flatten)]
    pub value: SecuredCollateral2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CounterpartyIdentification3ChoiceEnum {
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "SctrAndLctn", skip_serializing_if = "Option::is_none")]
    pub sctr_and_lctn: Option<SectorAndLocation1>,
    #[serde(rename = "NmAndLctn", skip_serializing_if = "Option::is_none")]
    pub nm_and_lctn: Option<NameAndLocation1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CounterpartyIdentification3Choice {
    #[serde(flatten)]
    pub value: CounterpartyIdentification3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SectorAndLocation1 {
    #[validate]
    #[serde(rename = "Sctr")]
    pub sctr: Sna2008SectorIdentifier,
    #[serde(rename = "Lctn")]
    pub lctn: CountryCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuredMarketTransaction4<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "RptdTxSts")]
    pub rptd_tx_sts: TransactionOperationType1Code,
    #[serde(rename = "NvtnSts", skip_serializing_if = "Option::is_none")]
    pub nvtn_sts: Option<NovationStatus1Code>,
    #[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
    pub brnch_id: Option<LeiIdentifier>,
    #[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
    pub unq_tx_idr: Option<Max105Text>,
    #[validate]
    #[serde(rename = "PrtryTxId")]
    pub prtry_tx_id: Max105Text,
    #[serde(rename = "RltdPrtryTxId", skip_serializing_if = "Option::is_none")]
    pub rltd_prtry_tx_id: Option<Max105Text>,
    #[serde(rename = "CtrPtyPrtryTxId", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_prtry_tx_id: Option<Max105Text>,
    #[serde(rename = "CtrPtyId")]
    pub ctr_pty_id: CounterpartyIdentification3Choice,
    #[serde(rename = "TrptyAgtId", skip_serializing_if = "Option::is_none")]
    pub trpty_agt_id: Option<LeiIdentifier>,
    #[serde(rename = "TradDt")]
    pub trad_dt: DateAndDateTimeChoice,
    #[validate]
    #[serde(rename = "SttlmDt")]
    pub sttlm_dt: IsoDate,
    #[validate]
    #[serde(rename = "MtrtyDt")]
    pub mtrty_dt: IsoDate,
    #[serde(rename = "TxTp")]
    pub tx_tp: MoneyMarketTransactionType1Code,
    #[validate]
    #[serde(rename = "TxNmnlAmt")]
    pub tx_nmnl_amt: ActiveCurrencyAndAmount,
    #[serde(rename = "RateTp")]
    pub rate_tp: InterestRateType1Code,
    #[serde(rename = "DealRate", skip_serializing_if = "Option::is_none")]
    pub deal_rate: Option<PercentageRate>,
    #[serde(rename = "FltgRateRpAgrmt", skip_serializing_if = "Option::is_none")]
    pub fltg_rate_rp_agrmt: Option<FloatingRateNote2>,
    #[serde(rename = "BrkrdDeal", skip_serializing_if = "Option::is_none")]
    pub brkrd_deal: Option<BrokeredDeal1Code>,
    #[validate]
    #[serde(rename = "Coll")]
    pub coll: Collateral18,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<A>>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SpecialCollateral2Code {
    #[serde(rename = "GENE")]
    Gene,
    #[serde(rename = "SPEC")]
    Spec,
    #[serde(rename = "MRRP")]
    Mrrp,
    #[default]
    Unknown,
}
