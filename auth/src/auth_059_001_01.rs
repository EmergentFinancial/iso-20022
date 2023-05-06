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
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.059.001.01".to_string()
}

#[derive(
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
pub struct IncomeStatement1 {
    #[validate]
    #[serde(rename = "ClrFees")]
    pub clr_fees: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "OthrOprgRvn")]
    pub othr_oprg_rvn: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "OprgExpnss")]
    pub oprg_expnss: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "OprgPrftOrLoss")]
    pub oprg_prft_or_loss: AmountAndDirection102,
    #[validate]
    #[serde(rename = "NetIntrstIncm")]
    pub net_intrst_incm: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "OthrNonOprgRvn")]
    pub othr_non_oprg_rvn: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "NonOprgExpnss")]
    pub non_oprg_expnss: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "PreTaxPrftOrLoss")]
    pub pre_tax_prft_or_loss: AmountAndDirection102,
    #[validate]
    #[serde(rename = "PstTaxPrftOrLoss")]
    pub pst_tax_prft_or_loss: AmountAndDirection102,
}
#[derive(
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
pub struct CcpIncomeStatementAndCapitalAdequacyReportV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "IncmStmt")]
    pub incm_stmt: IncomeStatement1,
    #[validate]
    #[serde(rename = "CptlRqrmnts")]
    pub cptl_rqrmnts: CapitalRequirement1,
    #[validate]
    #[serde(rename = "TtlCptl")]
    pub ttl_cptl: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "LqdFinRsrcs")]
    pub lqd_fin_rsrcs: ActiveCurrencyAndAmount,
    #[validate(length(min = 1,))]
    #[serde(rename = "HpthtclCptlMeasr", default)]
    pub hpthtcl_cptl_measr: Vec<HypotheticalCapitalMeasure1>,
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "CCPIncmStmtAndCptlAdqcyRpt")]
    pub ccp_incm_stmt_and_cptl_adqcy_rpt: CcpIncomeStatementAndCapitalAdequacyReportV01<A>,
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
pub struct HypotheticalCapitalMeasure1 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "DfltWtrfllId")]
    pub dflt_wtrfll_id: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CapitalRequirement1 {
    #[validate]
    #[serde(rename = "WndgDwnOrRstrgRsk")]
    pub wndg_dwn_or_rstrg_rsk: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "OprlAndLglRsk")]
    pub oprl_and_lgl_rsk: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "CdtRsk")]
    pub cdt_rsk: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "CntrPtyRsk")]
    pub cntr_pty_rsk: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "MktRsk")]
    pub mkt_rsk: ActiveCurrencyAndAmount,
    #[validate]
    #[serde(rename = "BizRsk")]
    pub biz_rsk: ActiveCurrencyAndAmount,
    #[serde(rename = "NtfctnBffr", skip_serializing_if = "Option::is_none")]
    pub ntfctn_bffr: Option<BaseOneRate>,
}
