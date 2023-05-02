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
    static ref BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:tsmt.023.001.03".to_string()
}

#[derive(
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
    #[serde(rename = "MisMtchRjctnNtfctn")]
    pub mis_mtch_rjctn_ntfctn: MisMatchRejectionNotificationV03,
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
pub struct DocumentIdentification3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "Vrsn")]
    pub vrsn: Number,
}
#[derive(
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
pub struct PendingActivity2 {
    #[serde(rename = "Tp")]
    pub tp: Action2Code,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Action2Code {
    #[serde(rename = "SBTW")]
    Sbtw,
    #[serde(rename = "RSTW")]
    Rstw,
    #[serde(rename = "RSBS")]
    Rsbs,
    #[serde(rename = "ARDM")]
    Ardm,
    #[serde(rename = "ARCS")]
    Arcs,
    #[serde(rename = "ARES")]
    Ares,
    #[serde(rename = "WAIT")]
    Wait,
    #[serde(rename = "UPDT")]
    Updt,
    #[serde(rename = "SBDS")]
    Sbds,
    #[serde(rename = "ARBA")]
    Arba,
    #[serde(rename = "ARRO")]
    Arro,
    #[serde(rename = "CINR")]
    Cinr,
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
pub struct RejectedElement1 {
    #[validate]
    #[serde(rename = "ElmtSeqNb")]
    pub elmt_seq_nb: Number,
    #[validate]
    #[serde(rename = "IndvRjctnRsn")]
    pub indv_rjctn_rsn: Max140Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionReason1ChoiceEnum {
    #[serde(rename = "RjctdElmt", skip_serializing_if = "Option::is_none")]
    pub rjctd_elmt: Option<RejectedElement1>,
    #[serde(rename = "GblRjctnRsn", skip_serializing_if = "Option::is_none")]
    pub gbl_rjctn_rsn: Option<Reason2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionReason1Choice {
    #[serde(flatten)]
    pub value: RejectionReason1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentIdentification5 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "IdIssr")]
    pub id_issr: BicIdentification1,
}
#[derive(
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
pub struct Reason2 {
    #[validate]
    #[serde(rename = "Desc")]
    pub desc: Max140Text,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MisMatchRejectionNotificationV03 {
    #[validate]
    #[serde(rename = "NtfctnId")]
    pub ntfctn_id: MessageIdentification1,
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: SimpleIdentificationInformation,
    #[validate]
    #[serde(rename = "EstblishdBaselnId")]
    pub estblishd_baseln_id: DocumentIdentification3,
    #[validate]
    #[serde(rename = "TxSts")]
    pub tx_sts: TransactionStatus4,
    #[validate(length(min = 0, max = 2,))]
    #[serde(rename = "UsrTxRef", default)]
    pub usr_tx_ref: Vec<DocumentIdentification5>,
    #[validate]
    #[serde(rename = "DataSetMtchRptRef")]
    pub data_set_mtch_rpt_ref: MessageIdentification1,
    #[serde(rename = "RjctnRsn")]
    pub rjctn_rsn: RejectionReason1Choice,
    #[serde(rename = "ReqForActn", skip_serializing_if = "Option::is_none")]
    pub req_for_actn: Option<PendingActivity2>,
}
#[derive(
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
pub struct TransactionStatus4 {
    #[serde(rename = "Sts")]
    pub sts: BaselineStatus3Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BicIdentifier {
    #[validate(regex = "BIC_IDENTIFIER_REGEX")]
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
pub struct BicIdentification1 {
    #[validate]
    #[serde(rename = "BIC")]
    pub bic: BicIdentifier,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BaselineStatus3Code {
    #[serde(rename = "PROP")]
    Prop,
    #[serde(rename = "CLSD")]
    Clsd,
    #[serde(rename = "PMTC")]
    Pmtc,
    #[serde(rename = "ESTD")]
    Estd,
    #[serde(rename = "ACTV")]
    Actv,
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "AMRQ")]
    Amrq,
    #[serde(rename = "RARQ")]
    Rarq,
    #[serde(rename = "CLRQ")]
    Clrq,
    #[serde(rename = "SCRQ")]
    Scrq,
    #[serde(rename = "SERQ")]
    Serq,
    #[serde(rename = "DARQ")]
    Darq,
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
#[derive(
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
