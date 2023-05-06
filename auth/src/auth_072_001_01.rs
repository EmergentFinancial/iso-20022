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
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_2_UPPER_CASE_ALPHA_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.072.001.01".to_string()
}

#[derive(
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
    #[serde(rename = "SttlmIntlrRpt")]
    pub sttlm_intlr_rpt: SettlementInternaliserReportV01<A>,
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
pub struct InternalisationData2 {
    #[validate]
    #[serde(rename = "Sttld")]
    pub sttld: InternalisationDataVolume1,
    #[validate]
    #[serde(rename = "Faild")]
    pub faild: InternalisationDataVolume1,
    #[validate]
    #[serde(rename = "Ttl")]
    pub ttl: InternalisationDataVolume1,
}
#[derive(
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
pub struct SupplementaryData1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
    pub plc_and_nm: Option<Max350Text>,
    #[validate]
    #[serde(rename = "Envlp")]
    pub envlp: SupplementaryDataEnvelope1<A>,
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
pub struct InternalisationDataVolume1 {
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
pub struct SettlementInternaliserTransactionType1 {
    #[validate]
    #[serde(rename = "SctiesBuyOrSell")]
    pub scties_buy_or_sell: InternalisationData1,
    #[validate]
    #[serde(rename = "CollMgmtOpr")]
    pub coll_mgmt_opr: InternalisationData1,
    #[validate]
    #[serde(rename = "SctiesLndgOrBrrwg")]
    pub scties_lndg_or_brrwg: InternalisationData1,
    #[validate]
    #[serde(rename = "RpAgrmt")]
    pub rp_agrmt: InternalisationData1,
    #[validate]
    #[serde(rename = "OthrTxs")]
    pub othr_txs: InternalisationData1,
}
#[derive(
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
pub struct SettlementInternaliserReportV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "RptHdr")]
    pub rpt_hdr: SettlementInternaliserReportHeader1,
    #[validate]
    #[serde(rename = "SttlmIntlr")]
    pub sttlm_intlr: SettlementInternaliser1,
    #[validate(length(min = 1,))]
    #[serde(rename = "IssrCSD", default)]
    pub issr_csd: Vec<IssuerCsdReport1>,
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
pub struct ContactDetails4 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max140Text,
    #[validate]
    #[serde(rename = "PhneNb")]
    pub phne_nb: PhoneNumber,
    #[validate]
    #[serde(rename = "EmailAdr")]
    pub email_adr: Max2048Text,
    #[validate]
    #[serde(rename = "Fctn")]
    pub fctn: Max140Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementInternaliserReportHeader1 {
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "RptgDt")]
    pub rptg_dt: IsoDate,
    #[serde(rename = "Ccy")]
    pub ccy: ActiveCurrencyCode,
    #[serde(rename = "RptSts")]
    pub rpt_sts: TransactionOperationType4Code,
}
#[derive(
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
pub struct InternalisationData1 {
    #[validate]
    #[serde(rename = "Aggt")]
    pub aggt: InternalisationData2,
    #[validate]
    #[serde(rename = "FaildRate")]
    pub faild_rate: InternalisationDataRate1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementInternaliserFinancialInstrument1 {
    #[validate]
    #[serde(rename = "Eqty")]
    pub eqty: InternalisationData1,
    #[validate]
    #[serde(rename = "SvrgnDebt")]
    pub svrgn_debt: InternalisationData1,
    #[validate]
    #[serde(rename = "Bd")]
    pub bd: InternalisationData1,
    #[validate]
    #[serde(rename = "OthrTrfblScties")]
    pub othr_trfbl_scties: InternalisationData1,
    #[validate]
    #[serde(rename = "XchgTradgFnds")]
    pub xchg_tradg_fnds: InternalisationData1,
    #[validate]
    #[serde(rename = "CllctvInvstmtUdrtkgs")]
    pub cllctv_invstmt_udrtkgs: InternalisationData1,
    #[validate]
    #[serde(rename = "MnyMktInstrm")]
    pub mny_mkt_instrm: InternalisationData1,
    #[validate]
    #[serde(rename = "EmssnAllwnc")]
    pub emssn_allwnc: InternalisationData1,
    #[validate]
    #[serde(rename = "OthrFinInstrms")]
    pub othr_fin_instrms: InternalisationData1,
}
#[derive(
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
pub struct SettlementInternaliserIdentification1 {
    #[validate]
    #[serde(rename = "LEI")]
    pub lei: LeiIdentifier,
    #[validate]
    #[serde(rename = "RspnsblPrsn")]
    pub rspnsbl_prsn: ContactDetails4,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
    #[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
    pub brnch_id: Option<Exact2UpperCaseAlphaText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementInternaliser1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: SettlementInternaliserIdentification1,
    #[validate]
    #[serde(rename = "OvrllTtl")]
    pub ovrll_ttl: InternalisationData1,
    #[validate]
    #[serde(rename = "FinInstrm")]
    pub fin_instrm: SettlementInternaliserFinancialInstrument1,
    #[validate]
    #[serde(rename = "TxTp")]
    pub tx_tp: SettlementInternaliserTransactionType1,
    #[validate]
    #[serde(rename = "ClntTp")]
    pub clnt_tp: SettlementInternaliserClientType1,
    #[validate]
    #[serde(rename = "TtlCshTrf")]
    pub ttl_csh_trf: InternalisationData1,
}
#[derive(
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
pub struct InternalisationDataRate1 {
    #[validate]
    #[serde(rename = "VolPctg")]
    pub vol_pctg: PercentageRate,
    #[validate]
    #[serde(rename = "Val")]
    pub val: PercentageRate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IssuerCsdIdentification1 {
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[validate]
    #[serde(rename = "FrstTwoCharsInstrmId")]
    pub frst_two_chars_instrm_id: Exact2UpperCaseAlphaText,
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
pub struct IssuerCsdReport1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: IssuerCsdIdentification1,
    #[validate]
    #[serde(rename = "OvrllTtl")]
    pub ovrll_ttl: InternalisationData1,
    #[validate]
    #[serde(rename = "FinInstrm")]
    pub fin_instrm: SettlementInternaliserFinancialInstrument1,
    #[validate]
    #[serde(rename = "TxTp")]
    pub tx_tp: SettlementInternaliserTransactionType1,
    #[validate]
    #[serde(rename = "ClntTp")]
    pub clnt_tp: SettlementInternaliserClientType1,
    #[validate]
    #[serde(rename = "TtlCshTrf")]
    pub ttl_csh_trf: InternalisationData1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementInternaliserClientType1 {
    #[validate]
    #[serde(rename = "Prfssnl")]
    pub prfssnl: InternalisationData1,
    #[validate]
    #[serde(rename = "Rtl")]
    pub rtl: InternalisationData1,
}
#[derive(
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
pub struct Exact2UpperCaseAlphaText {
    #[validate(regex = "EXACT_2_UPPER_CASE_ALPHA_TEXT_REGEX")]
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
pub struct Max140Text {
    #[validate(length(min = 1, max = 140,))]
    #[serde(rename = "$text")]
    pub value: String,
}
