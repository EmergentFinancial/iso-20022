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
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{12,12}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref UPIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{8,17}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_3_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BBAN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:seev.012.001.01".to_string()
}

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BeneficiaryCertificationType1Code {
    #[serde(rename = "ACCI")]
    Acci,
    #[serde(rename = "DOMI")]
    Domi,
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "QIBB")]
    Qibb,
    #[serde(rename = "TRBD")]
    Trbd,
    #[serde(rename = "NCOM")]
    Ncom,
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
pub struct CorporateActionOption1FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionOptionType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionOption1FormatChoice {
    #[serde(flatten)]
    pub value: CorporateActionOption1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccountIdentification1ChoiceEnum {
    #[serde(rename = "DmstAcct", skip_serializing_if = "Option::is_none")]
    pub dmst_acct: Option<SimpleIdentificationInformation>,
    #[serde(rename = "BBAN", skip_serializing_if = "Option::is_none")]
    pub bban: Option<BbanIdentifier>,
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<IbanIdentifier>,
    #[serde(rename = "UPIC", skip_serializing_if = "Option::is_none")]
    pub upic: Option<UpicIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccountIdentification1Choice {
    #[serde(flatten)]
    pub value: CashAccountIdentification1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification13 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max4AlphaNumericText,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max35Text,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CreditDebitCode {
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "DBIT")]
    Dbit,
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
pub struct CorporateActionEventProcessingType1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventProcessingType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventProcessingType1FormatChoice {
    #[serde(flatten)]
    pub value: CorporateActionEventProcessingType1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AgentCaElectionAdviceV01 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: DocumentIdentification8,
    #[serde(rename = "ElctnAdvcTpAndLkg")]
    pub elctn_advc_tp_and_lkg: ElectionAdviceFunction1,
    #[validate]
    #[serde(rename = "CorpActnGnlInf")]
    pub corp_actn_gnl_inf: CorporateActionInformation1,
    #[validate]
    #[serde(rename = "ElctnDtls")]
    pub elctn_dtls: CorporateActionElection3,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<CorporateActionAdditionalInformation1>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<ContactPerson1>,
}
#[derive(
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
pub struct CorporateActionInformation1 {
    #[serde(rename = "AgtId")]
    pub agt_id: PartyIdentification2Choice,
    #[serde(rename = "IssrCorpActnId", skip_serializing_if = "Option::is_none")]
    pub issr_corp_actn_id: Option<Max35Text>,
    #[serde(rename = "CorpActnPrcgId", skip_serializing_if = "Option::is_none")]
    pub corp_actn_prcg_id: Option<Max35Text>,
    #[serde(rename = "EvtTp")]
    pub evt_tp: CorporateActionEventType2FormatChoice,
    #[serde(rename = "MndtryVlntryEvtTp")]
    pub mndtry_vlntry_evt_tp: CorporateActionMandatoryVoluntary1FormatChoice,
    #[serde(rename = "EvtPrcgTp", skip_serializing_if = "Option::is_none")]
    pub evt_prcg_tp: Option<CorporateActionEventProcessingType1FormatChoice>,
    #[validate]
    #[serde(rename = "UndrlygScty")]
    pub undrlyg_scty: FinancialInstrumentDescription3,
}
#[derive(
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
pub struct SecuritiesAccount7 {
    #[serde(rename = "AcctOwnrId", skip_serializing_if = "Option::is_none")]
    pub acct_ownr_id: Option<PartyIdentification2Choice>,
    #[validate]
    #[serde(rename = "AcctId")]
    pub acct_id: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitOrFaceAmount1ChoiceEnum {
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitOrFaceAmount1Choice {
    #[serde(flatten)]
    pub value: UnitOrFaceAmount1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesAccount9 {
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "AcctOwnrId", skip_serializing_if = "Option::is_none")]
    pub acct_ownr_id: Option<PartyIdentification2Choice>,
    #[validate]
    #[serde(rename = "AcctId")]
    pub acct_id: Max35Text,
    #[serde(rename = "BalTp", skip_serializing_if = "Option::is_none")]
    pub bal_tp: Option<SecuritiesBalanceType10FormatChoice>,
    #[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
    pub optn_tp: Option<CorporateActionOption1FormatChoice>,
    #[serde(rename = "OptnNb", skip_serializing_if = "Option::is_none")]
    pub optn_nb: Option<Exact3NumericText>,
    #[serde(rename = "SctyHldgForm", skip_serializing_if = "Option::is_none")]
    pub scty_hldg_form: Option<FormOfSecurity1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AnyBicIdentifier {
    #[validate(regex = "ANY_BIC_IDENTIFIER_REGEX")]
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
    #[serde(rename = "AgtCAElctnAdvc")]
    pub agt_ca_elctn_advc: AgentCaElectionAdviceV01,
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
pub struct IsinIdentifier {
    #[validate(regex = "ISIN_IDENTIFIER_REGEX")]
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
pub enum CorporateActionEventType2Code {
    #[serde(rename = "ACTV")]
    Actv,
    #[serde(rename = "ATTI")]
    Atti,
    #[serde(rename = "BIDS")]
    Bids,
    #[serde(rename = "BONU")]
    Bonu,
    #[serde(rename = "BPUT")]
    Bput,
    #[serde(rename = "BRUP")]
    Brup,
    #[serde(rename = "CAPG")]
    Capg,
    #[serde(rename = "CAPI")]
    Capi,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "CHAN")]
    Chan,
    #[serde(rename = "CLSA")]
    Clsa,
    #[serde(rename = "CONS")]
    Cons,
    #[serde(rename = "CONV")]
    Conv,
    #[serde(rename = "COOP")]
    Coop,
    #[serde(rename = "DECR")]
    Decr,
    #[serde(rename = "DETI")]
    Deti,
    #[serde(rename = "DFLT")]
    Dflt,
    #[serde(rename = "DLST")]
    Dlst,
    #[serde(rename = "DRAW")]
    Draw,
    #[serde(rename = "DRIP")]
    Drip,
    #[serde(rename = "DSCL")]
    Dscl,
    #[serde(rename = "DTCH")]
    Dtch,
    #[serde(rename = "DVCA")]
    Dvca,
    #[serde(rename = "DVOP")]
    Dvop,
    #[serde(rename = "DVSC")]
    Dvsc,
    #[serde(rename = "DVSE")]
    Dvse,
    #[serde(rename = "EXOF")]
    Exof,
    #[serde(rename = "EXRI")]
    Exri,
    #[serde(rename = "EXTM")]
    Extm,
    #[serde(rename = "EXWA")]
    Exwa,
    #[serde(rename = "INCR")]
    Incr,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "LIQU")]
    Liqu,
    #[serde(rename = "MCAL")]
    Mcal,
    #[serde(rename = "MRGR")]
    Mrgr,
    #[serde(rename = "ODLT")]
    Odlt,
    #[serde(rename = "PARI")]
    Pari,
    #[serde(rename = "PCAL")]
    Pcal,
    #[serde(rename = "PDEF")]
    Pdef,
    #[serde(rename = "PINK")]
    Pink,
    #[serde(rename = "PLAC")]
    Plac,
    #[serde(rename = "PPMT")]
    Ppmt,
    #[serde(rename = "PRED")]
    Pred,
    #[serde(rename = "PRII")]
    Prii,
    #[serde(rename = "PRIO")]
    Prio,
    #[serde(rename = "REDM")]
    Redm,
    #[serde(rename = "REDO")]
    Redo,
    #[serde(rename = "REMK")]
    Remk,
    #[serde(rename = "RHDI")]
    Rhdi,
    #[serde(rename = "RHTS")]
    Rhts,
    #[serde(rename = "SHPR")]
    Shpr,
    #[serde(rename = "SMAL")]
    Smal,
    #[serde(rename = "SOFF")]
    Soff,
    #[serde(rename = "SPLF")]
    Splf,
    #[serde(rename = "SPLR")]
    Splr,
    #[serde(rename = "SUSP")]
    Susp,
    #[serde(rename = "TEND")]
    Tend,
    #[serde(rename = "TREC")]
    Trec,
    #[serde(rename = "WRTH")]
    Wrth,
    #[serde(rename = "WTRC")]
    Wtrc,
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
pub struct AlternateSecurityIdentification3Enum {
    #[serde(rename = "PrtryIdSrc", skip_serializing_if = "Option::is_none")]
    pub prtry_id_src: Option<Max35Text>,
    #[serde(rename = "DmstIdSrc", skip_serializing_if = "Option::is_none")]
    pub dmst_id_src: Option<CountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlternateSecurityIdentification3 {
    #[serde(flatten)]
    pub value: AlternateSecurityIdentification3Enum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IbanIdentifier {
    #[validate(regex = "IBAN_IDENTIFIER_REGEX")]
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
pub struct CorporateActionMandatoryVoluntary1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionMandatoryVoluntary1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionMandatoryVoluntary1FormatChoice {
    #[serde(flatten)]
    pub value: CorporateActionMandatoryVoluntary1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndDateTimeChoiceEnum {
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
pub struct DateAndDateTimeChoice {
    #[serde(flatten)]
    pub value: DateAndDateTimeChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BeneficiaryCertificationType1FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BeneficiaryCertificationType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BeneficiaryCertificationType1FormatChoice {
    #[serde(flatten)]
    pub value: BeneficiaryCertificationType1FormatChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DecimalNumber {
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
pub struct IsoDate {
    #[serde(rename = "$text")]
    pub value: ::chrono::NaiveDate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionEventProcessingType1Code {
    #[serde(rename = "GENL")]
    Genl,
    #[serde(rename = "DISN")]
    Disn,
    #[serde(rename = "REOR")]
    Reor,
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
pub struct UpicIdentifier {
    #[validate(regex = "UPIC_IDENTIFIER_REGEX")]
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
pub struct CorporateActionAdditionalInformation1 {
    #[validate(length(min = 0,))]
    #[serde(rename = "BnfclOwnrDtls", default)]
    pub bnfcl_ownr_dtls: Vec<BeneficialOwner1>,
    #[serde(rename = "RegnDtls", skip_serializing_if = "Option::is_none")]
    pub regn_dtls: Option<Max350Text>,
    #[serde(rename = "RcvrId", skip_serializing_if = "Option::is_none")]
    pub rcvr_id: Option<PartyIdentification2Choice>,
    #[serde(rename = "CertfctnInd", skip_serializing_if = "Option::is_none")]
    pub certfctn_ind: Option<YesNoIndicator>,
    #[serde(rename = "CertfctnTp", skip_serializing_if = "Option::is_none")]
    pub certfctn_tp: Option<BeneficiaryCertificationType1FormatChoice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DlvryDtls", default)]
    pub dlvry_dtls: Vec<ProceedsDelivery1>,
    #[serde(rename = "AddtlInstr", skip_serializing_if = "Option::is_none")]
    pub addtl_instr: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification7Enum {
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinIdentifier>,
    #[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
    pub othr_id: Option<AlternateSecurityIdentification3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification7 {
    #[serde(flatten)]
    pub value: SecurityIdentification7Enum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ElectionAdviceFunction1 {
    #[serde(rename = "ElctnTp")]
    pub elctn_tp: ElectionType1Code,
    #[serde(
        rename = "PrvsAgtCAElctnAdvcId",
        skip_serializing_if = "Option::is_none"
    )]
    pub prvs_agt_ca_elctn_advc_id: Option<DocumentIdentification8>,
    #[serde(
        rename = "AgtCAElctnStsAdvcId",
        skip_serializing_if = "Option::is_none"
    )]
    pub agt_ca_elctn_sts_advc_id: Option<DocumentIdentification8>,
    #[serde(
        rename = "AgtCAElctnAmdmntReqId",
        skip_serializing_if = "Option::is_none"
    )]
    pub agt_ca_elctn_amdmnt_req_id: Option<DocumentIdentification8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Exact3NumericText {
    #[validate(regex = "EXACT_3_NUMERIC_TEXT_REGEX")]
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
pub struct GenericIdentification16 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "IdTp")]
    pub id_tp: PersonIdentificationType3Choice,
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
pub struct ContactPerson1 {
    #[validate]
    #[serde(rename = "CtctPrsn")]
    pub ctct_prsn: ContactIdentification4,
    #[serde(rename = "InstnId", skip_serializing_if = "Option::is_none")]
    pub instn_id: Option<PartyIdentification2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionElection3 {
    #[serde(rename = "AcctDtls", skip_serializing_if = "Option::is_none")]
    pub acct_dtls: Option<SecuritiesAccount7>,
    #[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
    pub optn_tp: Option<CorporateActionOption1FormatChoice>,
    #[serde(rename = "OptnNb", skip_serializing_if = "Option::is_none")]
    pub optn_nb: Option<Exact3NumericText>,
    #[serde(
        rename = "InstdUndrlygSctiesQty",
        skip_serializing_if = "Option::is_none"
    )]
    pub instd_undrlyg_scties_qty: Option<UnitOrFaceAmount1Choice>,
    #[serde(
        rename = "InstdSctiesQtyToRcv",
        skip_serializing_if = "Option::is_none"
    )]
    pub instd_scties_qty_to_rcv: Option<UnitOrFaceAmount1Choice>,
    #[serde(rename = "PropsdRate", skip_serializing_if = "Option::is_none")]
    pub propsd_rate: Option<PercentageRate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshMvmntDtls", default)]
    pub csh_mvmnt_dtls: Vec<CorporateActionCashMovements2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SctiesMvmntDtls", default)]
    pub scties_mvmnt_dtls: Vec<CorporateActionSecuritiesMovement2>,
}
#[derive(
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
pub struct FinancialInstrumentDescription3 {
    #[serde(rename = "SctyId")]
    pub scty_id: SecurityIdentification7,
    #[serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none")]
    pub plc_of_listg: Option<MicIdentifier>,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<PartyIdentification2Choice>,
}
#[derive(
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
pub struct CashAccount19 {
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "AcctOwnrId", skip_serializing_if = "Option::is_none")]
    pub acct_ownr_id: Option<PartyIdentification2Choice>,
    #[serde(rename = "AcctId")]
    pub acct_id: AccountIdentification2Choice,
}
#[derive(
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
pub struct DocumentIdentification8 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
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
pub struct BbanIdentifier {
    #[validate(regex = "BBAN_IDENTIFIER_REGEX")]
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
pub struct ContactIdentification4 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix1Code>,
    #[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
    pub gvn_nm: Option<Max350Text>,
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Max35Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<PhoneNumber>,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max256Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FormOfSecurity1Code {
    #[serde(rename = "BEAR")]
    Bear,
    #[serde(rename = "REGD")]
    Regd,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NamePrefix1Code {
    #[serde(rename = "DOCT")]
    Doct,
    #[serde(rename = "MIST")]
    Mist,
    #[serde(rename = "MISS")]
    Miss,
    #[serde(rename = "MADM")]
    Madm,
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
pub struct PhoneNumber {
    #[validate(regex = "PHONE_NUMBER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecuritiesBalanceType10Code {
    #[serde(rename = "AVLB")]
    Avlb,
    #[serde(rename = "REST")]
    Rest,
    #[serde(rename = "RDIS")]
    Rdis,
    #[serde(rename = "RREM")]
    Rrem,
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
pub struct CorporateActionCashMovements2 {
    #[serde(rename = "PstngId", skip_serializing_if = "Option::is_none")]
    pub pstng_id: Option<Max35Text>,
    #[serde(rename = "PstngDtTm", skip_serializing_if = "Option::is_none")]
    pub pstng_dt_tm: Option<DateAndDateTimeChoice>,
    #[validate]
    #[serde(rename = "PstngAmt")]
    pub pstng_amt: ActiveCurrencyAndAmount,
    #[validate(length(min = 1, max = 2,))]
    #[serde(rename = "AcctDtls", default)]
    pub acct_dtls: Vec<CashAccount19>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification2ChoiceEnum {
    #[serde(rename = "SctiesAcctId", skip_serializing_if = "Option::is_none")]
    pub scties_acct_id: Option<Max35Text>,
    #[serde(rename = "CshAcctId", skip_serializing_if = "Option::is_none")]
    pub csh_acct_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification2Choice {
    #[serde(flatten)]
    pub value: AccountIdentification2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionMandatoryVoluntary1Code {
    #[serde(rename = "MAND")]
    Mand,
    #[serde(rename = "CHOS")]
    Chos,
    #[serde(rename = "VOLU")]
    Volu,
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
pub struct CorporateActionSecuritiesMovement2 {
    #[serde(rename = "SctyId")]
    pub scty_id: SecurityIdentification7,
    #[serde(rename = "PstngDtTm", skip_serializing_if = "Option::is_none")]
    pub pstng_dt_tm: Option<DateAndDateTimeChoice>,
    #[serde(rename = "PstngId", skip_serializing_if = "Option::is_none")]
    pub pstng_id: Option<Max35Text>,
    #[serde(rename = "PstngQty")]
    pub pstng_qty: UnitOrFaceAmount1Choice,
    #[validate(length(min = 1, max = 2,))]
    #[serde(rename = "AcctDtls", default)]
    pub acct_dtls: Vec<SecuritiesAccount9>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesBalanceType10FormatChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SecuritiesBalanceType10Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecuritiesBalanceType10FormatChoice {
    #[serde(flatten)]
    pub value: SecuritiesBalanceType10FormatChoiceEnum,
}
#[derive(
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
pub struct ProceedsDelivery1Enum {
    #[serde(rename = "CshAcctId", skip_serializing_if = "Option::is_none")]
    pub csh_acct_id: Option<CashAccountIdentification1Choice>,
    #[serde(rename = "SctiesAcctId", skip_serializing_if = "Option::is_none")]
    pub scties_acct_id: Option<Max35Text>,
    #[serde(rename = "AcctOwnrId", skip_serializing_if = "Option::is_none")]
    pub acct_ownr_id: Option<PartyIdentification2Choice>,
    #[serde(rename = "AcctSvcrId", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_id: Option<PartyIdentification2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProceedsDelivery1 {
    #[serde(flatten)]
    pub value: ProceedsDelivery1Enum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PersonIdentificationType3Code {
    #[serde(rename = "ARNU")]
    Arnu,
    #[serde(rename = "CCPT")]
    Ccpt,
    #[serde(rename = "EMID")]
    Emid,
    #[serde(rename = "DRLC")]
    Drlc,
    #[serde(rename = "FINN")]
    Finn,
    #[serde(rename = "TXID")]
    Txid,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CorporateActionOptionType1Code {
    #[serde(rename = "BSPL")]
    Bspl,
    #[serde(rename = "BUYA")]
    Buya,
    #[serde(rename = "CASE")]
    Case,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CEXC")]
    Cexc,
    #[serde(rename = "CTEN")]
    Cten,
    #[serde(rename = "CONN")]
    Conn,
    #[serde(rename = "CONY")]
    Cony,
    #[serde(rename = "EXER")]
    Exer,
    #[serde(rename = "LAPS")]
    Laps,
    #[serde(rename = "MPUT")]
    Mput,
    #[serde(rename = "NOAC")]
    Noac,
    #[serde(rename = "OFFR")]
    Offr,
    #[serde(rename = "OVER")]
    Over,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "SLLE")]
    Slle,
    #[serde(rename = "SPLI")]
    Spli,
    #[serde(rename = "NOQU")]
    Noqu,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "QINV")]
    Qinv,
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
pub struct PersonIdentificationType3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PersonIdentificationType3Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PersonIdentificationType3Choice {
    #[serde(flatten)]
    pub value: PersonIdentificationType3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventType2FormatChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CorporateActionEventType2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateActionEventType2FormatChoice {
    #[serde(flatten)]
    pub value: CorporateActionEventType2FormatChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ElectionType1Code {
    #[serde(rename = "NEWM")]
    Newm,
    #[serde(rename = "CHAN")]
    Chan,
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
pub struct PartyIdentification2ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
    #[serde(rename = "BICOrBEI", skip_serializing_if = "Option::is_none")]
    pub bic_or_bei: Option<AnyBicIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification2Choice {
    #[serde(flatten)]
    pub value: PartyIdentification2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BeneficialOwner1 {
    #[serde(rename = "BnfclOwnrId")]
    pub bnfcl_ownr_id: PartyIdentification2Choice,
    #[serde(rename = "AddtlId", skip_serializing_if = "Option::is_none")]
    pub addtl_id: Option<GenericIdentification16>,
    #[serde(rename = "Ntlty", skip_serializing_if = "Option::is_none")]
    pub ntlty: Option<CountryCode>,
    #[serde(rename = "DmclCtry", skip_serializing_if = "Option::is_none")]
    pub dmcl_ctry: Option<CountryCode>,
    #[serde(rename = "NonDmclCtry", skip_serializing_if = "Option::is_none")]
    pub non_dmcl_ctry: Option<CountryCode>,
    #[serde(rename = "CertfctnInd", skip_serializing_if = "Option::is_none")]
    pub certfctn_ind: Option<YesNoIndicator>,
    #[serde(rename = "CertfctnTp", skip_serializing_if = "Option::is_none")]
    pub certfctn_tp: Option<BeneficiaryCertificationType1FormatChoice>,
    #[serde(rename = "DclrtnDtls", skip_serializing_if = "Option::is_none")]
    pub dclrtn_dtls: Option<Max350Text>,
    #[serde(rename = "SctyId", skip_serializing_if = "Option::is_none")]
    pub scty_id: Option<SecurityIdentification7>,
    #[serde(rename = "ElctdSctiesQty")]
    pub elctd_scties_qty: UnitOrFaceAmount1Choice,
}
#[derive(
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
