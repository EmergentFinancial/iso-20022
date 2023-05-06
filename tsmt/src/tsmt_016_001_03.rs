// Copyright 2023 Emergent Financial, LLC - All Rights Reserved
//
//
// This software is licensed under the Emergent Financial Limited Public License Version 1.0
// (EF-LPLv1). You may use, copy, modify, and distribute this software under the terms and
// conditions of the EF-LPL. For more information, please refer to the full text of the license
// at https://github.com/emergentfinancial/ef-lpl.
//
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
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
    "urn:iso:std:iso:20022:tech:xsd:tsmt.016.001.03".to_string()
}

#[derive(
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
pub struct ElementIdentification3 {
    #[validate]
    #[serde(rename = "ElmtPth")]
    pub elmt_pth: Max350Text,
    #[validate]
    #[serde(rename = "ElmtNm")]
    pub elmt_nm: Max35Text,
    #[serde(rename = "ElmtVal", skip_serializing_if = "Option::is_none")]
    pub elmt_val: Option<Max140Text>,
}
#[derive(
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
#[derive(
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
pub struct Count1 {
    #[validate]
    #[serde(rename = "Nb")]
    pub nb: Number,
}
#[derive(
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
#[serde(rename = "Document")]
pub struct Document {
    #[validate]
    #[serde(rename = "ErrRpt")]
    pub err_rpt: ErrorReportV03,
    #[serde(rename = "@xmlns", default = "namespace")]
    pub xmlns: String,
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
pub struct ErrorReportV03 {
    #[validate]
    #[serde(rename = "RptId")]
    pub rpt_id: MessageIdentification1,
    #[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<SimpleIdentificationInformation>,
    #[serde(rename = "EstblishdBaselnId", skip_serializing_if = "Option::is_none")]
    pub estblishd_baseln_id: Option<DocumentIdentification3>,
    #[serde(rename = "TxSts", skip_serializing_if = "Option::is_none")]
    pub tx_sts: Option<TransactionStatus4>,
    #[serde(rename = "UsrTxRef", skip_serializing_if = "Option::is_none")]
    pub usr_tx_ref: Option<DocumentIdentification5>,
    #[serde(rename = "RjctdMsgRef", skip_serializing_if = "Option::is_none")]
    pub rjctd_msg_ref: Option<MessageIdentification1>,
    #[validate]
    #[serde(rename = "NbOfErrs")]
    pub nb_of_errs: Count1,
    #[validate(length(min = 1,))]
    #[serde(rename = "ErrDesc", default)]
    pub err_desc: Vec<ValidationResult3>,
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
pub struct PendingActivity2 {
    #[serde(rename = "Tp")]
    pub tp: Action2Code,
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
pub struct ValidationResult3 {
    #[validate]
    #[serde(rename = "SeqNb")]
    pub seq_nb: Number,
    #[validate]
    #[serde(rename = "RuleId")]
    pub rule_id: Max35Text,
    #[validate]
    #[serde(rename = "RuleDesc")]
    pub rule_desc: Max350Text,
    #[validate(length(min = 0,))]
    #[serde(rename = "Elmt", default)]
    pub elmt: Vec<ElementIdentification3>,
}
