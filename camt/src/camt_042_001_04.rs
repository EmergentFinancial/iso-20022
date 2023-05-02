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
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{12,12}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:swift:xsd:camt.042.001.04".to_string()
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
pub struct Pagination {
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
pub struct ValorenIdentifier {
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
pub struct Charge26 {
    #[serde(rename = "Tp")]
    pub tp: ChargeType4Choice,
    #[serde(rename = "ChrgApld")]
    pub chrg_apld: AmountOrRate3Choice,
}
#[derive(
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
    #[serde(rename = "FndDtldEstmtdCshFcstRpt")]
    pub fnd_dtld_estmtd_csh_fcst_rpt: FundDetailedEstimatedCashForecastReportV04,
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
pub struct ForeignExchangeTerms19 {
    #[serde(rename = "UnitCcy")]
    pub unit_ccy: ActiveCurrencyCode,
    #[serde(rename = "QtdCcy")]
    pub qtd_ccy: ActiveCurrencyCode,
    #[validate]
    #[serde(rename = "XchgRate")]
    pub xchg_rate: BaseOneRate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CurrencyDesignation1 {
    #[serde(rename = "CcyDsgnt", skip_serializing_if = "Option::is_none")]
    pub ccy_dsgnt: Option<CurrencyDesignation1Code>,
    #[serde(rename = "Lctn", skip_serializing_if = "Option::is_none")]
    pub lctn: Option<CountryCode>,
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
pub struct NetCashForecast3 {
    #[serde(rename = "NetAmt", skip_serializing_if = "Option::is_none")]
    pub net_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "NetUnitsNb", skip_serializing_if = "Option::is_none")]
    pub net_units_nb: Option<FinancialInstrumentQuantity1>,
    #[serde(rename = "FlowDrctn")]
    pub flow_drctn: FlowDirectionType1Code,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestmentFundTransactionOutType1Code {
    #[serde(rename = "REDM")]
    Redm,
    #[serde(rename = "SWIO")]
    Swio,
    #[serde(rename = "INSP")]
    Insp,
    #[serde(rename = "CROO")]
    Croo,
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
pub struct FundCashOutBreakdown3 {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "UnitsNb", skip_serializing_if = "Option::is_none")]
    pub units_nb: Option<FinancialInstrumentQuantity1>,
    #[serde(rename = "NewAmtInd", skip_serializing_if = "Option::is_none")]
    pub new_amt_ind: Option<YesNoIndicator>,
    #[serde(rename = "InvstmtFndTxOutTp")]
    pub invstmt_fnd_tx_out_tp: InvestmentFundTransactionOutType1Choice,
    #[serde(rename = "OrgnlOrdrQtyTp")]
    pub orgnl_ordr_qty_tp: QuantityType1Choice,
    #[validate(length(min = 0,))]
    #[serde(rename = "ChrgDtls", default)]
    pub chrg_dtls: Vec<Charge26>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ComssnDtls", default)]
    pub comssn_dtls: Vec<Commission21>,
    #[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
    pub sttlm_ccy: Option<ActiveCurrencyCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BreakdownByCurrency2 {
    #[serde(rename = "Ccy")]
    pub ccy: ActiveOrHistoricCurrencyCode,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshOutFcst", default)]
    pub csh_out_fcst: Vec<CashOutForecast5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshInFcst", default)]
    pub csh_in_fcst: Vec<CashInForecast5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NetCshFcst", default)]
    pub net_csh_fcst: Vec<NetCashForecast4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IsinIdentifier {
    #[validate(regex = "ISIN_IDENTIFIER_REGEX")]
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
pub struct ActiveCurrencyAnd13DecimalAmount {
    #[serde(rename = "ActiveCurrencyAnd13DecimalAmount")]
    pub value: ActiveCurrencyAnd13DecimalAmountSimpleType,
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
pub struct EstimatedFundCashForecast5 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "TradDtTm")]
    pub trad_dt_tm: DateAndDateTimeChoice,
    #[serde(rename = "PrvsTradDtTm", skip_serializing_if = "Option::is_none")]
    pub prvs_trad_dt_tm: Option<DateAndDateTimeChoice>,
    #[validate]
    #[serde(rename = "FinInstrmDtls")]
    pub fin_instrm_dtls: FinancialInstrument9,
    #[validate(length(min = 0,))]
    #[serde(rename = "EstmtdTtlNAV", default)]
    pub estmtd_ttl_nav: Vec<ActiveOrHistoricCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrvsTtlNAV", default)]
    pub prvs_ttl_nav: Vec<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "EstmtdTtlUnitsNb", skip_serializing_if = "Option::is_none")]
    pub estmtd_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
    #[serde(rename = "PrvsTtlUnitsNb", skip_serializing_if = "Option::is_none")]
    pub prvs_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
    #[serde(
        rename = "EstmtdTtlNAVChngRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub estmtd_ttl_nav_chng_rate: Option<PercentageRate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InvstmtCcy", default)]
    pub invstmt_ccy: Vec<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "CcySts", skip_serializing_if = "Option::is_none")]
    pub ccy_sts: Option<CurrencyDesignation1>,
    #[validate]
    #[serde(rename = "XcptnlNetCshFlowInd")]
    pub xcptnl_net_csh_flow_ind: YesNoIndicator,
    #[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
    pub pric: Option<UnitPrice19>,
    #[serde(rename = "FXRate", skip_serializing_if = "Option::is_none")]
    pub fx_rate: Option<ForeignExchangeTerms19>,
    #[serde(
        rename = "EstmtdPctgOfShrClssTtlNAV",
        skip_serializing_if = "Option::is_none"
    )]
    pub estmtd_pctg_of_shr_clss_ttl_nav: Option<PercentageRate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "BrkdwnByPty", default)]
    pub brkdwn_by_pty: Vec<BreakdownByParty3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "BrkdwnByCtry", default)]
    pub brkdwn_by_ctry: Vec<BreakdownByCountry2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "BrkdwnByCcy", default)]
    pub brkdwn_by_ccy: Vec<BreakdownByCurrency2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "BrkdwnByUsrDfndParam", default)]
    pub brkdwn_by_usr_dfnd_param: Vec<BreakdownByUserDefinedParameter3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "EstmtdNetCshFcstDtls", default)]
    pub estmtd_net_csh_fcst_dtls: Vec<NetCashForecast4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BreakdownByCountry2 {
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshInFcst", default)]
    pub csh_in_fcst: Vec<CashInForecast5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshOutFcst", default)]
    pub csh_out_fcst: Vec<CashOutForecast5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NetCshFcst", default)]
    pub net_csh_fcst: Vec<NetCashForecast4>,
}
#[derive(
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
pub struct EuroclearClearstreamIdentifier {
    #[validate(length(min = 1, max = 12,))]
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
pub struct BaseOneRate {
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CurrencyDesignation1Code {
    #[serde(rename = "ONSH")]
    Onsh,
    #[serde(rename = "OFFS")]
    Offs,
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
pub struct OtherIdentification4 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp")]
    pub tp: IdentificationSource5Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NetCashForecast4 {
    #[validate]
    #[serde(rename = "CshSttlmDt")]
    pub csh_sttlm_dt: IsoDate,
    #[serde(rename = "NetAmt", skip_serializing_if = "Option::is_none")]
    pub net_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "NetUnitsNb", skip_serializing_if = "Option::is_none")]
    pub net_units_nb: Option<FinancialInstrumentQuantity1>,
    #[serde(rename = "FlowDrctn")]
    pub flow_drctn: FlowDirectionType1Code,
    #[serde(rename = "AddtlBal", skip_serializing_if = "Option::is_none")]
    pub addtl_bal: Option<FundBalance1>,
}
#[derive(
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
pub enum FlowDirectionType1Code {
    #[serde(rename = "INCG")]
    Incg,
    #[serde(rename = "OUTG")]
    Outg,
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
pub struct BreakdownByUserDefinedParameter3 {
    #[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
    pub pty: Option<InvestmentAccount42>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "UsrDfnd", skip_serializing_if = "Option::is_none")]
    pub usr_dfnd: Option<DataFormat2Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshInFcst", default)]
    pub csh_in_fcst: Vec<CashInForecast5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshOutFcst", default)]
    pub csh_out_fcst: Vec<CashOutForecast5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NetCshFcst", default)]
    pub net_csh_fcst: Vec<NetCashForecast4>,
}
#[derive(
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
pub struct ChargeType4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ChargeType12Code>,
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
pub struct ChargeType4Choice {
    #[serde(flatten)]
    pub value: ChargeType4ChoiceEnum,
}
#[derive(
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
pub struct AmountOrRate3ChoiceEnum {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
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
pub struct AmountOrRate3Choice {
    #[serde(flatten)]
    pub value: AmountOrRate3ChoiceEnum,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationSource5ChoiceEnum {
    #[serde(rename = "PrtryIdSrc", skip_serializing_if = "Option::is_none")]
    pub prtry_id_src: Option<Max35Text>,
    #[serde(rename = "DmstIdSrc", skip_serializing_if = "Option::is_none")]
    pub dmst_id_src: Option<CountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationSource5Choice {
    #[serde(flatten)]
    pub value: IdentificationSource5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SimpleIdentificationInformation {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ChargeType12Code {
    #[serde(rename = "BEND")]
    Bend,
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "FEND")]
    Fend,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "REGF")]
    Regf,
    #[serde(rename = "SHIP")]
    Ship,
    #[serde(rename = "SPCN")]
    Spcn,
    #[serde(rename = "TRAN")]
    Tran,
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
pub struct FundDetailedEstimatedCashForecastReportV04 {
    #[validate]
    #[serde(rename = "MsgId")]
    pub msg_id: MessageIdentification1,
    #[serde(rename = "PoolRef", skip_serializing_if = "Option::is_none")]
    pub pool_ref: Option<AdditionalReference3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrvsRef", default)]
    pub prvs_ref: Vec<AdditionalReference3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RltdRef", default)]
    pub rltd_ref: Vec<AdditionalReference3>,
    #[validate]
    #[serde(rename = "MsgPgntn")]
    pub msg_pgntn: Pagination,
    #[serde(rename = "FndOrSubFndDtls", skip_serializing_if = "Option::is_none")]
    pub fnd_or_sub_fnd_dtls: Option<Fund3>,
    #[validate(length(min = 1,))]
    #[serde(rename = "EstmtdFndCshFcstDtls", default)]
    pub estmtd_fnd_csh_fcst_dtls: Vec<EstimatedFundCashForecast5>,
    #[serde(rename = "CnsltdNetCshFcst", skip_serializing_if = "Option::is_none")]
    pub cnsltd_net_csh_fcst: Option<NetCashForecast3>,
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
pub struct Fund3 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
    pub lgl_ntty_idr: Option<LeiIdentifier>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<OtherIdentification4>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "EstmtdTtlNAV", skip_serializing_if = "Option::is_none")]
    pub estmtd_ttl_nav: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "PrvsTtlNAV", skip_serializing_if = "Option::is_none")]
    pub prvs_ttl_nav: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "EstmtdTtlUnitsNb", skip_serializing_if = "Option::is_none")]
    pub estmtd_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
    #[serde(rename = "PrvsTtlUnitsNb", skip_serializing_if = "Option::is_none")]
    pub prvs_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
    #[serde(
        rename = "EstmtdPctgOfFndTtlNAV",
        skip_serializing_if = "Option::is_none"
    )]
    pub estmtd_pctg_of_fnd_ttl_nav: Option<PercentageRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashOutForecast5 {
    #[validate]
    #[serde(rename = "CshSttlmDt")]
    pub csh_sttlm_dt: IsoDate,
    #[serde(rename = "SubTtlAmt", skip_serializing_if = "Option::is_none")]
    pub sub_ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "SubTtlUnitsNb", skip_serializing_if = "Option::is_none")]
    pub sub_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
    #[serde(rename = "XcptnlCshFlowInd", skip_serializing_if = "Option::is_none")]
    pub xcptnl_csh_flow_ind: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshOutBrkdwnDtls", default)]
    pub csh_out_brkdwn_dtls: Vec<FundCashOutBreakdown3>,
    #[serde(rename = "AddtlBal", skip_serializing_if = "Option::is_none")]
    pub addtl_bal: Option<FundBalance1>,
}
#[derive(
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
pub struct BreakdownByParty3 {
    #[validate]
    #[serde(rename = "Pty")]
    pub pty: InvestmentAccount42,
    #[serde(rename = "AddtlParams", skip_serializing_if = "Option::is_none")]
    pub addtl_params: Option<AdditionalParameters1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshInFcst", default)]
    pub csh_in_fcst: Vec<CashInForecast5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshOutFcst", default)]
    pub csh_out_fcst: Vec<CashOutForecast5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NetCshFcst", default)]
    pub net_csh_fcst: Vec<NetCashForecast4>,
}
#[derive(
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
pub struct DataFormat2ChoiceEnum {
    #[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
    pub ustrd: Option<Max140Text>,
    #[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
    pub strd: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DataFormat2Choice {
    #[serde(flatten)]
    pub value: DataFormat2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FundCashInBreakdown3 {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "UnitsNb", skip_serializing_if = "Option::is_none")]
    pub units_nb: Option<FinancialInstrumentQuantity1>,
    #[serde(rename = "NewAmtInd", skip_serializing_if = "Option::is_none")]
    pub new_amt_ind: Option<YesNoIndicator>,
    #[serde(rename = "InvstmtFndTxInTp")]
    pub invstmt_fnd_tx_in_tp: InvestmentFundTransactionInType1Choice,
    #[serde(rename = "OrgnlOrdrQtyTp")]
    pub orgnl_ordr_qty_tp: QuantityType1Choice,
    #[validate(length(min = 0,))]
    #[serde(rename = "ChrgDtls", default)]
    pub chrg_dtls: Vec<Charge26>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ComssnDtls", default)]
    pub comssn_dtls: Vec<Commission21>,
    #[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
    pub sttlm_ccy: Option<ActiveCurrencyCode>,
}
#[derive(
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
pub struct AccountIdentification1 {
    #[validate]
    #[serde(rename = "Prtry")]
    pub prtry: SimpleIdentificationInformation,
}
#[derive(
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
pub struct AdditionalReference3 {
    #[validate]
    #[serde(rename = "Ref")]
    pub r#ref: Max35Text,
    #[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
    pub ref_issr: Option<PartyIdentification2Choice>,
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
pub struct InvestmentFundTransactionInType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestmentFundTransactionInType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestmentFundTransactionInType1Choice {
    #[serde(flatten)]
    pub value: InvestmentFundTransactionInType1ChoiceEnum,
}
#[derive(
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
pub struct SecurityIdentification3ChoiceEnum {
    #[serde(rename = "CTA", skip_serializing_if = "Option::is_none")]
    pub cta: Option<ConsolidatedTapeAssociationIdentifier>,
    #[serde(rename = "Cmon", skip_serializing_if = "Option::is_none")]
    pub cmon: Option<EuroclearClearstreamIdentifier>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinIdentifier>,
    #[serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none")]
    pub blmbrg: Option<BloombergIdentifier>,
    #[serde(rename = "Belgn", skip_serializing_if = "Option::is_none")]
    pub belgn: Option<BelgianIdentifier>,
    #[serde(rename = "RIC", skip_serializing_if = "Option::is_none")]
    pub ric: Option<RicIdentifier>,
    #[serde(rename = "CUSIP", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<CusipIdentifier>,
    #[serde(rename = "SEDOL", skip_serializing_if = "Option::is_none")]
    pub sedol: Option<SedolIdentifier>,
    #[serde(rename = "Wrtppr", skip_serializing_if = "Option::is_none")]
    pub wrtppr: Option<WertpapierIdentifier>,
    #[serde(rename = "Dtch", skip_serializing_if = "Option::is_none")]
    pub dtch: Option<DutchIdentifier>,
    #[serde(rename = "QUICK", skip_serializing_if = "Option::is_none")]
    pub quick: Option<QuickIdentifier>,
    #[serde(rename = "SCVM", skip_serializing_if = "Option::is_none")]
    pub scvm: Option<SicovamIdentifier>,
    #[serde(rename = "OthrPrtryId", skip_serializing_if = "Option::is_none")]
    pub othr_prtry_id: Option<AlternateSecurityIdentification1>,
    #[serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none")]
    pub tckr_symb: Option<TickerIdentifier>,
    #[serde(rename = "Vlrn", skip_serializing_if = "Option::is_none")]
    pub vlrn: Option<ValorenIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification3Choice {
    #[serde(flatten)]
    pub value: SecurityIdentification3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitPrice19 {
    #[serde(rename = "PricTp")]
    pub pric_tp: UnitPriceType2Choice,
    #[validate]
    #[serde(rename = "Val")]
    pub val: PriceValue1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Commission21 {
    #[serde(rename = "ComssnTp")]
    pub comssn_tp: CommissionType5Choice,
    #[serde(rename = "ComssnApld")]
    pub comssn_apld: AmountOrRate3Choice,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OrderQuantityType2Code {
    #[serde(rename = "UNIT")]
    Unit,
    #[serde(rename = "CASH")]
    Cash,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfPrice10Code {
    #[serde(rename = "BIDE")]
    Bide,
    #[serde(rename = "OFFR")]
    Offr,
    #[serde(rename = "NAVL")]
    Navl,
    #[serde(rename = "CREA")]
    Crea,
    #[serde(rename = "CANC")]
    Canc,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "SWNG")]
    Swng,
    #[serde(rename = "MIDD")]
    Midd,
    #[serde(rename = "RINV")]
    Rinv,
    #[serde(rename = "SWIC")]
    Swic,
    #[serde(rename = "DDVR")]
    Ddvr,
    #[serde(rename = "ACTU")]
    Actu,
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
pub struct AlternateSecurityIdentification1Enum {
    #[serde(rename = "DmstIdSrc", skip_serializing_if = "Option::is_none")]
    pub dmst_id_src: Option<CountryCode>,
    #[serde(rename = "PrtryIdSrc", skip_serializing_if = "Option::is_none")]
    pub prtry_id_src: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlternateSecurityIdentification1 {
    #[serde(flatten)]
    pub value: AlternateSecurityIdentification1Enum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrument9 {
    #[serde(rename = "Id")]
    pub id: SecurityIdentification3Choice,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none")]
    pub splmtry_id: Option<Max35Text>,
    #[serde(rename = "ReqdNAVCcy", skip_serializing_if = "Option::is_none")]
    pub reqd_nav_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "ClssTp", skip_serializing_if = "Option::is_none")]
    pub clss_tp: Option<Max35Text>,
    #[serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none")]
    pub scties_form: Option<FormOfSecurity1Code>,
    #[serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none")]
    pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
    #[validate]
    #[serde(rename = "DualFndInd")]
    pub dual_fnd_ind: YesNoIndicator,
}
#[derive(
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
pub struct QuantityType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OrderQuantityType2Code>,
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
pub struct QuantityType1Choice {
    #[serde(flatten)]
    pub value: QuantityType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BloombergIdentifier {
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
pub struct AdditionalParameters1 {
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "GeoArea", skip_serializing_if = "Option::is_none")]
    pub geo_area: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashInForecast5 {
    #[validate]
    #[serde(rename = "CshSttlmDt")]
    pub csh_sttlm_dt: IsoDate,
    #[serde(rename = "SubTtlAmt", skip_serializing_if = "Option::is_none")]
    pub sub_ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "SubTtlUnitsNb", skip_serializing_if = "Option::is_none")]
    pub sub_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
    #[serde(rename = "XcptnlCshFlowInd", skip_serializing_if = "Option::is_none")]
    pub xcptnl_csh_flow_ind: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshInBrkdwnDtls", default)]
    pub csh_in_brkdwn_dtls: Vec<FundCashInBreakdown3>,
    #[serde(rename = "AddtlBal", skip_serializing_if = "Option::is_none")]
    pub addtl_bal: Option<FundBalance1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitPriceType2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfPrice10Code>,
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
pub struct UnitPriceType2Choice {
    #[serde(flatten)]
    pub value: UnitPriceType2ChoiceEnum,
}
#[derive(
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
pub enum InvestmentFundTransactionInType1Code {
    #[serde(rename = "SUBS")]
    Subs,
    #[serde(rename = "SWII")]
    Swii,
    #[serde(rename = "INSP")]
    Insp,
    #[serde(rename = "CROI")]
    Croi,
    #[serde(rename = "RDIV")]
    Rdiv,
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
pub struct CommissionType5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CommissionType6Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommissionType5Choice {
    #[serde(flatten)]
    pub value: CommissionType5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestmentAccount42 {
    #[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<AccountIdentification1>,
    #[serde(rename = "OwnrId", skip_serializing_if = "Option::is_none")]
    pub ownr_id: Option<PartyIdentification2Choice>,
    #[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
    pub acct_svcr: Option<PartyIdentification2Choice>,
}
#[derive(
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
pub struct InvestmentFundTransactionOutType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestmentFundTransactionOutType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestmentFundTransactionOutType1Choice {
    #[serde(flatten)]
    pub value: InvestmentFundTransactionOutType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification2ChoiceEnum {
    #[serde(rename = "BICOrBEI", skip_serializing_if = "Option::is_none")]
    pub bic_or_bei: Option<AnyBicIdentifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification2Choice {
    #[serde(flatten)]
    pub value: PartyIdentification2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CommissionType6Code {
    #[serde(rename = "FEND")]
    Fend,
    #[serde(rename = "BEND")]
    Bend,
    #[serde(rename = "CDPL")]
    Cdpl,
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
pub struct FinancialInstrumentQuantity1 {
    #[validate]
    #[serde(rename = "Unit")]
    pub unit: DecimalNumber,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FundBalance1 {
    #[serde(
        rename = "TtlUnitsFrUnitOrdrs",
        skip_serializing_if = "Option::is_none"
    )]
    pub ttl_units_fr_unit_ordrs: Option<FinancialInstrumentQuantity1>,
    #[serde(rename = "TtlUnitsFrCshOrdrs", skip_serializing_if = "Option::is_none")]
    pub ttl_units_fr_csh_ordrs: Option<FinancialInstrumentQuantity1>,
    #[serde(rename = "TtlCshFrUnitOrdrs", skip_serializing_if = "Option::is_none")]
    pub ttl_csh_fr_unit_ordrs: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TtlCshFrCshOrdrs", skip_serializing_if = "Option::is_none")]
    pub ttl_csh_fr_csh_ordrs: Option<ActiveOrHistoricCurrencyAndAmount>,
}
#[derive(
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
