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
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:fxtr.031.001.01".to_string()
}

#[derive(
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
pub struct PremiumQuote1ChoiceEnum {
    #[serde(rename = "PctgOfPutAmt", skip_serializing_if = "Option::is_none")]
    pub pctg_of_put_amt: Option<PercentageRate>,
    #[serde(rename = "PtsOfCallAmt", skip_serializing_if = "Option::is_none")]
    pub pts_of_call_amt: Option<BaseOneRate>,
    #[serde(rename = "PctgOfCallAmt", skip_serializing_if = "Option::is_none")]
    pub pctg_of_call_amt: Option<PercentageRate>,
    #[serde(rename = "PtsOfPutAmt", skip_serializing_if = "Option::is_none")]
    pub pts_of_put_amt: Option<BaseOneRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PremiumQuote1Choice {
    #[serde(flatten)]
    pub value: PremiumQuote1ChoiceEnum,
}
#[derive(
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
pub struct ForeignExchangeTradeCaptureReportV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Hdr")]
    pub hdr: Header23,
    #[serde(rename = "RptId", skip_serializing_if = "Option::is_none")]
    pub rpt_id: Option<MessageIdentification1>,
    #[serde(rename = "TradgSdId", skip_serializing_if = "Option::is_none")]
    pub tradg_sd_id: Option<TradePartyIdentification7>,
    #[serde(rename = "CtrPtySdId", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_sd_id: Option<TradePartyIdentification7>,
    #[serde(rename = "TradDtl", skip_serializing_if = "Option::is_none")]
    pub trad_dtl: Option<Trade1>,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<AdditionalReferences>,
    #[validate]
    #[serde(rename = "ReqRspndr")]
    pub req_rspndr: YesNoIndicator,
    #[serde(rename = "ReqRjctd", skip_serializing_if = "Option::is_none")]
    pub req_rjctd: Option<YesNoIndicator>,
    #[serde(rename = "QryRjctRsn", skip_serializing_if = "Option::is_none")]
    pub qry_rjct_rsn: Option<Max35Text>,
    #[serde(rename = "TtlNbTrds", skip_serializing_if = "Option::is_none")]
    pub ttl_nb_trds: Option<Number>,
    #[serde(rename = "LastRptReqd", skip_serializing_if = "Option::is_none")]
    pub last_rpt_reqd: Option<YesNoIndicator>,
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
pub struct PartyIdentification90 {
    #[serde(rename = "IdTp")]
    pub id_tp: PartyIdentificationType1Code,
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
pub struct TradePartyIdentification7 {
    #[serde(rename = "FndInf", skip_serializing_if = "Option::is_none")]
    pub fnd_inf: Option<FundIdentification3>,
    #[serde(rename = "BuyrOrSellrInd")]
    pub buyr_or_sellr_ind: OptionParty1Code,
    #[serde(rename = "InitrInd")]
    pub initr_ind: OptionParty3Code,
    #[validate]
    #[serde(rename = "TradPtyId")]
    pub trad_pty_id: PartyIdentification78,
    #[validate]
    #[serde(rename = "SubmitgPty")]
    pub submitg_pty: PartyIdentificationAndAccount119,
}
#[derive(
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
pub enum OptionParty1Code {
    #[serde(rename = "SLLR")]
    Sllr,
    #[serde(rename = "BYER")]
    Byer,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OrderStatus8Code {
    #[serde(rename = "CANC")]
    Canc,
    #[serde(rename = "NEWW")]
    Neww,
    #[serde(rename = "REPL")]
    Repl,
    #[serde(rename = "STOP")]
    Stop,
    #[serde(rename = "REJT")]
    Rejt,
    #[serde(rename = "EXPI")]
    Expi,
    #[serde(rename = "STNP")]
    Stnp,
    #[serde(rename = "RECE")]
    Rece,
    #[serde(rename = "CANP")]
    Canp,
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
pub struct SecurityIdentification22ChoiceEnum {
    #[serde(rename = "CTA", skip_serializing_if = "Option::is_none")]
    pub cta: Option<ConsolidatedTapeAssociationIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternateIdentification1>,
    #[serde(rename = "RIC", skip_serializing_if = "Option::is_none")]
    pub ric: Option<RicIdentifier>,
    #[serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none")]
    pub blmbrg: Option<BloombergIdentifier>,
    #[serde(rename = "Cmon", skip_serializing_if = "Option::is_none")]
    pub cmon: Option<EuroclearClearstreamIdentifier>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none")]
    pub tckr_symb: Option<TickerIdentifier>,
}
#[derive(
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
pub struct AdditionalReferences {
    #[validate]
    #[serde(rename = "Ref")]
    pub r#ref: Max35Text,
    #[serde(rename = "MsgNm", skip_serializing_if = "Option::is_none")]
    pub msg_nm: Option<Max35Text>,
    #[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
    pub ref_issr: Option<PartyIdentification>,
}
#[derive(
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
pub struct Trade1 {
    #[validate]
    #[serde(rename = "TradId")]
    pub trad_id: Max35Text,
    #[validate]
    #[serde(rename = "DtAndTm")]
    pub dt_and_tm: IsoDateTime,
    #[serde(rename = "FXTradPdct", skip_serializing_if = "Option::is_none")]
    pub fx_trad_pdct: Option<UnderlyingProductIdentifier1Code>,
    #[serde(rename = "TradgCcy", skip_serializing_if = "Option::is_none")]
    pub tradg_ccy: Option<CurrencyCode>,
    #[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
    pub sttlm_ccy: Option<CurrencyCode>,
    #[serde(rename = "TradgMtd")]
    pub tradg_mtd: TradingMethodType1Code,
    #[serde(rename = "TradgMd", skip_serializing_if = "Option::is_none")]
    pub tradg_md: Option<TradingModeType1Code>,
    #[serde(rename = "ClrMtd")]
    pub clr_mtd: ClearingMethod1Code,
    #[serde(rename = "ExctnTp")]
    pub exctn_tp: OrderStatus8Code,
    #[validate]
    #[serde(rename = "Symb")]
    pub symb: Max35Text,
    #[serde(rename = "PlcOfConf", skip_serializing_if = "Option::is_none")]
    pub plc_of_conf: Option<Max35Text>,
    #[serde(rename = "TxTm", skip_serializing_if = "Option::is_none")]
    pub tx_tm: Option<IsoDateTime>,
    #[serde(rename = "FXDtls", skip_serializing_if = "Option::is_none")]
    pub fx_dtls: Option<Trade3>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SwpLeg", default)]
    pub swp_leg: Vec<InstrumentLeg6>,
    #[serde(rename = "Optn", skip_serializing_if = "Option::is_none")]
    pub optn: Option<Option10>,
    #[serde(rename = "PdctId", skip_serializing_if = "Option::is_none")]
    pub pdct_id: Option<SecurityIdentification22Choice>,
}
#[derive(
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
pub struct ConsolidatedTapeAssociationIdentifier {
    #[validate(length(min = 1, max = 35,))]
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
pub struct PartyIdentification {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max35Text,
}
#[derive(
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
pub struct AccountIdentification30 {
    #[serde(rename = "AcctTp")]
    pub acct_tp: AccountInformationType1Code,
    #[validate]
    #[serde(rename = "Id")]
    pub id: AccountIdentification26,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification19ChoiceEnum {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<PartyIdentification44>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification19Choice {
    #[serde(flatten)]
    pub value: PartyIdentification19ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AccountInformationType1Code {
    #[serde(rename = "IBND")]
    Ibnd,
    #[serde(rename = "IBCC")]
    Ibcc,
    #[serde(rename = "IBDC")]
    Ibdc,
    #[serde(rename = "BIBC")]
    Bibc,
    #[serde(rename = "BIBD")]
    Bibd,
    #[serde(rename = "BINC")]
    Binc,
    #[serde(rename = "BIND")]
    Bind,
    #[serde(rename = "BICC")]
    Bicc,
    #[serde(rename = "BIDC")]
    Bidc,
    #[serde(rename = "CMSA")]
    Cmsa,
    #[serde(rename = "CBBC")]
    Cbbc,
    #[serde(rename = "CBBD")]
    Cbbd,
    #[serde(rename = "CBNC")]
    Cbnc,
    #[serde(rename = "CBND")]
    Cbnd,
    #[serde(rename = "CBCC")]
    Cbcc,
    #[serde(rename = "CBDC")]
    Cbdc,
    #[serde(rename = "CUAC")]
    Cuac,
    #[serde(rename = "DEAC")]
    Deac,
    #[serde(rename = "FCAA")]
    Fcaa,
    #[serde(rename = "FCAN")]
    Fcan,
    #[serde(rename = "FCBN")]
    Fcbn,
    #[serde(rename = "IBBC")]
    Ibbc,
    #[serde(rename = "IBBD")]
    Ibbd,
    #[serde(rename = "IBNC")]
    Ibnc,
    #[serde(rename = "MCAA")]
    Mcaa,
    #[serde(rename = "MCAN")]
    Mcan,
    #[serde(rename = "MCIC")]
    Mcic,
    #[serde(rename = "MCIN")]
    Mcin,
    #[serde(rename = "MSAA")]
    Msaa,
    #[serde(rename = "MSBN")]
    Msbn,
    #[serde(rename = "MCAD")]
    Mcad,
    #[serde(rename = "NODC")]
    Nodc,
    #[serde(rename = "SCAC")]
    Scac,
    #[serde(rename = "SCAA")]
    Scaa,
    #[serde(rename = "OMSA")]
    Omsa,
    #[serde(rename = "NOCC")]
    Nocc,
    #[serde(rename = "MSBS")]
    Msbs,
    #[serde(rename = "MSAN")]
    Msan,
    #[serde(rename = "SCAN")]
    Scan,
    #[serde(rename = "SCIC")]
    Scic,
    #[serde(rename = "SCIN")]
    Scin,
    #[serde(rename = "SOCA")]
    Soca,
    #[serde(rename = "SSCA")]
    Ssca,
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
pub struct MessageIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
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
pub struct PartyIdentification44 {
    #[validate]
    #[serde(rename = "AnyBIC")]
    pub any_bic: AnyBicIdentifier,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "AltrntvIdr", default)]
    pub altrntv_idr: Vec<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PremiumAmount3 {
    #[serde(rename = "PrmQt")]
    pub prm_qt: PremiumQuote1Choice,
    #[serde(rename = "PrmCcy")]
    pub prm_ccy: ActiveOrHistoricCurrencyCode,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "DcmlPlcs")]
    pub dcml_plcs: Number,
    #[validate]
    #[serde(rename = "PrmSttlmDt")]
    pub prm_sttlm_dt: IsoDate,
    #[validate]
    #[serde(rename = "PyerPtyRef")]
    pub pyer_pty_ref: Max35Text,
    #[validate]
    #[serde(rename = "RcvrPtyRef")]
    pub rcvr_pty_ref: Max35Text,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DerivativeExerciseStatus1Code {
    #[serde(rename = "EXEC")]
    Exec,
    #[serde(rename = "EXPI")]
    Expi,
    #[serde(rename = "VALI")]
    Vali,
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
pub struct AmountsAndValueDate4 {
    #[validate]
    #[serde(rename = "CallAmt")]
    pub call_amt: ActiveOrHistoricCurrencyAndAmount,
    #[validate]
    #[serde(rename = "PutAmt")]
    pub put_amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "OptnSttlmCcy", skip_serializing_if = "Option::is_none")]
    pub optn_sttlm_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[validate]
    #[serde(rename = "FnlSttlmDt")]
    pub fnl_sttlm_dt: IsoDate,
}
#[derive(
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
pub struct Max70Text {
    #[validate(length(min = 1, max = 70,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionPayoutType1Code {
    #[serde(rename = "BINA")]
    Bina,
    #[serde(rename = "CAPP")]
    Capp,
    #[serde(rename = "VANI")]
    Vani,
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
pub struct Option10 {
    #[serde(rename = "Data")]
    pub data: DataType1Code,
    #[serde(rename = "ExrcSts")]
    pub exrc_sts: DerivativeExerciseStatus1Code,
    #[serde(rename = "ExrcStyle")]
    pub exrc_style: OptionStyle2Code,
    #[serde(rename = "OptnTp")]
    pub optn_tp: OptionType1Code,
    #[validate]
    #[serde(rename = "DerivOptnId")]
    pub deriv_optn_id: Max35Text,
    #[serde(rename = "OptnPyoutTp")]
    pub optn_pyout_tp: OptionPayoutType1Code,
    #[validate]
    #[serde(rename = "ValtnRate")]
    pub valtn_rate: AgreedRate3,
    #[validate]
    #[serde(rename = "StrkPric")]
    pub strk_pric: AgreedRate3,
    #[validate]
    #[serde(rename = "VoltlyMrgn")]
    pub voltly_mrgn: PercentageRate,
    #[validate]
    #[serde(rename = "RskAmt")]
    pub rsk_amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "XpryDtAndTm")]
    pub xpry_dt_and_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "XpryLctn")]
    pub xpry_lctn: Max4AlphaNumericText,
    #[serde(rename = "SttlmTp")]
    pub sttlm_tp: SettlementDateCode,
    #[validate]
    #[serde(rename = "OptnAmts")]
    pub optn_amts: AmountsAndValueDate4,
    #[validate]
    #[serde(rename = "Prm")]
    pub prm: PremiumAmount3,
    #[serde(rename = "SttlmAmtTp")]
    pub sttlm_amt_tp: SettlementType1Code,
    #[validate]
    #[serde(rename = "AddtlOptnInf")]
    pub addtl_optn_inf: Max140Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum IdentificationType1Code {
    #[serde(rename = "BASC")]
    Basc,
    #[serde(rename = "BICO")]
    Bico,
    #[serde(rename = "CFET")]
    Cfet,
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "FXTradCaptrRpt")]
    pub fx_trad_captr_rpt: ForeignExchangeTradeCaptureReportV01<A>,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DataType1Code {
    #[serde(rename = "EXDA")]
    Exda,
    #[serde(rename = "TRDA")]
    Trda,
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
pub struct NameAndAddress8 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<PostalAddress1>,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "AltrntvIdr", default)]
    pub altrntv_idr: Vec<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount119 {
    #[validate(length(min = 1,))]
    #[serde(rename = "PtyId", default)]
    pub pty_id: Vec<PartyIdentification90>,
    #[validate(length(min = 1,))]
    #[serde(rename = "AcctId", default)]
    pub acct_id: Vec<AccountIdentification30>,
}
#[derive(
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
pub struct PartyIdentification78 {
    #[serde(rename = "PtySrc", skip_serializing_if = "Option::is_none")]
    pub pty_src: Option<IdentificationType1Code>,
    #[validate]
    #[serde(rename = "TradPtyId")]
    pub trad_pty_id: Max35Text,
}
#[derive(
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
pub struct AccountIdentification26 {
    #[validate]
    #[serde(rename = "Prtry")]
    pub prtry: SimpleIdentificationInformation4,
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
pub struct FundIdentification3 {
    #[validate]
    #[serde(rename = "FndId")]
    pub fnd_id: Max35Text,
    #[serde(rename = "AcctIdWthCtdn", skip_serializing_if = "Option::is_none")]
    pub acct_id_wth_ctdn: Option<Max35Text>,
    #[serde(rename = "CtdnId", skip_serializing_if = "Option::is_none")]
    pub ctdn_id: Option<PartyIdentification19Choice>,
}
#[derive(
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
pub struct PercentageRate {
    #[serde(rename = "$text")]
    pub value: f64,
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
pub enum PartyIdentificationType1Code {
    #[serde(rename = "FXID")]
    Fxid,
    #[serde(rename = "FXSN")]
    Fxsn,
    #[serde(rename = "INGN")]
    Ingn,
    #[serde(rename = "IICS")]
    Iics,
    #[serde(rename = "IGBT")]
    Igbt,
    #[serde(rename = "MAMA")]
    Mama,
    #[serde(rename = "MEOC")]
    Meoc,
    #[serde(rename = "METY")]
    Mety,
    #[serde(rename = "NOMM")]
    Nomm,
    #[serde(rename = "OSCO")]
    Osco,
    #[serde(rename = "PASS")]
    Pass,
    #[serde(rename = "PONU")]
    Ponu,
    #[serde(rename = "POAD")]
    Poad,
    #[serde(rename = "RMID")]
    Rmid,
    #[serde(rename = "SLCN")]
    Slcn,
    #[serde(rename = "SLNF")]
    Slnf,
    #[serde(rename = "TACN")]
    Tacn,
    #[serde(rename = "TRCO")]
    Trco,
    #[serde(rename = "TANA")]
    Tana,
    #[serde(rename = "USIT")]
    Usit,
    #[serde(rename = "USNA")]
    Usna,
    #[serde(rename = "AUIT")]
    Auit,
    #[serde(rename = "BRID")]
    Brid,
    #[serde(rename = "CLIN")]
    Clin,
    #[serde(rename = "CMID")]
    Cmid,
    #[serde(rename = "COIN")]
    Coin,
    #[serde(rename = "CMOT")]
    Cmot,
    #[serde(rename = "CONU")]
    Conu,
    #[serde(rename = "CMIN")]
    Cmin,
    #[serde(rename = "DECN")]
    Decn,
    #[serde(rename = "DEPA")]
    Depa,
    #[serde(rename = "ELCO")]
    Elco,
    #[serde(rename = "EXVE")]
    Exve,
    #[serde(rename = "FICO")]
    Fico,
    #[serde(rename = "FIID")]
    Fiid,
    #[serde(rename = "FLCN")]
    Flcn,
    #[serde(rename = "FLNF")]
    Flnf,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionParty3Code {
    #[serde(rename = "MAKE")]
    Make,
    #[serde(rename = "TAKE")]
    Take,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OptionStyle2Code {
    #[serde(rename = "AMER")]
    Amer,
    #[serde(rename = "EURO")]
    Euro,
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
pub struct CountryCode {
    #[validate(regex = "COUNTRY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
