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
    static ref MAX_2_MB_BINARY_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Za-z0-9+/]{4}*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_2_A_LANGUAGE_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:tsrv.002.001.01".to_string()
}

#[derive(
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
pub struct AmountOrPercentage1ChoiceEnum {
    #[serde(rename = "DfndAmt", skip_serializing_if = "Option::is_none")]
    pub dfnd_amt: Option<UndertakingAmount4>,
    #[serde(rename = "PctgAmt", skip_serializing_if = "Option::is_none")]
    pub pctg_amt: Option<Percentage1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountOrPercentage1Choice {
    #[serde(flatten)]
    pub value: AmountOrPercentage1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Party11ChoiceEnum {
    #[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
    pub prvt_id: Option<PersonIdentification5>,
    #[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<OrganisationIdentification8>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Party11Choice {
    #[serde(flatten)]
    pub value: Party11ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PlaceOrUnderConfirmationChoice1Enum {
    #[serde(rename = "PresntnUdrConf", skip_serializing_if = "Option::is_none")]
    pub presntn_udr_conf: Option<PresentationParty1Code>,
    #[serde(rename = "PlcOfPresntn", skip_serializing_if = "Option::is_none")]
    pub plc_of_presntn: Option<PlaceOfPresentation1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PlaceOrUnderConfirmationChoice1 {
    #[serde(flatten)]
    pub value: PlaceOrUnderConfirmationChoice1Enum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UndertakingIssuanceName1Code {
    #[serde(rename = "STBY")]
    Stby,
    #[serde(rename = "DGAR")]
    Dgar,
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
pub struct DateInformation1 {
    #[validate]
    #[serde(rename = "StartDt")]
    pub start_dt: IsoDate,
    #[serde(rename = "Frqcy")]
    pub frqcy: ExternalDateFrequency1Code,
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
pub struct Document9<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "Tp")]
    pub tp: UndertakingDocumentType1Choice,
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Frmt", skip_serializing_if = "Option::is_none")]
    pub frmt: Option<DocumentFormat1Choice>,
    #[validate]
    #[serde(rename = "Nclsr")]
    pub nclsr: Max2MbBinary,
    #[serde(rename = "DgtlSgntr", skip_serializing_if = "Option::is_none")]
    pub dgtl_sgntr: Option<PartyAndSignature2<A>>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GovernanceIdentification1Code {
    #[serde(rename = "ISPR")]
    Ispr,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "UCPR")]
    Ucpr,
    #[serde(rename = "URDG")]
    Urdg,
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
pub struct ModelFormIdentification1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalModelFormIdentification1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModelFormIdentification1Choice {
    #[serde(flatten)]
    pub value: ModelFormIdentification1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum IssuanceType1Code {
    #[serde(rename = "CRQL")]
    Crql,
    #[serde(rename = "CRQC")]
    Crqc,
    #[serde(rename = "ISSU")]
    Issu,
    #[serde(rename = "ISCO")]
    Isco,
    #[serde(rename = "ISAD")]
    Isad,
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
pub struct NonExtension1 {
    #[serde(rename = "NtfctnPrd", skip_serializing_if = "Option::is_none")]
    pub ntfctn_prd: Option<Number>,
    #[serde(rename = "NtfctnMtd", skip_serializing_if = "Option::is_none")]
    pub ntfctn_mtd: Option<CommunicationMethod1Choice>,
    #[serde(rename = "NtfctnRcptTp", skip_serializing_if = "Option::is_none")]
    pub ntfctn_rcpt_tp: Option<PartyType1Choice>,
    #[serde(rename = "NtfctnRcptNm", skip_serializing_if = "Option::is_none")]
    pub ntfctn_rcpt_nm: Option<Max140Text>,
    #[serde(rename = "NtfctnRcptAdr", skip_serializing_if = "Option::is_none")]
    pub ntfctn_rcpt_adr: Option<PostalAddress6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyAndSignature2<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Pty")]
    pub pty: PartyIdentification43,
    #[validate]
    #[serde(rename = "Sgntr")]
    pub sgntr: ProprietaryData3<A>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalTypeOfParty1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyType1Choice {
    #[serde(flatten)]
    pub value: PartyType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Trigger1 {
    #[serde(rename = "DtChc", skip_serializing_if = "Option::is_none")]
    pub dt_chc: Option<FixedOrRecurrentDate1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "DcmntryEvt", default)]
    pub dcmntry_evt: Vec<Document10>,
}
#[derive(
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
pub struct Max2MbBinary {
    #[validate(length(min = 1, max = 2097152,), regex = "MAX_2_MB_BINARY_REGEX")]
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
pub struct DateAndPlaceOfBirth {
    #[validate]
    #[serde(rename = "BirthDt")]
    pub birth_dt: IsoDate,
    #[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
    pub prvc_of_birth: Option<Max35Text>,
    #[validate]
    #[serde(rename = "CityOfBirth")]
    pub city_of_birth: Max35Text,
    #[serde(rename = "CtryOfBirth")]
    pub ctry_of_birth: CountryCode,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericOrganisationIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
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
pub struct GovernanceIdentification1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<GovernanceIdentification1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GovernanceIdentification1Choice {
    #[serde(flatten)]
    pub value: GovernanceIdentification1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GovernanceRules1 {
    #[serde(rename = "RuleId")]
    pub rule_id: GovernanceIdentification1Choice,
    #[serde(rename = "AplblLaw", skip_serializing_if = "Option::is_none")]
    pub aplbl_law: Option<Location1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Jursdctn", default)]
    pub jursdctn: Vec<Location1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification43 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress6>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Party11Choice>,
    #[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<CountryCode>,
    #[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<ContactDetails2>,
}
#[derive(
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
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[serde(rename = "UdrtkgIssncAdvc")]
    pub udrtkg_issnc_advc: UndertakingIssuanceAdviceV01<A, B, C, D>,
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
pub struct ExternalDocumentFormat1Code {
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
pub struct UndertakingAmount4 {
    #[validate]
    #[serde(rename = "VartnAmt")]
    pub vartn_amt: ActiveCurrencyAndAmount,
    #[serde(rename = "BalAmt", skip_serializing_if = "Option::is_none")]
    pub bal_amt: Option<ActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UndertakingIssuanceAdviceV01<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    D: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "AdvsgPty")]
    pub advsg_pty: PartyIdentification43,
    #[serde(rename = "ScndAdvsgPty", skip_serializing_if = "Option::is_none")]
    pub scnd_advsg_pty: Option<PartyIdentification43>,
    #[serde(rename = "DtOfAdvc")]
    pub dt_of_advc: DateAndDateTimeChoice,
    #[serde(rename = "UdrtkgIssncAdvcDtls")]
    pub udrtkg_issnc_advc_dtls: UndertakingAdvice1<A, B, C>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "BkToBkInf", default)]
    pub bk_to_bk_inf: Vec<Max2000Text>,
    #[serde(rename = "DgtlSgntr", skip_serializing_if = "Option::is_none")]
    pub dgtl_sgntr: Option<PartyAndSignature2<D>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Channel1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalChannel1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Channel1Choice {
    #[serde(flatten)]
    pub value: Channel1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalRelativeTo1Code {
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
pub struct PresentationDocumentFormat1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalUndertakingDocumentType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PresentationDocumentFormat1Choice {
    #[serde(flatten)]
    pub value: PresentationDocumentFormat1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalOrganisationIdentification1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UndertakingName1Code {
    #[serde(rename = "STBY")]
    Stby,
    #[serde(rename = "DGAR")]
    Dgar,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum VariationType1Code {
    #[serde(rename = "DECR")]
    Decr,
    #[serde(rename = "INCR")]
    Incr,
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
pub struct PersonIdentification5 {
    #[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<GenericPersonIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalDateFrequency1Code {
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
pub struct Presentation1 {
    #[serde(rename = "Mdm", skip_serializing_if = "Option::is_none")]
    pub mdm: Option<PresentationMedium1Choice>,
    #[serde(
        rename = "PlcOfPresntnOrUdrConfChc",
        skip_serializing_if = "Option::is_none"
    )]
    pub plc_of_presntn_or_udr_conf_chc: Option<PlaceOrUnderConfirmationChoice1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Doc", default)]
    pub doc: Vec<Document8>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<Max2000Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Percentage1 {
    #[validate]
    #[serde(rename = "Rate")]
    pub rate: PercentageRate,
    #[serde(rename = "RltvTo")]
    pub rltv_to: ExternalRelativeTo1Code,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericPersonIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
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
pub struct ContactDetails2 {
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix1Code>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
    #[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
    pub mob_nb: Option<PhoneNumber>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<PhoneNumber>,
    #[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max2048Text>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AutomaticVariation1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp")]
    pub tp: VariationType1Code,
    #[validate(length(min = 1,))]
    #[serde(rename = "AmtAndTrggr", default)]
    pub amt_and_trggr: Vec<AmountAndTrigger1>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<Max2000Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalModelFormIdentification1Code {
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
pub struct UndertakingConfirmation1 {
    #[validate]
    #[serde(rename = "Cnfrmr")]
    pub cnfrmr: PartyIdentification43,
    #[validate]
    #[serde(rename = "RefNb")]
    pub ref_nb: Max35Text,
    #[serde(rename = "Dt")]
    pub dt: DateAndDateTimeChoice,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "Conf", default)]
    pub conf: Vec<Max2000Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UndertakingDocumentType2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalUndertakingDocumentType2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UndertakingDocumentType2Choice {
    #[serde(flatten)]
    pub value: UndertakingDocumentType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Location1 {
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<CountrySubdivision1Choice>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "Txt", default)]
    pub txt: Vec<Max2000Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max2000Text {
    #[validate(length(min = 1, max = 2000,))]
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
pub struct ExternalTypeOfParty1Code {
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
pub struct ExpiryTerms1 {
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<DateAndDateTimeChoice>,
    #[serde(rename = "AutoXtnsn", skip_serializing_if = "Option::is_none")]
    pub auto_xtnsn: Option<AutoExtension1>,
    #[serde(rename = "Cond", skip_serializing_if = "Option::is_none")]
    pub cond: Option<Max2000Text>,
    #[serde(rename = "OpnEnddInd", skip_serializing_if = "Option::is_none")]
    pub opn_endd_ind: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyAndType1 {
    #[serde(rename = "Tp")]
    pub tp: PartyType1Choice,
    #[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
    pub pty: Option<PartyIdentification43>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AutoExtension1 {
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<AutoExtend1Choice>,
    #[serde(rename = "FnlXpryDt", skip_serializing_if = "Option::is_none")]
    pub fnl_xpry_dt: Option<IsoDate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NonXtnsnNtfctn", default)]
    pub non_xtnsn_ntfctn: Vec<NonExtension1>,
}
#[derive(
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
pub struct NarrativeType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalNarrativeType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NarrativeType1Choice {
    #[serde(flatten)]
    pub value: NarrativeType1ChoiceEnum,
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
pub struct ExternalNarrativeType1Code {
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
pub struct ExternalUndertakingType1Code {
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
pub struct Narrative1 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<NarrativeType1Choice>,
    #[validate(length(min = 1, max = 5,))]
    #[serde(rename = "Txt", default)]
    pub txt: Vec<Max20000Text>,
}
#[derive(
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
pub struct Document10 {
    #[serde(rename = "DocTp")]
    pub doc_tp: UndertakingDocumentType2Choice,
    #[serde(rename = "PresntnChanl", skip_serializing_if = "Option::is_none")]
    pub presntn_chanl: Option<Channel1Choice>,
    #[serde(rename = "DocFrmt", skip_serializing_if = "Option::is_none")]
    pub doc_frmt: Option<DocumentFormat1Choice>,
    #[serde(rename = "CpyInd", skip_serializing_if = "Option::is_none")]
    pub cpy_ind: Option<YesNoIndicator>,
    #[serde(rename = "SgndInd", skip_serializing_if = "Option::is_none")]
    pub sgnd_ind: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PersonIdentificationSchemeName1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalPersonIdentification1Code>,
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
pub struct PersonIdentificationSchemeName1Choice {
    #[serde(flatten)]
    pub value: PersonIdentificationSchemeName1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PostalAddress12 {
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
pub struct UnderlyingTradeTransaction1 {
    #[serde(rename = "Tp")]
    pub tp: UnderlyingTradeTransactionType1Choice,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "TxDt", skip_serializing_if = "Option::is_none")]
    pub tx_dt: Option<IsoDate>,
    #[serde(rename = "TndrClsgDt", skip_serializing_if = "Option::is_none")]
    pub tndr_clsg_dt: Option<IsoDate>,
    #[serde(rename = "TxAmt", skip_serializing_if = "Option::is_none")]
    pub tx_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "CtrctAmtPctg", skip_serializing_if = "Option::is_none")]
    pub ctrct_amt_pctg: Option<PercentageRate>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<Max2000Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CountrySubdivision1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CountrySubdivision1Choice {
    #[serde(flatten)]
    pub value: CountrySubdivision1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UndertakingWording1 {
    #[serde(rename = "MdlForm", skip_serializing_if = "Option::is_none")]
    pub mdl_form: Option<ModelFormIdentification1>,
    #[serde(rename = "ReqdWrdgLang", skip_serializing_if = "Option::is_none")]
    pub reqd_wrdg_lang: Option<Iso2ALanguageCode>,
    #[validate(length(min = 0,))]
    #[serde(rename = "UdrtkgTermsAndConds", default)]
    pub udrtkg_terms_and_conds: Vec<Narrative1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalUnderlyingTradeTransactionType1Code {
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
pub struct CommunicationChannel1 {
    #[serde(rename = "Mtd")]
    pub mtd: ExternalChannel1Code,
    #[serde(rename = "DlvrToPtyTp")]
    pub dlvr_to_pty_tp: PartyType1Choice,
    #[serde(rename = "DlvrToNm", skip_serializing_if = "Option::is_none")]
    pub dlvr_to_nm: Option<Max140Text>,
    #[serde(rename = "DlvrToAdr", skip_serializing_if = "Option::is_none")]
    pub dlvr_to_adr: Option<PostalAddress6>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalChannel1Code {
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
pub struct ExternalPersonIdentification1Code {
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
pub struct PresentationMedium1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PresentationMedium1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PresentationMedium1Choice {
    #[serde(flatten)]
    pub value: PresentationMedium1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModelFormIdentification1 {
    #[serde(rename = "Id")]
    pub id: ModelFormIdentification1Choice,
    #[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UndertakingIssuanceMessage<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "UdrtkgDtls")]
    pub udrtkg_dtls: Undertaking3<A>,
    #[serde(rename = "DgtlSgntr", skip_serializing_if = "Option::is_none")]
    pub dgtl_sgntr: Option<PartyAndSignature2<B>>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PostalAddress6 {
    #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<AddressType2Code>,
    #[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
    pub dept: Option<Max70Text>,
    #[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
    pub sub_dept: Option<Max70Text>,
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
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[validate(length(min = 0, max = 7,))]
    #[serde(rename = "AdrLine", default)]
    pub adr_line: Vec<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationIdentificationSchemeName1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalOrganisationIdentification1Code>,
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
pub struct OrganisationIdentificationSchemeName1Choice {
    #[serde(flatten)]
    pub value: OrganisationIdentificationSchemeName1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UndertakingType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalUndertakingType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UndertakingType1Choice {
    #[serde(flatten)]
    pub value: UndertakingType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max20000Text {
    #[validate(length(min = 1, max = 20000,))]
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
pub struct FixedOrRecurrentDate1ChoiceEnum {
    #[serde(rename = "FxdDt", skip_serializing_if = "Option::is_none")]
    pub fxd_dt: Option<IsoDate>,
    #[serde(rename = "RcrntDt", skip_serializing_if = "Option::is_none")]
    pub rcrnt_dt: Option<DateInformation1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FixedOrRecurrentDate1Choice {
    #[serde(flatten)]
    pub value: FixedOrRecurrentDate1ChoiceEnum,
}
#[derive(
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
pub struct Document8 {
    #[serde(rename = "Tp")]
    pub tp: PresentationDocumentFormat1Choice,
    #[serde(rename = "Wrdg", skip_serializing_if = "Option::is_none")]
    pub wrdg: Option<Max20000Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ElctrncDtls", default)]
    pub elctrnc_dtls: Vec<Presentation3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AmountAndTrigger1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "AmtDtlsChc")]
    pub amt_dtls_chc: AmountOrPercentage1Choice,
    #[validate(length(min = 1,))]
    #[serde(rename = "Trggr", default)]
    pub trggr: Vec<Trigger1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdvisingPartyAdditionalInformation1 {
    #[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
    pub ref_nb: Option<Max35Text>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "BkToBnfcryInf", default)]
    pub bk_to_bnfcry_inf: Vec<Max2000Text>,
}
#[derive(
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
pub struct OrganisationIdentification8 {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicIdentifier>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Othr", default)]
    pub othr: Vec<GenericOrganisationIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AutoExtend1ChoiceEnum {
    #[serde(rename = "Days", skip_serializing_if = "Option::is_none")]
    pub days: Option<Number>,
    #[serde(rename = "Yrs", skip_serializing_if = "Option::is_none")]
    pub yrs: Option<Number>,
    #[serde(rename = "Mnths", skip_serializing_if = "Option::is_none")]
    pub mnths: Option<Number>,
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
pub struct AutoExtend1Choice {
    #[serde(flatten)]
    pub value: AutoExtend1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProprietaryData3<
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
pub struct CommunicationMethod1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalChannel1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommunicationMethod1Choice {
    #[serde(flatten)]
    pub value: CommunicationMethod1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Presentation3 {
    #[serde(rename = "Frmt", skip_serializing_if = "Option::is_none")]
    pub frmt: Option<DocumentFormat1Choice>,
    #[serde(rename = "Chanl", skip_serializing_if = "Option::is_none")]
    pub chanl: Option<Channel1Choice>,
    #[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
    pub adr: Option<Max256Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Undertaking3<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Nm")]
    pub nm: UndertakingIssuanceName1Code,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<UndertakingType1Choice>,
    #[serde(rename = "IssncTp")]
    pub issnc_tp: IssuanceType1Code,
    #[validate(length(min = 0,))]
    #[serde(rename = "Applcnt", default)]
    pub applcnt: Vec<PartyIdentification43>,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: PartyIdentification43,
    #[validate(length(min = 1,))]
    #[serde(rename = "Bnfcry", default)]
    pub bnfcry: Vec<PartyIdentification43>,
    #[validate]
    #[serde(rename = "DtOfIssnc")]
    pub dt_of_issnc: IsoDate,
    #[serde(rename = "PlcOfIsse", skip_serializing_if = "Option::is_none")]
    pub plc_of_isse: Option<PostalAddress12>,
    #[serde(rename = "AdvsgPty", skip_serializing_if = "Option::is_none")]
    pub advsg_pty: Option<PartyIdentification43>,
    #[serde(rename = "ScndAdvsgPty", skip_serializing_if = "Option::is_none")]
    pub scnd_advsg_pty: Option<PartyIdentification43>,
    #[validate]
    #[serde(rename = "UdrtkgAmt")]
    pub udrtkg_amt: UndertakingAmount1,
    #[validate]
    #[serde(rename = "XpryDtls")]
    pub xpry_dtls: ExpiryDetails1,
    #[serde(rename = "ConfInd", skip_serializing_if = "Option::is_none")]
    pub conf_ind: Option<YesNoIndicator>,
    #[serde(rename = "ConfPtyTp", skip_serializing_if = "Option::is_none")]
    pub conf_pty_tp: Option<ExternalTypeOfParty1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlPty", default)]
    pub addtl_pty: Vec<PartyAndType1>,
    #[validate]
    #[serde(rename = "GovncRulesAndLaw")]
    pub govnc_rules_and_law: GovernanceRules1,
    #[validate(length(min = 0,))]
    #[serde(rename = "UndrlygTx", default)]
    pub undrlyg_tx: Vec<UnderlyingTradeTransaction1>,
    #[serde(rename = "PresntnDtls", skip_serializing_if = "Option::is_none")]
    pub presntn_dtls: Option<Presentation1>,
    #[validate(length(min = 1,))]
    #[serde(rename = "UdrtkgTermsAndConds", default)]
    pub udrtkg_terms_and_conds: Vec<Narrative1>,
    #[serde(rename = "MltplDmndInd", skip_serializing_if = "Option::is_none")]
    pub mltpl_dmnd_ind: Option<YesNoIndicator>,
    #[serde(rename = "PrtlDmndInd", skip_serializing_if = "Option::is_none")]
    pub prtl_dmnd_ind: Option<YesNoIndicator>,
    #[serde(rename = "ConfChrgsPyblBy", skip_serializing_if = "Option::is_none")]
    pub conf_chrgs_pybl_by: Option<ExternalTypeOfParty1Code>,
    #[serde(rename = "TrfChrgsPyblBy", skip_serializing_if = "Option::is_none")]
    pub trf_chrgs_pybl_by: Option<ExternalTypeOfParty1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AutomtcAmtVartn", default)]
    pub automtc_amt_vartn: Vec<AutomaticVariation1>,
    #[serde(rename = "DlvryChanl", skip_serializing_if = "Option::is_none")]
    pub dlvry_chanl: Option<CommunicationChannel1>,
    #[serde(rename = "TrfInd", skip_serializing_if = "Option::is_none")]
    pub trf_ind: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "NclsdFile", default)]
    pub nclsd_file: Vec<Document9<A>>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<Max2000Text>,
    #[serde(rename = "ReqdLclUdrtkg", skip_serializing_if = "Option::is_none")]
    pub reqd_lcl_udrtkg: Option<Undertaking4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExpiryDetails1 {
    #[serde(rename = "XpryTerms", skip_serializing_if = "Option::is_none")]
    pub xpry_terms: Option<ExpiryTerms1>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "AddtlXpryInf", default)]
    pub addtl_xpry_inf: Vec<Max2000Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UndertakingDocumentType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalUndertakingDocumentType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UndertakingDocumentType1Choice {
    #[serde(flatten)]
    pub value: UndertakingDocumentType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalUndertakingDocumentType2Code {
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
pub struct DocumentFormat1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalDocumentFormat1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DocumentFormat1Choice {
    #[serde(flatten)]
    pub value: DocumentFormat1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PlaceOfPresentation1 {
    #[serde(rename = "Plc")]
    pub plc: ExternalTypeOfParty1Code,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PresentationMedium1Code {
    #[serde(rename = "BOTH")]
    Both,
    #[serde(rename = "ELEC")]
    Elec,
    #[serde(rename = "PAPR")]
    Papr,
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
pub struct UndertakingAdvice1<
    A: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    B: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
    C: std::fmt::Debug + Default + Clone + PartialEq + ::serde::Serialize + ::validator::Validate,
> {
    #[validate]
    #[serde(rename = "UdrtkgIssncMsg")]
    pub udrtkg_issnc_msg: UndertakingIssuanceMessage<A, B>,
    #[serde(
        rename = "FrstAdvsgPtyAddtlInf",
        skip_serializing_if = "Option::is_none"
    )]
    pub frst_advsg_pty_addtl_inf: Option<AdvisingPartyAdditionalInformation1>,
    #[serde(
        rename = "ScndAdvsgPtyAddtlInf",
        skip_serializing_if = "Option::is_none"
    )]
    pub scnd_advsg_pty_addtl_inf: Option<AdvisingPartyAdditionalInformation1>,
    #[serde(rename = "ConfDtls", skip_serializing_if = "Option::is_none")]
    pub conf_dtls: Option<UndertakingConfirmation1>,
    #[validate(length(min = 0, max = 3,))]
    #[serde(rename = "DgtlSgntr", default)]
    pub dgtl_sgntr: Vec<PartyAndSignature2<C>>,
}
#[derive(
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
pub struct Undertaking4 {
    #[serde(rename = "Nm")]
    pub nm: UndertakingName1Code,
    #[serde(rename = "Tp")]
    pub tp: ExternalUndertakingType1Code,
    #[validate(length(min = 1,))]
    #[serde(rename = "Applcnt", default)]
    pub applcnt: Vec<PartyIdentification43>,
    #[validate(length(min = 1,))]
    #[serde(rename = "Bnfcry", default)]
    pub bnfcry: Vec<PartyIdentification43>,
    #[serde(rename = "DtOfIssnc", skip_serializing_if = "Option::is_none")]
    pub dt_of_issnc: Option<IsoDate>,
    #[serde(rename = "AdvsgPty", skip_serializing_if = "Option::is_none")]
    pub advsg_pty: Option<PartyIdentification43>,
    #[serde(rename = "ScndAdvsgPty", skip_serializing_if = "Option::is_none")]
    pub scnd_advsg_pty: Option<PartyIdentification43>,
    #[validate]
    #[serde(rename = "LclUdrtkgAmt")]
    pub lcl_udrtkg_amt: UndertakingAmount1,
    #[validate]
    #[serde(rename = "XpryDtls")]
    pub xpry_dtls: ExpiryDetails1,
    #[serde(rename = "ConfInd", skip_serializing_if = "Option::is_none")]
    pub conf_ind: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlPty", default)]
    pub addtl_pty: Vec<PartyAndType1>,
    #[validate]
    #[serde(rename = "GovncRulesAndLaw")]
    pub govnc_rules_and_law: GovernanceRules1,
    #[validate(length(min = 0,))]
    #[serde(rename = "UndrlygTx", default)]
    pub undrlyg_tx: Vec<UnderlyingTradeTransaction1>,
    #[serde(rename = "PresntnDtls", skip_serializing_if = "Option::is_none")]
    pub presntn_dtls: Option<Presentation1>,
    #[validate]
    #[serde(rename = "UdrtkgWrdg")]
    pub udrtkg_wrdg: UndertakingWording1,
    #[serde(rename = "MltplDmndInd", skip_serializing_if = "Option::is_none")]
    pub mltpl_dmnd_ind: Option<YesNoIndicator>,
    #[serde(rename = "PrtlDmndInd", skip_serializing_if = "Option::is_none")]
    pub prtl_dmnd_ind: Option<YesNoIndicator>,
    #[serde(rename = "ConfChrgsPyblBy", skip_serializing_if = "Option::is_none")]
    pub conf_chrgs_pybl_by: Option<ExternalTypeOfParty1Code>,
    #[serde(rename = "TrfChrgsPyblBy", skip_serializing_if = "Option::is_none")]
    pub trf_chrgs_pybl_by: Option<ExternalTypeOfParty1Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AutomtcAmtVartn", default)]
    pub automtc_amt_vartn: Vec<AutomaticVariation1>,
    #[serde(rename = "DlvryChanl", skip_serializing_if = "Option::is_none")]
    pub dlvry_chanl: Option<CommunicationChannel1>,
    #[serde(rename = "TrfInd", skip_serializing_if = "Option::is_none")]
    pub trf_ind: Option<YesNoIndicator>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<Max2000Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UndertakingAmount1 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
    #[serde(rename = "PlusTlrnce", skip_serializing_if = "Option::is_none")]
    pub plus_tlrnce: Option<PercentageRate>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<Max2000Text>,
}
#[derive(
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
pub struct UnderlyingTradeTransactionType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalUnderlyingTradeTransactionType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnderlyingTradeTransactionType1Choice {
    #[serde(flatten)]
    pub value: UnderlyingTradeTransactionType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalUndertakingDocumentType1Code {
    #[validate(length(min = 1, max = 4,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PresentationParty1Code {
    #[serde(rename = "ETHR")]
    Ethr,
    #[serde(rename = "EXCN")]
    Excn,
    #[serde(rename = "EXIS")]
    Exis,
    #[default]
    Unknown,
}
