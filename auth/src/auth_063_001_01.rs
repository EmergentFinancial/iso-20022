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
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.063.001.01".to_string()
}

#[derive(
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
pub struct LiquidResourceInformation1 {
    #[serde(rename = "CntrPtyId", skip_serializing_if = "Option::is_none")]
    pub cntr_pty_id: Option<Max35Text>,
    #[validate]
    #[serde(rename = "LqdRsrcVal")]
    pub lqd_rsrc_val: AmountAndDirection102,
    #[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
    pub mkt_val: Option<AmountAndDirection102>,
    #[validate]
    #[serde(rename = "Scrd")]
    pub scrd: TrueFalseIndicator,
    #[validate]
    #[serde(rename = "AsstNcmbrd")]
    pub asst_ncmbrd: TrueFalseIndicator,
    #[validate]
    #[serde(rename = "QlfygRsrc")]
    pub qlfyg_rsrc: TrueFalseIndicator,
    #[validate]
    #[serde(rename = "AgcyArrgmnts")]
    pub agcy_arrgmnts: TrueFalseIndicator,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementDate6Code {
    #[serde(rename = "TFIV")]
    Tfiv,
    #[serde(rename = "TFOR")]
    Tfor,
    #[serde(rename = "TONE")]
    Tone,
    #[serde(rename = "TTRE")]
    Ttre,
    #[serde(rename = "TTWO")]
    Ttwo,
    #[serde(rename = "SAMD")]
    Samd,
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
pub struct LiquidResources1 {
    #[validate(length(min = 1,))]
    #[serde(rename = "CshDue", default)]
    pub csh_due: Vec<LiquidResourceInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FcltiesCmmtdLinesOfCdt", default)]
    pub fclties_cmmtd_lines_of_cdt: Vec<LiquidResourceInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FcltiesCmmtdRpAgrmts", default)]
    pub fclties_cmmtd_rp_agrmts: Vec<LiquidResourceInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FcltiesCmmtdFxSwps", default)]
    pub fclties_cmmtd_fx_swps: Vec<LiquidResourceInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FcltiesOthrCmmtd", default)]
    pub fclties_othr_cmmtd: Vec<LiquidResourceInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FcltiesUcmmtd", default)]
    pub fclties_ucmmtd: Vec<LiquidResourceInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FinInstrmsCCP", default)]
    pub fin_instrms_ccp: Vec<LiquidResourceInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FinInstrmsTrsrInvstmts", default)]
    pub fin_instrms_trsr_invstmts: Vec<LiquidResourceInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FinInstrmsDfltrsSttlmColl", default)]
    pub fin_instrms_dfltrs_sttlm_coll: Vec<LiquidResourceInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FinInstrmsDfltrsNonCshColl", default)]
    pub fin_instrms_dfltrs_non_csh_coll: Vec<LiquidResourceInformation1>,
}
#[derive(
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
pub struct LiquidityStressTestResult1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max256Text,
    #[validate]
    #[serde(rename = "ScnroDfltrs")]
    pub scnro_dfltrs: CoverTwoDefaulters1,
    #[validate(length(min = 6, max = 6,))]
    #[serde(rename = "LqdtyReqrdAndAvlbl", default)]
    pub lqdty_reqrd_and_avlbl: Vec<LiquidityRequiredAndAvailable1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TrueFalseIndicator {
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
pub struct StressLiquidResourceRequirement1 {
    #[validate]
    #[serde(rename = "OprlOutflw")]
    pub oprl_outflw: AmountAndDirection102,
    #[validate]
    #[serde(rename = "VartnMrgnPmtOblgtn")]
    pub vartn_mrgn_pmt_oblgtn: AmountAndDirection102,
    #[validate]
    #[serde(rename = "SttlmOrDlvry")]
    pub sttlm_or_dlvry: AmountAndDirection102,
    #[validate]
    #[serde(rename = "Othr")]
    pub othr: AmountAndDirection102,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CcpLiquidityStressTestingResultReportV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate(length(min = 1,))]
    #[serde(rename = "LqdtyStrssTstRslt", default)]
    pub lqdty_strss_tst_rslt: Vec<LiquidityStressTestResult1>,
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "CCPLqdtyStrssTstgRsltRpt")]
    pub ccp_lqdty_strss_tstg_rslt_rpt: CcpLiquidityStressTestingResultReportV01<A>,
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
pub struct Max256Text {
    #[validate(length(min = 1, max = 256,))]
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
pub struct LiquidityRequiredAndAvailable1 {
    #[validate]
    #[serde(rename = "LqdRsrcs")]
    pub lqd_rsrcs: LiquidResources1,
    #[serde(rename = "LqdtyHrzn")]
    pub lqdty_hrzn: SettlementDate6Code,
    #[validate]
    #[serde(rename = "StrssLqdRsrcRqrmnt")]
    pub strss_lqd_rsrc_rqrmnt: StressLiquidResourceRequirement1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CoverTwoDefaulters1 {
    #[validate]
    #[serde(rename = "Cover1Id")]
    pub cover_1_id: LeiIdentifier,
    #[validate]
    #[serde(rename = "Cover2Id")]
    pub cover_2_id: LeiIdentifier,
}
#[derive(
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
pub struct PlusOrMinusIndicator {
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
pub struct AmountAndDirection102 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "Sgn")]
    pub sgn: PlusOrMinusIndicator,
}
#[derive(
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
pub struct SupplementaryDataEnvelope1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(flatten)]
    pub value: A,
}
