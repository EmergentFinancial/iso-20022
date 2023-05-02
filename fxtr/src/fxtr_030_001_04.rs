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
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:fxtr.030.001.04".to_string()
}

#[derive(
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
pub enum AllocationIndicator1Code {
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PREA")]
    Prea,
    #[serde(rename = "UNAL")]
    Unal,
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
pub struct CounterpartySideTransactionReporting1 {
    #[serde(rename = "RptgJursdctn", skip_serializing_if = "Option::is_none")]
    pub rptg_jursdctn: Option<Max35Text>,
    #[serde(rename = "RptgPty", skip_serializing_if = "Option::is_none")]
    pub rptg_pty: Option<PartyIdentification73Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CtrPtySdUnqTxIdr", default)]
    pub ctr_pty_sd_unq_tx_idr: Vec<UniqueTransactionIdentifier2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Exact42Text {
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
pub struct PartyIdentification59 {
    #[serde(rename = "PtyNm", skip_serializing_if = "Option::is_none")]
    pub pty_nm: Option<Max34Text>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<PartyIdentification44>,
    #[serde(rename = "AcctNb", skip_serializing_if = "Option::is_none")]
    pub acct_nb: Option<Max34Text>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Max105Text>,
    #[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
    #[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
    pub lgl_ntty_idr: Option<LeiIdentifier>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CollateralisationIndicator1Code {
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "ONEW")]
    Onew,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "UNCO")]
    Unco,
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
pub struct Pagination {
    #[validate]
    #[serde(rename = "PgNb")]
    pub pg_nb: Max5NumericText,
    #[validate]
    #[serde(rename = "LastPgInd")]
    pub last_pg_ind: YesNoIndicator,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeStatus7Code {
    #[serde(rename = "INVA")]
    Inva,
    #[serde(rename = "UMTC")]
    Umtc,
    #[serde(rename = "FMTC")]
    Fmtc,
    #[serde(rename = "SMAT")]
    Smat,
    #[serde(rename = "SUSP")]
    Susp,
    #[serde(rename = "SMAP")]
    Smap,
    #[serde(rename = "PFIX")]
    Pfix,
    #[serde(rename = "FUMT")]
    Fumt,
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
pub struct ExternalClearingSystemIdentification1Code {
    #[validate(length(min = 1, max = 5,))]
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
pub struct ForeignExchangeTradeBulkStatusNotificationV04<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "StsDtls")]
    pub sts_dtls: TradeData12,
    #[validate(length(min = 1,))]
    #[serde(rename = "TradData", default)]
    pub trad_data: Vec<TradeData11>,
    #[serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none")]
    pub msg_pgntn: Option<Pagination>,
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
pub struct TradeData11 {
    #[serde(rename = "OrgtrRef", skip_serializing_if = "Option::is_none")]
    pub orgtr_ref: Option<Max35Text>,
    #[validate]
    #[serde(rename = "MtchgSysUnqRef")]
    pub mtchg_sys_unq_ref: Max35Text,
    #[serde(rename = "MtchgSysMtchgRef", skip_serializing_if = "Option::is_none")]
    pub mtchg_sys_mtchg_ref: Option<Max35Text>,
    #[serde(rename = "MtchgSysMtchdSdRef", skip_serializing_if = "Option::is_none")]
    pub mtchg_sys_mtchd_sd_ref: Option<Max35Text>,
    #[serde(rename = "CurSttlmDt", skip_serializing_if = "Option::is_none")]
    pub cur_sttlm_dt: Option<IsoDate>,
    #[serde(rename = "NewSttlmDt", skip_serializing_if = "Option::is_none")]
    pub new_sttlm_dt: Option<IsoDate>,
    #[serde(rename = "CurStsDtTm", skip_serializing_if = "Option::is_none")]
    pub cur_sts_dt_tm: Option<IsoDateTime>,
    #[serde(rename = "PdctTp", skip_serializing_if = "Option::is_none")]
    pub pdct_tp: Option<Max35Text>,
    #[serde(rename = "SttlmSsnIdr", skip_serializing_if = "Option::is_none")]
    pub sttlm_ssn_idr: Option<Exact4AlphaNumericText>,
    #[serde(rename = "RgltryRptg", skip_serializing_if = "Option::is_none")]
    pub rgltry_rptg: Option<RegulatoryReporting4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeData12 {
    #[validate]
    #[serde(rename = "MsgId")]
    pub msg_id: Max35Text,
    #[serde(rename = "StsOrgtr", skip_serializing_if = "Option::is_none")]
    pub sts_orgtr: Option<Max35Text>,
    #[validate]
    #[serde(rename = "CurSts")]
    pub cur_sts: StatusAndSubStatus2,
    #[serde(rename = "CurStsSubTp", skip_serializing_if = "Option::is_none")]
    pub cur_sts_sub_tp: Option<StatusSubType2Code>,
    #[validate]
    #[serde(rename = "CurStsDtTm")]
    pub cur_sts_dt_tm: IsoDateTime,
    #[serde(rename = "PrvsSts", skip_serializing_if = "Option::is_none")]
    pub prvs_sts: Option<Status28Choice>,
    #[serde(rename = "PrvsStsSubTp", skip_serializing_if = "Option::is_none")]
    pub prvs_sts_sub_tp: Option<StatusSubType2Code>,
    #[serde(rename = "PdctTp", skip_serializing_if = "Option::is_none")]
    pub pdct_tp: Option<Max35Text>,
    #[serde(rename = "SttlmSsnIdr", skip_serializing_if = "Option::is_none")]
    pub sttlm_ssn_idr: Option<Exact4AlphaNumericText>,
    #[serde(rename = "LkdRptId", skip_serializing_if = "Option::is_none")]
    pub lkd_rpt_id: Option<Max35Text>,
}
#[derive(
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
pub struct PartyIdentification73ChoiceEnum {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<PartyIdentification44>,
    #[serde(rename = "PtyId", skip_serializing_if = "Option::is_none")]
    pub pty_id: Option<PartyIdentification59>,
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
pub struct PartyIdentification73Choice {
    #[serde(flatten)]
    pub value: PartyIdentification73ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClearingBrokerIdentification1 {
    #[serde(rename = "SdInd")]
    pub sd_ind: SideIndicator1Code,
    #[validate]
    #[serde(rename = "ClrBrkrId")]
    pub clr_brkr_id: Max35Text,
}
#[derive(
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
pub struct Status28ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TradeStatus7Code>,
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
pub struct Status28Choice {
    #[serde(flatten)]
    pub value: Status28ChoiceEnum,
}
#[derive(
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
pub struct TradingSideTransactionReporting1 {
    #[serde(rename = "RptgJursdctn", skip_serializing_if = "Option::is_none")]
    pub rptg_jursdctn: Option<Max35Text>,
    #[serde(rename = "RptgPty", skip_serializing_if = "Option::is_none")]
    pub rptg_pty: Option<PartyIdentification73Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TradgSdUnqTxIdr", default)]
    pub tradg_sd_unq_tx_idr: Vec<UniqueTransactionIdentifier2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UniqueTransactionIdentifier2 {
    #[validate]
    #[serde(rename = "UnqTxIdr")]
    pub unq_tx_idr: Max52Text,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrrUnqTxIdr", default)]
    pub prr_unq_tx_idr: Vec<Max52Text>,
}
#[derive(
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
pub struct StatusAndSubStatus2 {
    #[serde(rename = "StsCd")]
    pub sts_cd: Status27Choice,
    #[serde(rename = "SubStsCd", skip_serializing_if = "Option::is_none")]
    pub sub_sts_cd: Option<Exact4AlphaNumericText>,
}
#[derive(
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
pub enum CorporateSectorIdentifier1Code {
    #[serde(rename = "L")]
    L,
    #[serde(rename = "A")]
    A,
    #[serde(rename = "C")]
    C,
    #[serde(rename = "I")]
    I,
    #[serde(rename = "F")]
    F,
    #[serde(rename = "O")]
    O,
    #[serde(rename = "R")]
    R,
    #[serde(rename = "U")]
    U,
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
pub struct RegulatoryReporting4 {
    #[validate(length(min = 0,))]
    #[serde(rename = "TradgSdTxRptg", default)]
    pub tradg_sd_tx_rptg: Vec<TradingSideTransactionReporting1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CtrPtySdTxRptg", default)]
    pub ctr_pty_sd_tx_rptg: Vec<CounterpartySideTransactionReporting1>,
    #[serde(rename = "CntrlCtrPtyClrHs", skip_serializing_if = "Option::is_none")]
    pub cntrl_ctr_pty_clr_hs: Option<PartyIdentification73Choice>,
    #[serde(rename = "ClrBrkr", skip_serializing_if = "Option::is_none")]
    pub clr_brkr: Option<PartyIdentification73Choice>,
    #[serde(rename = "ClrXcptnPty", skip_serializing_if = "Option::is_none")]
    pub clr_xcptn_pty: Option<PartyIdentification73Choice>,
    #[serde(rename = "ClrBrkrId", skip_serializing_if = "Option::is_none")]
    pub clr_brkr_id: Option<ClearingBrokerIdentification1>,
    #[serde(rename = "ClrThrshldInd", skip_serializing_if = "Option::is_none")]
    pub clr_thrshld_ind: Option<YesNoIndicator>,
    #[serde(rename = "ClrdPdctId", skip_serializing_if = "Option::is_none")]
    pub clrd_pdct_id: Option<Max35Text>,
    #[serde(rename = "UndrlygPdctIdr", skip_serializing_if = "Option::is_none")]
    pub undrlyg_pdct_idr: Option<UnderlyingProductIdentifier1Code>,
    #[serde(rename = "AllcnInd", skip_serializing_if = "Option::is_none")]
    pub allcn_ind: Option<AllocationIndicator1Code>,
    #[serde(rename = "CollstnInd", skip_serializing_if = "Option::is_none")]
    pub collstn_ind: Option<CollateralisationIndicator1Code>,
    #[serde(rename = "ExctnVn", skip_serializing_if = "Option::is_none")]
    pub exctn_vn: Option<Max35Text>,
    #[serde(rename = "ExctnTmstmp", skip_serializing_if = "Option::is_none")]
    pub exctn_tmstmp: Option<DateAndDateTimeChoice>,
    #[serde(rename = "NonStdFlg", skip_serializing_if = "Option::is_none")]
    pub non_std_flg: Option<YesNoIndicator>,
    #[serde(rename = "LkSwpId", skip_serializing_if = "Option::is_none")]
    pub lk_swp_id: Option<Exact42Text>,
    #[serde(
        rename = "FinNtrOfTheCtrPtyInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub fin_ntr_of_the_ctr_pty_ind: Option<YesNoIndicator>,
    #[serde(rename = "CollPrtflInd", skip_serializing_if = "Option::is_none")]
    pub coll_prtfl_ind: Option<YesNoIndicator>,
    #[serde(rename = "CollPrtflCd", skip_serializing_if = "Option::is_none")]
    pub coll_prtfl_cd: Option<Max10Text>,
    #[serde(rename = "PrtflCmprssnInd", skip_serializing_if = "Option::is_none")]
    pub prtfl_cmprssn_ind: Option<YesNoIndicator>,
    #[serde(rename = "CorpSctrInd", skip_serializing_if = "Option::is_none")]
    pub corp_sctr_ind: Option<CorporateSectorIdentifier1Code>,
    #[serde(
        rename = "TradWthNonEEACtrPtyInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub trad_wth_non_eea_ctr_pty_ind: Option<YesNoIndicator>,
    #[serde(rename = "NtrgrpTradInd", skip_serializing_if = "Option::is_none")]
    pub ntrgrp_trad_ind: Option<YesNoIndicator>,
    #[serde(
        rename = "ComrclOrTrsrFincgInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub comrcl_or_trsr_fincg_ind: Option<YesNoIndicator>,
    #[serde(rename = "AddtlRptgInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rptg_inf: Option<Max210Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SideIndicator1Code {
    #[serde(rename = "CCPL")]
    Ccpl,
    #[serde(rename = "CLNT")]
    Clnt,
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
pub struct Max52Text {
    #[validate(length(min = 1, max = 52,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum StatusSubType2Code {
    #[serde(rename = "SMDY")]
    Smdy,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeStatus6Code {
    #[serde(rename = "INVA")]
    Inva,
    #[serde(rename = "FMTC")]
    Fmtc,
    #[serde(rename = "SMAP")]
    Smap,
    #[serde(rename = "RJCT")]
    Rjct,
    #[serde(rename = "RSCD")]
    Rscd,
    #[serde(rename = "STLD")]
    Stld,
    #[serde(rename = "SPLI")]
    Spli,
    #[serde(rename = "UMTC")]
    Umtc,
    #[serde(rename = "SMAT")]
    Smat,
    #[serde(rename = "FUMT")]
    Fumt,
    #[serde(rename = "NETT")]
    Nett,
    #[serde(rename = "PFIX")]
    Pfix,
    #[serde(rename = "OMTC")]
    Omtc,
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
pub struct ClearingSystemIdentification2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalClearingSystemIdentification1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClearingSystemIdentification2Choice {
    #[serde(flatten)]
    pub value: ClearingSystemIdentification2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max10Text {
    #[validate(length(min = 1, max = 10,))]
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
pub struct Status27ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TradeStatus6Code>,
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
pub struct Status27Choice {
    #[serde(flatten)]
    pub value: Status27ChoiceEnum,
}
#[derive(
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "FXTradBlkStsNtfctn")]
    pub fx_trad_blk_sts_ntfctn: ForeignExchangeTradeBulkStatusNotificationV04<A>,
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
pub struct Max34Text {
    #[validate(length(min = 1, max = 34,))]
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
