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
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_2_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.100.001.01".to_string()
}

#[derive(
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
pub struct SettlementFailsDailyCsd3 {
    #[serde(rename = "IntraCSD")]
    pub intra_csd: SettlementFailsDailyInstructionType1Choice,
    #[serde(rename = "CrossCSD")]
    pub cross_csd: SettlementFailsDailyInstructionType1Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementFailsDailyTransactionType1ChoiceEnum {
    #[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<SettlementFailsDailyTransactionType3>,
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
pub struct SettlementFailsDailyTransactionType1Choice {
    #[serde(flatten)]
    pub value: SettlementFailsDailyTransactionType1ChoiceEnum,
}
#[derive(
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
pub struct SettlementFailsInstrument2 {
    #[serde(rename = "Eqty")]
    pub eqty: SettlementTotalData1Choice,
    #[serde(rename = "SvrgnDebt")]
    pub svrgn_debt: SettlementTotalData1Choice,
    #[serde(rename = "Bd")]
    pub bd: SettlementTotalData1Choice,
    #[serde(rename = "OthrTrfblScties")]
    pub othr_trfbl_scties: SettlementTotalData1Choice,
    #[serde(rename = "XchgTraddFnds")]
    pub xchg_tradd_fnds: SettlementTotalData1Choice,
    #[serde(rename = "CllctvInvstmtUdrtkgs")]
    pub cllctv_invstmt_udrtkgs: SettlementTotalData1Choice,
    #[serde(rename = "MnyMktInstrm")]
    pub mny_mkt_instrm: SettlementTotalData1Choice,
    #[serde(rename = "EmssnAllwnc")]
    pub emssn_allwnc: SettlementTotalData1Choice,
    #[serde(rename = "Othr")]
    pub othr: SettlementTotalData1Choice,
}
#[derive(
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
pub struct Max2048Text {
    #[validate(length(min = 1, max = 2048,))]
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
pub struct Max2Fraction1NonNegativeNumber {
    #[validate(range(min = 0, max = 9.9,))]
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
pub struct SettlementDailyFailureReason3 {
    #[serde(rename = "FaildScties")]
    pub faild_scties: SettlementTotalData1Choice,
    #[serde(rename = "FaildCsh")]
    pub faild_csh: SettlementTotalData1Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementDataVolume2 {
    #[validate]
    #[serde(rename = "Vol")]
    pub vol: Max20PositiveNumber,
    #[validate]
    #[serde(rename = "Val")]
    pub val: Max20PositiveDecimalNumber,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementFailsDailyData3 {
    #[validate]
    #[serde(rename = "RptgDt")]
    pub rptg_dt: IsoDate,
    #[validate]
    #[serde(rename = "DalyRcrd")]
    pub daly_rcrd: SettlementFailsDailyInstrument3,
}
#[derive(
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
pub struct PhoneNumber {
    #[validate(regex = "PHONE_NUMBER_REGEX")]
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
pub struct SettlementFailsData3 {
    #[validate]
    #[serde(rename = "Ttl")]
    pub ttl: SettlementTotalData1,
    #[serde(rename = "PtcptInFail", skip_serializing_if = "Option::is_none")]
    pub ptcpt_in_fail: Option<SettlementFailsParticipantRange1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FlsPerCcy", default)]
    pub fls_per_ccy: Vec<SettlementFailsCurrency2>,
    #[serde(rename = "FlsPerFinInstrmTp", skip_serializing_if = "Option::is_none")]
    pub fls_per_fin_instrm_tp: Option<SettlementFailsInstrument2>,
    #[serde(rename = "SctiesInFail", skip_serializing_if = "Option::is_none")]
    pub scties_in_fail: Option<SettlementFailsSecuritiesRange1>,
    #[serde(rename = "FlsPerTxTp", skip_serializing_if = "Option::is_none")]
    pub fls_per_tx_tp: Option<SettlementFailsTransactionType2>,
    #[serde(rename = "TtlSttlmPnlties", skip_serializing_if = "Option::is_none")]
    pub ttl_sttlm_pnlties: Option<SettlementDataVolume2>,
    #[validate]
    #[serde(rename = "FailrRsn")]
    pub failr_rsn: SettlementFailureReason3,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementFailsDailyInstructionType1ChoiceEnum {
    #[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<SettlementFailsDailyInstructionType3>,
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
pub struct SettlementFailsDailyInstructionType1Choice {
    #[serde(flatten)]
    pub value: SettlementFailsDailyInstructionType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Contact9 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max140Text,
    #[validate]
    #[serde(rename = "PhneNb")]
    pub phne_nb: PhoneNumber,
    #[validate]
    #[serde(rename = "EmailAdr")]
    pub email_adr: Max256Text,
    #[serde(rename = "Fctn", skip_serializing_if = "Option::is_none")]
    pub fctn: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max20PositiveDecimalNumber {
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
pub struct SettlementFailsDailyCsd1ChoiceEnum {
    #[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<SettlementFailsDailyCsd3>,
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
pub struct SettlementFailsDailyCsd1Choice {
    #[serde(flatten)]
    pub value: SettlementFailsDailyCsd1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementFailsParticipant1 {
    #[validate]
    #[serde(rename = "LEI")]
    pub lei: LeiIdentifier,
    #[validate]
    #[serde(rename = "Rank")]
    pub rank: Max2NumericText,
    #[validate]
    #[serde(rename = "Aggt")]
    pub aggt: SettlementTotalData1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementFailsTransactionType2 {
    #[serde(rename = "SctiesBuyOrSell")]
    pub scties_buy_or_sell: SettlementTotalData1Choice,
    #[serde(rename = "CollMgmtOpr")]
    pub coll_mgmt_opr: SettlementTotalData1Choice,
    #[serde(rename = "SctiesLndgOrBrrwg")]
    pub scties_lndg_or_brrwg: SettlementTotalData1Choice,
    #[serde(rename = "RpAgrmt")]
    pub rp_agrmt: SettlementTotalData1Choice,
    #[serde(rename = "Othr")]
    pub othr: SettlementTotalData1Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementFailsReportHeader2 {
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "RptgPrd")]
    pub rptg_prd: DatePeriod2,
    #[serde(rename = "Ccy")]
    pub ccy: ActiveCurrencyCode,
    #[serde(rename = "RptSts")]
    pub rpt_sts: TransactionOperationType4Code,
    #[validate]
    #[serde(rename = "SctiesSttlmSys")]
    pub scties_sttlm_sys: SecuritiesSettlementSystemIdentification2,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementTotalData1 {
    #[validate]
    #[serde(rename = "Sttld")]
    pub sttld: SettlementDataVolume2,
    #[validate]
    #[serde(rename = "Faild")]
    pub faild: SettlementDataVolume2,
    #[validate]
    #[serde(rename = "Ttl")]
    pub ttl: SettlementDataVolume2,
    #[validate]
    #[serde(rename = "FaildRate")]
    pub faild_rate: SettlementDataRate2,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementTotalData1ChoiceEnum {
    #[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
    pub data_set_actn: Option<ReportPeriodActivity1Code>,
    #[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<SettlementTotalData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementTotalData1Choice {
    #[serde(flatten)]
    pub value: SettlementTotalData1ChoiceEnum,
}
#[derive(
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
pub struct Max2NumericText {
    #[validate(regex = "MAX_2_NUMERIC_TEXT_REGEX")]
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
pub struct SecuritiesSettlementSystemIdentification2 {
    #[validate]
    #[serde(rename = "SysId")]
    pub sys_id: Max35Text,
    #[serde(rename = "SysNm", skip_serializing_if = "Option::is_none")]
    pub sys_nm: Option<Max140Text>,
    #[serde(rename = "CtryOfJursdctn", skip_serializing_if = "Option::is_none")]
    pub ctry_of_jursdctn: Option<CountryCode>,
    #[serde(rename = "CSDLglNm", skip_serializing_if = "Option::is_none")]
    pub csd_lgl_nm: Option<Max140Text>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RspnsblPty", default)]
    pub rspnsbl_pty: Vec<Contact9>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementFailsMonthlyReportV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "RptHdr")]
    pub rpt_hdr: SettlementFailsReportHeader2,
    #[validate]
    #[serde(rename = "MnthlyAggt")]
    pub mnthly_aggt: SettlementFailsData3,
    #[validate(length(min = 1,))]
    #[serde(rename = "DalyData", default)]
    pub daly_data: Vec<SettlementFailsDailyData3>,
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
pub struct SettlementFailsDailyInstructionType3 {
    #[serde(rename = "DlvryVrssPmt")]
    pub dlvry_vrss_pmt: SettlementDailyFailureReason1Choice,
    #[serde(rename = "DlvryWthPmt")]
    pub dlvry_wth_pmt: SettlementDailyFailureReason1Choice,
    #[serde(rename = "PmtFreeOfDlvry")]
    pub pmt_free_of_dlvry: SettlementDailyFailureReason1Choice,
    #[serde(rename = "FreeOfPmt")]
    pub free_of_pmt: SettlementDailyFailureReason1Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementFailsDailyTransactionType3 {
    #[serde(rename = "SctiesBuyOrSell")]
    pub scties_buy_or_sell: SettlementFailsDailyCsd1Choice,
    #[serde(rename = "CollMgmtOpr")]
    pub coll_mgmt_opr: SettlementFailsDailyCsd1Choice,
    #[serde(rename = "SctiesLndgOrBrrwg")]
    pub scties_lndg_or_brrwg: SettlementFailsDailyCsd1Choice,
    #[serde(rename = "RpAgrmt")]
    pub rp_agrmt: SettlementFailsDailyCsd1Choice,
    #[serde(rename = "Othr")]
    pub othr: SettlementFailsDailyCsd1Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementFailsSecurities1 {
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: SecurityIdentification19,
    #[validate]
    #[serde(rename = "Rank")]
    pub rank: Max2NumericText,
}
#[derive(
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
pub struct SettlementFailsParticipantRange1 {
    #[validate(length(min = 1,))]
    #[serde(rename = "HghstInVol", default)]
    pub hghst_in_vol: Vec<SettlementFailsParticipant1>,
    #[validate(length(min = 1,))]
    #[serde(rename = "HghstInVal", default)]
    pub hghst_in_val: Vec<SettlementFailsParticipant1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementFailureReason2 {
    #[validate]
    #[serde(rename = "MainRsns")]
    pub main_rsns: Max2048Text,
    #[validate]
    #[serde(rename = "EffcncyImprvmt")]
    pub effcncy_imprvmt: Max2048Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementFailsDailyInstrument3 {
    #[serde(rename = "Eqty")]
    pub eqty: SettlementFailsDailyTransactionType1Choice,
    #[serde(rename = "SvrgnDebt")]
    pub svrgn_debt: SettlementFailsDailyTransactionType1Choice,
    #[serde(rename = "Bd")]
    pub bd: SettlementFailsDailyTransactionType1Choice,
    #[serde(rename = "OthrTrfblScties")]
    pub othr_trfbl_scties: SettlementFailsDailyTransactionType1Choice,
    #[serde(rename = "XchgTraddFnds")]
    pub xchg_tradd_fnds: SettlementFailsDailyTransactionType1Choice,
    #[serde(rename = "CllctvInvstmtUdrtkgs")]
    pub cllctv_invstmt_udrtkgs: SettlementFailsDailyTransactionType1Choice,
    #[serde(rename = "MnyMktInstrm")]
    pub mny_mkt_instrm: SettlementFailsDailyTransactionType1Choice,
    #[serde(rename = "EmssnAllwnc")]
    pub emssn_allwnc: SettlementFailsDailyTransactionType1Choice,
    #[serde(rename = "Othr")]
    pub othr: SettlementFailsDailyTransactionType1Choice,
}
#[derive(
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
    #[serde(rename = "SttlmFlsMnthlyRpt")]
    pub sttlm_fls_mnthly_rpt: SettlementFailsMonthlyReportV01<A>,
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
pub struct SettlementDailyFailureReason1ChoiceEnum {
    #[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<SettlementDailyFailureReason3>,
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
pub struct SettlementDailyFailureReason1Choice {
    #[serde(flatten)]
    pub value: SettlementDailyFailureReason1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementFailsCurrency2 {
    #[serde(rename = "Ccy")]
    pub ccy: ActiveCurrencyCode,
    #[validate]
    #[serde(rename = "Data")]
    pub data: SettlementTotalData1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementFailureReason3 {
    #[serde(rename = "AvrgDrtn", skip_serializing_if = "Option::is_none")]
    pub avrg_drtn: Option<Max2Fraction1NonNegativeNumber>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Desc", default)]
    pub desc: Vec<SettlementFailureReason2>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransactionOperationType4Code {
    #[serde(rename = "NEWT")]
    Newt,
    #[serde(rename = "AMND")]
    Amnd,
    #[serde(rename = "CANC")]
    Canc,
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
pub struct SettlementFailsSecuritiesRange1 {
    #[validate(length(min = 1,))]
    #[serde(rename = "HghstInVol", default)]
    pub hghst_in_vol: Vec<SettlementFailsSecurities1>,
    #[validate(length(min = 1,))]
    #[serde(rename = "HghstInVal", default)]
    pub hghst_in_val: Vec<SettlementFailsSecurities1>,
}
#[derive(
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
pub struct SettlementDataRate2 {
    #[validate]
    #[serde(rename = "Vol")]
    pub vol: PercentageRate,
    #[validate]
    #[serde(rename = "Val")]
    pub val: PercentageRate,
}
