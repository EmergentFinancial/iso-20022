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
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:seev.042.001.11".to_string()
}

#[derive(
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
pub struct PendingCancellationStatusReason7 {
    #[serde(rename = "RsnCd")]
    pub rsn_cd: PendingCancellationReason5Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<Max210Text>,
}
#[derive(
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
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
    #[serde(rename = "DgtlTknUnit", skip_serializing_if = "Option::is_none")]
    pub dgtl_tkn_unit: Option<Max30DecimalNumber>,
}
#[derive(
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
pub struct CorporateActionBalance46 {
    #[serde(rename = "TtlElgblBal")]
    pub ttl_elgbl_bal: Quantity49Choice,
    #[serde(rename = "UinstdBal")]
    pub uinstd_bal: BalanceFormat11Choice,
    #[validate]
    #[serde(rename = "TtlInstdBalDtls")]
    pub ttl_instd_bal_dtls: InstructedBalance16,
    #[serde(rename = "BlckdBal", skip_serializing_if = "Option::is_none")]
    pub blckd_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "BrrwdBal", skip_serializing_if = "Option::is_none")]
    pub brrwd_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "CollInBal", skip_serializing_if = "Option::is_none")]
    pub coll_in_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "CollOutBal", skip_serializing_if = "Option::is_none")]
    pub coll_out_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "OnLnBal", skip_serializing_if = "Option::is_none")]
    pub on_ln_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "OutForRegnBal", skip_serializing_if = "Option::is_none")]
    pub out_for_regn_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "SttlmPosBal", skip_serializing_if = "Option::is_none")]
    pub sttlm_pos_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "StrtPosBal", skip_serializing_if = "Option::is_none")]
    pub strt_pos_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "TradDtPosBal", skip_serializing_if = "Option::is_none")]
    pub trad_dt_pos_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "InTrnsShipmntBal", skip_serializing_if = "Option::is_none")]
    pub in_trns_shipmnt_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "RegdBal", skip_serializing_if = "Option::is_none")]
    pub regd_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "OblgtdBal", skip_serializing_if = "Option::is_none")]
    pub oblgtd_bal: Option<SignedQuantityFormat10>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PdgDlvryBal", default)]
    pub pdg_dlvry_bal: Vec<PendingBalance7>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PdgRctBal", default)]
    pub pdg_rct_bal: Vec<PendingBalance7>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionOption30ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionOption11Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionOption30Choice {
    #[serde(flatten)]
    pub value: CorporateActionOption30ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionStatementReportingType1Code {
    #[serde(rename = "MASE")]
    Mase,
    #[serde(rename = "SAME")]
    Same,
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
pub struct OriginalAndCurrentQuantities6 {
    #[serde(rename = "ShrtLngPos")]
    pub shrt_lng_pos: ShortLong1Code,
    #[validate]
    #[serde(rename = "FaceAmt")]
    pub face_amt: ImpliedCurrencyAndAmount,
    #[validate]
    #[serde(rename = "AmtsdVal")]
    pub amtsd_val: ImpliedCurrencyAndAmount,
}
#[derive(
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
pub struct PendingBalance7 {
    #[validate]
    #[serde(rename = "Bal")]
    pub bal: SignedQuantityFormat10,
    #[validate(length(min = 0,))]
    #[serde(rename = "PdgTxs", default)]
    pub pdg_txs: Vec<SettlementTypeAndIdentification25>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PercentagePrice1 {
    #[serde(rename = "PctgPricTp")]
    pub pctg_pric_tp: PriceRateType3Code,
    #[validate]
    #[serde(rename = "PricVal")]
    pub pric_val: PercentageRate,
}
#[derive(
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
pub struct EventInformation15 {
    #[validate]
    #[serde(rename = "CorpActnEvtId")]
    pub corp_actn_evt_id: Max35Text,
    #[serde(rename = "OffclCorpActnEvtId", skip_serializing_if = "Option::is_none")]
    pub offcl_corp_actn_evt_id: Option<Max35Text>,
    #[serde(rename = "EvtTp")]
    pub evt_tp: CorporateActionEventType102Choice,
    #[serde(rename = "MndtryVlntryEvtTp")]
    pub mndtry_vlntry_evt_tp: CorporateActionMandatoryVoluntary3Choice,
    #[serde(rename = "LastNtfctnId", skip_serializing_if = "Option::is_none")]
    pub last_ntfctn_id: Option<NotificationIdentification5>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CancelledStatusReason6Code {
    #[serde(rename = "CANI")]
    Cani,
    #[serde(rename = "CANO")]
    Cano,
    #[serde(rename = "CANS")]
    Cans,
    #[serde(rename = "CSUB")]
    Csub,
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
pub struct DateCodeAndTimeFormat3 {
    #[serde(rename = "DtCd")]
    pub dt_cd: DateCode21Choice,
    #[validate]
    #[serde(rename = "Tm")]
    pub tm: IsoTime,
}
#[derive(
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
pub struct ProprietaryQuantity7 {
    #[serde(rename = "ShrtLngPos", skip_serializing_if = "Option::is_none")]
    pub shrt_lng_pos: Option<ShortLong1Code>,
    #[validate]
    #[serde(rename = "Qty")]
    pub qty: DecimalNumber,
    #[validate]
    #[serde(rename = "QtyTp")]
    pub qty_tp: Exact4AlphaNumericText,
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
pub struct ProprietaryQuantity8 {
    #[validate]
    #[serde(rename = "Qty")]
    pub qty: DecimalNumber,
    #[validate]
    #[serde(rename = "QtyTp")]
    pub qty_tp: Exact4AlphaNumericText,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SafekeepingPlace2Code {
    #[serde(rename = "SHHE")]
    Shhe,
    #[serde(rename = "ALLP")]
    Allp,
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
pub struct CorporateActionEventType102ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventType34Code>,
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
pub struct CorporateActionEventType102Choice {
    #[serde(flatten)]
    pub value: CorporateActionEventType102ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementTypeAndIdentification25 {
    #[serde(rename = "Pmt")]
    pub pmt: DeliveryReceiptType2Code,
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: Max35Text,
    #[serde(rename = "SttlmDt", skip_serializing_if = "Option::is_none")]
    pub sttlm_dt: Option<DateAndDateTime2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity48ChoiceEnum {
    #[serde(rename = "PrtryQty", skip_serializing_if = "Option::is_none")]
    pub prtry_qty: Option<ProprietaryQuantity8>,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<FinancialInstrumentQuantity33Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity48Choice {
    #[serde(flatten)]
    pub value: Quantity48ChoiceEnum,
}
#[derive(
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
pub enum DateType7Code {
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
pub struct SignedQuantityFormat10 {
    #[serde(rename = "ShrtLngPos")]
    pub shrt_lng_pos: ShortLong1Code,
    #[serde(rename = "Qty")]
    pub qty: FinancialInstrumentQuantity33Choice,
}
#[derive(
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
pub struct DatePeriod2 {
    #[validate]
    #[serde(rename = "FrDt")]
    pub fr_dt: IsoDate,
    #[validate]
    #[serde(rename = "ToDt")]
    pub to_dt: IsoDate,
}
#[derive(
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
    #[serde(rename = "CorpActnInstrStmtRpt")]
    pub corp_actn_instr_stmt_rpt: CorporateActionInstructionStatementReportV11<A, B>,
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
pub struct IsoTime {
    #[serde(rename = "$value")]
    pub value: ::chrono::naive::NaiveTime,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max15Text {
    #[validate(length(min = 1, max = 15,))]
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
pub struct PendingCancellationReason5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PendingCancellationReason5Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingCancellationReason5Choice {
    #[serde(flatten)]
    pub value: PendingCancellationReason5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InstructedBalance16 {
    #[serde(rename = "TtlInstdBal")]
    pub ttl_instd_bal: BalanceFormat11Choice,
    #[serde(rename = "TtlAccptdInstrBal", skip_serializing_if = "Option::is_none")]
    pub ttl_accptd_instr_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "TtlCancInstrBal", skip_serializing_if = "Option::is_none")]
    pub ttl_canc_instr_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "TtlPdgInstrBal", skip_serializing_if = "Option::is_none")]
    pub ttl_pdg_instr_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "TtlRjctdInstrBal", skip_serializing_if = "Option::is_none")]
    pub ttl_rjctd_instr_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "TtlPrtctInstrBal", skip_serializing_if = "Option::is_none")]
    pub ttl_prtct_instr_bal: Option<SignedQuantityFormat10>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OptnDtls", default)]
    pub optn_dtls: Vec<InstructedCorporateActionOption17>,
}
#[derive(
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
pub struct Quantity49ChoiceEnum {
    #[serde(rename = "QtyChc", skip_serializing_if = "Option::is_none")]
    pub qty_chc: Option<Quantity50Choice>,
    #[serde(rename = "PrtryQty", skip_serializing_if = "Option::is_none")]
    pub prtry_qty: Option<ProprietaryQuantity7>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity49Choice {
    #[serde(flatten)]
    pub value: Quantity49ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SignedQuantityFormat11 {
    #[serde(rename = "ShrtLngPos")]
    pub shrt_lng_pos: ShortLong1Code,
    #[serde(rename = "QtyChc")]
    pub qty_chc: Quantity48Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceValueType10Code {
    #[serde(rename = "UKWN")]
    Ukwn,
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
pub struct Statement72 {
    #[serde(rename = "StmtTp")]
    pub stmt_tp: CorporateActionStatementType2Code,
    #[serde(rename = "RptgTp")]
    pub rptg_tp: CorporateActionStatementReportingType1Code,
    #[validate]
    #[serde(rename = "StmtId")]
    pub stmt_id: Max35Text,
    #[serde(rename = "InstrAggtnPrd", skip_serializing_if = "Option::is_none")]
    pub instr_aggtn_prd: Option<DatePeriod2>,
    #[serde(rename = "RptNb", skip_serializing_if = "Option::is_none")]
    pub rpt_nb: Option<Max5NumericText>,
    #[serde(rename = "StmtDtTm")]
    pub stmt_dt_tm: DateAndDateTime2Choice,
    #[serde(rename = "Frqcy")]
    pub frqcy: Frequency25Choice,
    #[serde(rename = "UpdTp")]
    pub upd_tp: UpdateType15Choice,
    #[validate]
    #[serde(rename = "ActvtyInd")]
    pub actvty_ind: YesNoIndicator,
    #[serde(rename = "NtfctnDdlnPrd", skip_serializing_if = "Option::is_none")]
    pub ntfctn_ddln_prd: Option<DateOrDateTimePeriod1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventDeadlines3 {
    #[serde(rename = "EarlyRspnDdln", skip_serializing_if = "Option::is_none")]
    pub early_rspn_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "RspnDdln", skip_serializing_if = "Option::is_none")]
    pub rspn_ddln: Option<DateFormat44Choice>,
    #[serde(rename = "MktDdln", skip_serializing_if = "Option::is_none")]
    pub mkt_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "PrtctDdln", skip_serializing_if = "Option::is_none")]
    pub prtct_ddln: Option<DateFormat43Choice>,
    #[serde(rename = "CoverPrtctDdln", skip_serializing_if = "Option::is_none")]
    pub cover_prtct_ddln: Option<DateFormat43Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventAndBalance22<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "GnlInf")]
    pub gnl_inf: EventInformation15,
    #[validate]
    #[serde(rename = "UndrlygScty")]
    pub undrlyg_scty: SecurityIdentification19,
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<CorporateActionBalance46>,
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
pub struct YesNoIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionMandatoryVoluntary1Code {
    #[serde(rename = "MAND")]
    Mand,
    #[serde(rename = "CHOS")]
    Chos,
    #[serde(rename = "VOLU")]
    Volu,
    #[default]
    Unknown,
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
pub struct AmountPrice3 {
    #[serde(rename = "AmtPricTp")]
    pub amt_pric_tp: AmountPriceType1Code,
    #[validate]
    #[serde(rename = "PricVal")]
    pub pric_val: ActiveCurrencyAnd13DecimalAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateOrDateTimePeriod1ChoiceEnum {
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<DateTimePeriod1>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DatePeriod2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateOrDateTimePeriod1Choice {
    #[serde(flatten)]
    pub value: DateOrDateTimePeriod1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Exact3NumericText {
    #[validate(regex = "EXACT_3_NUMERIC_TEXT_REGEX")]
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
pub struct Frequency25ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EventFrequency4Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Frequency25Choice {
    #[serde(flatten)]
    pub value: Frequency25ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectedStatusReason41 {
    #[serde(rename = "RsnCd")]
    pub rsn_cd: RejectedReason43Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<Max210Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PendingCancellationReason5Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "DQCS")]
    Dqcs,
    #[serde(rename = "LATE")]
    Late,
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
pub struct Max210Text {
    #[validate(length(min = 1, max = 210,))]
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
pub struct OptionInstructionDetails7 {
    #[validate]
    #[serde(rename = "InstrId")]
    pub instr_id: Max15Text,
    #[serde(rename = "InstrSeqNb", skip_serializing_if = "Option::is_none")]
    pub instr_seq_nb: Option<Max3NumericText>,
    #[serde(rename = "PrtctInd", skip_serializing_if = "Option::is_none")]
    pub prtct_ind: Option<ProtectTransactionType2Code>,
    #[serde(rename = "InstrQty")]
    pub instr_qty: FinancialInstrumentQuantity33Choice,
    #[validate]
    #[serde(rename = "InstrDt")]
    pub instr_dt: IsoDate,
    #[serde(rename = "PrtctDt", skip_serializing_if = "Option::is_none")]
    pub prtct_dt: Option<IsoDate>,
    #[serde(rename = "CoverPrtctDt", skip_serializing_if = "Option::is_none")]
    pub cover_prtct_dt: Option<IsoDate>,
    #[serde(rename = "BidPric", skip_serializing_if = "Option::is_none")]
    pub bid_pric: Option<PriceFormat45Choice>,
    #[serde(rename = "CondlQty", skip_serializing_if = "Option::is_none")]
    pub condl_qty: Option<FinancialInstrumentQuantity33Choice>,
    #[serde(rename = "CstmrRef", skip_serializing_if = "Option::is_none")]
    pub cstmr_ref: Option<Max50Text>,
    #[serde(rename = "InstrNrrtv", skip_serializing_if = "Option::is_none")]
    pub instr_nrrtv: Option<Max350Text>,
    #[serde(rename = "InstrSts")]
    pub instr_sts: InstructionProcessingStatus47Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification58<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<Max35Text>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<Max140Text>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<PartyIdentification127Choice>,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<SafekeepingPlaceFormat28Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CorpActnEvtAndBal", default)]
    pub corp_actn_evt_and_bal: Vec<CorporateActionEventAndBalance22<A>>,
}
#[derive(
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
pub struct DateFormat44ChoiceEnum {
    #[serde(rename = "DtCdAndTm", skip_serializing_if = "Option::is_none")]
    pub dt_cd_and_tm: Option<DateCodeAndTimeFormat3>,
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<DateCode19Choice>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat44Choice {
    #[serde(flatten)]
    pub value: DateFormat44ChoiceEnum,
}
#[derive(
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
pub struct NotificationIdentification5 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm: Option<DateAndDateTime2Choice>,
}
#[derive(
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
pub struct PendingCancellationStatus7ChoiceEnum {
    #[serde(rename = "NotSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<PendingCancellationStatusReason7>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingCancellationStatus7Choice {
    #[serde(flatten)]
    pub value: PendingCancellationStatus7ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceTypeAndText6 {
    #[serde(rename = "SfkpgPlcTp")]
    pub sfkpg_plc_tp: SafekeepingPlace2Code,
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
pub struct PriceFormat45ChoiceEnum {
    #[serde(rename = "AmtPric", skip_serializing_if = "Option::is_none")]
    pub amt_pric: Option<AmountPrice3>,
    #[serde(rename = "NotSpcfdPric", skip_serializing_if = "Option::is_none")]
    pub not_spcfd_pric: Option<PriceValueType10Code>,
    #[serde(rename = "PctgPric", skip_serializing_if = "Option::is_none")]
    pub pctg_pric: Option<PercentagePrice1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceFormat45Choice {
    #[serde(flatten)]
    pub value: PriceFormat45ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionInstructionStatementReportV11<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Pgntn")]
    pub pgntn: Pagination1,
    #[validate]
    #[serde(rename = "StmtGnlDtls")]
    pub stmt_gnl_dtls: Statement72,
    #[validate(length(min = 1,))]
    #[serde(rename = "AcctAndStmtDtls", default)]
    pub acct_and_stmt_dtls: Vec<AccountIdentification58<A>>,
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
pub struct RejectedStatus43ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<RejectedStatusReason41>,
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
pub struct RejectedStatus43Choice {
    #[serde(flatten)]
    pub value: RejectedStatus43ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum StatementUpdateType1Code {
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "DELT")]
    Delt,
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
pub struct PartyIdentification127ChoiceEnum {
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
pub struct PartyIdentification127Choice {
    #[serde(flatten)]
    pub value: PartyIdentification127ChoiceEnum,
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
pub struct DateFormat43ChoiceEnum {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<DateCode19Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat43Choice {
    #[serde(flatten)]
    pub value: DateFormat43ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DefaultProcessingOrStandingInstruction1ChoiceEnum {
    #[serde(rename = "StgInstrInd", skip_serializing_if = "Option::is_none")]
    pub stg_instr_ind: Option<YesNoIndicator>,
    #[serde(rename = "DfltOptnInd", skip_serializing_if = "Option::is_none")]
    pub dflt_optn_ind: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DefaultProcessingOrStandingInstruction1Choice {
    #[serde(flatten)]
    pub value: DefaultProcessingOrStandingInstruction1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancelledReason8ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CancelledStatusReason6Code>,
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
pub struct CancelledReason8Choice {
    #[serde(flatten)]
    pub value: CancelledReason8ChoiceEnum,
}
#[derive(
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
pub struct DateCode19ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType8Code>,
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
pub struct DateCode19Choice {
    #[serde(flatten)]
    pub value: DateCode19ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity50ChoiceEnum {
    #[serde(rename = "OrgnlAndCurFaceAmt", skip_serializing_if = "Option::is_none")]
    pub orgnl_and_cur_face_amt: Option<OriginalAndCurrentQuantities6>,
    #[serde(rename = "SgndQty", skip_serializing_if = "Option::is_none")]
    pub sgnd_qty: Option<SignedQuantityFormat10>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Quantity50Choice {
    #[serde(flatten)]
    pub value: Quantity50ChoiceEnum,
}
#[derive(
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
pub struct SafekeepingPlaceFormat28ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification78>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<SafekeepingPlaceTypeAndText6>,
    #[serde(rename = "TpAndId", skip_serializing_if = "Option::is_none")]
    pub tp_and_id: Option<SafekeepingPlaceTypeAndIdentification1>,
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
pub struct SafekeepingPlaceFormat28Choice {
    #[serde(flatten)]
    pub value: SafekeepingPlaceFormat28ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionEventType34Code {
    #[serde(rename = "ACTV")]
    Actv,
    #[serde(rename = "ATTI")]
    Atti,
    #[serde(rename = "BRUP")]
    Brup,
    #[serde(rename = "DFLT")]
    Dflt,
    #[serde(rename = "BONU")]
    Bonu,
    #[serde(rename = "EXRI")]
    Exri,
    #[serde(rename = "CAPD")]
    Capd,
    #[serde(rename = "CAPG")]
    Capg,
    #[serde(rename = "CAPI")]
    Capi,
    #[serde(rename = "DRCA")]
    Drca,
    #[serde(rename = "DVCA")]
    Dvca,
    #[serde(rename = "CHAN")]
    Chan,
    #[serde(rename = "COOP")]
    Coop,
    #[serde(rename = "CLSA")]
    Clsa,
    #[serde(rename = "CONS")]
    Cons,
    #[serde(rename = "CONV")]
    Conv,
    #[serde(rename = "CREV")]
    Crev,
    #[serde(rename = "DECR")]
    Decr,
    #[serde(rename = "DETI")]
    Deti,
    #[serde(rename = "DSCL")]
    Dscl,
    #[serde(rename = "DVOP")]
    Dvop,
    #[serde(rename = "DRIP")]
    Drip,
    #[serde(rename = "DRAW")]
    Draw,
    #[serde(rename = "DTCH")]
    Dtch,
    #[serde(rename = "EXOF")]
    Exof,
    #[serde(rename = "REDM")]
    Redm,
    #[serde(rename = "MCAL")]
    Mcal,
    #[serde(rename = "INCR")]
    Incr,
    #[serde(rename = "PPMT")]
    Ppmt,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "RHDI")]
    Rhdi,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "EXTM")]
    Extm,
    #[serde(rename = "MRGR")]
    Mrgr,
    #[serde(rename = "NOOF")]
    Noof,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "ODLT")]
    Odlt,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PARI")]
    Pari,
    #[serde(rename = "PCAL")]
    Pcal,
    #[serde(rename = "PRED")]
    Pred,
    #[serde(rename = "PINK")]
    Pink,
    #[serde(rename = "PLAC")]
    Plac,
    #[serde(rename = "PDEF")]
    Pdef,
    #[serde(rename = "PRIO")]
    Prio,
    #[serde(rename = "BPUT")]
    Bput,
    #[serde(rename = "REDO")]
    Redo,
    #[serde(rename = "REMK")]
    Remk,
    #[serde(rename = "BIDS")]
    Bids,
    #[serde(rename = "SPLR")]
    Splr,
    #[serde(rename = "RHTS")]
    Rhts,
    #[serde(rename = "DVSC")]
    Dvsc,
    #[serde(rename = "SHPR")]
    Shpr,
    #[serde(rename = "SMAL")]
    Smal,
    #[serde(rename = "SOFF")]
    Soff,
    #[serde(rename = "DVSE")]
    Dvse,
    #[serde(rename = "SPLF")]
    Splf,
    #[serde(rename = "TREC")]
    Trec,
    #[serde(rename = "TEND")]
    Tend,
    #[serde(rename = "DLST")]
    Dlst,
    #[serde(rename = "SUSP")]
    Susp,
    #[serde(rename = "EXWA")]
    Exwa,
    #[serde(rename = "WTRC")]
    Wtrc,
    #[serde(rename = "WRTH")]
    Wrth,
    #[serde(rename = "ACCU")]
    Accu,
    #[default]
    Unknown,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionStatementType2Code {
    #[serde(rename = "MISS")]
    Miss,
    #[serde(rename = "ALLL")]
    Alll,
    #[serde(rename = "BALO")]
    Balo,
    #[serde(rename = "BALI")]
    Bali,
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
pub struct InstructionProcessingStatus47ChoiceEnum {
    #[serde(rename = "Accptd", skip_serializing_if = "Option::is_none")]
    pub accptd: Option<NoSpecifiedReason1>,
    #[serde(rename = "Canc", skip_serializing_if = "Option::is_none")]
    pub canc: Option<CancelledStatus12Choice>,
    #[serde(rename = "Pdg", skip_serializing_if = "Option::is_none")]
    pub pdg: Option<NoSpecifiedReason1>,
    #[serde(rename = "PdgCxl", skip_serializing_if = "Option::is_none")]
    pub pdg_cxl: Option<PendingCancellationStatus7Choice>,
    #[serde(rename = "AccptdForFrthrPrcg", skip_serializing_if = "Option::is_none")]
    pub accptd_for_frthr_prcg: Option<NoSpecifiedReason1>,
    #[serde(rename = "Ucvrd", skip_serializing_if = "Option::is_none")]
    pub ucvrd: Option<NoSpecifiedReason1>,
    #[serde(rename = "Cvrd", skip_serializing_if = "Option::is_none")]
    pub cvrd: Option<NoSpecifiedReason1>,
    #[serde(rename = "Rjctd", skip_serializing_if = "Option::is_none")]
    pub rjctd: Option<RejectedStatus43Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InstructionProcessingStatus47Choice {
    #[serde(flatten)]
    pub value: InstructionProcessingStatus47ChoiceEnum,
}
#[derive(
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
pub struct CancelledStatusReason11 {
    #[serde(rename = "RsnCd")]
    pub rsn_cd: CancelledReason8Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<Max210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BalanceFormat11ChoiceEnum {
    #[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<SignedQuantityFormat11>,
    #[serde(rename = "ElgblBal", skip_serializing_if = "Option::is_none")]
    pub elgbl_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "NotElgblBal", skip_serializing_if = "Option::is_none")]
    pub not_elgbl_bal: Option<SignedQuantityFormat10>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BalanceFormat11Choice {
    #[serde(flatten)]
    pub value: BalanceFormat11ChoiceEnum,
}
#[derive(
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
pub struct RejectedReason43ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RejectionReason64Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectedReason43Choice {
    #[serde(flatten)]
    pub value: RejectedReason43ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RejectionReason64Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "INVA")]
    Inva,
    #[serde(rename = "OPTY")]
    Opty,
    #[serde(rename = "ULNK")]
    Ulnk,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "LACK")]
    Lack,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "NMTY")]
    Nmty,
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "CANC")]
    Canc,
    #[serde(rename = "INTV")]
    Intv,
    #[serde(rename = "OPNM")]
    Opnm,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "REFT")]
    Reft,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "EVNM")]
    Evnm,
    #[serde(rename = "DQCS")]
    Dqcs,
    #[serde(rename = "DQCC")]
    Dqcc,
    #[serde(rename = "DQAM")]
    Dqam,
    #[serde(rename = "IRDQ")]
    Irdq,
    #[serde(rename = "DQBV")]
    Dqbv,
    #[serde(rename = "DQBI")]
    Dqbi,
    #[serde(rename = "DCAN")]
    Dcan,
    #[serde(rename = "DPRG")]
    Dprg,
    #[serde(rename = "INIR")]
    Inir,
    #[serde(rename = "SHAR")]
    Shar,
    #[serde(rename = "BSTR")]
    Bstr,
    #[serde(rename = "CTCT")]
    Ctct,
    #[serde(rename = "DUPL")]
    Dupl,
    #[serde(rename = "PROI")]
    Proi,
    #[serde(rename = "PROT")]
    Prot,
    #[serde(rename = "PRON")]
    Pron,
    #[serde(rename = "TRTI")]
    Trti,
    #[serde(rename = "REJA")]
    Reja,
    #[serde(rename = "IPAW")]
    Ipaw,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EventFrequency4Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "ADHO")]
    Adho,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "WEEK")]
    Week,
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
pub struct InstructedCorporateActionOption17 {
    #[serde(rename = "OptnNb", skip_serializing_if = "Option::is_none")]
    pub optn_nb: Option<Exact3NumericText>,
    #[serde(rename = "OptnTp")]
    pub optn_tp: CorporateActionOption30Choice,
    #[serde(rename = "InstdBal")]
    pub instd_bal: BalanceFormat11Choice,
    #[serde(rename = "DfltActn", skip_serializing_if = "Option::is_none")]
    pub dflt_actn: Option<DefaultProcessingOrStandingInstruction1Choice>,
    #[serde(rename = "OptnAccptdInstdBal", skip_serializing_if = "Option::is_none")]
    pub optn_accptd_instd_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "OptnCancInstrBal", skip_serializing_if = "Option::is_none")]
    pub optn_canc_instr_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "OptnPdgInstrBal", skip_serializing_if = "Option::is_none")]
    pub optn_pdg_instr_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "OptnRjctdInstrBal", skip_serializing_if = "Option::is_none")]
    pub optn_rjctd_instr_bal: Option<SignedQuantityFormat10>,
    #[serde(rename = "OptnPrtctInstrBal", skip_serializing_if = "Option::is_none")]
    pub optn_prtct_instr_bal: Option<SignedQuantityFormat10>,
    #[validate]
    #[serde(rename = "EvtDdlns")]
    pub evt_ddlns: CorporateActionEventDeadlines3,
    #[validate(length(min = 0,))]
    #[serde(rename = "OptnInstrDtls", default)]
    pub optn_instr_dtls: Vec<OptionInstructionDetails7>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AmountPriceType1Code {
    #[serde(rename = "ACTU")]
    Actu,
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "PLOT")]
    Plot,
    #[serde(rename = "PREM")]
    Prem,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionOption11Code {
    #[serde(rename = "ABST")]
    Abst,
    #[serde(rename = "BSPL")]
    Bspl,
    #[serde(rename = "BUYA")]
    Buya,
    #[serde(rename = "CASE")]
    Case,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CEXC")]
    Cexc,
    #[serde(rename = "CONN")]
    Conn,
    #[serde(rename = "CONY")]
    Cony,
    #[serde(rename = "CTEN")]
    Cten,
    #[serde(rename = "EXER")]
    Exer,
    #[serde(rename = "LAPS")]
    Laps,
    #[serde(rename = "MPUT")]
    Mput,
    #[serde(rename = "NOAC")]
    Noac,
    #[serde(rename = "NOQU")]
    Noqu,
    #[serde(rename = "OFFR")]
    Offr,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "OVER")]
    Over,
    #[serde(rename = "QINV")]
    Qinv,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "SLLE")]
    Slle,
    #[serde(rename = "PRUN")]
    Prun,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceRateType3Code {
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "PREM")]
    Prem,
    #[serde(rename = "PRCT")]
    Prct,
    #[serde(rename = "YIEL")]
    Yiel,
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
pub struct CorporateActionMandatoryVoluntary3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionMandatoryVoluntary1Code>,
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
pub struct CorporateActionMandatoryVoluntary3Choice {
    #[serde(flatten)]
    pub value: CorporateActionMandatoryVoluntary3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateCode21ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DateType7Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateCode21Choice {
    #[serde(flatten)]
    pub value: DateCode21ChoiceEnum,
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
pub enum ProtectTransactionType2Code {
    #[serde(rename = "PROT")]
    Prot,
    #[serde(rename = "COVP")]
    Covp,
    #[serde(rename = "COVR")]
    Covr,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NoSpecifiedReason1 {
    #[serde(rename = "NoSpcfdRsn")]
    pub no_spcfd_rsn: NoReasonCode,
}
#[derive(
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
pub struct UpdateType15ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<StatementUpdateType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UpdateType15Choice {
    #[serde(flatten)]
    pub value: UpdateType15ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancelledStatus12ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<CancelledStatusReason11>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancelledStatus12Choice {
    #[serde(flatten)]
    pub value: CancelledStatus12ChoiceEnum,
}
