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

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:camt.020.001.04".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BusinessInformationQueryDefinition3 {
    #[serde(rename = "QryTp", skip_serializing_if = "Option::is_none")]
    pub qry_tp: Option<QueryType2Code>,
    #[serde(rename = "GnlBizInfCrit", skip_serializing_if = "Option::is_none")]
    pub gnl_biz_inf_crit: Option<GeneralBusinessInformationCriteriaDefinition1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RequestedIndicator {
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
pub struct GeneralBusinessInformationCriteriaDefinition1ChoiceEnum {
    #[serde(rename = "NewCrit", skip_serializing_if = "Option::is_none")]
    pub new_crit: Option<BusinessInformationCriteria1>,
    #[serde(rename = "QryNm", skip_serializing_if = "Option::is_none")]
    pub qry_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeneralBusinessInformationCriteriaDefinition1Choice {
    #[serde(flatten)]
    pub value: GeneralBusinessInformationCriteriaDefinition1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Priority1Code {
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "NORM")]
    Norm,
    #[serde(rename = "LOWW")]
    Loww,
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
pub struct GeneralBusinessInformationReturnCriteria1 {
    #[serde(rename = "QlfrInd", skip_serializing_if = "Option::is_none")]
    pub qlfr_ind: Option<RequestedIndicator>,
    #[serde(rename = "SbjtInd", skip_serializing_if = "Option::is_none")]
    pub sbjt_ind: Option<RequestedIndicator>,
    #[serde(rename = "SbjtDtlsInd", skip_serializing_if = "Option::is_none")]
    pub sbjt_dtls_ind: Option<RequestedIndicator>,
}
#[derive(
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
    #[serde(rename = "GetGnlBizInf")]
    pub get_gnl_biz_inf: GetGeneralBusinessInformationV04<A>,
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
pub struct CharacterSearch1ChoiceEnum {
    #[serde(rename = "NCT", skip_serializing_if = "Option::is_none")]
    pub nct: Option<Max35Text>,
    #[serde(rename = "NEQ", skip_serializing_if = "Option::is_none")]
    pub neq: Option<Max35Text>,
    #[serde(rename = "CT", skip_serializing_if = "Option::is_none")]
    pub ct: Option<Max35Text>,
    #[serde(rename = "EQ", skip_serializing_if = "Option::is_none")]
    pub eq: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CharacterSearch1Choice {
    #[serde(flatten)]
    pub value: CharacterSearch1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum QueryType2Code {
    #[serde(rename = "ALLL")]
    Alll,
    #[serde(rename = "CHNG")]
    Chng,
    #[serde(rename = "MODF")]
    Modf,
    #[serde(rename = "DELD")]
    Deld,
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
pub struct BusinessInformationCriteria1 {
    #[serde(rename = "NewQryNm", skip_serializing_if = "Option::is_none")]
    pub new_qry_nm: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SchCrit", default)]
    pub sch_crit: Vec<GeneralBusinessInformationSearchCriteria1>,
    #[serde(rename = "RtrCrit", skip_serializing_if = "Option::is_none")]
    pub rtr_crit: Option<GeneralBusinessInformationReturnCriteria1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InformationQualifierType1 {
    #[serde(rename = "IsFrmtd", skip_serializing_if = "Option::is_none")]
    pub is_frmtd: Option<YesNoIndicator>,
    #[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
    pub prty: Option<Priority1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GetGeneralBusinessInformationV04<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "MsgHdr")]
    pub msg_hdr: MessageHeader1,
    #[serde(rename = "GnlBizInfQryDef", skip_serializing_if = "Option::is_none")]
    pub gnl_biz_inf_qry_def: Option<BusinessInformationQueryDefinition3>,
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
pub struct MessageHeader1 {
    #[validate]
    #[serde(rename = "MsgId")]
    pub msg_id: Max35Text,
    #[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm: Option<IsoDateTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeneralBusinessInformationSearchCriteria1 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Ref", default)]
    pub r#ref: Vec<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Sbjt", default)]
    pub sbjt: Vec<CharacterSearch1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Qlfr", default)]
    pub qlfr: Vec<InformationQualifierType1>,
}
#[derive(
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
