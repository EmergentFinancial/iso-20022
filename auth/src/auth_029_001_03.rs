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
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:auth.029.001.03".to_string()
}

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ModificationLevel1Code {
    #[serde(rename = "PSTN")]
    Pstn,
    #[serde(rename = "TCTN")]
    Tctn,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FinancialInstrumentContractType2Code {
    #[serde(rename = "CFDS")]
    Cfds,
    #[serde(rename = "FRAS")]
    Fras,
    #[serde(rename = "FUTR")]
    Futr,
    #[serde(rename = "FORW")]
    Forw,
    #[serde(rename = "OPTN")]
    Optn,
    #[serde(rename = "SPDB")]
    Spdb,
    #[serde(rename = "SWAP")]
    Swap,
    #[serde(rename = "SWPT")]
    Swpt,
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
pub struct ProductClassificationCriteria1 {
    #[validate(length(min = 0,))]
    #[serde(rename = "ClssfctnFinInstrm", default)]
    pub clssfctn_fin_instrm: Vec<CfiOct2015Identifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "UnqPdctIdr", default)]
    pub unq_pdct_idr: Vec<Max52Text>,
}
#[derive(
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
pub struct PartyIdentification121ChoiceEnum {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
    #[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
    pub lgl_ntty_idr: Option<LeiIdentifier>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification121Choice {
    #[serde(flatten)]
    pub value: PartyIdentification121ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NotAvailable1Code {
    #[serde(rename = "NTAV")]
    Ntav,
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
pub struct DayOfMonthNumber {
    #[validate(range(min = 1, max = 31,))]
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
pub struct Max50Text {
    #[validate(length(min = 1, max = 50,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransactionOperationType8Code {
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
pub struct DerivativesTradeReportQueryV03<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "RqstngAuthrty")]
    pub rqstng_authrty: PartyIdentification121Choice,
    #[serde(rename = "TradQryData")]
    pub trad_qry_data: TradeReportQuery16Choice,
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
pub struct TrueFalseIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Frequency14Code {
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "ADHO")]
    Adho,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProductType4Code {
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "CURR")]
    Curr,
    #[serde(rename = "EQUI")]
    Equi,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "COMM")]
    Comm,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FinancialPartySectorType2Code {
    #[serde(rename = "AIFD")]
    Aifd,
    #[serde(rename = "CSDS")]
    Csds,
    #[serde(rename = "CCPS")]
    Ccps,
    #[serde(rename = "CDTI")]
    Cdti,
    #[serde(rename = "INUN")]
    Inun,
    #[serde(rename = "ORPI")]
    Orpi,
    #[serde(rename = "INVF")]
    Invf,
    #[serde(rename = "REIN")]
    Rein,
    #[serde(rename = "UCIT")]
    Ucit,
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
pub struct DateOrBlankQuery2ChoiceEnum {
    #[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
    pub not_rptd: Option<NotReported1Code>,
    #[serde(rename = "Rg", skip_serializing_if = "Option::is_none")]
    pub rg: Option<DatePeriod1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateOrBlankQuery2Choice {
    #[serde(flatten)]
    pub value: DateOrBlankQuery2ChoiceEnum,
}
#[derive(
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
pub struct TradeDateTimeQueryCriteria5 {
    #[serde(rename = "RptgDtTm", skip_serializing_if = "Option::is_none")]
    pub rptg_dt_tm: Option<DateTimePeriod1>,
    #[serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none")]
    pub exctn_dt_tm: Option<DateTimePeriod1>,
    #[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
    pub mtrty_dt: Option<DateOrBlankQuery2Choice>,
    #[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
    pub fctv_dt: Option<DatePeriod1>,
    #[serde(rename = "ValtnDtTm", skip_serializing_if = "Option::is_none")]
    pub valtn_dt_tm: Option<DateTimePeriod1>,
    #[serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none")]
    pub xprtn_dt: Option<DateOrBlankQuery2Choice>,
    #[serde(rename = "EarlyTermntnDt", skip_serializing_if = "Option::is_none")]
    pub early_termntn_dt: Option<DatePeriod1>,
    #[serde(rename = "CollTmStmp", skip_serializing_if = "Option::is_none")]
    pub coll_tm_stmp: Option<DateTimeOrBlankQuery1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max1000Text {
    #[validate(length(min = 1, max = 1000,))]
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
pub struct TradeAdditionalQueryCriteria9 {
    #[validate(length(min = 0,))]
    #[serde(rename = "ActnTp", default)]
    pub actn_tp: Vec<TransactionOperationType8Code>,
    #[serde(rename = "ExctnVn", skip_serializing_if = "Option::is_none")]
    pub exctn_vn: Option<SecuritiesTradeVenueCriteria1Choice>,
    #[serde(rename = "NtrOfCtrPty", skip_serializing_if = "Option::is_none")]
    pub ntr_of_ctr_pty: Option<PartyNatureType1Code>,
    #[serde(rename = "CorpSctr", skip_serializing_if = "Option::is_none")]
    pub corp_sctr: Option<CorporateSectorCriteria6>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AsstClss", default)]
    pub asst_clss: Vec<ProductType4Code>,
    #[serde(rename = "PdctClssfctn", skip_serializing_if = "Option::is_none")]
    pub pdct_clssfctn: Option<ProductClassificationCriteria1>,
    #[serde(rename = "Lvl", skip_serializing_if = "Option::is_none")]
    pub lvl: Option<ModificationLevel1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "EvtTp", default)]
    pub evt_tp: Vec<DerivativeEventType3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DatePeriod1 {
    #[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
    pub fr_dt: Option<IsoDate>,
    #[validate]
    #[serde(rename = "ToDt")]
    pub to_dt: IsoDate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AnyMic1Code {
    #[serde(rename = "ANYM")]
    Anym,
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
pub struct SecuritiesTradeVenueCriteria1ChoiceEnum {
    #[serde(rename = "AnyMIC", skip_serializing_if = "Option::is_none")]
    pub any_mic: Option<AnyMic1Code>,
    #[serde(rename = "MIC", skip_serializing_if = "Option::is_none")]
    pub mic: Option<MicIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesTradeVenueCriteria1Choice {
    #[serde(flatten)]
    pub value: SecuritiesTradeVenueCriteria1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyNatureType1Code {
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "NFIN")]
    Nfin,
    #[serde(rename = "FIIN")]
    Fiin,
    #[serde(rename = "CCPS")]
    Ccps,
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
pub struct TradePartyIdentificationQuery8 {
    #[validate(length(min = 0,))]
    #[serde(rename = "LEI", default)]
    pub lei: Vec<LeiIdentifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AnyBIC", default)]
    pub any_bic: Vec<AnyBicDec2014Identifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ClntId", default)]
    pub clnt_id: Vec<Max50Text>,
    #[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
    pub not_rptd: Option<NotReported1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradePartyQueryCriteria6 {
    #[serde(rename = "Oprtr")]
    pub oprtr: Operation3Code,
    #[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
    pub rptg_ctr_pty: Option<TradePartyIdentificationQuery8>,
    #[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
    pub othr_ctr_pty: Option<TradePartyIdentificationQuery8>,
    #[serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none")]
    pub bnfcry: Option<TradePartyIdentificationQuery8>,
    #[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
    pub ntty_rspnsbl_for_rpt: Option<TradePartyIdentificationQuery8>,
    #[serde(rename = "SubmitgAgt", skip_serializing_if = "Option::is_none")]
    pub submitg_agt: Option<TradePartyIdentificationQuery8>,
    #[serde(rename = "Brkr", skip_serializing_if = "Option::is_none")]
    pub brkr: Option<TradePartyIdentificationQuery8>,
    #[serde(rename = "CCP", skip_serializing_if = "Option::is_none")]
    pub ccp: Option<TradePartyIdentificationQuery8>,
    #[serde(rename = "ClrMmb", skip_serializing_if = "Option::is_none")]
    pub clr_mmb: Option<TradePartyIdentificationQuery8>,
}
#[derive(
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
pub struct Max25Text {
    #[validate(length(min = 1, max = 25,))]
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
pub struct DateTimeOrBlankQuery1ChoiceEnum {
    #[serde(rename = "Rg", skip_serializing_if = "Option::is_none")]
    pub rg: Option<DateTimePeriod1>,
    #[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
    pub not_rptd: Option<NotReported1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateTimeOrBlankQuery1Choice {
    #[serde(flatten)]
    pub value: DateTimeOrBlankQuery1ChoiceEnum,
}
#[derive(
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
pub struct BasketQuery1 {
    #[serde(rename = "Strr", skip_serializing_if = "Option::is_none")]
    pub strr: Option<LeiIdentifier>,
    #[serde(rename = "Idr", skip_serializing_if = "Option::is_none")]
    pub idr: Option<Max52Text>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateSectorCriteria6 {
    #[validate(length(min = 0,))]
    #[serde(rename = "FISctr", default)]
    pub fi_sctr: Vec<FinancialPartySectorType2Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NFISctr", default)]
    pub nfi_sctr: Vec<NonFinancialPartySector1Code>,
    #[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
    pub not_rptd: Option<NotReported1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeReportQuery16ChoiceEnum {
    #[serde(rename = "RcrntQry", skip_serializing_if = "Option::is_none")]
    pub rcrnt_qry: Option<TradeRecurrentQuery5>,
    #[serde(rename = "AdHocQry", skip_serializing_if = "Option::is_none")]
    pub ad_hoc_qry: Option<TradeQueryCriteria13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeReportQuery16Choice {
    #[serde(flatten)]
    pub value: TradeReportQuery16ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NotReported1Code {
    #[serde(rename = "NORP")]
    Norp,
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
pub struct TradeQueryExecutionFrequency3 {
    #[serde(rename = "FrqcyTp")]
    pub frqcy_tp: Frequency14Code,
    #[validate(length(min = 0,))]
    #[serde(rename = "DlvryDay", default)]
    pub dlvry_day: Vec<WeekDay3Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DayOfMnth", default)]
    pub day_of_mnth: Vec<DayOfMonthNumber>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeRecurrentQuery5 {
    #[validate]
    #[serde(rename = "QryTp")]
    pub qry_tp: Max1000Text,
    #[validate]
    #[serde(rename = "Frqcy")]
    pub frqcy: TradeQueryExecutionFrequency3,
    #[validate]
    #[serde(rename = "VldUntil")]
    pub vld_until: IsoDate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Operation3Code {
    #[serde(rename = "ANDD")]
    Andd,
    #[serde(rename = "ORRR")]
    Orrr,
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
pub struct TradeSecurityIdentificationQueryCriteria3 {
    #[serde(rename = "Oprtr")]
    pub oprtr: Operation3Code,
    #[validate(length(min = 0,))]
    #[serde(rename = "Id", default)]
    pub id: Vec<SecurityIdentificationQueryCriteria1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CtrctTp", default)]
    pub ctrct_tp: Vec<FinancialInstrumentContractType2Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ISIN", default)]
    pub isin: Vec<IsinQueryCriteria1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "UnqPdctIdr", default)]
    pub unq_pdct_idr: Vec<UpiQueryCriteria1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "UndrlygInstrmId", default)]
    pub undrlyg_instrm_id: Vec<SecurityIdentificationQuery4Choice>,
}
#[derive(
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
pub struct UpiQueryCriteria1 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Idr", default)]
    pub idr: Vec<Max52Text>,
    #[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
    pub not_rptd: Option<NotReported1Code>,
}
#[derive(
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
pub struct DateTimePeriod1 {
    #[validate]
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm: IsoDateTime,
    #[validate]
    #[serde(rename = "ToDtTm")]
    pub to_dt_tm: IsoDateTime,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TradeQueryCriteria13 {
    #[validate]
    #[serde(rename = "TradLifeCyclHstry")]
    pub trad_life_cycl_hstry: TrueFalseIndicator,
    #[validate]
    #[serde(rename = "OutsdngTradInd")]
    pub outsdng_trad_ind: TrueFalseIndicator,
    #[serde(rename = "TradPtyCrit", skip_serializing_if = "Option::is_none")]
    pub trad_pty_crit: Option<TradePartyQueryCriteria6>,
    #[serde(rename = "FinInstrmCrit", skip_serializing_if = "Option::is_none")]
    pub fin_instrm_crit: Option<TradeSecurityIdentificationQueryCriteria3>,
    #[serde(rename = "TmCrit", skip_serializing_if = "Option::is_none")]
    pub tm_crit: Option<TradeDateTimeQueryCriteria5>,
    #[serde(rename = "OthrCrit", skip_serializing_if = "Option::is_none")]
    pub othr_crit: Option<TradeAdditionalQueryCriteria9>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NonFinancialPartySector1Code {
    #[serde(rename = "WTER")]
    Wter,
    #[serde(rename = "MING")]
    Ming,
    #[serde(rename = "MAFG")]
    Mafg,
    #[serde(rename = "SPLY")]
    Sply,
    #[serde(rename = "CSTR")]
    Cstr,
    #[serde(rename = "AGRI")]
    Agri,
    #[serde(rename = "ACAF")]
    Acaf,
    #[serde(rename = "EDUC")]
    Educ,
    #[serde(rename = "AEAR")]
    Aear,
    #[serde(rename = "FINA")]
    Fina,
    #[serde(rename = "HHSW")]
    Hhsw,
    #[serde(rename = "INCO")]
    Inco,
    #[serde(rename = "WRRM")]
    Wrrm,
    #[serde(rename = "OTSA")]
    Otsa,
    #[serde(rename = "PSTA")]
    Psta,
    #[serde(rename = "PADS")]
    Pads,
    #[serde(rename = "RESA")]
    Resa,
    #[serde(rename = "TRAS")]
    Tras,
    #[serde(rename = "ASSA")]
    Assa,
    #[serde(rename = "AHAE")]
    Ahae,
    #[serde(rename = "AEOB")]
    Aeob,
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
pub struct SecurityIdentificationQuery4ChoiceEnum {
    #[serde(rename = "Bskt", skip_serializing_if = "Option::is_none")]
    pub bskt: Option<BasketQuery1>,
    #[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
    pub indx: Option<SecurityIdentification20Choice>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
    pub not_rptd: Option<NotReported1Code>,
    #[serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none")]
    pub altrntv_instrm_id: Option<Max52Text>,
    #[serde(rename = "NotAvlbl", skip_serializing_if = "Option::is_none")]
    pub not_avlbl: Option<NotAvailable1Code>,
    #[serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none")]
    pub unq_pdct_idr: Option<Max52Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentificationQuery4Choice {
    #[serde(flatten)]
    pub value: SecurityIdentificationQuery4ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "DerivsTradRptQry")]
    pub derivs_trad_rpt_qry: DerivativesTradeReportQueryV03<A>,
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
pub struct IsinQueryCriteria1 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Idr", default)]
    pub idr: Vec<IsinOct2015Identifier>,
    #[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
    pub not_rptd: Option<NotReported1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CfiOct2015Identifier {
    #[validate(regex = "CFI_OCT_2015_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum WeekDay3Code {
    #[serde(rename = "ALLD")]
    Alld,
    #[serde(rename = "XBHL")]
    Xbhl,
    #[serde(rename = "IBHL")]
    Ibhl,
    #[serde(rename = "FRID")]
    Frid,
    #[serde(rename = "MOND")]
    Mond,
    #[serde(rename = "SATD")]
    Satd,
    #[serde(rename = "SUND")]
    Sund,
    #[serde(rename = "THUD")]
    Thud,
    #[serde(rename = "TUED")]
    Tued,
    #[serde(rename = "WEDD")]
    Wedd,
    #[serde(rename = "WDAY")]
    Wday,
    #[serde(rename = "WEND")]
    Wend,
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
pub struct SecurityIdentification20ChoiceEnum {
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max25Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification20Choice {
    #[serde(flatten)]
    pub value: SecurityIdentification20ChoiceEnum,
}
#[derive(
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
pub struct SecurityIdentificationQueryCriteria1 {
    #[validate(length(min = 0,))]
    #[serde(rename = "ISIN", default)]
    pub isin: Vec<IsinOct2015Identifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AltrntvInstrmId", default)]
    pub altrntv_instrm_id: Vec<Max52Text>,
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
