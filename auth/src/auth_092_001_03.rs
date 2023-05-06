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
    static ref MAX_15_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,15}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref UTI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18}[0-9]{2}[A-Z0-9]{0,32}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.092.001.03".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionReason71 {
    #[validate]
    #[serde(rename = "TxId")]
    pub tx_id: TradeTransactionIdentification24,
    #[serde(rename = "Sts")]
    pub sts: ReportingMessageStatus2Code,
    #[validate(length(min = 0,))]
    #[serde(rename = "DtldVldtnRule", default)]
    pub dtld_vldtn_rule: Vec<GenericValidationRuleIdentification1>,
}
#[derive(
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
pub struct GenericIdentification175 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max72Text,
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
pub struct DateAndDateTime2ChoiceEnum {
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
pub struct OrganisationIdentification15ChoiceEnum {
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<OrganisationIdentification38>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
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
pub struct OrganisationIdentification15Choice {
    #[serde(flatten)]
    pub value: OrganisationIdentification15ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalAgreementType1Code {
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
pub struct DetailedReportStatistics6 {
    #[validate]
    #[serde(rename = "TtlNbOfRpts")]
    pub ttl_nb_of_rpts: Max15NumericText,
    #[validate]
    #[serde(rename = "TtlNbOfRptsAccptd")]
    pub ttl_nb_of_rpts_accptd: Max15NumericText,
    #[validate]
    #[serde(rename = "TtlNbOfRptsRjctd")]
    pub ttl_nb_of_rpts_rjctd: Max15NumericText,
    #[validate(length(min = 0,))]
    #[serde(rename = "NbOfRptsRjctdPerErr", default)]
    pub nb_of_rpts_rjctd_per_err: Vec<NumberOfTransactionsPerValidationRule6>,
}
#[derive(
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
pub struct OrganisationIdentification38 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: GenericIdentification175,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max105Text>,
    #[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
    pub dmcl: Option<Max500Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericValidationRuleIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max350Text>,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<ValidationRuleSchemeName1Choice>,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReportingMessageStatus2Code {
    #[serde(rename = "ACPT")]
    Acpt,
    #[serde(rename = "RJCT")]
    Rjct,
    #[serde(rename = "INCF")]
    Incf,
    #[serde(rename = "CRPT")]
    Crpt,
    #[serde(rename = "NAUT")]
    Naut,
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
pub struct DetailedTransactionStatistics29 {
    #[validate]
    #[serde(rename = "TtlNbOfTxs")]
    pub ttl_nb_of_txs: Max15NumericText,
    #[validate]
    #[serde(rename = "TtlNbOfTxsAccptd")]
    pub ttl_nb_of_txs_accptd: StatisticsPerActionType1,
    #[validate]
    #[serde(rename = "TtlNbOfTxsRjctd")]
    pub ttl_nb_of_txs_rjctd: StatisticsPerActionType1,
    #[serde(rename = "TtlCrrctdRjctns", skip_serializing_if = "Option::is_none")]
    pub ttl_crrctd_rjctns: Option<StatisticsPerActionType1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TxsRjctnsRsn", default)]
    pub txs_rjctns_rsn: Vec<RejectionReason71>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralPortfolioCode5ChoiceEnum {
    #[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
    pub prtfl: Option<PortfolioCode3Choice>,
    #[serde(rename = "MrgnPrtflCd", skip_serializing_if = "Option::is_none")]
    pub mrgn_prtfl_cd: Option<MarginPortfolio3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CollateralPortfolioCode5Choice {
    #[serde(flatten)]
    pub value: CollateralPortfolioCode5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LegalPersonIdentification1 {
    #[serde(rename = "Id")]
    pub id: OrganisationIdentification15Choice,
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
#[serde(rename = "Document")]
pub struct Document<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "DerivsTradRjctnSttstclRpt")]
    pub derivs_trad_rjctn_sttstcl_rpt: DerivativesTradeRejectionStatisticalReportV03<A>,
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
pub struct Max72Text {
    #[validate(length(min = 1, max = 72,))]
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
pub struct Max15NumericText {
    #[validate(regex = "MAX_15_NUMERIC_TEXT_REGEX")]
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
pub struct Max52Text {
    #[validate(length(min = 1, max = 52,))]
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
pub struct PortfolioCode3ChoiceEnum {
    #[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
    pub no_prtfl: Option<NotApplicable1Code>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Max52Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PortfolioCode3Choice {
    #[serde(flatten)]
    pub value: PortfolioCode3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DetailedTransactionStatistics6ChoiceEnum {
    #[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
    pub data_set_actn: Option<ReportPeriodActivity1Code>,
    #[serde(rename = "DtldSttstcs", skip_serializing_if = "Option::is_none")]
    pub dtld_sttstcs: Option<DetailedTransactionStatistics29>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DetailedTransactionStatistics6Choice {
    #[serde(flatten)]
    pub value: DetailedTransactionStatistics6ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max500Text {
    #[validate(length(min = 1, max = 500,))]
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
pub struct UniqueTransactionIdentifier2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification175>,
    #[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
    pub unq_tx_idr: Option<UtiIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UniqueTransactionIdentifier2Choice {
    #[serde(flatten)]
    pub value: UniqueTransactionIdentifier2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionReason70 {
    #[validate]
    #[serde(rename = "MsgRptId")]
    pub msg_rpt_id: Max140Text,
    #[serde(rename = "Sts")]
    pub sts: ReportingMessageStatus2Code,
    #[serde(rename = "DtldVldtnRule", skip_serializing_if = "Option::is_none")]
    pub dtld_vldtn_rule: Option<GenericValidationRuleIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PortfolioIdentification3 {
    #[validate]
    #[serde(rename = "Cd")]
    pub cd: Max52Text,
    #[serde(rename = "PrtflTxXmptn", skip_serializing_if = "Option::is_none")]
    pub prtfl_tx_xmptn: Option<TrueFalseIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PortfolioCode5ChoiceEnum {
    #[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
    pub no_prtfl: Option<NotApplicable1Code>,
    #[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
    pub prtfl: Option<PortfolioIdentification3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PortfolioCode5Choice {
    #[serde(flatten)]
    pub value: PortfolioCode5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct StatisticsPerCounterparty17ChoiceEnum {
    #[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
    pub rpt: Option<DetailedStatisticsPerCounterparty18>,
    #[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
    pub data_set_actn: Option<ReportPeriodActivity1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct StatisticsPerCounterparty17Choice {
    #[serde(flatten)]
    pub value: StatisticsPerCounterparty17ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NaturalPersonIdentification3 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: NaturalPersonIdentification2,
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
pub struct TrueFalseIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NotApplicable1Code {
    #[serde(rename = "NOAP")]
    Noap,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ReportPeriodActivity1Code {
    #[serde(rename = "NOTX")]
    Notx,
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
pub struct DerivativesTradeRejectionStatisticalReportV03<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "RjctnSttstcs")]
    pub rjctn_sttstcs: StatisticsPerCounterparty17Choice,
    #[validate(length(min = 0,))]
    #[serde(rename = "SplmtryData", default)]
    pub splmtry_data: Vec<SupplementaryData1<A>>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransactionOperationType10Code {
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "CORR")]
    Corr,
    #[serde(rename = "EROR")]
    Eror,
    #[serde(rename = "MODI")]
    Modi,
    #[serde(rename = "NEWT")]
    Newt,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "POSC")]
    Posc,
    #[serde(rename = "REVI")]
    Revi,
    #[serde(rename = "TERM")]
    Term,
    #[serde(rename = "VALU")]
    Valu,
    #[serde(rename = "MARU")]
    Maru,
    #[serde(rename = "PRTO")]
    Prto,
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
pub struct TradeTransactionIdentification24 {
    #[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
    pub tech_rcrd_id: Option<Max140Text>,
    #[serde(rename = "ActnTp", skip_serializing_if = "Option::is_none")]
    pub actn_tp: Option<TransactionOperationType10Code>,
    #[serde(rename = "RptgTmStmp", skip_serializing_if = "Option::is_none")]
    pub rptg_tm_stmp: Option<IsoDateTime>,
    #[serde(rename = "DerivEvtTp", skip_serializing_if = "Option::is_none")]
    pub deriv_evt_tp: Option<DerivativeEventType3Code>,
    #[serde(rename = "DerivEvtTmStmp", skip_serializing_if = "Option::is_none")]
    pub deriv_evt_tm_stmp: Option<DateAndDateTime2Choice>,
    #[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
    pub othr_ctr_pty: Option<PartyIdentification248Choice>,
    #[serde(rename = "UnqIdr", skip_serializing_if = "Option::is_none")]
    pub unq_idr: Option<UniqueTransactionIdentifier2Choice>,
    #[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
    pub mstr_agrmt: Option<MasterAgreement8>,
    #[serde(rename = "CollPrtflCd", skip_serializing_if = "Option::is_none")]
    pub coll_prtfl_cd: Option<CollateralPortfolioCode5Choice>,
}
#[derive(
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
pub struct MasterAgreement8 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<AgreementType2Choice>,
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Max50Text>,
    #[serde(rename = "OthrMstrAgrmtDtls", skip_serializing_if = "Option::is_none")]
    pub othr_mstr_agrmt_dtls: Option<Max350Text>,
}
#[derive(
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
pub struct PartyIdentification248ChoiceEnum {
    #[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
    pub ntrl: Option<NaturalPersonIdentification3>,
    #[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
    pub lgl: Option<LegalPersonIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification248Choice {
    #[serde(flatten)]
    pub value: PartyIdentification248ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RejectionStatistics8 {
    #[validate]
    #[serde(rename = "CtrPtyId")]
    pub ctr_pty_id: CounterpartyData92,
    #[validate]
    #[serde(rename = "RptSttstcs")]
    pub rpt_sttstcs: DetailedReportStatistics6,
    #[serde(rename = "DerivSttstcs")]
    pub deriv_sttstcs: DetailedTransactionStatistics6Choice,
}
#[derive(
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
pub struct CounterpartyData92 {
    #[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
    pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "RptSubmitgNtty", skip_serializing_if = "Option::is_none")]
    pub rpt_submitg_ntty: Option<OrganisationIdentification15Choice>,
    #[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
    pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max20PositiveNumber {
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
pub struct UtiIdentifier {
    #[validate(regex = "UTI_IDENTIFIER_REGEX")]
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
pub struct AgreementType2ChoiceEnum {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ExternalAgreementType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max50Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgreementType2Choice {
    #[serde(flatten)]
    pub value: AgreementType2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DerivativeEventType3Code {
    #[serde(rename = "ALOC")]
    Aloc,
    #[serde(rename = "CLRG")]
    Clrg,
    #[serde(rename = "CLAL")]
    Clal,
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "CORP")]
    Corp,
    #[serde(rename = "CREV")]
    Crev,
    #[serde(rename = "ETRM")]
    Etrm,
    #[serde(rename = "EXER")]
    Exer,
    #[serde(rename = "INCP")]
    Incp,
    #[serde(rename = "NOVA")]
    Nova,
    #[serde(rename = "PTNG")]
    Ptng,
    #[serde(rename = "TRAD")]
    Trad,
    #[serde(rename = "UPDT")]
    Updt,
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
pub struct StatisticsPerActionType1 {
    #[validate]
    #[serde(rename = "All")]
    pub all: Max20PositiveNumber,
    #[validate]
    #[serde(rename = "New")]
    pub new: Max20PositiveNumber,
    #[validate]
    #[serde(rename = "Mod")]
    pub r#mod: Max20PositiveNumber,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NaturalPersonIdentification2 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: GenericIdentification175,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max105Text>,
    #[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
    pub dmcl: Option<Max500Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalValidationRuleIdentification1Code {
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
pub struct MarginPortfolio3 {
    #[serde(rename = "InitlMrgnPrtflCd")]
    pub initl_mrgn_prtfl_cd: PortfolioCode5Choice,
    #[serde(rename = "VartnMrgnPrtflCd", skip_serializing_if = "Option::is_none")]
    pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NumberOfTransactionsPerValidationRule6 {
    #[validate]
    #[serde(rename = "DtldNb")]
    pub dtld_nb: Max15NumericText,
    #[validate(length(min = 1,))]
    #[serde(rename = "RptSts", default)]
    pub rpt_sts: Vec<RejectionReason70>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DetailedStatisticsPerCounterparty18 {
    #[validate]
    #[serde(rename = "RefDt")]
    pub ref_dt: IsoDate,
    #[validate]
    #[serde(rename = "TtlNbOfRpts")]
    pub ttl_nb_of_rpts: Max15NumericText,
    #[validate]
    #[serde(rename = "TtlNbOfRptsAccptd")]
    pub ttl_nb_of_rpts_accptd: Max15NumericText,
    #[validate]
    #[serde(rename = "TtlNbOfRptsRjctd")]
    pub ttl_nb_of_rpts_rjctd: Max15NumericText,
    #[validate]
    #[serde(rename = "TtlNbOfTxs")]
    pub ttl_nb_of_txs: Max15NumericText,
    #[validate]
    #[serde(rename = "TtlNbOfTxsAccptd")]
    pub ttl_nb_of_txs_accptd: StatisticsPerActionType1,
    #[validate]
    #[serde(rename = "TtlNbOfTxsRjctd")]
    pub ttl_nb_of_txs_rjctd: StatisticsPerActionType1,
    #[serde(rename = "TtlCrrctdRjctns", skip_serializing_if = "Option::is_none")]
    pub ttl_crrctd_rjctns: Option<StatisticsPerActionType1>,
    #[validate(length(min = 1,))]
    #[serde(rename = "RjctnSttstcs", default)]
    pub rjctn_sttstcs: Vec<RejectionStatistics8>,
}
#[derive(
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
pub struct ValidationRuleSchemeName1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalValidationRuleIdentification1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ValidationRuleSchemeName1Choice {
    #[serde(flatten)]
    pub value: ValidationRuleSchemeName1ChoiceEnum,
}
