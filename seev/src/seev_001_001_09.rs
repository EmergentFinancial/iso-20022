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
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_2_A_LANGUAGE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_5_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[0-9]{1,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:seev.001.001.09".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification238 {
    #[validate]
    #[serde(rename = "NmAndAdr")]
    pub nm_and_adr: PersonName3,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max256Text>,
    #[validate]
    #[serde(rename = "Id")]
    pub id: NaturalPersonIdentification1,
    #[serde(rename = "Ntlty", skip_serializing_if = "Option::is_none")]
    pub ntlty: Option<CountryCode>,
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityPosition15 {
    #[validate]
    #[serde(rename = "FinInstrmId")]
    pub fin_instrm_id: SecurityIdentification19,
    #[validate(length(min = 0, max = 1000,))]
    #[serde(rename = "Pos", default)]
    pub pos: Vec<EligiblePosition12>,
}
#[derive(
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
pub enum MeetingType4Code {
    #[serde(rename = "XMET")]
    Xmet,
    #[serde(rename = "GMET")]
    Gmet,
    #[serde(rename = "MIXD")]
    Mixd,
    #[serde(rename = "SPCL")]
    Spcl,
    #[serde(rename = "BMET")]
    Bmet,
    #[serde(rename = "CMET")]
    Cmet,
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
pub struct PriceRateOrAmount3ChoiceEnum {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PriceRateOrAmount3Choice {
    #[serde(flatten)]
    pub value: PriceRateOrAmount3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ThresholdBasis1Code {
    #[serde(rename = "ALSH")]
    Alsh,
    #[serde(rename = "ALSM")]
    Alsm,
    #[serde(rename = "ALVO")]
    Alvo,
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
pub struct AdditionalRights3 {
    #[serde(rename = "AddtlRght")]
    pub addtl_rght: AdditionalRightCode1Choice,
    #[serde(rename = "AddtlRghtInfURLAdr", skip_serializing_if = "Option::is_none")]
    pub addtl_rght_inf_url_adr: Option<Max2048Text>,
    #[serde(rename = "AddtlRghtDdln", skip_serializing_if = "Option::is_none")]
    pub addtl_rght_ddln: Option<DateFormat58Choice>,
    #[serde(rename = "AddtlRghtMktDdln", skip_serializing_if = "Option::is_none")]
    pub addtl_rght_mkt_ddln: Option<DateFormat58Choice>,
    #[serde(rename = "AddtlRghtThrshld", skip_serializing_if = "Option::is_none")]
    pub addtl_rght_thrshld: Option<AdditionalRightThreshold1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IncentivePremium5 {
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max350Text>,
    #[serde(rename = "Amt")]
    pub amt: PriceRateOrAmount3Choice,
    #[serde(rename = "Tp")]
    pub tp: IncentivePremiumType2Choice,
    #[serde(rename = "PmtDt", skip_serializing_if = "Option::is_none")]
    pub pmt_dt: Option<DateFormat3Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LocationFormat1ChoiceEnum {
    #[serde(rename = "LctnCd", skip_serializing_if = "Option::is_none")]
    pub lctn_cd: Option<PlaceType1Code>,
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
pub struct LocationFormat1Choice {
    #[serde(flatten)]
    pub value: LocationFormat1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Iso2ALanguageCode {
    #[validate(regex = "ISO_2_A_LANGUAGE_CODE_REGEX")]
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
pub struct PartyIdentification231ChoiceEnum {
    #[serde(rename = "LglPrsn", skip_serializing_if = "Option::is_none")]
    pub lgl_prsn: Option<PartyIdentification221>,
    #[serde(rename = "NtrlPrsn", skip_serializing_if = "Option::is_none")]
    pub ntrl_prsn: Option<PartyIdentification238>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification231Choice {
    #[serde(flatten)]
    pub value: PartyIdentification231ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MeetingNotificationV09<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Pgntn", skip_serializing_if = "Option::is_none")]
    pub pgntn: Option<Pagination1>,
    #[validate]
    #[serde(rename = "NtfctnGnlInf")]
    pub ntfctn_gnl_inf: NotificationGeneralInformation3,
    #[serde(rename = "NtfctnUpd", skip_serializing_if = "Option::is_none")]
    pub ntfctn_upd: Option<NotificationUpdate2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "EvtsLkg", default)]
    pub evts_lkg: Vec<MeetingEventReference1>,
    #[validate]
    #[serde(rename = "Mtg")]
    pub mtg: MeetingNotice7,
    #[validate(length(min = 1, max = 5,))]
    #[serde(rename = "MtgDtls", default)]
    pub mtg_dtls: Vec<Meeting6>,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: IssuerInformation3,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "IssrAgt", default)]
    pub issr_agt: Vec<IssuerAgent3>,
    #[validate(length(min = 1, max = 200,))]
    #[serde(rename = "Scty", default)]
    pub scty: Vec<SecurityPosition15>,
    #[validate(length(min = 0, max = 1000,))]
    #[serde(rename = "Rsltn", default)]
    pub rsltn: Vec<Resolution6>,
    #[serde(rename = "Vote", skip_serializing_if = "Option::is_none")]
    pub vote: Option<VoteParameters7>,
    #[serde(rename = "PwrOfAttnyRqrmnts", skip_serializing_if = "Option::is_none")]
    pub pwr_of_attny_rqrmnts: Option<PowerOfAttorneyRequirements4>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<CorporateEventNarrative4>,
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
pub struct Proxy5ChoiceEnum {
    #[serde(rename = "PrxyNotAllwd", skip_serializing_if = "Option::is_none")]
    pub prxy_not_allwd: Option<ProxyNotAllowed1Code>,
    #[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
    pub prxy: Option<ProxyAppointmentInformation6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Proxy5Choice {
    #[serde(flatten)]
    pub value: Proxy5ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification198ChoiceEnum {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
    #[serde(rename = "NtlRegnNb", skip_serializing_if = "Option::is_none")]
    pub ntl_regn_nb: Option<Max35Text>,
    #[serde(rename = "ClntId", skip_serializing_if = "Option::is_none")]
    pub clnt_id: Option<Max50Text>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification198Choice {
    #[serde(flatten)]
    pub value: PartyIdentification198ChoiceEnum,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProxyAppointmentInformation6 {
    #[serde(rename = "RegnMtd", skip_serializing_if = "Option::is_none")]
    pub regn_mtd: Option<Max350Text>,
    #[serde(rename = "Ddln", skip_serializing_if = "Option::is_none")]
    pub ddln: Option<DateFormat58Choice>,
    #[serde(rename = "MktDdln", skip_serializing_if = "Option::is_none")]
    pub mkt_ddln: Option<DateFormat58Choice>,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "AuthrsdPrxy", default)]
    pub authrsd_prxy: Vec<Proxy11>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct VoteMethods4 {
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "VoteThrghNtwk", default)]
    pub vote_thrgh_ntwk: Vec<AnyBicDec2014Identifier>,
    #[serde(rename = "VoteByMail", skip_serializing_if = "Option::is_none")]
    pub vote_by_mail: Option<MailAddress1>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "ElctrncVote", default)]
    pub elctrnc_vote: Vec<CommunicationAddress12>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "VoteByTel", default)]
    pub vote_by_tel: Vec<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max5NumericText {
    #[validate(regex = "MAX_5_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NamePrefix2Code {
    #[serde(rename = "DOCT")]
    Doct,
    #[serde(rename = "MADM")]
    Madm,
    #[serde(rename = "MISS")]
    Miss,
    #[serde(rename = "MIST")]
    Mist,
    #[serde(rename = "MIKS")]
    Miks,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProcessingPosition3Code {
    #[serde(rename = "AFTE")]
    Afte,
    #[serde(rename = "WITH")]
    With,
    #[serde(rename = "BEFO")]
    Befo,
    #[serde(rename = "INFO")]
    Info,
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
pub struct GenericIdentification36 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum VoteType1Code {
    #[serde(rename = "ADVI")]
    Advi,
    #[serde(rename = "BNDG")]
    Bndg,
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
pub struct HoldingBalance9 {
    #[serde(rename = "Bal")]
    pub bal: FinancialInstrumentQuantity18Choice,
    #[serde(rename = "BalTp")]
    pub bal_tp: SecuritiesEntryType2Code,
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<SafekeepingPlaceFormat28Choice>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct VoteParameters7 {
    #[serde(
        rename = "SctiesQtyReqrdToVote",
        skip_serializing_if = "Option::is_none"
    )]
    pub scties_qty_reqrd_to_vote: Option<FinancialInstrumentQuantity18Choice>,
    #[validate]
    #[serde(rename = "PrtlVoteAllwd")]
    pub prtl_vote_allwd: YesNoIndicator,
    #[validate]
    #[serde(rename = "SpltVoteAllwd")]
    pub splt_vote_allwd: YesNoIndicator,
    #[serde(rename = "VoteDdln", skip_serializing_if = "Option::is_none")]
    pub vote_ddln: Option<DateFormat58Choice>,
    #[serde(rename = "VoteMktDdln", skip_serializing_if = "Option::is_none")]
    pub vote_mkt_ddln: Option<DateFormat58Choice>,
    #[serde(rename = "VoteMthds", skip_serializing_if = "Option::is_none")]
    pub vote_mthds: Option<VoteMethods4>,
    #[serde(rename = "VtngBlltElctrncAdr", skip_serializing_if = "Option::is_none")]
    pub vtng_bllt_elctrnc_adr: Option<CommunicationAddress11>,
    #[serde(rename = "VtngBlltReqAdr", skip_serializing_if = "Option::is_none")]
    pub vtng_bllt_req_adr: Option<PostalAddress1>,
    #[serde(rename = "RvcbltyDdln", skip_serializing_if = "Option::is_none")]
    pub rvcblty_ddln: Option<DateFormat58Choice>,
    #[serde(rename = "RvcbltyMktDdln", skip_serializing_if = "Option::is_none")]
    pub rvcblty_mkt_ddln: Option<DateFormat58Choice>,
    #[serde(rename = "BnfclOwnrDsclsr", skip_serializing_if = "Option::is_none")]
    pub bnfcl_ownr_dsclsr: Option<YesNoIndicator>,
    #[serde(rename = "EarlyIncntivPrm", skip_serializing_if = "Option::is_none")]
    pub early_incntiv_prm: Option<IncentivePremium5>,
    #[serde(rename = "IncntivPrm", skip_serializing_if = "Option::is_none")]
    pub incntiv_prm: Option<IncentivePremium5>,
    #[serde(
        rename = "EarlyVoteWthPrmDdln",
        skip_serializing_if = "Option::is_none"
    )]
    pub early_vote_wth_prm_ddln: Option<DateFormat58Choice>,
    #[serde(rename = "VoteWthPrmDdln", skip_serializing_if = "Option::is_none")]
    pub vote_wth_prm_ddln: Option<DateFormat58Choice>,
    #[serde(rename = "VoteWthPrmMktDdln", skip_serializing_if = "Option::is_none")]
    pub vote_wth_prm_mkt_ddln: Option<DateFormat58Choice>,
    #[serde(rename = "AddtlVtngRqrmnts", skip_serializing_if = "Option::is_none")]
    pub addtl_vtng_rqrmnts: Option<Max350Text>,
    #[serde(
        rename = "PrvsInstrInvldtyInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub prvs_instr_invldty_ind: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalRightCode1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification13>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AdditionalRight1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalRightCode1Choice {
    #[serde(flatten)]
    pub value: AdditionalRightCode1ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<Max35Text>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
}
#[derive(
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
pub struct VotingRightsThreshold1 {
    #[serde(rename = "Thrshld")]
    pub thrshld: NumberOrPercentage1Choice,
    #[serde(rename = "ThrshldBsis", skip_serializing_if = "Option::is_none")]
    pub thrshld_bsis: Option<ThresholdBasis1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat1 {
    #[serde(rename = "Dt")]
    pub dt: DateFormat3Choice,
    #[serde(rename = "DtMd", skip_serializing_if = "Option::is_none")]
    pub dt_md: Option<DateMode1Code>,
}
#[derive(
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
pub struct NotificationGeneralInformation3 {
    #[serde(rename = "NtfctnTp")]
    pub ntfctn_tp: NotificationType3Code,
    #[validate]
    #[serde(rename = "NtfctnSts")]
    pub ntfctn_sts: EventStatus1,
    #[serde(
        rename = "ShrhldrRghtsDrctvInd",
        skip_serializing_if = "Option::is_none"
    )]
    pub shrhldr_rghts_drctv_ind: Option<YesNoIndicator>,
    #[serde(rename = "ConfOfHldgReqrd", skip_serializing_if = "Option::is_none")]
    pub conf_of_hldg_reqrd: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IssuerInformation3 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification129Choice,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max2048Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NumberOrPercentage1ChoiceEnum {
    #[serde(rename = "ThrshldNb", skip_serializing_if = "Option::is_none")]
    pub thrshld_nb: Option<Number>,
    #[serde(rename = "ThrshldPctg", skip_serializing_if = "Option::is_none")]
    pub thrshld_pctg: Option<PercentageRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NumberOrPercentage1Choice {
    #[serde(flatten)]
    pub value: NumberOrPercentage1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PowerOfAttorneyLegalisation1Code {
    #[serde(rename = "NOTA")]
    Nota,
    #[serde(rename = "LOCA")]
    Loca,
    #[serde(rename = "APOS")]
    Apos,
    #[serde(rename = "COUN")]
    Coun,
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
pub struct DateFormat3ChoiceEnum {
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<DateType1Code>,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat3Choice {
    #[serde(flatten)]
    pub value: DateFormat3ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MeetingDateStatus2Code {
    #[serde(rename = "CNFR")]
    Cnfr,
    #[serde(rename = "TNTA")]
    Tnta,
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
pub struct IdentificationType45ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfIdentification4Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationType45Choice {
    #[serde(flatten)]
    pub value: IdentificationType45ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceTypeAndText6 {
    #[serde(rename = "SfkpgPlcTp")]
    pub sfkpg_plc_tp: SafekeepingPlace2Code,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ParticipationMethod3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<VotingParticipationMethod3Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ParticipationMethod3Choice {
    #[serde(flatten)]
    pub value: ParticipationMethod3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum VotingParticipationMethod3Code {
    #[serde(rename = "MAIL")]
    Mail,
    #[serde(rename = "EVOT")]
    Evot,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "PHNV")]
    Phnv,
    #[serde(rename = "PRXY")]
    Prxy,
    #[serde(rename = "VIRT")]
    Virt,
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
pub struct IssuerAgent3 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification129Choice,
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<AgentRole1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceTypeAndIdentification1 {
    #[serde(rename = "SfkpgPlcTp")]
    pub sfkpg_plc_tp: SafekeepingPlace1Code,
    #[validate]
    #[serde(rename = "Id")]
    pub id: AnyBicDec2014Identifier,
}
#[derive(
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
pub struct MailAddress1 {
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "Crspdc", default)]
    pub crspdc: Vec<PostalAddress1>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "EmailAdr", default)]
    pub email_adr: Vec<Max256Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MeetingTypeClassification2Code {
    #[serde(rename = "AMET")]
    Amet,
    #[serde(rename = "CLAS")]
    Clas,
    #[serde(rename = "ISSU")]
    Issu,
    #[serde(rename = "OMET")]
    Omet,
    #[serde(rename = "VRHI")]
    Vrhi,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AttendanceAdmissionConditions2Code {
    #[serde(rename = "MASH")]
    Mash,
    #[serde(rename = "MASL")]
    Masl,
    #[serde(rename = "MAPO")]
    Mapo,
    #[serde(rename = "MAAL")]
    Maal,
    #[serde(rename = "MALR")]
    Malr,
    #[serde(rename = "MAHI")]
    Mahi,
    #[serde(rename = "MATK")]
    Matk,
    #[serde(rename = "MADS")]
    Mads,
    #[serde(rename = "MANP")]
    Manp,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DateMode1Code {
    #[serde(rename = "BODY")]
    Body,
    #[serde(rename = "EODY")]
    Eody,
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
pub struct Exact4AlphaNumericText {
    #[validate(regex = "EXACT_4_ALPHA_NUMERIC_TEXT_REGEX")]
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
pub struct IncentivePremiumType2ChoiceEnum {
    #[serde(rename = "PerScty", skip_serializing_if = "Option::is_none")]
    pub per_scty: Option<Number>,
    #[serde(rename = "PerAttndee", skip_serializing_if = "Option::is_none")]
    pub per_attndee: Option<YesNoIndicator>,
    #[serde(rename = "PerVote", skip_serializing_if = "Option::is_none")]
    pub per_vote: Option<VoteTypeAndQuantity1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IncentivePremiumType2Choice {
    #[serde(flatten)]
    pub value: IncentivePremiumType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification221 {
    #[validate]
    #[serde(rename = "NmAndAdr")]
    pub nm_and_adr: PersonName2,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max256Text>,
    #[serde(rename = "Id")]
    pub id: PartyIdentification198Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification30 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Exact4AlphaNumericText,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SecuritiesEntryType2Code {
    #[serde(rename = "BLOK")]
    Blok,
    #[serde(rename = "ELIG")]
    Elig,
    #[serde(rename = "PEND")]
    Pend,
    #[serde(rename = "PENR")]
    Penr,
    #[serde(rename = "NOMI")]
    Nomi,
    #[serde(rename = "SETD")]
    Setd,
    #[serde(rename = "BORR")]
    Borr,
    #[serde(rename = "LOAN")]
    Loan,
    #[serde(rename = "SPOS")]
    Spos,
    #[serde(rename = "TRAD")]
    Trad,
    #[serde(rename = "COLI")]
    Coli,
    #[serde(rename = "COLO")]
    Colo,
    #[serde(rename = "UNBA")]
    Unba,
    #[serde(rename = "INBA")]
    Inba,
    #[serde(rename = "REGO")]
    Rego,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SafekeepingPlace2Code {
    #[serde(rename = "SHHE")]
    Shhe,
    #[serde(rename = "ALLP")]
    Allp,
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
pub struct PartyIdentification250 {
    #[validate]
    #[serde(rename = "NmAndAdr")]
    pub nm_and_adr: PersonName3,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max256Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<NaturalPersonIdentification1>,
    #[serde(rename = "Ntlty", skip_serializing_if = "Option::is_none")]
    pub ntlty: Option<CountryCode>,
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth2>,
    #[serde(rename = "CpnyRegrShrhldrId", skip_serializing_if = "Option::is_none")]
    pub cpny_regr_shrhldr_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PersonName3 {
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix2Code>,
    #[validate]
    #[serde(rename = "FrstNm")]
    pub frst_nm: Max350Text,
    #[validate]
    #[serde(rename = "Srnm")]
    pub srnm: Max350Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<PostalAddress26>,
}
#[derive(
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
    #[serde(rename = "MtgNtfctn")]
    pub mtg_ntfctn: MeetingNotificationV09<A>,
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
pub struct ImpliedCurrencyAndAmount {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
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
pub struct Max1025Text {
    #[validate(length(min = 1, max = 1025,))]
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
pub struct CountryCode {
    #[validate(regex = "COUNTRY_CODE_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfIdentification4Code {
    #[serde(rename = "ARNU")]
    Arnu,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "CORP")]
    Corp,
    #[serde(rename = "DRLC")]
    Drlc,
    #[serde(rename = "IDCD")]
    Idcd,
    #[serde(rename = "NRIN")]
    Nrin,
    #[serde(rename = "CCPT")]
    Ccpt,
    #[serde(rename = "SOCS")]
    Socs,
    #[serde(rename = "TXID")]
    Txid,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification129ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification36>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification129Choice {
    #[serde(flatten)]
    pub value: PartyIdentification129ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
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
pub struct EligiblePosition12 {
    #[validate]
    #[serde(rename = "AcctId")]
    pub acct_id: Max35Text,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<PartyIdentification231Choice>,
    #[validate(length(min = 0, max = 15,))]
    #[serde(rename = "HldgBal", default)]
    pub hldg_bal: Vec<HoldingBalance9>,
    #[validate(length(min = 0, max = 250,))]
    #[serde(rename = "RghtsHldr", default)]
    pub rghts_hldr: Vec<PartyIdentification237Choice>,
}
#[derive(
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
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
    #[serde(rename = "ActiveOrHistoricCurrencyAnd13DecimalAmount")]
    pub value: ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveOrHistoricCurrencyCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ParticipationMethod2 {
    #[serde(rename = "PrtcptnMtd")]
    pub prtcptn_mtd: ParticipationMethod3Choice,
    #[serde(rename = "IssrDdlnForVtng")]
    pub issr_ddln_for_vtng: DateFormat58Choice,
    #[serde(rename = "SpprtdByAcctSvcr", skip_serializing_if = "Option::is_none")]
    pub spprtd_by_acct_svcr: Option<YesNoIndicator>,
    #[serde(rename = "RspnDdlnForVtng", skip_serializing_if = "Option::is_none")]
    pub rspn_ddln_for_vtng: Option<DateFormat58Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PersonName2 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<PostalAddress26>,
}
#[derive(
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
pub struct NaturalPersonIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "IdTp", skip_serializing_if = "Option::is_none")]
    pub id_tp: Option<IdentificationType45Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LanguageSpecifiedNarrative1 {
    #[serde(rename = "Lang")]
    pub lang: Iso2ALanguageCode,
    #[validate]
    #[serde(rename = "AddtlInf")]
    pub addtl_inf: Max8000Text,
}
#[derive(
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
pub struct MeetingEventReference1ChoiceEnum {
    #[serde(rename = "LkdMtgId", skip_serializing_if = "Option::is_none")]
    pub lkd_mtg_id: Option<Max35Text>,
    #[serde(rename = "LkdIssrMtgId", skip_serializing_if = "Option::is_none")]
    pub lkd_issr_mtg_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MeetingEventReference1Choice {
    #[serde(flatten)]
    pub value: MeetingEventReference1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QuorumQuantity1ChoiceEnum {
    #[serde(rename = "QrmQty", skip_serializing_if = "Option::is_none")]
    pub qrm_qty: Option<Max35Text>,
    #[serde(rename = "QrmQtyPctg", skip_serializing_if = "Option::is_none")]
    pub qrm_qty_pctg: Option<PercentageRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QuorumQuantity1Choice {
    #[serde(flatten)]
    pub value: QuorumQuantity1ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EventCompletenessStatus1Code {
    #[serde(rename = "COMP")]
    Comp,
    #[serde(rename = "INCO")]
    Inco,
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
pub struct DateFormat58ChoiceEnum {
    #[serde(rename = "DtOrDtTm", skip_serializing_if = "Option::is_none")]
    pub dt_or_dt_tm: Option<DateAndDateTime2Choice>,
    #[serde(rename = "DtCd", skip_serializing_if = "Option::is_none")]
    pub dt_cd: Option<DateType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateFormat58Choice {
    #[serde(flatten)]
    pub value: DateFormat58ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommunicationAddress11 {
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max256Text>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max2048Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Pagination1 {
    #[validate]
    #[serde(rename = "PgNb")]
    pub pg_nb: Max5NumericText,
    #[validate]
    #[serde(rename = "LastPgInd")]
    pub last_pg_ind: YesNoIndicator,
}
#[derive(
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
pub struct MeetingEventReference1 {
    #[serde(rename = "EvtId")]
    pub evt_id: MeetingEventReference1Choice,
    #[serde(rename = "LkgTp", skip_serializing_if = "Option::is_none")]
    pub lkg_tp: Option<ProcessingPosition3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CorporateEventNarrative4 {
    #[validate(length(min = 0,))]
    #[serde(rename = "Dsclmr", default)]
    pub dsclmr: Vec<LanguageSpecifiedNarrative1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PrcgTxtForNxtIntrmy", default)]
    pub prcg_txt_for_nxt_intrmy: Vec<Max8000Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ItemDescription1 {
    #[serde(rename = "Lang")]
    pub lang: Iso2ALanguageCode,
    #[serde(rename = "Titl", skip_serializing_if = "Option::is_none")]
    pub titl: Option<Max350Text>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max1025Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ResolutionStatus1Code {
    #[serde(rename = "ACTV")]
    Actv,
    #[serde(rename = "WDRA")]
    Wdra,
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
pub struct FinancialInstrumentQuantity18ChoiceEnum {
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrumentQuantity18Choice {
    #[serde(flatten)]
    pub value: FinancialInstrumentQuantity18ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification237ChoiceEnum {
    #[serde(rename = "NtrlPrsn", skip_serializing_if = "Option::is_none")]
    pub ntrl_prsn: Option<PartyIdentification250>,
    #[serde(rename = "LglPrsn", skip_serializing_if = "Option::is_none")]
    pub lgl_prsn: Option<PartyIdentification248>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification237Choice {
    #[serde(flatten)]
    pub value: PartyIdentification237ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProxyNotAllowed1Code {
    #[serde(rename = "NPRO")]
    Npro,
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
pub struct Proxy11 {
    #[serde(rename = "PrxyTp")]
    pub prxy_tp: ProxyType3Code,
    #[serde(rename = "PrsnDtls", skip_serializing_if = "Option::is_none")]
    pub prsn_dtls: Option<IndividualPerson43>,
}
#[derive(
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
pub struct Resolution6 {
    #[validate]
    #[serde(rename = "IssrLabl")]
    pub issr_labl: Max35Text,
    #[validate(length(min = 0,))]
    #[serde(rename = "Desc", default)]
    pub desc: Vec<ItemDescription1>,
    #[serde(rename = "ListgGrpRsltnLabl", skip_serializing_if = "Option::is_none")]
    pub listg_grp_rsltn_labl: Option<Max35Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ResolutionType2Code>,
    #[validate]
    #[serde(rename = "ForInfOnly")]
    pub for_inf_only: YesNoIndicator,
    #[serde(rename = "VoteTp", skip_serializing_if = "Option::is_none")]
    pub vote_tp: Option<VoteType1Code>,
    #[serde(rename = "Sts")]
    pub sts: ResolutionStatus1Code,
    #[serde(rename = "SubmittdBySctyHldr", skip_serializing_if = "Option::is_none")]
    pub submittd_by_scty_hldr: Option<YesNoIndicator>,
    #[serde(rename = "RghtToWdrwInd", skip_serializing_if = "Option::is_none")]
    pub rght_to_wdrw_ind: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "VoteInstrTp", default)]
    pub vote_instr_tp: Vec<VoteInstructionType1>,
    #[serde(rename = "MgmtRcmmndtn", skip_serializing_if = "Option::is_none")]
    pub mgmt_rcmmndtn: Option<VoteInstruction5Code>,
    #[serde(rename = "NtifngPtyRcmmndtn", skip_serializing_if = "Option::is_none")]
    pub ntifng_pty_rcmmndtn: Option<VoteInstruction5Code>,
    #[serde(rename = "Entitlmnt", skip_serializing_if = "Option::is_none")]
    pub entitlmnt: Option<Entitlement1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "VtngRghtsThrshldForApprvl", default)]
    pub vtng_rghts_thrshld_for_apprvl: Vec<VotingRightsThreshold1>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max2048Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceFormat28ChoiceEnum {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<SafekeepingPlaceTypeAndText6>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "TpAndId", skip_serializing_if = "Option::is_none")]
    pub tp_and_id: Option<SafekeepingPlaceTypeAndIdentification1>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification78>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SafekeepingPlaceFormat28Choice {
    #[serde(flatten)]
    pub value: SafekeepingPlaceFormat28ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MeetingNotice7 {
    #[validate]
    #[serde(rename = "MtgId")]
    pub mtg_id: Max35Text,
    #[serde(rename = "IssrMtgId", skip_serializing_if = "Option::is_none")]
    pub issr_mtg_id: Option<Max35Text>,
    #[serde(rename = "Tp")]
    pub tp: MeetingType4Code,
    #[serde(rename = "Clssfctn", skip_serializing_if = "Option::is_none")]
    pub clssfctn: Option<MeetingTypeClassification2Choice>,
    #[serde(rename = "AnncmntDt", skip_serializing_if = "Option::is_none")]
    pub anncmnt_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "OneManOneVoteInd", skip_serializing_if = "Option::is_none")]
    pub one_man_one_vote_ind: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Prtcptn", default)]
    pub prtcptn: Vec<ParticipationMethod2>,
    #[serde(rename = "Attndnc", skip_serializing_if = "Option::is_none")]
    pub attndnc: Option<Attendance2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlDcmnttnURLAdr", default)]
    pub addtl_dcmnttn_url_adr: Vec<Max2048Text>,
    #[serde(rename = "EvtPrcgWebSiteAdr", skip_serializing_if = "Option::is_none")]
    pub evt_prcg_web_site_adr: Option<Max2048Text>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "AddtlPrcdrDtls", default)]
    pub addtl_prcdr_dtls: Vec<AdditionalRights3>,
    #[serde(
        rename = "TtlNbOfSctiesOutsdng",
        skip_serializing_if = "Option::is_none"
    )]
    pub ttl_nb_of_scties_outsdng: Option<FinancialInstrumentQuantity18Choice>,
    #[serde(rename = "TtlNbOfVtngRghts", skip_serializing_if = "Option::is_none")]
    pub ttl_nb_of_vtng_rghts: Option<Number>,
    #[serde(
        rename = "PrxyAppntmntNtfctnAdr",
        skip_serializing_if = "Option::is_none"
    )]
    pub prxy_appntmnt_ntfctn_adr: Option<PostalAddress1>,
    #[serde(rename = "PrxyChc", skip_serializing_if = "Option::is_none")]
    pub prxy_chc: Option<Proxy5Choice>,
    #[validate(length(min = 0, max = 12,))]
    #[serde(rename = "CtctPrsnDtls", default)]
    pub ctct_prsn_dtls: Vec<MeetingContactPerson3>,
    #[serde(rename = "RsltPblctnDt", skip_serializing_if = "Option::is_none")]
    pub rslt_pblctn_dt: Option<DateFormat3Choice>,
    #[serde(
        rename = "SctiesBlckgPrdEndDt",
        skip_serializing_if = "Option::is_none"
    )]
    pub scties_blckg_prd_end_dt: Option<IsoDateTime>,
    #[serde(rename = "EntitlmntFxgDt", skip_serializing_if = "Option::is_none")]
    pub entitlmnt_fxg_dt: Option<DateFormat1>,
    #[serde(rename = "RegnSctiesDdln", skip_serializing_if = "Option::is_none")]
    pub regn_scties_ddln: Option<DateFormat58Choice>,
    #[serde(rename = "RegnSctiesMktDdln", skip_serializing_if = "Option::is_none")]
    pub regn_scties_mkt_ddln: Option<DateFormat58Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ContactIdentification1 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max35Text,
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix1Code>,
    #[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
    pub gvn_nm: Option<Max35Text>,
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Max35Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<PhoneNumber>,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max256Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MeetingContactPerson3 {
    #[serde(rename = "CtctPrsn", skip_serializing_if = "Option::is_none")]
    pub ctct_prsn: Option<ContactIdentification1>,
    #[serde(rename = "EmplngPty", skip_serializing_if = "Option::is_none")]
    pub emplng_pty: Option<PartyIdentification129Choice>,
    #[serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none")]
    pub plc_of_listg: Option<MicIdentifier>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NotificationType3Code {
    #[serde(rename = "NEWM")]
    Newm,
    #[serde(rename = "REPL")]
    Repl,
    #[serde(rename = "RMDR")]
    Rmdr,
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
pub enum EventConfirmationStatus1Code {
    #[serde(rename = "CONF")]
    Conf,
    #[serde(rename = "UCON")]
    Ucon,
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
pub struct PostalAddress26 {
    #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<AddressType2Code>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "AdrLine", default)]
    pub adr_line: Vec<Max70Text>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max16Text>,
    #[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
    pub pst_bx: Option<Max16Text>,
    #[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
    pub pst_cd: Option<Max16Text>,
    #[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
    pub twn_nm: Option<Max35Text>,
    #[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<Max35Text>,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EventStatus1 {
    #[serde(rename = "EvtCmpltnsSts")]
    pub evt_cmpltns_sts: EventCompletenessStatus1Code,
    #[serde(rename = "EvtConfSts")]
    pub evt_conf_sts: EventConfirmationStatus1Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ThresholdBasis1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ThresholdBasis1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ThresholdBasis1Choice {
    #[serde(flatten)]
    pub value: ThresholdBasis1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DateType1Code {
    #[serde(rename = "UKWN")]
    Ukwn,
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
pub struct Max256Text {
    #[validate(length(min = 1, max = 256,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AdditionalRight1Code {
    #[serde(rename = "WQPS")]
    Wqps,
    #[serde(rename = "RSPS")]
    Rsps,
    #[serde(rename = "AIPS")]
    Aips,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProxyType3Code {
    #[serde(rename = "CHRM")]
    Chrm,
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "NEPR")]
    Nepr,
    #[serde(rename = "HLDR")]
    Hldr,
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
pub struct ActiveOrHistoricCurrencyCode {
    #[validate(regex = "ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX")]
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
pub struct PartyIdentification248 {
    #[validate]
    #[serde(rename = "NmAndAdr")]
    pub nm_and_adr: PersonName2,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max256Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<PartyIdentification198Choice>,
    #[serde(rename = "CpnyRegrShrhldrId", skip_serializing_if = "Option::is_none")]
    pub cpny_regr_shrhldr_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct VoteTypeAndQuantity1 {
    #[serde(rename = "VoteInstrTp")]
    pub vote_instr_tp: VoteInstructionType1Choice,
    #[validate]
    #[serde(rename = "VoteQty")]
    pub vote_qty: Number,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AttendanceAdmissionConditions2 {
    #[serde(rename = "Cd")]
    pub cd: AttendanceAdmissionConditions2Code,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommunicationAddress12 {
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max2048Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateAndPlaceOfBirth2 {
    #[validate]
    #[serde(rename = "BirthDt")]
    pub birth_dt: IsoDate,
    #[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
    pub prvc_of_birth: Option<Max35Text>,
    #[serde(rename = "CityOfBirth", skip_serializing_if = "Option::is_none")]
    pub city_of_birth: Option<Max35Text>,
    #[serde(rename = "CtryOfBirth", skip_serializing_if = "Option::is_none")]
    pub ctry_of_birth: Option<CountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct VoteInstructionType1 {
    #[serde(rename = "VoteInstrTpCd")]
    pub vote_instr_tp_cd: VoteInstructionType1Choice,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PowerOfAttorneyRequirements4 {
    #[validate(length(min = 0, max = 4,))]
    #[serde(rename = "LglRqrmnt", default)]
    pub lgl_rqrmnt: Vec<PowerOfAttorneyLegalisation1Code>,
    #[serde(rename = "OthrDcmnttn", skip_serializing_if = "Option::is_none")]
    pub othr_dcmnttn: Option<Max350Text>,
    #[serde(rename = "DocSubmissnDdln", skip_serializing_if = "Option::is_none")]
    pub doc_submissn_ddln: Option<DateFormat58Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Attendance2 {
    #[validate(length(min = 0, max = 7,))]
    #[serde(rename = "AdmssnConds", default)]
    pub admssn_conds: Vec<AttendanceAdmissionConditions2>,
    #[serde(rename = "ConfInf", skip_serializing_if = "Option::is_none")]
    pub conf_inf: Option<Max350Text>,
    #[serde(rename = "ConfDdln", skip_serializing_if = "Option::is_none")]
    pub conf_ddln: Option<DateFormat58Choice>,
    #[serde(rename = "ConfMktDdln", skip_serializing_if = "Option::is_none")]
    pub conf_mkt_ddln: Option<DateFormat58Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Entitlement1ChoiceEnum {
    #[serde(rename = "EntitlmntDesc", skip_serializing_if = "Option::is_none")]
    pub entitlmnt_desc: Option<Max35Text>,
    #[serde(rename = "EntitlmntRatio", skip_serializing_if = "Option::is_none")]
    pub entitlmnt_ratio: Option<DecimalNumber>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Entitlement1Choice {
    #[serde(flatten)]
    pub value: Entitlement1ChoiceEnum,
}
#[derive(
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
pub struct MeetingTypeClassification2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MeetingTypeClassification2Code>,
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
pub struct MeetingTypeClassification2Choice {
    #[serde(flatten)]
    pub value: MeetingTypeClassification2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum VoteInstruction5Code {
    #[serde(rename = "ABST")]
    Abst,
    #[serde(rename = "CAGS")]
    Cags,
    #[serde(rename = "CHRM")]
    Chrm,
    #[serde(rename = "CFOR")]
    Cfor,
    #[serde(rename = "NOAC")]
    Noac,
    #[serde(rename = "WTHH")]
    Wthh,
    #[serde(rename = "ONEY")]
    Oney,
    #[serde(rename = "THRY")]
    Thry,
    #[serde(rename = "TWOY")]
    Twoy,
    #[serde(rename = "BLNK")]
    Blnk,
    #[serde(rename = "NREC")]
    Nrec,
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
pub struct GenericIdentification78 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: GenericIdentification30,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IndividualPerson43 {
    #[serde(rename = "PrssgndPrxy", skip_serializing_if = "Option::is_none")]
    pub prssgnd_prxy: Option<PartyIdentification232Choice>,
    #[serde(rename = "EmplngPty", skip_serializing_if = "Option::is_none")]
    pub emplng_pty: Option<PartyIdentification129Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PlaceType1Code {
    #[serde(rename = "UKWN")]
    Ukwn,
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
pub struct AdditionalRightThreshold1ChoiceEnum {
    #[serde(rename = "AddtlRghtThrshld", skip_serializing_if = "Option::is_none")]
    pub addtl_rght_thrshld: Option<Max35Text>,
    #[serde(
        rename = "AddtlRghtThrshldPctg",
        skip_serializing_if = "Option::is_none"
    )]
    pub addtl_rght_thrshld_pctg: Option<PercentageRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalRightThreshold1Choice {
    #[serde(flatten)]
    pub value: AdditionalRightThreshold1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum VoteInstruction6Code {
    #[serde(rename = "ABST")]
    Abst,
    #[serde(rename = "CAGS")]
    Cags,
    #[serde(rename = "AMGT")]
    Amgt,
    #[serde(rename = "BLNK")]
    Blnk,
    #[serde(rename = "CHRM")]
    Chrm,
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "CFOR")]
    Cfor,
    #[serde(rename = "NOAC")]
    Noac,
    #[serde(rename = "ONEY")]
    Oney,
    #[serde(rename = "THRY")]
    Thry,
    #[serde(rename = "TWOY")]
    Twoy,
    #[serde(rename = "WTHH")]
    Wthh,
    #[serde(rename = "WMGT")]
    Wmgt,
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
pub struct VoteInstructionType1ChoiceEnum {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<VoteInstruction6Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct VoteInstructionType1Choice {
    #[serde(flatten)]
    pub value: VoteInstructionType1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ResolutionType2Code {
    #[serde(rename = "EXTR")]
    Extr,
    #[serde(rename = "SPCL")]
    Spcl,
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
pub struct PartyIdentification232ChoiceEnum {
    #[serde(rename = "LglPrsn", skip_serializing_if = "Option::is_none")]
    pub lgl_prsn: Option<PartyIdentification221>,
    #[serde(rename = "NtrlPrsn", skip_serializing_if = "Option::is_none")]
    pub ntrl_prsn: Option<PartyIdentification238>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification232Choice {
    #[serde(flatten)]
    pub value: PartyIdentification232ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SafekeepingPlace1Code {
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "ICSD")]
    Icsd,
    #[serde(rename = "NCSD")]
    Ncsd,
    #[serde(rename = "SHHE")]
    Shhe,
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
pub struct Max8000Text {
    #[validate(length(min = 1, max = 8000,))]
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
pub struct Meeting6 {
    #[serde(rename = "DtAndTm")]
    pub dt_and_tm: DateFormat58Choice,
    #[serde(rename = "DtSts", skip_serializing_if = "Option::is_none")]
    pub dt_sts: Option<MeetingDateStatus2Code>,
    #[serde(rename = "QrmReqrd", skip_serializing_if = "Option::is_none")]
    pub qrm_reqrd: Option<YesNoIndicator>,
    #[validate(length(min = 1, max = 5,))]
    #[serde(rename = "Lctn", default)]
    pub lctn: Vec<LocationFormat1Choice>,
    #[serde(rename = "QrmQty", skip_serializing_if = "Option::is_none")]
    pub qrm_qty: Option<QuorumQuantity1Choice>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max2048Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NotificationUpdate2 {
    #[validate]
    #[serde(rename = "PrvsNtfctnId")]
    pub prvs_ntfctn_id: Max35Text,
    #[serde(rename = "RcnfrmInstrs", skip_serializing_if = "Option::is_none")]
    pub rcnfrm_instrs: Option<YesNoIndicator>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AgentRole1Code {
    #[serde(rename = "PRIN")]
    Prin,
    #[serde(rename = "SUBA")]
    Suba,
    #[default]
    Unknown,
}
