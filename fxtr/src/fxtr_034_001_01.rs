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
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_35_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,35}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:fxtr.034.001.01".to_string()
}

#[derive(
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
pub struct Max6Text {
    #[validate(length(min = 1, max = 6,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyType4Code {
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ITAG")]
    Itag,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "TAXH")]
    Taxh,
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
pub struct ForeignExchangeTradeConfirmationRequestV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Hdr")]
    pub hdr: Header23,
    #[validate]
    #[serde(rename = "ReqId")]
    pub req_id: MessageIdentification1,
    #[validate]
    #[serde(rename = "TradDtl")]
    pub trad_dtl: Trade2,
    #[serde(rename = "ConfTp")]
    pub conf_tp: ConfirmationRequest1Code,
    #[validate]
    #[serde(rename = "QryPrd")]
    pub qry_prd: Period4,
    #[validate]
    #[serde(rename = "QryStartNb")]
    pub qry_start_nb: Max35NumericText,
    #[serde(rename = "QryTradSts")]
    pub qry_trad_sts: QueryTradeStatus1Code,
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
pub struct Max70Text {
    #[validate(length(min = 1, max = 70,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradingModeType1Code {
    #[serde(rename = "QUDR")]
    Qudr,
    #[serde(rename = "ORDR")]
    Ordr,
    #[serde(rename = "NETR")]
    Netr,
    #[serde(rename = "AUCT")]
    Auct,
    #[serde(rename = "MARC")]
    Marc,
    #[serde(rename = "BILA")]
    Bila,
    #[serde(rename = "ANON")]
    Anon,
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
pub struct ActiveCurrencyAndAmountSimpleType {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradingMethodType1Code {
    #[serde(rename = "BITR")]
    Bitr,
    #[serde(rename = "CERB")]
    Cerb,
    #[serde(rename = "CUMA")]
    Cuma,
    #[serde(rename = "LIOR")]
    Lior,
    #[serde(rename = "NETR")]
    Netr,
    #[serde(rename = "ONCT")]
    Onct,
    #[serde(rename = "QUAU")]
    Quau,
    #[serde(rename = "TEAU")]
    Teau,
    #[serde(rename = "ANCL")]
    Ancl,
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
pub struct Header23 {
    #[validate]
    #[serde(rename = "FrmtVrsn")]
    pub frmt_vrsn: Max6Text,
    #[validate]
    #[serde(rename = "XchgId")]
    pub xchg_id: Max3NumericText,
    #[validate]
    #[serde(rename = "InitgPty")]
    pub initg_pty: GenericIdentification32,
    #[serde(rename = "RcptPty", skip_serializing_if = "Option::is_none")]
    pub rcpt_pty: Option<GenericIdentification32>,
    #[validate]
    #[serde(rename = "MsgSeqNb")]
    pub msg_seq_nb: Number,
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
pub struct DateFormat18ChoiceEnum {
    #[serde(rename = "NotSpcfdDt", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_dt: Option<DateType8Code>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTimeChoice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat18Choice {
    #[serde(flatten)]
    pub value: DateFormat18ChoiceEnum,
}
#[derive(
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
pub struct InstrumentLeg6 {
    #[serde(rename = "LegSd")]
    pub leg_sd: Side1Code,
    #[serde(rename = "LegSttlmTp")]
    pub leg_sttlm_tp: SettlementDateCode,
    #[validate]
    #[serde(rename = "LegSttlmDt")]
    pub leg_sttlm_dt: IsoDateTime,
    #[validate]
    #[serde(rename = "LegLastPric")]
    pub leg_last_pric: ActiveCurrencyAnd13DecimalAmount,
    #[serde(rename = "LegSttlmCcy")]
    pub leg_sttlm_ccy: CurrencyCode,
    #[validate]
    #[serde(rename = "LegOrdrQty")]
    pub leg_ordr_qty: CurrencyAndAmount,
    #[validate]
    #[serde(rename = "LegFwdPts")]
    pub leg_fwd_pts: DecimalNumber,
    #[validate]
    #[serde(rename = "LegClctdCtrPtyCcyLastQty")]
    pub leg_clctd_ctr_pty_ccy_last_qty: CurrencyAndAmount,
    #[validate]
    #[serde(rename = "LegRskAmt")]
    pub leg_rsk_amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "LegValtnRate")]
    pub leg_valtn_rate: AgreedRate3,
    #[validate]
    #[serde(rename = "LegValDt")]
    pub leg_val_dt: IsoDate,
    #[serde(rename = "LegCcy")]
    pub leg_ccy: CurrencyCode,
    #[validate]
    #[serde(rename = "LegSymb")]
    pub leg_symb: Max35Text,
    #[validate]
    #[serde(rename = "LegSctyId")]
    pub leg_scty_id: SecurityIdentification18,
}
#[derive(
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
pub struct Trade2 {
    #[validate]
    #[serde(rename = "TradId")]
    pub trad_id: Max35Text,
    #[validate]
    #[serde(rename = "TradDt")]
    pub trad_dt: IsoDate,
    #[serde(rename = "FXTradPdct")]
    pub fx_trad_pdct: UnderlyingProductIdentifier1Code,
    #[serde(rename = "TradgCcy", skip_serializing_if = "Option::is_none")]
    pub tradg_ccy: Option<CurrencyCode>,
    #[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
    pub sttlm_ccy: Option<CurrencyCode>,
    #[serde(rename = "TradgMtd", skip_serializing_if = "Option::is_none")]
    pub tradg_mtd: Option<TradingMethodType1Code>,
    #[serde(rename = "TradgMd")]
    pub tradg_md: TradingModeType1Code,
    #[serde(rename = "ClrMtd")]
    pub clr_mtd: ClearingMethod1Code,
    #[serde(rename = "Symb", skip_serializing_if = "Option::is_none")]
    pub symb: Option<Max35Text>,
    #[serde(rename = "PlcOfConf", skip_serializing_if = "Option::is_none")]
    pub plc_of_conf: Option<AnyBicIdentifier>,
    #[serde(rename = "FXDtls", skip_serializing_if = "Option::is_none")]
    pub fx_dtls: Option<Trade3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SwpLeg", default)]
    pub swp_leg: Vec<InstrumentLeg6>,
    #[serde(rename = "PdctId", skip_serializing_if = "Option::is_none")]
    pub pdct_id: Option<SecurityIdentification22Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AssoctdTradRef", default)]
    pub assoctd_trad_ref: Vec<Max70Text>,
}
#[derive(
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
pub enum Side1Code {
    #[serde(rename = "BUYI")]
    Buyi,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "TWOS")]
    Twos,
    #[serde(rename = "BUMI")]
    Bumi,
    #[serde(rename = "SEPL")]
    Sepl,
    #[serde(rename = "SESH")]
    Sesh,
    #[serde(rename = "SSEX")]
    Ssex,
    #[serde(rename = "CROS")]
    Cros,
    #[serde(rename = "CRSH")]
    Crsh,
    #[serde(rename = "CSHE")]
    Cshe,
    #[serde(rename = "DEFI")]
    Defi,
    #[serde(rename = "OPPO")]
    Oppo,
    #[serde(rename = "UNDI")]
    Undi,
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
pub enum DateType8Code {
    #[serde(rename = "UKWN")]
    Ukwn,
    #[serde(rename = "ONGO")]
    Ongo,
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
pub struct SecurityIdentification22ChoiceEnum {
    #[serde(rename = "CTA", skip_serializing_if = "Option::is_none")]
    pub cta: Option<ConsolidatedTapeAssociationIdentifier>,
    #[serde(rename = "Cmon", skip_serializing_if = "Option::is_none")]
    pub cmon: Option<EuroclearClearstreamIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternateIdentification1>,
    #[serde(rename = "RIC", skip_serializing_if = "Option::is_none")]
    pub ric: Option<RicIdentifier>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none")]
    pub tckr_symb: Option<TickerIdentifier>,
    #[serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none")]
    pub blmbrg: Option<BloombergIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification22Choice {
    #[serde(flatten)]
    pub value: SecurityIdentification22ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgreedRate3 {
    #[validate]
    #[serde(rename = "XchgRate")]
    pub xchg_rate: BaseOneRate,
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
pub struct CurrencyCode {
    #[validate(regex = "CURRENCY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementDateCode {
    #[serde(rename = "REGU")]
    Regu,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "NXTD")]
    Nxtd,
    #[serde(rename = "TONE")]
    Tone,
    #[serde(rename = "TTWO")]
    Ttwo,
    #[serde(rename = "TTRE")]
    Ttre,
    #[serde(rename = "TFOR")]
    Tfor,
    #[serde(rename = "TFIV")]
    Tfiv,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "FUTU")]
    Futu,
    #[serde(rename = "ASAP")]
    Asap,
    #[serde(rename = "ENDC")]
    Endc,
    #[serde(rename = "WHIF")]
    Whif,
    #[serde(rename = "WDIS")]
    Wdis,
    #[serde(rename = "WHID")]
    Whid,
    #[serde(rename = "TBAT")]
    Tbat,
    #[serde(rename = "MONT")]
    Mont,
    #[serde(rename = "CLEA")]
    Clea,
    #[serde(rename = "SAVE")]
    Save,
    #[serde(rename = "WISS")]
    Wiss,
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
pub struct AlternateIdentification1 {
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
pub struct Period4 {
    #[serde(rename = "StartDt")]
    pub start_dt: DateFormat18Choice,
    #[serde(rename = "EndDt")]
    pub end_dt: DateFormat18Choice,
}
#[derive(
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
pub enum QueryTradeStatus1Code {
    #[serde(rename = "QAST")]
    Qast,
    #[serde(rename = "QCTR")]
    Qctr,
    #[serde(rename = "QCIR")]
    Qcir,
    #[serde(rename = "QETR")]
    Qetr,
    #[serde(rename = "QNTR")]
    Qntr,
    #[serde(rename = "QRTR")]
    Qrtr,
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
pub struct GenericIdentification32 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<PartyType3Code>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyType4Code>,
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
pub struct CurrencyAndAmount {
    #[serde(rename = "CurrencyAndAmount")]
    pub value: CurrencyAndAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: CurrencyCode,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ConfirmationRequest1Code {
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "CNRR")]
    Cnrr,
    #[serde(rename = "STAT")]
    Stat,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnderlyingProductIdentifier1Code {
    #[serde(rename = "FORW")]
    Forw,
    #[serde(rename = "NDFO")]
    Ndfo,
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "SWAP")]
    Swap,
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
pub struct Trade3 {
    #[validate]
    #[serde(rename = "ExctnPric")]
    pub exctn_pric: ActiveCurrencyAnd13DecimalAmount,
    #[validate]
    #[serde(rename = "LastQty")]
    pub last_qty: CurrencyAndAmount,
    #[serde(rename = "SttlmTp")]
    pub sttlm_tp: SettlementDateCode,
    #[validate]
    #[serde(rename = "SttlmDt")]
    pub sttlm_dt: IsoDate,
    #[validate]
    #[serde(rename = "ValtnRate")]
    pub valtn_rate: AgreedRate3,
    #[serde(rename = "FwdPts", skip_serializing_if = "Option::is_none")]
    pub fwd_pts: Option<DecimalNumber>,
    #[validate]
    #[serde(rename = "ClctdCtrPtyCcyLastQty")]
    pub clctd_ctr_pty_ccy_last_qty: CurrencyAndAmount,
    #[validate]
    #[serde(rename = "ValDt")]
    pub val_dt: IsoDate,
    #[validate]
    #[serde(rename = "RskAmt")]
    pub rsk_amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "SctyId")]
    pub scty_id: SecurityIdentification18,
    #[serde(rename = "FxgCcy", skip_serializing_if = "Option::is_none")]
    pub fxg_ccy: Option<CurrencyCode>,
    #[serde(rename = "FxgDt", skip_serializing_if = "Option::is_none")]
    pub fxg_dt: Option<IsoDate>,
    #[serde(rename = "OptnInd", skip_serializing_if = "Option::is_none")]
    pub optn_ind: Option<YesNoIndicator>,
    #[serde(rename = "DltaInd", skip_serializing_if = "Option::is_none")]
    pub dlta_ind: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AssoctdTradRef", default)]
    pub assoctd_trad_ref: Vec<Max70Text>,
}
#[derive(
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
pub enum PartyType3Code {
    #[serde(rename = "OPOI")]
    Opoi,
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ITAG")]
    Itag,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "DLIS")]
    Dlis,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum IdentificationType2Code {
    #[serde(rename = "CDCO")]
    Cdco,
    #[serde(rename = "CFET")]
    Cfet,
    #[serde(rename = "RICC")]
    Ricc,
    #[serde(rename = "USDE")]
    Usde,
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "FXTradConfReq")]
    pub fx_trad_conf_req: ForeignExchangeTradeConfirmationRequestV01<A>,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ClearingMethod1Code {
    #[serde(rename = "GRNE")]
    Grne,
    #[serde(rename = "NEMA")]
    Nema,
    #[serde(rename = "NENE")]
    Nene,
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
pub struct SecurityIdentification18 {
    #[serde(rename = "SctyIdSrc")]
    pub scty_id_src: IdentificationType2Code,
    #[validate]
    #[serde(rename = "SctyId")]
    pub scty_id: Max35Text,
}
#[derive(
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
