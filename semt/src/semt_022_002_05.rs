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
    static ref RESTRICTED_FINX_MAX_210_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.\n\r,'\+ ]{1,210}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_16_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9a-zA-Z\-\?:\(\)\.,'\+ ]([0-9a-zA-Z\-\?:\(\)\.,'\+ ]*(/[0-9a-zA-Z\-\?:\(\)\.,'\+ ])?)*)"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_35_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.,'\+ ]{1,35}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_34_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"([0-9a-zA-Z\-\?:\(\)\.,'\+ ]([0-9a-zA-Z\-\?:\(\)\.,'\+ ]*(/[0-9a-zA-Z\-\?:\(\)\.,'\+ ])?)*)"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_70_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.\n\r,'\+ ]{1,70}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RESTRICTED_FINX_MAX_140_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9a-zA-Z/\-\?:\(\)\.\n\r,'\+ ]{1,140}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:semt.022.002.05".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingStatus50ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<PendingReason24>,
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
pub struct PendingStatus50Choice {
    #[serde(flatten)]
    pub value: PendingStatus50ChoiceEnum,
}
#[derive(
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
pub struct RejectionAndRepairReason42ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RejectionReason74Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionAndRepairReason42Choice {
    #[serde(flatten)]
    pub value: RejectionAndRepairReason42ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AcknowledgementReason3Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "SMPG")]
    Smpg,
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
pub struct RestrictedFinxMax210Text {
    #[validate(
        length(min = 1, max = 210,),
        regex = "RESTRICTED_FINX_MAX_210_TEXT_REGEX"
    )]
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
pub struct ExternalOrganisationIdentification1Code {
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
pub struct FailingReason10 {
    #[serde(rename = "Cd")]
    pub cd: FailingReason11Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModificationReason5 {
    #[serde(rename = "Cd")]
    pub cd: ModificationReason5Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesAccount37 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: RestrictedFinxMax35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification47>,
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
pub struct RestrictedFinxMax16Text {
    #[validate(
        length(min = 1, max = 16,),
        regex = "RESTRICTED_FINX_MAX_16_TEXT_REGEX"
    )]
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
pub struct MatchingStatus30ChoiceEnum {
    #[serde(rename = "Mtchd", skip_serializing_if = "Option::is_none")]
    pub mtchd: Option<ProprietaryReason5>,
    #[serde(rename = "Umtchd", skip_serializing_if = "Option::is_none")]
    pub umtchd: Option<UnmatchedStatus20Choice>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryStatusAndReason7>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MatchingStatus30Choice {
    #[serde(flatten)]
    pub value: MatchingStatus30ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessingStatus15ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<PendingProcessingReason12>,
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
pub struct PendingProcessingStatus15Choice {
    #[serde(flatten)]
    pub value: PendingProcessingStatus15ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgedAcceptedStatus25ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<AcknowledgementReason13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgedAcceptedStatus25Choice {
    #[serde(flatten)]
    pub value: AcknowledgedAcceptedStatus25ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PendingProcessingReason2Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CAIS")]
    Cais,
    #[serde(rename = "DOCY")]
    Docy,
    #[serde(rename = "NOFX")]
    Nofx,
    #[serde(rename = "BLOC")]
    Bloc,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "GLOB")]
    Glob,
    #[serde(rename = "YCOL")]
    Ycol,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "FLIM")]
    Flim,
    #[serde(rename = "NEXT")]
    Next,
    #[serde(rename = "LACK")]
    Lack,
    #[serde(rename = "LALO")]
    Lalo,
    #[serde(rename = "MONY")]
    Mony,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "DENO")]
    Deno,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "CSDH")]
    Csdh,
    #[serde(rename = "CVAL")]
    Cval,
    #[serde(rename = "CDEL")]
    Cdel,
    #[serde(rename = "CDLR")]
    Cdlr,
    #[serde(rename = "CDAC")]
    Cdac,
    #[serde(rename = "INBC")]
    Inbc,
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
pub struct DeniedReason17 {
    #[serde(rename = "Cd")]
    pub cd: DeniedReason24Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModificationReason5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ModifiedStatusReason1Code>,
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
pub struct ModificationReason5Choice {
    #[serde(flatten)]
    pub value: ModificationReason5ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AcknowledgementReason6Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "SMPG")]
    Smpg,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "NSTP")]
    Nstp,
    #[serde(rename = "LATE")]
    Late,
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
pub struct RepairReason13 {
    #[serde(rename = "Cd")]
    pub cd: RepairReason14Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PendingReason6Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "CDRG")]
    Cdrg,
    #[serde(rename = "CDCY")]
    Cdcy,
    #[serde(rename = "CDRE")]
    Cdre,
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
pub struct PendingStatus51ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<PendingReason25>,
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
pub struct PendingStatus51Choice {
    #[serde(flatten)]
    pub value: PendingStatus51ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionOrRepairReason41 {
    #[serde(rename = "Cd")]
    pub cd: RejectionAndRepairReason41Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingReason41ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PendingReason2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingReason41Choice {
    #[serde(flatten)]
    pub value: PendingReason41ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeniedStatus19ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<DeniedReason17>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeniedStatus19Choice {
    #[serde(flatten)]
    pub value: DeniedStatus19ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RepairReason4Code {
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "CAEV")]
    Caev,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CASY")]
    Casy,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "DDEA")]
    Ddea,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "IIND")]
    Iind,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "PLCE")]
    Plce,
    #[serde(rename = "REFE")]
    Refe,
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "SETR")]
    Setr,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "TXST")]
    Txst,
    #[serde(rename = "INPS")]
    Inps,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "IEXE")]
    Iexe,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "ICUS")]
    Icus,
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
pub enum PendingReason2Code {
    #[serde(rename = "AWMO")]
    Awmo,
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CAIS")]
    Cais,
    #[serde(rename = "REFU")]
    Refu,
    #[serde(rename = "AWSH")]
    Awsh,
    #[serde(rename = "PHSE")]
    Phse,
    #[serde(rename = "TAMM")]
    Tamm,
    #[serde(rename = "DOCY")]
    Docy,
    #[serde(rename = "DOCC")]
    Docc,
    #[serde(rename = "BLOC")]
    Bloc,
    #[serde(rename = "CHAS")]
    Chas,
    #[serde(rename = "NEWI")]
    Newi,
    #[serde(rename = "CLAC")]
    Clac,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "GLOB")]
    Glob,
    #[serde(rename = "PREA")]
    Prea,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "NMAS")]
    Nmas,
    #[serde(rename = "NOFX")]
    Nofx,
    #[serde(rename = "CMON")]
    Cmon,
    #[serde(rename = "YCOL")]
    Ycol,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "DEPO")]
    Depo,
    #[serde(rename = "FLIM")]
    Flim,
    #[serde(rename = "INCA")]
    Inca,
    #[serde(rename = "LINK")]
    Link,
    #[serde(rename = "FUTU")]
    Futu,
    #[serde(rename = "LACK")]
    Lack,
    #[serde(rename = "LALO")]
    Lalo,
    #[serde(rename = "MONY")]
    Mony,
    #[serde(rename = "NCON")]
    Ncon,
    #[serde(rename = "REFS")]
    Refs,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "CYCL")]
    Cycl,
    #[serde(rename = "SBLO")]
    Sblo,
    #[serde(rename = "CPEC")]
    Cpec,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "IAAD")]
    Iaad,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PHCK")]
    Phck,
    #[serde(rename = "BENO")]
    Beno,
    #[serde(rename = "BOTH")]
    Both,
    #[serde(rename = "CLHT")]
    Clht,
    #[serde(rename = "DENO")]
    Deno,
    #[serde(rename = "DISA")]
    Disa,
    #[serde(rename = "DKNY")]
    Dkny,
    #[serde(rename = "FROZ")]
    Froz,
    #[serde(rename = "LAAW")]
    Laaw,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "PRCY")]
    Prcy,
    #[serde(rename = "REGT")]
    Regt,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "PRSY")]
    Prsy,
    #[serde(rename = "CSDH")]
    Csdh,
    #[serde(rename = "CVAL")]
    Cval,
    #[serde(rename = "CDLR")]
    Cdlr,
    #[serde(rename = "INBC")]
    Inbc,
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
pub struct AcknowledgementReason13 {
    #[serde(rename = "Cd")]
    pub cd: AcknowledgementReason16Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification84 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: RestrictedFinxMax34Text,
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
pub struct RejectionReason48ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RejectionReason72Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionReason48Choice {
    #[serde(flatten)]
    pub value: RejectionReason48ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionStatus41ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<RejectionReason62>,
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
pub struct RejectionStatus41Choice {
    #[serde(flatten)]
    pub value: RejectionStatus41ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionIdentifications34 {
    #[validate]
    #[serde(rename = "AcctOwnrTxId")]
    pub acct_ownr_tx_id: RestrictedFinxMax16Text,
    #[serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_tx_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none")]
    pub mkt_infrstrctr_tx_id: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "PrcrTxId", skip_serializing_if = "Option::is_none")]
    pub prcr_tx_id: Option<RestrictedFinxMax16Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgementReason21ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AcknowledgementReason6Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgementReason21Choice {
    #[serde(flatten)]
    pub value: AcknowledgementReason21ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "SctiesSttlmTxAudtTrlRpt")]
    pub scties_sttlm_tx_audt_trl_rpt: SecuritiesSettlementTransactionAuditTrailReport002V05<A>,
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
pub struct AcknowledgedAcceptedStatus31ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<AcknowledgementReason19>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgedAcceptedStatus31Choice {
    #[serde(flatten)]
    pub value: AcknowledgedAcceptedStatus31ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingReason24 {
    #[serde(rename = "Cd")]
    pub cd: PendingReason41Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingReason42ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PendingReason9Code>,
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
pub struct PendingReason42Choice {
    #[serde(flatten)]
    pub value: PendingReason42ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AcknowledgementReason5Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "SMPG")]
    Smpg,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "CDCY")]
    Cdcy,
    #[serde(rename = "CDRG")]
    Cdrg,
    #[serde(rename = "CDRE")]
    Cdre,
    #[serde(rename = "NSTP")]
    Nstp,
    #[serde(rename = "RQWV")]
    Rqwv,
    #[serde(rename = "LATE")]
    Late,
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
pub struct ProprietaryStatusAndReason7 {
    #[validate]
    #[serde(rename = "PrtrySts")]
    pub prtry_sts: GenericIdentification47,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrtryRsn", default)]
    pub prtry_rsn: Vec<ProprietaryReason5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinxMax35Text {
    #[validate(
        length(min = 1, max = 35,),
        regex = "RESTRICTED_FINX_MAX_35_TEXT_REGEX"
    )]
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
pub struct UnmatchedStatus20ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<UnmatchedReason19>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnmatchedStatus20Choice {
    #[serde(flatten)]
    pub value: UnmatchedStatus20ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessingReason14ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PendingProcessingReason2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessingReason14Choice {
    #[serde(flatten)]
    pub value: PendingProcessingReason14ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionOrRepairReason42 {
    #[serde(rename = "Cd")]
    pub cd: RejectionAndRepairReason42Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationReason24 {
    #[serde(rename = "Cd")]
    pub cd: CancellationReason37Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgementReason19 {
    #[serde(rename = "Cd")]
    pub cd: AcknowledgementReason22Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingReason20 {
    #[serde(rename = "Cd")]
    pub cd: PendingReason37Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgementReason18 {
    #[serde(rename = "Cd")]
    pub cd: AcknowledgementReason21Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationStatus25ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<CancellationReason24>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationStatus25Choice {
    #[serde(flatten)]
    pub value: CancellationStatus25ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessingReason12 {
    #[serde(rename = "Cd")]
    pub cd: PendingProcessingReason14Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationReason37ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CancelledStatusReason16Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationReason37Choice {
    #[serde(flatten)]
    pub value: CancellationReason37ChoiceEnum,
}
#[derive(
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
pub enum RejectionReason72Code {
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "DDEA")]
    Ddea,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "PLCE")]
    Plce,
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "REFE")]
    Refe,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "TXST")]
    Txst,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "IIND")]
    Iind,
    #[serde(rename = "CAEV")]
    Caev,
    #[serde(rename = "CASY")]
    Casy,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "SETR")]
    Setr,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "INPS")]
    Inps,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "ICUS")]
    Icus,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "IEXE")]
    Iexe,
    #[serde(rename = "INVL")]
    Invl,
    #[serde(rename = "INVB")]
    Invb,
    #[serde(rename = "INVN")]
    Invn,
    #[serde(rename = "VALR")]
    Valr,
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
pub struct RepairStatus17ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<RepairReason13>,
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
pub struct RepairStatus17Choice {
    #[serde(flatten)]
    pub value: RepairStatus17ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionReason62 {
    #[serde(rename = "Cd")]
    pub cd: RejectionReason47Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DeniedReason6Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CDCY")]
    Cdcy,
    #[serde(rename = "CDRE")]
    Cdre,
    #[serde(rename = "DCAN")]
    Dcan,
    #[serde(rename = "DSET")]
    Dset,
    #[serde(rename = "DPRG")]
    Dprg,
    #[serde(rename = "DREP")]
    Drep,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "CDRG")]
    Cdrg,
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
pub struct PartyIdentification156 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification136Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CancelledStatusReason5Code {
    #[serde(rename = "CANI")]
    Cani,
    #[serde(rename = "OTHR")]
    Othr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CancelledStatusReason16Code {
    #[serde(rename = "SCEX")]
    Scex,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "CXLR")]
    Cxlr,
    #[serde(rename = "BYIY")]
    Byiy,
    #[serde(rename = "CTHP")]
    Cthp,
    #[serde(rename = "CANZ")]
    Canz,
    #[serde(rename = "CANT")]
    Cant,
    #[serde(rename = "CSUB")]
    Csub,
    #[serde(rename = "CANS")]
    Cans,
    #[serde(rename = "CANI")]
    Cani,
    #[serde(rename = "CORP")]
    Corp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DeniedReason4Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "DCAN")]
    Dcan,
    #[serde(rename = "DPRG")]
    Dprg,
    #[serde(rename = "DREP")]
    Drep,
    #[serde(rename = "DSET")]
    Dset,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "CDRG")]
    Cdrg,
    #[serde(rename = "CDCY")]
    Cdcy,
    #[serde(rename = "CDRE")]
    Cdre,
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
pub struct OrganisationIdentification32 {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<GenericOrganisationIdentification2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairReason14ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RepairReason4Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairReason14Choice {
    #[serde(flatten)]
    pub value: RepairReason14ChoiceEnum,
}
#[derive(
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
pub struct FailingReason11ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FailingReason2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FailingReason11Choice {
    #[serde(flatten)]
    pub value: FailingReason11ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationReason18 {
    #[serde(rename = "Cd")]
    pub cd: CancellationReason28Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessingReason13 {
    #[serde(rename = "Cd")]
    pub cd: PendingProcessingReason15Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RejectionReason73Code {
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "DDEA")]
    Ddea,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "PLCE")]
    Plce,
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "REFE")]
    Refe,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "TXST")]
    Txst,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "IIND")]
    Iind,
    #[serde(rename = "CAEV")]
    Caev,
    #[serde(rename = "CASY")]
    Casy,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "SETR")]
    Setr,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "INPS")]
    Inps,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "ICUS")]
    Icus,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "IEXE")]
    Iexe,
    #[serde(rename = "INVB")]
    Invb,
    #[serde(rename = "INVL")]
    Invl,
    #[serde(rename = "INVN")]
    Invn,
    #[serde(rename = "VALR")]
    Valr,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ModifiedStatusReason1Code {
    #[serde(rename = "MDBY")]
    Mdby,
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
pub struct DeniedReason16 {
    #[serde(rename = "Cd")]
    pub cd: DeniedReason21Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModificationStatus5ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<ModificationReason5>,
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
pub struct ModificationStatus5Choice {
    #[serde(flatten)]
    pub value: ModificationStatus5ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PendingProcessingReason3Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "BLOC")]
    Bloc,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "NEXT")]
    Next,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "DENO")]
    Deno,
    #[serde(rename = "CERT")]
    Cert,
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
pub struct ProprietaryReason5 {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<GenericIdentification47>,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairReason12 {
    #[serde(rename = "Cd")]
    pub cd: RepairReason14Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionOrRepairStatus46ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<RejectionOrRepairReason41>,
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
pub struct RejectionOrRepairStatus46Choice {
    #[serde(flatten)]
    pub value: RejectionOrRepairStatus46ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinxMax34Text {
    #[validate(
        length(min = 1, max = 34,),
        regex = "RESTRICTED_FINX_MAX_34_TEXT_REGEX"
    )]
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
pub struct PendingStatus46ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<PendingReason20>,
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
pub struct PendingStatus46Choice {
    #[serde(flatten)]
    pub value: PendingStatus46ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgedAcceptedStatus30ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<AcknowledgementReason18>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgedAcceptedStatus30Choice {
    #[serde(flatten)]
    pub value: AcknowledgedAcceptedStatus30ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingStatus93ChoiceEnum {
    #[serde(rename = "PdgCxl", skip_serializing_if = "Option::is_none")]
    pub pdg_cxl: Option<PendingStatus51Choice>,
    #[serde(rename = "AckdAccptd", skip_serializing_if = "Option::is_none")]
    pub ackd_accptd: Option<AcknowledgedAcceptedStatus31Choice>,
    #[serde(rename = "Rpr", skip_serializing_if = "Option::is_none")]
    pub rpr: Option<RejectionOrRepairStatus46Choice>,
    #[serde(rename = "Rjctd", skip_serializing_if = "Option::is_none")]
    pub rjctd: Option<RejectionOrRepairStatus47Choice>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryStatusAndReason7>,
    #[serde(rename = "Canc", skip_serializing_if = "Option::is_none")]
    pub canc: Option<CancellationStatus20Choice>,
    #[serde(rename = "Dnd", skip_serializing_if = "Option::is_none")]
    pub dnd: Option<DeniedStatus21Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingStatus93Choice {
    #[serde(flatten)]
    pub value: ProcessingStatus93ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionReason63 {
    #[serde(rename = "Cd")]
    pub cd: RejectionReason48Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesSettlementTransactionAuditTrailReport002V05<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Pgntn")]
    pub pgntn: Pagination1,
    #[serde(rename = "QryRef", skip_serializing_if = "Option::is_none")]
    pub qry_ref: Option<RestrictedFinxMax16Text>,
    #[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<TransactionIdentifications34>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount37>,
    #[serde(rename = "BlckChainAdrOrWllt", skip_serializing_if = "Option::is_none")]
    pub blck_chain_adr_or_wllt: Option<BlockChainAddressWallet7>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<PartyIdentification156>,
    #[validate(length(min = 0,))]
    #[serde(rename = "StsTrl", default)]
    pub sts_trl: Vec<StatusTrail11<A>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification136ChoiceEnum {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification84>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification136Choice {
    #[serde(flatten)]
    pub value: PartyIdentification136ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinxMax70Text {
    #[validate(
        length(min = 1, max = 70,),
        regex = "RESTRICTED_FINX_MAX_70_TEXT_REGEX"
    )]
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
pub struct RejectionReason47ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RejectionReason73Code>,
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
pub struct RejectionReason47Choice {
    #[serde(flatten)]
    pub value: RejectionReason47ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FailingStatus12ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<FailingReason10>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FailingStatus12Choice {
    #[serde(flatten)]
    pub value: FailingStatus12ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RejectionReason74Code {
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "NRGN")]
    Nrgn,
    #[serde(rename = "REFE")]
    Refe,
    #[serde(rename = "NRGM")]
    Nrgm,
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
pub struct GenericOrganisationIdentification2 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: RestrictedFinxMax35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<OrganisationIdentificationSchemeName2Choice>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<RestrictedFinxMax35Text>,
}
#[derive(
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
pub struct RejectionAndRepairReason41ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RejectionReason71Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionAndRepairReason41Choice {
    #[serde(flatten)]
    pub value: RejectionAndRepairReason41ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeniedStatus21ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<DeniedReason16>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeniedStatus21Choice {
    #[serde(flatten)]
    pub value: DeniedStatus21ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionOrRepairStatus47ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<RejectionOrRepairReason42>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionOrRepairStatus47Choice {
    #[serde(flatten)]
    pub value: RejectionOrRepairStatus47ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationReason28ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CancelledStatusReason5Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationReason28Choice {
    #[serde(flatten)]
    pub value: CancellationReason28ChoiceEnum,
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
pub struct RejectionStatus42ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<RejectionReason63>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionStatus42Choice {
    #[serde(flatten)]
    pub value: RejectionStatus42ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictedFinxMax140Text {
    #[validate(
        length(min = 1, max = 140,),
        regex = "RESTRICTED_FINX_MAX_140_TEXT_REGEX"
    )]
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
pub struct SettlementStatus22ChoiceEnum {
    #[serde(rename = "Pdg", skip_serializing_if = "Option::is_none")]
    pub pdg: Option<PendingStatus50Choice>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryStatusAndReason7>,
    #[serde(rename = "Flng", skip_serializing_if = "Option::is_none")]
    pub flng: Option<FailingStatus12Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementStatus22Choice {
    #[serde(flatten)]
    pub value: SettlementStatus22ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnmatchedReason26ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<UnmatchedReason12Code>,
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
pub struct UnmatchedReason26Choice {
    #[serde(flatten)]
    pub value: UnmatchedReason26ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RejectionReason71Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "NRGM")]
    Nrgm,
    #[serde(rename = "NRGN")]
    Nrgn,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "REFE")]
    Refe,
    #[serde(rename = "INVM")]
    Invm,
    #[serde(rename = "INVL")]
    Invl,
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
pub struct AcknowledgementReason16ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AcknowledgementReason5Code>,
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
pub struct AcknowledgementReason16Choice {
    #[serde(flatten)]
    pub value: AcknowledgementReason16ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FailingReason2Code {
    #[serde(rename = "AWMO")]
    Awmo,
    #[serde(rename = "BYIY")]
    Byiy,
    #[serde(rename = "CLAT")]
    Clat,
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CANR")]
    Canr,
    #[serde(rename = "CAIS")]
    Cais,
    #[serde(rename = "OBJT")]
    Objt,
    #[serde(rename = "AWSH")]
    Awsh,
    #[serde(rename = "PHSE")]
    Phse,
    #[serde(rename = "STCD")]
    Stcd,
    #[serde(rename = "DOCY")]
    Docy,
    #[serde(rename = "MLAT")]
    Mlat,
    #[serde(rename = "DOCC")]
    Docc,
    #[serde(rename = "BLOC")]
    Bloc,
    #[serde(rename = "CHAS")]
    Chas,
    #[serde(rename = "NEWI")]
    Newi,
    #[serde(rename = "CLAC")]
    Clac,
    #[serde(rename = "MUNO")]
    Muno,
    #[serde(rename = "GLOB")]
    Glob,
    #[serde(rename = "PREA")]
    Prea,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "NOFX")]
    Nofx,
    #[serde(rename = "CMON")]
    Cmon,
    #[serde(rename = "YCOL")]
    Ycol,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "DEPO")]
    Depo,
    #[serde(rename = "FLIM")]
    Flim,
    #[serde(rename = "INCA")]
    Inca,
    #[serde(rename = "LINK")]
    Link,
    #[serde(rename = "LACK")]
    Lack,
    #[serde(rename = "LALO")]
    Lalo,
    #[serde(rename = "MONY")]
    Mony,
    #[serde(rename = "NCON")]
    Ncon,
    #[serde(rename = "REFS")]
    Refs,
    #[serde(rename = "SDUT")]
    Sdut,
    #[serde(rename = "BATC")]
    Batc,
    #[serde(rename = "CYCL")]
    Cycl,
    #[serde(rename = "SBLO")]
    Sblo,
    #[serde(rename = "CPEC")]
    Cpec,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "IAAD")]
    Iaad,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PHCK")]
    Phck,
    #[serde(rename = "BENO")]
    Beno,
    #[serde(rename = "BOTH")]
    Both,
    #[serde(rename = "CLHT")]
    Clht,
    #[serde(rename = "DENO")]
    Deno,
    #[serde(rename = "DISA")]
    Disa,
    #[serde(rename = "DKNY")]
    Dkny,
    #[serde(rename = "FROZ")]
    Froz,
    #[serde(rename = "LAAW")]
    Laaw,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "PRCY")]
    Prcy,
    #[serde(rename = "REGT")]
    Regt,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "PRSY")]
    Prsy,
    #[serde(rename = "CDLR")]
    Cdlr,
    #[serde(rename = "CSDH")]
    Csdh,
    #[serde(rename = "CVAL")]
    Cval,
    #[serde(rename = "INBC")]
    Inbc,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentificationSchemeName2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<RestrictedFinxMax35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalOrganisationIdentification1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentificationSchemeName2Choice {
    #[serde(flatten)]
    pub value: OrganisationIdentificationSchemeName2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessingReason15ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PendingProcessingReason3Code>,
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
pub struct PendingProcessingReason15Choice {
    #[serde(flatten)]
    pub value: PendingProcessingReason15ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingReason25 {
    #[serde(rename = "Cd")]
    pub cd: PendingReason42Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BlockChainAddressWallet7 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: RestrictedFinxMax140Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification47>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<RestrictedFinxMax70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnmatchedReason19 {
    #[serde(rename = "Cd")]
    pub cd: UnmatchedReason26Choice,
    #[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rsn_inf: Option<RestrictedFinxMax210Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessingStatus16ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<PendingProcessingReason13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingProcessingStatus16Choice {
    #[serde(flatten)]
    pub value: PendingProcessingStatus16ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeniedReason24ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DeniedReason6Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeniedReason24Choice {
    #[serde(flatten)]
    pub value: DeniedReason24ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModificationProcessingStatus11ChoiceEnum {
    #[serde(rename = "Dnd", skip_serializing_if = "Option::is_none")]
    pub dnd: Option<DeniedStatus19Choice>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryStatusAndReason7>,
    #[serde(rename = "Modfd", skip_serializing_if = "Option::is_none")]
    pub modfd: Option<ModificationStatus5Choice>,
    #[serde(rename = "AckdAccptd", skip_serializing_if = "Option::is_none")]
    pub ackd_accptd: Option<AcknowledgedAcceptedStatus30Choice>,
    #[serde(rename = "Rprd", skip_serializing_if = "Option::is_none")]
    pub rprd: Option<RepairStatus17Choice>,
    #[serde(rename = "Rjctd", skip_serializing_if = "Option::is_none")]
    pub rjctd: Option<RejectionStatus42Choice>,
    #[serde(rename = "PdgPrcg", skip_serializing_if = "Option::is_none")]
    pub pdg_prcg: Option<PendingProcessingStatus16Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModificationProcessingStatus11Choice {
    #[serde(flatten)]
    pub value: ModificationProcessingStatus11ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingReason37ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PendingReason6Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PendingReason37Choice {
    #[serde(flatten)]
    pub value: PendingReason37ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgementReason22ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AcknowledgementReason3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AcknowledgementReason22Choice {
    #[serde(flatten)]
    pub value: AcknowledgementReason22ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UnmatchedReason12Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "ACRU")]
    Acru,
    #[serde(rename = "IIND")]
    Iind,
    #[serde(rename = "CPCA")]
    Cpca,
    #[serde(rename = "CLAT")]
    Clat,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "DDEA")]
    Ddea,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "LEOG")]
    Leog,
    #[serde(rename = "LATE")]
    Late,
    #[serde(rename = "MIME")]
    Mime,
    #[serde(rename = "CMIS")]
    Cmis,
    #[serde(rename = "NMAS")]
    Nmas,
    #[serde(rename = "DTRA")]
    Dtra,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "FRAP")]
    Frap,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "INPS")]
    Inps,
    #[serde(rename = "PLCE")]
    Plce,
    #[serde(rename = "PODU")]
    Podu,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "ICUS")]
    Icus,
    #[serde(rename = "IEXE")]
    Iexe,
    #[serde(rename = "REGD")]
    Regd,
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "SETS")]
    Sets,
    #[serde(rename = "SETR")]
    Setr,
    #[serde(rename = "TXST")]
    Txst,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "DELN")]
    Deln,
    #[serde(rename = "UNBR")]
    Unbr,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "DMCT")]
    Dmct,
    #[serde(rename = "DCMX")]
    Dcmx,
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
pub struct CancellationStatus20ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<CancellationReason18>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CancellationStatus20Choice {
    #[serde(flatten)]
    pub value: CancellationStatus20ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeniedReason21ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DeniedReason4Code>,
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
pub struct DeniedReason21Choice {
    #[serde(flatten)]
    pub value: DeniedReason21ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct StatusTrail11<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "StsDt")]
    pub sts_dt: IsoDateTime,
    #[serde(rename = "SndgOrgId", skip_serializing_if = "Option::is_none")]
    pub sndg_org_id: Option<OrganisationIdentification32>,
    #[serde(rename = "UsrId", skip_serializing_if = "Option::is_none")]
    pub usr_id: Option<RestrictedFinxMax35Text>,
    #[serde(rename = "PrcgSts", skip_serializing_if = "Option::is_none")]
    pub prcg_sts: Option<ProcessingStatus92Choice>,
    #[serde(rename = "IfrrdMtchgSts", skip_serializing_if = "Option::is_none")]
    pub ifrrd_mtchg_sts: Option<MatchingStatus30Choice>,
    #[serde(rename = "MtchgSts", skip_serializing_if = "Option::is_none")]
    pub mtchg_sts: Option<MatchingStatus30Choice>,
    #[serde(rename = "SttlmSts", skip_serializing_if = "Option::is_none")]
    pub sttlm_sts: Option<SettlementStatus22Choice>,
    #[serde(rename = "ModPrcgSts", skip_serializing_if = "Option::is_none")]
    pub mod_prcg_sts: Option<ModificationProcessingStatus11Choice>,
    #[serde(rename = "CxlSts", skip_serializing_if = "Option::is_none")]
    pub cxl_sts: Option<ProcessingStatus93Choice>,
    #[serde(rename = "Sttld", skip_serializing_if = "Option::is_none")]
    pub sttld: Option<ProprietaryReason5>,
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
pub struct ProcessingStatus92ChoiceEnum {
    #[serde(rename = "PdgPrcg", skip_serializing_if = "Option::is_none")]
    pub pdg_prcg: Option<PendingProcessingStatus15Choice>,
    #[serde(rename = "Rpr", skip_serializing_if = "Option::is_none")]
    pub rpr: Option<RepairStatus16Choice>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryStatusAndReason7>,
    #[serde(rename = "Canc", skip_serializing_if = "Option::is_none")]
    pub canc: Option<CancellationStatus25Choice>,
    #[serde(rename = "Rjctd", skip_serializing_if = "Option::is_none")]
    pub rjctd: Option<RejectionStatus41Choice>,
    #[serde(rename = "CxlReqd", skip_serializing_if = "Option::is_none")]
    pub cxl_reqd: Option<ProprietaryReason5>,
    #[serde(rename = "ModReqd", skip_serializing_if = "Option::is_none")]
    pub mod_reqd: Option<ProprietaryReason5>,
    #[serde(rename = "PdgCxl", skip_serializing_if = "Option::is_none")]
    pub pdg_cxl: Option<PendingStatus46Choice>,
    #[serde(rename = "AckdAccptd", skip_serializing_if = "Option::is_none")]
    pub ackd_accptd: Option<AcknowledgedAcceptedStatus25Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProcessingStatus92Choice {
    #[serde(flatten)]
    pub value: ProcessingStatus92ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairStatus16ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<RepairReason12>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RepairStatus16Choice {
    #[serde(flatten)]
    pub value: RepairStatus16ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PendingReason9Code {
    #[serde(rename = "ADEA")]
    Adea,
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "CDRG")]
    Cdrg,
    #[serde(rename = "CDCY")]
    Cdcy,
    #[serde(rename = "CDRE")]
    Cdre,
    #[serde(rename = "CDAC")]
    Cdac,
    #[serde(rename = "INBC")]
    Inbc,
    #[default]
    Unknown,
}
