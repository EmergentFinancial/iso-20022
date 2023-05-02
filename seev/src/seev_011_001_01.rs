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
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{12,12}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:seev.011.001.01".to_string()
}

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProcessedStatus1Code {
    #[serde(rename = "RECE")]
    Rece,
    #[serde(rename = "SENT")]
    Sent,
    #[serde(rename = "WARN")]
    Warn,
    #[serde(rename = "PACK")]
    Pack,
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
pub struct AlternateSecurityIdentification3Enum {
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
pub struct AlternateSecurityIdentification3 {
    #[serde(flatten)]
    pub value: AlternateSecurityIdentification3Enum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentIdentification8 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm: Option<IsoDateTime>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionEventType2Code {
    #[serde(rename = "ACTV")]
    Actv,
    #[serde(rename = "ATTI")]
    Atti,
    #[serde(rename = "BIDS")]
    Bids,
    #[serde(rename = "BONU")]
    Bonu,
    #[serde(rename = "BPUT")]
    Bput,
    #[serde(rename = "BRUP")]
    Brup,
    #[serde(rename = "CAPG")]
    Capg,
    #[serde(rename = "CAPI")]
    Capi,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "CHAN")]
    Chan,
    #[serde(rename = "CLSA")]
    Clsa,
    #[serde(rename = "CONS")]
    Cons,
    #[serde(rename = "CONV")]
    Conv,
    #[serde(rename = "COOP")]
    Coop,
    #[serde(rename = "DECR")]
    Decr,
    #[serde(rename = "DETI")]
    Deti,
    #[serde(rename = "DFLT")]
    Dflt,
    #[serde(rename = "DLST")]
    Dlst,
    #[serde(rename = "DRAW")]
    Draw,
    #[serde(rename = "DRIP")]
    Drip,
    #[serde(rename = "DSCL")]
    Dscl,
    #[serde(rename = "DTCH")]
    Dtch,
    #[serde(rename = "DVCA")]
    Dvca,
    #[serde(rename = "DVOP")]
    Dvop,
    #[serde(rename = "DVSC")]
    Dvsc,
    #[serde(rename = "DVSE")]
    Dvse,
    #[serde(rename = "EXOF")]
    Exof,
    #[serde(rename = "EXRI")]
    Exri,
    #[serde(rename = "EXTM")]
    Extm,
    #[serde(rename = "EXWA")]
    Exwa,
    #[serde(rename = "INCR")]
    Incr,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "MCAL")]
    Mcal,
    #[serde(rename = "MRGR")]
    Mrgr,
    #[serde(rename = "ODLT")]
    Odlt,
    #[serde(rename = "PARI")]
    Pari,
    #[serde(rename = "PCAL")]
    Pcal,
    #[serde(rename = "PDEF")]
    Pdef,
    #[serde(rename = "PINK")]
    Pink,
    #[serde(rename = "PLAC")]
    Plac,
    #[serde(rename = "PPMT")]
    Ppmt,
    #[serde(rename = "PRED")]
    Pred,
    #[serde(rename = "PRII")]
    Prii,
    #[serde(rename = "PRIO")]
    Prio,
    #[serde(rename = "REDM")]
    Redm,
    #[serde(rename = "REDO")]
    Redo,
    #[serde(rename = "REMK")]
    Remk,
    #[serde(rename = "RHDI")]
    Rhdi,
    #[serde(rename = "RHTS")]
    Rhts,
    #[serde(rename = "SHPR")]
    Shpr,
    #[serde(rename = "SMAL")]
    Smal,
    #[serde(rename = "SOFF")]
    Soff,
    #[serde(rename = "SPLF")]
    Splf,
    #[serde(rename = "SPLR")]
    Splr,
    #[serde(rename = "SUSP")]
    Susp,
    #[serde(rename = "TEND")]
    Tend,
    #[serde(rename = "TREC")]
    Trec,
    #[serde(rename = "WRTH")]
    Wrth,
    #[serde(rename = "WTRC")]
    Wtrc,
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
pub struct CorporateActionEventType2FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventType2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventType2FormatChoice {
    #[serde(flatten)]
    pub value: CorporateActionEventType2FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NotificationCancellationProcessingStatus1 {
    #[serde(rename = "Sts")]
    pub sts: ProcessedStatus2FormatChoice,
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
pub struct ProcessedStatus1FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ProcessedStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessedStatus1FormatChoice {
    #[serde(flatten)]
    pub value: ProcessedStatus1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NotificationCancellationRequestStatus1ChoiceEnum {
    #[serde(rename = "RjctdSts", skip_serializing_if = "Option::is_none")]
    pub rjctd_sts: Option<NotificationCancellationRejectionReason1>,
    #[serde(rename = "PrcdSts", skip_serializing_if = "Option::is_none")]
    pub prcd_sts: Option<NotificationCancellationProcessingStatus1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NotificationCancellationRequestStatus1Choice {
    #[serde(flatten)]
    pub value: NotificationCancellationRequestStatus1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RejectionReason6Code {
    #[serde(rename = "FAIL")]
    Fail,
    #[serde(rename = "SECO")]
    Seco,
    #[serde(rename = "AGID")]
    Agid,
    #[serde(rename = "ENDP")]
    Endp,
    #[serde(rename = "STAR")]
    Star,
    #[serde(rename = "RESU")]
    Resu,
    #[serde(rename = "SECT")]
    Sect,
    #[serde(rename = "DISP")]
    Disp,
    #[serde(rename = "ELIG")]
    Elig,
    #[serde(rename = "XRAT")]
    Xrat,
    #[serde(rename = "VALU")]
    Valu,
    #[serde(rename = "ACRU")]
    Acru,
    #[serde(rename = "NOAC")]
    Noac,
    #[serde(rename = "MCER")]
    Mcer,
    #[serde(rename = "BDAY")]
    Bday,
    #[serde(rename = "PDAY")]
    Pday,
    #[serde(rename = "GAMN")]
    Gamn,
    #[serde(rename = "DFLT")]
    Dflt,
    #[serde(rename = "NPAT")]
    Npat,
    #[serde(rename = "PERI")]
    Peri,
    #[serde(rename = "RDTE")]
    Rdte,
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
pub struct NotificationRejectionReason1 {
    #[validate(length(min = 1,))]
    #[serde(rename = "Rsn", default)]
    pub rsn: Vec<RejectionReason6FormatChoice>,
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
pub struct FinancialInstrumentDescription3 {
    #[serde(rename = "SctyId")]
    pub scty_id: SecurityIdentification7,
    #[serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none")]
    pub plc_of_listg: Option<MicIdentifier>,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<PartyIdentification2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NotificationProcessingStatus1 {
    #[serde(rename = "Sts")]
    pub sts: ProcessedStatus1FormatChoice,
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
pub struct AgentCaNotificationStatusAdviceV01Enum {
    #[serde(rename = "NtfctnAdvcSts", skip_serializing_if = "Option::is_none")]
    pub ntfctn_advc_sts: Option<NotificationAdviceStatus1Choice>,
    #[serde(rename = "NtfctnCxlReqSts", skip_serializing_if = "Option::is_none")]
    pub ntfctn_cxl_req_sts: Option<NotificationCancellationRequestStatus1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgentCaNotificationStatusAdviceV01 {
    #[serde(flatten)]
    pub value: AgentCaNotificationStatusAdviceV01Enum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RejectionReason11Code {
    #[serde(rename = "FAIL")]
    Fail,
    #[serde(rename = "DCAN")]
    Dcan,
    #[serde(rename = "DPRG")]
    Dprg,
    #[serde(rename = "REFI")]
    Refi,
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
pub struct SecurityIdentification7Enum {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinIdentifier>,
    #[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
    pub othr_id: Option<AlternateSecurityIdentification3>,
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
pub struct SecurityIdentification7 {
    #[serde(flatten)]
    pub value: SecurityIdentification7Enum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProcessedStatus2Code {
    #[serde(rename = "RECE")]
    Rece,
    #[serde(rename = "COMP")]
    Comp,
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
pub struct CorporateActionInformation2 {
    #[serde(rename = "AgtId")]
    pub agt_id: PartyIdentification2Choice,
    #[validate]
    #[serde(rename = "IssrCorpActnId")]
    pub issr_corp_actn_id: Max35Text,
    #[serde(rename = "CorpActnPrcgId", skip_serializing_if = "Option::is_none")]
    pub corp_actn_prcg_id: Option<Max35Text>,
    #[serde(rename = "EvtTp")]
    pub evt_tp: CorporateActionEventType2FormatChoice,
    #[serde(rename = "EvtPrcgTp", skip_serializing_if = "Option::is_none")]
    pub evt_prcg_tp: Option<CorporateActionEventProcessingType1FormatChoice>,
    #[serde(rename = "MndtryVlntryEvtTp")]
    pub mndtry_vlntry_evt_tp: CorporateActionMandatoryVoluntary1FormatChoice,
    #[validate]
    #[serde(rename = "UndrlygScty")]
    pub undrlyg_scty: FinancialInstrumentDescription3,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrUndrlygScty", default)]
    pub othr_undrlyg_scty: Vec<FinancialInstrumentDescription3>,
}
#[derive(
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
pub struct ProcessedStatus2FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ProcessedStatus2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessedStatus2FormatChoice {
    #[serde(flatten)]
    pub value: ProcessedStatus2FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionMandatoryVoluntary1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionMandatoryVoluntary1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionMandatoryVoluntary1FormatChoice {
    #[serde(flatten)]
    pub value: CorporateActionMandatoryVoluntary1FormatChoiceEnum,
}
#[derive(
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
pub struct CorporateActionEventProcessingType1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventProcessingType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventProcessingType1FormatChoice {
    #[serde(flatten)]
    pub value: CorporateActionEventProcessingType1FormatChoiceEnum,
}
#[derive(
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
pub struct RejectionReason6FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RejectionReason6Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionReason6FormatChoice {
    #[serde(flatten)]
    pub value: RejectionReason6FormatChoiceEnum,
}
#[derive(
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
#[derive(
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
pub struct NotificationCancellationRejectionReason1 {
    #[validate(length(min = 1,))]
    #[serde(rename = "Rsn", default)]
    pub rsn: Vec<RejectionReason11FormatChoice>,
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
pub struct NotificationAdviceStatus1ChoiceEnum {
    #[serde(rename = "PrcdSts", skip_serializing_if = "Option::is_none")]
    pub prcd_sts: Option<NotificationProcessingStatus1>,
    #[serde(rename = "RjctdSts", skip_serializing_if = "Option::is_none")]
    pub rjctd_sts: Option<NotificationRejectionReason1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NotificationAdviceStatus1Choice {
    #[serde(flatten)]
    pub value: NotificationAdviceStatus1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionReason11FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RejectionReason11Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionReason11FormatChoice {
    #[serde(flatten)]
    pub value: RejectionReason11FormatChoiceEnum,
}
#[derive(
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
pub struct PartyIdentification2ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "BICOrBEI", skip_serializing_if = "Option::is_none")]
    pub bic_or_bei: Option<AnyBicIdentifier>,
}
#[derive(
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
pub enum CorporateActionEventProcessingType1Code {
    #[serde(rename = "GENL")]
    Genl,
    #[serde(rename = "DISN")]
    Disn,
    #[serde(rename = "REOR")]
    Reor,
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
#[serde(rename = "Document")]
pub struct Document {
    #[serde(rename = "AgtCANtfctnStsAdvc")]
    pub agt_ca_ntfctn_sts_advc: AgentCaNotificationStatusAdviceV01,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
}
