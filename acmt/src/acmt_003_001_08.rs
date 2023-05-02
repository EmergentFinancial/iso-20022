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
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IRISH_NSC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"IE[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SPANISH_DOMESTIC_INTERBANKING_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"ES[0-9]{8,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BLOOMBERG_2_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"(BBG)[BCDFGHJKLMNPQRSTVWXYZ\d]{8}\d"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref UK_DOMESTIC_SORT_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SC[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref GERMAN_BANKLEITZAHL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"BL[0-9]{8,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref HONG_KONG_BANK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"HK[0-9]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref FEDWIRE_ROUTING_NUMBER_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"FW[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CANADIAN_PAYMENTS_ARN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CA[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SWISS_BC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SW[0-9]{3,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXTENSIVE_BRANCH_NETWORK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AU[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_YEAR_MONTH_REGEX: ::regex::Regex = ::regex::Regex::new(r#"^-?\d{4}-(0[1-9]|1[0-2])([+-]\d{2}:\d{2}|Z)?$"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref NEW_ZEALAND_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"NZ[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PORTUGUESE_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"PT[0-9]{8,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ITALIAN_DOMESTIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"IT[0-9]{10,10}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RUSSIAN_CENTRAL_BANK_IDENTIFICATION_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"RU[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SMALL_NETWORK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AU[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref AUSTRIAN_BANKLEITZAHL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AT[0-9]{5,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IBAN_2007_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BICFI_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SOUTH_AFRICAN_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"ZA[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SWISS_SIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SW[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CHIPS_UNIVERSAL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CH[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CHIPS_PARTICIPANT_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CP[0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:acmt.003.001.08".to_string()
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CompanyLink1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CompanyLink1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CompanyLink1Choice {
    #[serde(flatten)]
    pub value: CompanyLink1ChoiceEnum,
}
#[derive(
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
pub struct ExtendedParty15 {
    #[serde(rename = "XtndedPtyRole")]
    pub xtnded_pty_role: Extended350Code,
    #[validate]
    #[serde(rename = "OthrPtyDtls")]
    pub othr_pty_dtls: InvestmentAccountOwnershipInformation17,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommunicationMethod3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CommunicationMethod1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommunicationMethod3Choice {
    #[serde(flatten)]
    pub value: CommunicationMethod3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NameAndAddress4 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[validate]
    #[serde(rename = "Adr")]
    pub adr: PostalAddress1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PoliticallyExposedPersonStatus1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PoliticallyExposedPersonStatus1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PoliticallyExposedPersonStatus1Choice {
    #[serde(flatten)]
    pub value: PoliticallyExposedPersonStatus1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CertificationType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CertificateType2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CertificationType1Choice {
    #[serde(flatten)]
    pub value: CertificationType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitsOrAmountOrPercentage1ChoiceEnum {
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<PercentageRate>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitsOrAmountOrPercentage1Choice {
    #[serde(flatten)]
    pub value: UnitsOrAmountOrPercentage1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestmentFundRole6Code {
    #[serde(rename = "CACO")]
    Caco,
    #[serde(rename = "CONC")]
    Conc,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "DATP")]
    Datp,
    #[serde(rename = "DIST")]
    Dist,
    #[serde(rename = "FACT")]
    Fact,
    #[serde(rename = "FIAD")]
    Fiad,
    #[serde(rename = "FIAG")]
    Fiag,
    #[serde(rename = "FMCO")]
    Fmco,
    #[serde(rename = "FNBR")]
    Fnbr,
    #[serde(rename = "FTAG")]
    Ftag,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "INVE")]
    Inve,
    #[serde(rename = "INVS")]
    Invs,
    #[serde(rename = "PAYI")]
    Payi,
    #[serde(rename = "REGI")]
    Regi,
    #[serde(rename = "TRAG")]
    Trag,
    #[serde(rename = "TRAN")]
    Tran,
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
pub struct PostalAddress21 {
    #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<AddressType2Choice>,
    #[serde(rename = "MlngInd", skip_serializing_if = "Option::is_none")]
    pub mlng_ind: Option<YesNoIndicator>,
    #[serde(rename = "RegnAdrInd", skip_serializing_if = "Option::is_none")]
    pub regn_adr_ind: Option<YesNoIndicator>,
    #[serde(rename = "CareOf", skip_serializing_if = "Option::is_none")]
    pub care_of: Option<Max70Text>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "AdrLine", default)]
    pub adr_line: Vec<Max70Text>,
    #[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max16Text>,
    #[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
    pub bldg_nm: Option<Max35Text>,
    #[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
    pub pst_bx: Option<Max10Text>,
    #[serde(rename = "SdInBldg", skip_serializing_if = "Option::is_none")]
    pub sd_in_bldg: Option<Max35Text>,
    #[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
    pub flr: Option<Max70Text>,
    #[serde(rename = "SuiteId", skip_serializing_if = "Option::is_none")]
    pub suite_id: Option<Max10Text>,
    #[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
    pub pst_cd: Option<Max16Text>,
    #[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
    pub dstrct_nm: Option<Max35Text>,
    #[serde(rename = "Vllg", skip_serializing_if = "Option::is_none")]
    pub vllg: Option<Max70Text>,
    #[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
    pub twn_nm: Option<Max35Text>,
    #[serde(rename = "Stat", skip_serializing_if = "Option::is_none")]
    pub stat: Option<Max70Text>,
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
pub struct DateAndAmount1 {
    #[validate]
    #[serde(rename = "Dt")]
    pub dt: IsoDate,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CompanyLink1Code {
    #[serde(rename = "AGEN")]
    Agen,
    #[serde(rename = "BROK")]
    Brok,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "MEMB")]
    Memb,
    #[serde(rename = "PCOM")]
    Pcom,
    #[serde(rename = "RELA")]
    Rela,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OrderOriginatorEligibility1Code {
    #[serde(rename = "ELIG")]
    Elig,
    #[serde(rename = "RETL")]
    Retl,
    #[serde(rename = "PROF")]
    Prof,
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
pub struct IndividualPerson38 {
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix1Choice>,
    #[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
    pub gvn_nm: Option<Max35Text>,
    #[serde(rename = "MddlNm", skip_serializing_if = "Option::is_none")]
    pub mddl_nm: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "NmSfx", skip_serializing_if = "Option::is_none")]
    pub nm_sfx: Option<Max35Text>,
    #[serde(rename = "Gndr", skip_serializing_if = "Option::is_none")]
    pub gndr: Option<Gender1Code>,
    #[serde(rename = "BirthDt", skip_serializing_if = "Option::is_none")]
    pub birth_dt: Option<IsoDate>,
    #[serde(rename = "CtryOfBirth", skip_serializing_if = "Option::is_none")]
    pub ctry_of_birth: Option<CountryCode>,
    #[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
    pub prvc_of_birth: Option<Max35Text>,
    #[serde(rename = "CityOfBirth", skip_serializing_if = "Option::is_none")]
    pub city_of_birth: Option<Max35Text>,
    #[serde(rename = "Prfssn", skip_serializing_if = "Option::is_none")]
    pub prfssn: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ModfdPstlAdr", default)]
    pub modfd_pstl_adr: Vec<ModificationScope34>,
    #[validate(length(min = 0, max = 3,))]
    #[serde(rename = "ModfdCtznsh", default)]
    pub modfd_ctznsh: Vec<ModificationScope39>,
    #[serde(rename = "EmplngCpny", skip_serializing_if = "Option::is_none")]
    pub emplng_cpny: Option<Max140Text>,
    #[serde(rename = "BizFctn", skip_serializing_if = "Option::is_none")]
    pub biz_fctn: Option<Max35Text>,
    #[serde(rename = "PltclyXpsdPrsn", skip_serializing_if = "Option::is_none")]
    pub pltcly_xpsd_prsn: Option<PoliticallyExposedPerson1>,
    #[serde(rename = "DthDt", skip_serializing_if = "Option::is_none")]
    pub dth_dt: Option<IsoDate>,
    #[serde(rename = "CvlSts", skip_serializing_if = "Option::is_none")]
    pub cvl_sts: Option<CivilStatus1Choice>,
    #[serde(rename = "EdctnLvl", skip_serializing_if = "Option::is_none")]
    pub edctn_lvl: Option<Max35Text>,
    #[serde(rename = "FmlyInf", skip_serializing_if = "Option::is_none")]
    pub fmly_inf: Option<PersonalInformation1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "GDPRData", default)]
    pub gdpr_data: Vec<GdprData1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Party48ChoiceEnum {
    #[serde(rename = "Org", skip_serializing_if = "Option::is_none")]
    pub org: Option<Organisation40>,
    #[serde(rename = "IndvPrsn", skip_serializing_if = "Option::is_none")]
    pub indv_prsn: Option<IndividualPerson38>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Party48Choice {
    #[serde(flatten)]
    pub value: Party48ChoiceEnum,
}
#[derive(
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
pub struct BlockedStatusReason2 {
    #[serde(rename = "TxTp")]
    pub tx_tp: TransactionType5Choice,
    #[validate]
    #[serde(rename = "Blckd")]
    pub blckd: YesNoIndicator,
    #[validate(length(min = 0,))]
    #[serde(rename = "Rsn", default)]
    pub rsn: Vec<BlockedReason2Choice>,
    #[validate]
    #[serde(rename = "AddtlInf")]
    pub addtl_inf: Max350Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyRole5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PartyRole1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyRole5Choice {
    #[serde(flatten)]
    pub value: PartyRole5ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementInstructionReason1Code {
    #[serde(rename = "CSHI")]
    Cshi,
    #[serde(rename = "ALLL")]
    Alll,
    #[serde(rename = "CSHO")]
    Csho,
    #[serde(rename = "CHAR")]
    Char,
    #[serde(rename = "DIVI")]
    Divi,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "SAVP")]
    Savp,
    #[serde(rename = "REDM")]
    Redm,
    #[serde(rename = "SAVE")]
    Save,
    #[serde(rename = "BUYI")]
    Buyi,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "SUBS")]
    Subs,
    #[serde(rename = "WTHP")]
    Wthp,
    #[serde(rename = "CORP")]
    Corp,
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
pub struct BelgianIdentifier {
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxWithholdingMethod3Code {
    #[serde(rename = "MITX")]
    Mitx,
    #[serde(rename = "INVE")]
    Inve,
    #[serde(rename = "ACCT")]
    Acct,
    #[serde(rename = "EXMT")]
    Exmt,
    #[serde(rename = "REPT")]
    Rept,
    #[serde(rename = "CRTF")]
    Crtf,
    #[serde(rename = "WHCO")]
    Whco,
    #[serde(rename = "WTHD")]
    Wthd,
    #[serde(rename = "WTRE")]
    Wtre,
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
pub struct Organisation40 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<PartyIdentification177Choice>,
    #[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
    pub lgl_ntty_idr: Option<LeiIdentifier>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    pub purp: Option<Max35Text>,
    #[serde(rename = "RegnCtry", skip_serializing_if = "Option::is_none")]
    pub regn_ctry: Option<CountryCode>,
    #[serde(rename = "RegnDt", skip_serializing_if = "Option::is_none")]
    pub regn_dt: Option<IsoDate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ModfdPstlAdr", default)]
    pub modfd_pstl_adr: Vec<ModificationScope34>,
    #[serde(rename = "TpOfOrg", skip_serializing_if = "Option::is_none")]
    pub tp_of_org: Option<OrganisationType1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PlcOfListg", default)]
    pub plc_of_listg: Vec<MicIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RiskLevel2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RiskLevel1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RiskLevel2Choice {
    #[serde(flatten)]
    pub value: RiskLevel2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitsOrAmount1ChoiceEnum {
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UnitsOrAmount1Choice {
    #[serde(flatten)]
    pub value: UnitsOrAmount1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DataBaseCheck1 {
    #[validate]
    #[serde(rename = "DBChck")]
    pub db_chck: YesNoIndicator,
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InformationDistribution1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InformationDistribution2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InformationDistribution1Choice {
    #[serde(flatten)]
    pub value: InformationDistribution1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IrishNscIdentifier {
    #[validate(regex = "IRISH_NSC_IDENTIFIER_REGEX")]
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
pub struct Liability1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Liability1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Liability1Choice {
    #[serde(flatten)]
    pub value: Liability1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct StatementFrequencyReason2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EventFrequency9Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct StatementFrequencyReason2Choice {
    #[serde(flatten)]
    pub value: StatementFrequencyReason2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum NoReasonCode {
    #[serde(rename = "NORE")]
    Nore,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AccountOwnershipType4Code {
    #[serde(rename = "UNCO")]
    Unco,
    #[serde(rename = "LIPA")]
    Lipa,
    #[serde(rename = "ENTR")]
    Entr,
    #[serde(rename = "CORP")]
    Corp,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "EURE")]
    Eure,
    #[serde(rename = "PART")]
    Part,
    #[serde(rename = "TRUS")]
    Trus,
    #[serde(rename = "GOVO")]
    Govo,
    #[serde(rename = "JOIT")]
    Joit,
    #[serde(rename = "COMO")]
    Como,
    #[serde(rename = "JOIN")]
    Join,
    #[serde(rename = "LLCO")]
    Llco,
    #[serde(rename = "NOMI")]
    Nomi,
    #[serde(rename = "NFPO")]
    Nfpo,
    #[serde(rename = "ONIS")]
    Onis,
    #[serde(rename = "RGIC")]
    Rgic,
    #[serde(rename = "SIGL")]
    Sigl,
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
pub struct FinancialInstrument55 {
    #[serde(rename = "Id")]
    pub id: SecurityIdentification25Choice,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none")]
    pub splmtry_id: Option<Max35Text>,
    #[serde(rename = "ClssTp", skip_serializing_if = "Option::is_none")]
    pub clss_tp: Option<Max35Text>,
    #[serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none")]
    pub scties_form: Option<FormOfSecurity1Code>,
    #[serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none")]
    pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
    #[serde(rename = "PdctGrp", skip_serializing_if = "Option::is_none")]
    pub pdct_grp: Option<Max140Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ExternalAccountIdentification1Code {
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
pub struct ActiveCurrencyAnd13DecimalAmount {
    #[serde(rename = "ActiveCurrencyAnd13DecimalAmount")]
    pub value: ActiveCurrencyAnd13DecimalAmountSimpleType,
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
pub struct IndividualPersonIdentification3ChoiceEnum {
    #[serde(rename = "PrsnNm", skip_serializing_if = "Option::is_none")]
    pub prsn_nm: Option<IndividualPerson35>,
    #[serde(rename = "IdNb", skip_serializing_if = "Option::is_none")]
    pub id_nb: Option<GenericIdentification81>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IndividualPersonIdentification3Choice {
    #[serde(flatten)]
    pub value: IndividualPersonIdentification3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NewIssueAllocation2 {
    #[validate]
    #[serde(rename = "Rstrctd")]
    pub rstrctd: YesNoIndicator,
    #[serde(rename = "XmptPrsnRsn", skip_serializing_if = "Option::is_none")]
    pub xmpt_prsn_rsn: Option<Max350Text>,
    #[serde(rename = "DeMnms", skip_serializing_if = "Option::is_none")]
    pub de_mnms: Option<DeMinimus1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification47 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Exact4AlphaNumericText,
    #[validate]
    #[serde(rename = "Issr")]
    pub issr: Max4AlphaNumericText,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max4AlphaNumericText>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RoundingParameters1 {
    #[serde(rename = "RndgMdlus", skip_serializing_if = "Option::is_none")]
    pub rndg_mdlus: Option<DecimalNumber>,
    #[serde(rename = "RndgDrctn")]
    pub rndg_drctn: RoundingDirection1Code,
}
#[derive(
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
pub struct PercentageBoundedRate {
    #[validate(range(min = 0, max = 100,))]
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
pub struct SpanishDomesticInterbankingIdentifier {
    #[validate(regex = "SPANISH_DOMESTIC_INTERBANKING_IDENTIFIER_REGEX")]
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
pub struct AdditionalReference13 {
    #[validate]
    #[serde(rename = "Ref")]
    pub r#ref: Max35Text,
    #[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
    pub ref_issr: Option<PartyIdentification125Choice>,
    #[serde(rename = "MsgNm", skip_serializing_if = "Option::is_none")]
    pub msg_nm: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FatcaStatus1Code {
    #[serde(rename = "F101")]
    F101,
    #[serde(rename = "F102")]
    F102,
    #[serde(rename = "F103")]
    F103,
    #[serde(rename = "F104")]
    F104,
    #[serde(rename = "F105")]
    F105,
    #[serde(rename = "F201")]
    F201,
    #[serde(rename = "F202")]
    F202,
    #[serde(rename = "F203")]
    F203,
    #[serde(rename = "F204")]
    F204,
    #[serde(rename = "F205")]
    F205,
    #[serde(rename = "F206")]
    F206,
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
pub struct TreasuryProfile1 {
    #[validate]
    #[serde(rename = "Dt")]
    pub dt: IsoDate,
    #[serde(rename = "TradrTp")]
    pub tradr_tp: PartyRole5Choice,
    #[validate]
    #[serde(rename = "Rate")]
    pub rate: PercentageRate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Referred1Code {
    #[serde(rename = "REFR")]
    Refr,
    #[serde(rename = "NRFR")]
    Nrfr,
    #[serde(rename = "UKNW")]
    Uknw,
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
pub struct ModificationScope45 {
    #[serde(rename = "ModScpIndctn")]
    pub mod_scp_indctn: DataModification1Code,
    #[validate(length(min = 1,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditiononalInformation13>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LevelOfControl1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<LevelOfControl1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LevelOfControl1Choice {
    #[serde(flatten)]
    pub value: LevelOfControl1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FatcaSourceStatus1Code {
    #[serde(rename = "CALC")]
    Calc,
    #[serde(rename = "DECL")]
    Decl,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FundCashAccount4Code {
    #[serde(rename = "HEDG")]
    Hedg,
    #[serde(rename = "CPFO")]
    Cpfo,
    #[serde(rename = "CPFS")]
    Cpfs,
    #[serde(rename = "SRSA")]
    Srsa,
    #[serde(rename = "CSDO")]
    Csdo,
    #[serde(rename = "TOFF")]
    Toff,
    #[serde(rename = "ICSA")]
    Icsa,
    #[serde(rename = "CSDM")]
    Csdm,
    #[serde(rename = "CSDP")]
    Csdp,
    #[serde(rename = "PPEN")]
    Ppen,
    #[serde(rename = "CPEN")]
    Cpen,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CrsFormType1Code {
    #[serde(rename = "CER4")]
    Cer4,
    #[serde(rename = "CER3")]
    Cer3,
    #[serde(rename = "CER5")]
    Cer5,
    #[serde(rename = "CER6")]
    Cer6,
    #[serde(rename = "CER8")]
    Cer8,
    #[serde(rename = "CER1")]
    Cer1,
    #[serde(rename = "CER2")]
    Cer2,
    #[serde(rename = "CER7")]
    Cer7,
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
pub struct AccountModificationInstructionV08 {
    #[validate]
    #[serde(rename = "MsgId")]
    pub msg_id: MessageIdentification1,
    #[serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none")]
    pub prvs_ref: Option<AdditionalReference13>,
    #[serde(rename = "InstrDtls", skip_serializing_if = "Option::is_none")]
    pub instr_dtls: Option<InvestmentAccountModification4>,
    #[serde(rename = "InvstmtAcctSelctn")]
    pub invstmt_acct_selctn: AccountSelection3Choice,
    #[serde(rename = "ModfdInvstmtAcct", skip_serializing_if = "Option::is_none")]
    pub modfd_invstmt_acct: Option<InvestmentAccount75>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ModfdAcctPties", default)]
    pub modfd_acct_pties: Vec<AccountParties18>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ModfdIntrmies", default)]
    pub modfd_intrmies: Vec<ModificationScope40>,
    #[serde(rename = "ModfdPlcmnt", skip_serializing_if = "Option::is_none")]
    pub modfd_plcmnt: Option<ModificationScope43>,
    #[serde(rename = "ModfdIsseAllcn", skip_serializing_if = "Option::is_none")]
    pub modfd_isse_allcn: Option<ModificationScope21>,
    #[validate(length(min = 0, max = 50,))]
    #[serde(rename = "ModfdSvgsInvstmtPlan", default)]
    pub modfd_svgs_invstmt_plan: Vec<ModificationScope41>,
    #[validate(length(min = 0, max = 10,))]
    #[serde(rename = "ModfdWdrwlInvstmtPlan", default)]
    pub modfd_wdrwl_invstmt_plan: Vec<ModificationScope41>,
    #[validate(length(min = 0, max = 8,))]
    #[serde(rename = "ModfdCshSttlm", default)]
    pub modfd_csh_sttlm: Vec<CashSettlement4>,
    #[validate(length(min = 0, max = 30,))]
    #[serde(rename = "ModfdSvcLvlAgrmt", default)]
    pub modfd_svc_lvl_agrmt: Vec<ModificationScope44>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ModfdAddtlInf", default)]
    pub modfd_addtl_inf: Vec<ModificationScope45>,
    #[serde(rename = "MktPrctcVrsn", skip_serializing_if = "Option::is_none")]
    pub mkt_prctc_vrsn: Option<MarketPracticeVersion1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Xtnsn", default)]
    pub xtnsn: Vec<Extension1>,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestorProfileStatus1Code {
    #[serde(rename = "DISA")]
    Disa,
    #[serde(rename = "DISG")]
    Disg,
    #[serde(rename = "ENAB")]
    Enab,
    #[serde(rename = "ENBG")]
    Enbg,
    #[serde(rename = "ADMI")]
    Admi,
    #[serde(rename = "ANLY")]
    Anly,
    #[serde(rename = "NAPP")]
    Napp,
    #[serde(rename = "PSUS")]
    Psus,
    #[serde(rename = "PEND")]
    Pend,
    #[serde(rename = "SUPS")]
    Sups,
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
pub struct ThirdPartyRights2 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<IsoDateTime>,
    #[serde(rename = "Hldr", skip_serializing_if = "Option::is_none")]
    pub hldr: Option<PartyIdentification125Choice>,
    #[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
    pub lgl_ntty_idr: Option<LeiIdentifier>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InsuranceType2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Insurance1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InsuranceType2Choice {
    #[serde(flatten)]
    pub value: InsuranceType2ChoiceEnum,
}
#[derive(
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
pub struct OrganisationType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OrganisationType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OrganisationType1Choice {
    #[serde(flatten)]
    pub value: OrganisationType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Bloomberg2Identifier {
    #[validate(regex = "BLOOMBERG_2_IDENTIFIER_REGEX")]
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
pub struct KycCheckType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<KnowYourCustomerCheckType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct KycCheckType1Choice {
    #[serde(flatten)]
    pub value: KycCheckType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherIdentification3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PartyIdentificationType7Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherIdentification3Choice {
    #[serde(flatten)]
    pub value: OtherIdentification3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct UkDomesticSortCodeIdentifier {
    #[validate(regex = "UK_DOMESTIC_SORT_CODE_IDENTIFIER_REGEX")]
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ValorenIdentifier {
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
pub struct Account23 {
    #[validate]
    #[serde(rename = "AcctId")]
    pub acct_id: Max35Text,
    #[serde(rename = "RltdAcctDtls", skip_serializing_if = "Option::is_none")]
    pub rltd_acct_dtls: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictionStatus1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RestrictionStatus1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RestrictionStatus1Choice {
    #[serde(flatten)]
    pub value: RestrictionStatus1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ProfileType1Code {
    #[serde(rename = "HEDG")]
    Hedg,
    #[serde(rename = "HFTR")]
    Hftr,
    #[serde(rename = "MAKE")]
    Make,
    #[serde(rename = "TREA")]
    Trea,
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
pub struct DateAndDateTime1ChoiceEnum {
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
pub struct DateAndDateTime1Choice {
    #[serde(flatten)]
    pub value: DateAndDateTime1ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum IncomePreference2Code {
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "SECU")]
    Secu,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FundIntention1Code {
    #[serde(rename = "YQUA")]
    Yqua,
    #[serde(rename = "NQUA")]
    Nqua,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InformationDistribution2Code {
    #[serde(rename = "ELEC")]
    Elec,
    #[serde(rename = "NONE")]
    None,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountType2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FundCashAccount4Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountType2Choice {
    #[serde(flatten)]
    pub value: AccountType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashSettlement4 {
    #[serde(rename = "ModScpIndctn")]
    pub mod_scp_indctn: DataModification2Code,
    #[validate(length(min = 0,))]
    #[serde(rename = "CshAcctDtls", default)]
    pub csh_acct_dtls: Vec<CashAccount204>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrCshSttlmDtls", default)]
    pub othr_csh_sttlm_dtls: Vec<PaymentInstrument17>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IndividualPerson35 {
    #[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
    pub gvn_nm: Option<Max35Text>,
    #[serde(rename = "MddlNm", skip_serializing_if = "Option::is_none")]
    pub mddl_nm: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "Gndr", skip_serializing_if = "Option::is_none")]
    pub gndr: Option<Gender1Code>,
    #[serde(rename = "BirthDt", skip_serializing_if = "Option::is_none")]
    pub birth_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CrsSource1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CrsSourceStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CrsSource1Choice {
    #[serde(flatten)]
    pub value: CrsSource1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GdprData1 {
    #[serde(rename = "CnsntTp")]
    pub cnsnt_tp: GdprDataConsent1Choice,
    #[validate]
    #[serde(rename = "CnsntInd")]
    pub cnsnt_ind: YesNoIndicator,
    #[validate]
    #[serde(rename = "CnsntDt")]
    pub cnsnt_dt: IsoDate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrument87 {
    #[serde(rename = "Id")]
    pub id: SecurityIdentification25Choice,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none")]
    pub splmtry_id: Option<Max35Text>,
    #[serde(rename = "ClssTp", skip_serializing_if = "Option::is_none")]
    pub clss_tp: Option<Max35Text>,
    #[serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none")]
    pub scties_form: Option<FormOfSecurity1Code>,
    #[serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none")]
    pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
    #[serde(rename = "PdctGrp", skip_serializing_if = "Option::is_none")]
    pub pdct_grp: Option<Max140Text>,
    #[serde(rename = "BlckdHldgDtls", skip_serializing_if = "Option::is_none")]
    pub blckd_hldg_dtls: Option<BlockedHoldingDetails2>,
    #[serde(rename = "Pldgg", skip_serializing_if = "Option::is_none")]
    pub pldgg: Option<Eligible1Code>,
    #[serde(rename = "Coll", skip_serializing_if = "Option::is_none")]
    pub coll: Option<Collateral1Code>,
    #[serde(rename = "ThrdPtyRghts", skip_serializing_if = "Option::is_none")]
    pub thrd_pty_rghts: Option<ThirdPartyRights2>,
    #[serde(rename = "FndOwnrsh", skip_serializing_if = "Option::is_none")]
    pub fnd_ownrsh: Option<FundOwnership1Code>,
    #[serde(rename = "FndIntntn", skip_serializing_if = "Option::is_none")]
    pub fnd_intntn: Option<FundIntention1Code>,
    #[serde(rename = "OprlSts", skip_serializing_if = "Option::is_none")]
    pub oprl_sts: Option<OperationalStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GermanBankleitzahlIdentifier {
    #[validate(regex = "GERMAN_BANKLEITZAHL_IDENTIFIER_REGEX")]
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
pub struct HongKongBankIdentifier {
    #[validate(regex = "HONG_KONG_BANK_IDENTIFIER_REGEX")]
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
pub struct AccountStatusUpdateInstruction1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AccountStatusUpdateInstruction1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountStatusUpdateInstruction1Choice {
    #[serde(flatten)]
    pub value: AccountStatusUpdateInstruction1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementFrequency1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EventFrequency10Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementFrequency1Choice {
    #[serde(flatten)]
    pub value: SettlementFrequency1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyIdentificationType7Code {
    #[serde(rename = "ATIN")]
    Atin,
    #[serde(rename = "IDCD")]
    Idcd,
    #[serde(rename = "NRIN")]
    Nrin,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "PASS")]
    Pass,
    #[serde(rename = "POCD")]
    Pocd,
    #[serde(rename = "SOCS")]
    Socs,
    #[serde(rename = "SRSA")]
    Srsa,
    #[serde(rename = "GUNL")]
    Gunl,
    #[serde(rename = "GTIN")]
    Gtin,
    #[serde(rename = "ITIN")]
    Itin,
    #[serde(rename = "CPFA")]
    Cpfa,
    #[serde(rename = "AREG")]
    Areg,
    #[serde(rename = "DRLC")]
    Drlc,
    #[serde(rename = "EMID")]
    Emid,
    #[serde(rename = "NINV")]
    Ninv,
    #[serde(rename = "INCL")]
    Incl,
    #[serde(rename = "GIIN")]
    Giin,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GdprDataConsent1Code {
    #[serde(rename = "DP00")]
    Dp00,
    #[serde(rename = "DP03")]
    Dp03,
    #[serde(rename = "DP01")]
    Dp01,
    #[serde(rename = "DP02")]
    Dp02,
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
pub struct PaymentCard29 {
    #[serde(rename = "Tp")]
    pub tp: CardType1Code,
    #[validate]
    #[serde(rename = "Nb")]
    pub nb: Max35Text,
    #[validate]
    #[serde(rename = "HldrNm")]
    pub hldr_nm: Max35Text,
    #[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
    pub start_dt: Option<IsoYearMonth>,
    #[validate]
    #[serde(rename = "XpryDt")]
    pub xpry_dt: IsoYearMonth,
    #[serde(rename = "CardIssrNm", skip_serializing_if = "Option::is_none")]
    pub card_issr_nm: Option<Max35Text>,
    #[serde(rename = "CardIssrId", skip_serializing_if = "Option::is_none")]
    pub card_issr_id: Option<PartyIdentification125Choice>,
    #[serde(rename = "SctyCd", skip_serializing_if = "Option::is_none")]
    pub scty_cd: Option<Max35Text>,
    #[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<Max3Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OperationalStatus1Code {
    #[serde(rename = "ENAB")]
    Enab,
    #[serde(rename = "SPEC")]
    Spec,
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
pub struct HighFrequencyTradingProfile1 {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[serde(rename = "SttlmFrqcy", skip_serializing_if = "Option::is_none")]
    pub sttlm_frqcy: Option<SettlementFrequency1Choice>,
    #[serde(rename = "CnsldtnTp", skip_serializing_if = "Option::is_none")]
    pub cnsldtn_tp: Option<ConsolidationType1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct QuickIdentifier {
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
pub struct Repartition6 {
    #[serde(rename = "Qty")]
    pub qty: UnitsOrAmountOrPercentage1Choice,
    #[validate]
    #[serde(rename = "FinInstrm")]
    pub fin_instrm: FinancialInstrument87,
    #[serde(rename = "CcyOfPlan", skip_serializing_if = "Option::is_none")]
    pub ccy_of_plan: Option<ActiveOrHistoricCurrencyCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PoliticalExposureType2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PoliticalExposureType2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PoliticalExposureType2Choice {
    #[serde(flatten)]
    pub value: PoliticalExposureType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FedwireRoutingNumberIdentifier {
    #[validate(regex = "FEDWIRE_ROUTING_NUMBER_IDENTIFIER_REGEX")]
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
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
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
pub struct ConsolidatedTapeAssociationIdentifier {
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
pub struct AccountingStatus1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AccountingStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountingStatus1Choice {
    #[serde(flatten)]
    pub value: AccountingStatus1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PoliticallyExposedPerson1 {
    #[serde(rename = "PltclyXpsdPrsnTp")]
    pub pltcly_xpsd_prsn_tp: PoliticalExposureType2Choice,
    #[serde(rename = "PltclyXpsdPrsnSts", skip_serializing_if = "Option::is_none")]
    pub pltcly_xpsd_prsn_sts: Option<PoliticallyExposedPersonStatus1Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CivilStatus1Code {
    #[serde(rename = "DIVO")]
    Divo,
    #[serde(rename = "LDIV")]
    Ldiv,
    #[serde(rename = "MARR")]
    Marr,
    #[serde(rename = "SEPA")]
    Sepa,
    #[serde(rename = "SING")]
    Sing,
    #[serde(rename = "UNIO")]
    Unio,
    #[serde(rename = "WIDO")]
    Wido,
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
pub struct BlockedStatusReason2ChoiceEnum {
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<BlockedStatusReason2>,
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BlockedStatusReason2Choice {
    #[serde(flatten)]
    pub value: BlockedStatusReason2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PlanStatus1Code {
    #[serde(rename = "ACTV")]
    Actv,
    #[serde(rename = "CLOS")]
    Clos,
    #[serde(rename = "SUSP")]
    Susp,
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
pub struct LetterIntent1 {
    #[validate]
    #[serde(rename = "LttrInttRef")]
    pub lttr_intt_ref: Max35Text,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
    pub start_dt: Option<IsoDate>,
    #[serde(rename = "EndDt", skip_serializing_if = "Option::is_none")]
    pub end_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max34Text {
    #[validate(length(min = 1, max = 34,))]
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
pub struct Account32 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "AcctSvcr")]
    pub acct_svcr: PartyIdentification125Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CanadianPaymentsArnIdentifier {
    #[validate(regex = "CANADIAN_PAYMENTS_ARN_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CreditDebit3Code {
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "DBIT")]
    Dbit,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EventFrequency1Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "SEMI")]
    Semi,
    #[serde(rename = "QUTR")]
    Qutr,
    #[serde(rename = "TOMN")]
    Tomn,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "TWMN")]
    Twmn,
    #[serde(rename = "TOWK")]
    Towk,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "ADHO")]
    Adho,
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "OVNG")]
    Ovng,
    #[serde(rename = "ONDE")]
    Onde,
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
pub struct AccountStatusUpdateInstructionReason2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AccountStatusUpdateRequestReason1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountStatusUpdateInstructionReason2Choice {
    #[serde(flatten)]
    pub value: AccountStatusUpdateInstructionReason2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeMinimus1ChoiceEnum {
    #[serde(rename = "DeMnmsNotAplbl", skip_serializing_if = "Option::is_none")]
    pub de_mnms_not_aplbl: Option<DeMinimusNotApplicable1>,
    #[serde(rename = "DeMnmsAplbl", skip_serializing_if = "Option::is_none")]
    pub de_mnms_aplbl: Option<DeMinimusApplicable1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DeMinimus1Choice {
    #[serde(flatten)]
    pub value: DeMinimus1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Intermediary47 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification125Choice,
    #[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
    pub lgl_ntty_idr: Option<LeiIdentifier>,
    #[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
    pub acct: Option<Account32>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestorProfileStatus1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestorProfileStatus1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestorProfileStatus1Choice {
    #[serde(flatten)]
    pub value: InvestorProfileStatus1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LevelOfControl1Code {
    #[serde(rename = "TRAN")]
    Tran,
    #[serde(rename = "VIEW")]
    View,
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
pub struct ReferredAgent3 {
    #[serde(rename = "Rfrd")]
    pub rfrd: Referred1Code,
    #[serde(rename = "RfrdPlcmntAgt", skip_serializing_if = "Option::is_none")]
    pub rfrd_plcmnt_agt: Option<PartyIdentification125Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SwissBcIdentifier {
    #[validate(regex = "SWISS_BC_IDENTIFIER_REGEX")]
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
pub struct PartyProfileInformation5 {
    #[serde(rename = "CertfctnInd", skip_serializing_if = "Option::is_none")]
    pub certfctn_ind: Option<YesNoIndicator>,
    #[serde(rename = "VldtngPty", skip_serializing_if = "Option::is_none")]
    pub vldtng_pty: Option<Max140Text>,
    #[serde(rename = "ChckngPty", skip_serializing_if = "Option::is_none")]
    pub chckng_pty: Option<Max140Text>,
    #[serde(rename = "RspnsblPty", skip_serializing_if = "Option::is_none")]
    pub rspnsbl_pty: Option<Max140Text>,
    #[serde(rename = "CertTp", skip_serializing_if = "Option::is_none")]
    pub cert_tp: Option<CertificationType1Choice>,
    #[serde(rename = "ChckngDt", skip_serializing_if = "Option::is_none")]
    pub chckng_dt: Option<IsoDate>,
    #[serde(rename = "ChckngFrqcy", skip_serializing_if = "Option::is_none")]
    pub chckng_frqcy: Option<EventFrequency1Code>,
    #[serde(rename = "NxtRvsnDt", skip_serializing_if = "Option::is_none")]
    pub nxt_rvsn_dt: Option<IsoDate>,
    #[serde(rename = "SlryRg", skip_serializing_if = "Option::is_none")]
    pub slry_rg: Option<Max35Text>,
    #[serde(rename = "SrcOfWlth", skip_serializing_if = "Option::is_none")]
    pub src_of_wlth: Option<Max140Text>,
    #[serde(rename = "CstmrCndctClssfctn", skip_serializing_if = "Option::is_none")]
    pub cstmr_cndct_clssfctn: Option<CustomerConductClassification1Choice>,
    #[serde(rename = "RskLvl", skip_serializing_if = "Option::is_none")]
    pub rsk_lvl: Option<RiskLevel2Choice>,
    #[serde(
        rename = "KnowYourCstmrChckTp",
        skip_serializing_if = "Option::is_none"
    )]
    pub know_your_cstmr_chck_tp: Option<KycCheckType1Choice>,
    #[serde(
        rename = "KnowYourCstmrDBChck",
        skip_serializing_if = "Option::is_none"
    )]
    pub know_your_cstmr_db_chck: Option<DataBaseCheck1>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DataModification1Code {
    #[serde(rename = "INSE")]
    Inse,
    #[serde(rename = "UPDT")]
    Updt,
    #[serde(rename = "DELT")]
    Delt,
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
pub struct AccountStatusUpdateInstructionReason1 {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AccountStatusUpdateInstructionReason2Choice>,
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
pub struct DutchIdentifier {
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
pub struct RegisteredShareholderName1ChoiceEnum {
    #[serde(rename = "Org", skip_serializing_if = "Option::is_none")]
    pub org: Option<Organisation23>,
    #[serde(rename = "IndvPrsn", skip_serializing_if = "Option::is_none")]
    pub indv_prsn: Option<IndividualPerson29>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RegisteredShareholderName1Choice {
    #[serde(flatten)]
    pub value: RegisteredShareholderName1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Liability1Code {
    #[serde(rename = "INVE")]
    Inve,
    #[serde(rename = "BROK")]
    Brok,
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
pub struct PartyIdentification125ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification125Choice {
    #[serde(flatten)]
    pub value: PartyIdentification125ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Holding1Code {
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "NPRH")]
    Nprh,
    #[serde(rename = "PRTH")]
    Prth,
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
pub struct CashAccount204 {
    #[serde(rename = "SttlmCcy")]
    pub sttlm_ccy: ActiveCurrencyCode,
    #[validate]
    #[serde(rename = "Id")]
    pub id: AccountIdentificationAndName5,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<PartyIdentification125Choice>,
    #[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
    pub acct_svcr: Option<FinancialInstitutionIdentification11Choice>,
    #[serde(rename = "AcctSvcrBrnch", skip_serializing_if = "Option::is_none")]
    pub acct_svcr_brnch: Option<BranchData4>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AcctOwnrOthrId", default)]
    pub acct_ownr_othr_id: Vec<GenericIdentification82>,
    #[serde(rename = "InvstmtAcctTp", skip_serializing_if = "Option::is_none")]
    pub invstmt_acct_tp: Option<AccountType2Choice>,
    #[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
    pub cdt_dbt: Option<CreditDebit3Code>,
    #[serde(rename = "SttlmInstrRsn", skip_serializing_if = "Option::is_none")]
    pub sttlm_instr_rsn: Option<SettlementInstructionReason1Choice>,
    #[serde(rename = "CshAcctPurp", skip_serializing_if = "Option::is_none")]
    pub csh_acct_purp: Option<CashAccountType3Choice>,
    #[serde(rename = "CshAcctDsgnt", skip_serializing_if = "Option::is_none")]
    pub csh_acct_dsgnt: Option<AccountDesignation1Choice>,
    #[serde(rename = "DvddPctg", skip_serializing_if = "Option::is_none")]
    pub dvdd_pctg: Option<PercentageBoundedRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProfileType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ProfileType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ProfileType1Choice {
    #[serde(flatten)]
    pub value: ProfileType1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MailType1Code {
    #[serde(rename = "AIRM")]
    Airm,
    #[serde(rename = "ORDM")]
    Ordm,
    #[serde(rename = "REGM")]
    Regm,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CommunicationMethod1Code {
    #[serde(rename = "SWMT")]
    Swmt,
    #[serde(rename = "SWMX")]
    Swmx,
    #[serde(rename = "FAXI")]
    Faxi,
    #[serde(rename = "EMAL")]
    Emal,
    #[serde(rename = "PROP")]
    Prop,
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
pub struct TickerIdentifier {
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
pub struct Max4AlphaNumericText {
    #[validate(length(min = 1, max = 4,), regex = "MAX_4_ALPHA_NUMERIC_TEXT_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TransactionChannel2Code {
    #[serde(rename = "FIAD")]
    Fiad,
    #[serde(rename = "HOBA")]
    Hoba,
    #[serde(rename = "BRAN")]
    Bran,
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
pub struct InvestmentAccount75 {
    #[serde(rename = "AcctStsUpdInstr", skip_serializing_if = "Option::is_none")]
    pub acct_sts_upd_instr: Option<AccountStatusUpdateInstruction1>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[serde(rename = "Dsgnt", skip_serializing_if = "Option::is_none")]
    pub dsgnt: Option<Max35Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<AccountType2Choice>,
    #[serde(rename = "OwnrshTp", skip_serializing_if = "Option::is_none")]
    pub ownrsh_tp: Option<OwnershipType2Choice>,
    #[serde(rename = "TaxXmptn", skip_serializing_if = "Option::is_none")]
    pub tax_xmptn: Option<TaxExemptionReason2Choice>,
    #[serde(rename = "StmtFrqcy", skip_serializing_if = "Option::is_none")]
    pub stmt_frqcy: Option<StatementFrequencyReason2Choice>,
    #[serde(rename = "RefCcy", skip_serializing_if = "Option::is_none")]
    pub ref_ccy: Option<ActiveCurrencyCode>,
    #[serde(rename = "Lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageCode>,
    #[serde(rename = "IncmPref", skip_serializing_if = "Option::is_none")]
    pub incm_pref: Option<IncomePreference2Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RinvstmtDtls", default)]
    pub rinvstmt_dtls: Vec<Reinvestment4>,
    #[serde(rename = "TaxWhldgMtd", skip_serializing_if = "Option::is_none")]
    pub tax_whldg_mtd: Option<TaxWithholdingMethod3Code>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxRptg", default)]
    pub tax_rptg: Vec<TaxReporting3>,
    #[serde(rename = "LttrInttDtls", skip_serializing_if = "Option::is_none")]
    pub lttr_intt_dtls: Option<LetterIntent1>,
    #[serde(rename = "AcmltnRghtRef", skip_serializing_if = "Option::is_none")]
    pub acmltn_rght_ref: Option<Max35Text>,
    #[serde(rename = "ReqrdSgntriesNb", skip_serializing_if = "Option::is_none")]
    pub reqrd_sgntries_nb: Option<Number>,
    #[serde(rename = "FndFmlyNm", skip_serializing_if = "Option::is_none")]
    pub fnd_fmly_nm: Option<Max350Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ModfdFinInstrmDtls", default)]
    pub modfd_fin_instrm_dtls: Vec<ModificationScope42>,
    #[serde(rename = "RndgDtls", skip_serializing_if = "Option::is_none")]
    pub rndg_dtls: Option<RoundingParameters1>,
    #[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
    pub acct_svcr: Option<PartyIdentification125Choice>,
    #[serde(rename = "BlckdSts", skip_serializing_if = "Option::is_none")]
    pub blckd_sts: Option<BlockedStatusReason2Choice>,
    #[serde(rename = "AcctUsgTp", skip_serializing_if = "Option::is_none")]
    pub acct_usg_tp: Option<AccountUsageType2Choice>,
    #[serde(rename = "FrgnStsCertfctn", skip_serializing_if = "Option::is_none")]
    pub frgn_sts_certfctn: Option<Provided1Code>,
    #[serde(rename = "AcctSgntrDtTm", skip_serializing_if = "Option::is_none")]
    pub acct_sgntr_dt_tm: Option<DateAndDateTime1Choice>,
    #[serde(rename = "TxChanlTp", skip_serializing_if = "Option::is_none")]
    pub tx_chanl_tp: Option<TransactionChannelType1Choice>,
    #[serde(rename = "InvstmtAcctCtgy", skip_serializing_if = "Option::is_none")]
    pub invstmt_acct_ctgy: Option<InvestmentAccountCategory1Choice>,
    #[serde(rename = "Pldgg", skip_serializing_if = "Option::is_none")]
    pub pldgg: Option<Eligible1Code>,
    #[serde(rename = "Coll", skip_serializing_if = "Option::is_none")]
    pub coll: Option<Collateral1Code>,
    #[serde(rename = "ThrdPtyRghts", skip_serializing_if = "Option::is_none")]
    pub thrd_pty_rghts: Option<ThirdPartyRights2>,
    #[serde(
        rename = "PwrOfAttnyLvlOfCtrl",
        skip_serializing_if = "Option::is_none"
    )]
    pub pwr_of_attny_lvl_of_ctrl: Option<LevelOfControl1Choice>,
    #[serde(rename = "AcctgSts", skip_serializing_if = "Option::is_none")]
    pub acctg_sts: Option<AccountingStatus1Choice>,
    #[serde(rename = "OpngDt", skip_serializing_if = "Option::is_none")]
    pub opng_dt: Option<DateAndDateTime1Choice>,
    #[serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none")]
    pub clsg_dt: Option<DateAndDateTime1Choice>,
    #[serde(rename = "NegInd", skip_serializing_if = "Option::is_none")]
    pub neg_ind: Option<YesNoIndicator>,
    #[serde(rename = "PrcgOrdr", skip_serializing_if = "Option::is_none")]
    pub prcg_ordr: Option<PositionEffect3Code>,
    #[serde(rename = "Lblty", skip_serializing_if = "Option::is_none")]
    pub lblty: Option<Liability1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ModfdInvstrPrfl", default)]
    pub modfd_invstr_prfl: Vec<ModificationScope46>,
    #[serde(rename = "FsclYr", skip_serializing_if = "Option::is_none")]
    pub fscl_yr: Option<FiscalYear1Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxExemptReason3Code {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "MASA")]
    Masa,
    #[serde(rename = "MISA")]
    Misa,
    #[serde(rename = "SISA")]
    Sisa,
    #[serde(rename = "IISA")]
    Iisa,
    #[serde(rename = "CUYP")]
    Cuyp,
    #[serde(rename = "PRYP")]
    Pryp,
    #[serde(rename = "ASTR")]
    Astr,
    #[serde(rename = "EMPY")]
    Empy,
    #[serde(rename = "EMCY")]
    Emcy,
    #[serde(rename = "EPRY")]
    Epry,
    #[serde(rename = "ECYE")]
    Ecye,
    #[serde(rename = "NFPI")]
    Nfpi,
    #[serde(rename = "NFQP")]
    Nfqp,
    #[serde(rename = "DECP")]
    Decp,
    #[serde(rename = "IRAC")]
    Irac,
    #[serde(rename = "IRAR")]
    Irar,
    #[serde(rename = "KEOG")]
    Keog,
    #[serde(rename = "PFSP")]
    Pfsp,
    #[serde(rename = "401K")]
    X401K,
    #[serde(rename = "SIRA")]
    Sira,
    #[serde(rename = "403B")]
    X403B,
    #[serde(rename = "457X")]
    X457X,
    #[serde(rename = "RIRA")]
    Rira,
    #[serde(rename = "RIAN")]
    Rian,
    #[serde(rename = "RCRF")]
    Rcrf,
    #[serde(rename = "RCIP")]
    Rcip,
    #[serde(rename = "EIFP")]
    Eifp,
    #[serde(rename = "EIOP")]
    Eiop,
    #[serde(rename = "FORE")]
    Fore,
    #[serde(rename = "INCA")]
    Inca,
    #[serde(rename = "MINO")]
    Mino,
    #[serde(rename = "ASSO")]
    Asso,
    #[serde(rename = "DIPL")]
    Dipl,
    #[serde(rename = "DOME")]
    Dome,
    #[serde(rename = "FORP")]
    Forp,
    #[serde(rename = "ORDR")]
    Ordr,
    #[serde(rename = "PENF")]
    Penf,
    #[serde(rename = "REFU")]
    Refu,
    #[serde(rename = "RIHO")]
    Riho,
    #[serde(rename = "ADMI")]
    Admi,
    #[serde(rename = "TANR")]
    Tanr,
    #[serde(rename = "OANR")]
    Oanr,
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
pub struct ModificationScope46 {
    #[serde(rename = "ModScpIndctn")]
    pub mod_scp_indctn: DataModification1Code,
    #[validate]
    #[serde(rename = "InvstrPrfl")]
    pub invstr_prfl: InvestorProfile2,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RiskLevel1Code {
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "LOWW")]
    Loww,
    #[serde(rename = "MEDM")]
    Medm,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RoundingDirection1Code {
    #[serde(rename = "RDUP")]
    Rdup,
    #[serde(rename = "RDWN")]
    Rdwn,
    #[serde(rename = "STAN")]
    Stan,
    #[serde(rename = "DIST")]
    Dist,
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
pub struct AccountIdentificationAndName5 {
    #[serde(rename = "Id")]
    pub id: AccountIdentification4Choice,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentInstrument24ChoiceEnum {
    #[serde(rename = "DrctDbtDtls", skip_serializing_if = "Option::is_none")]
    pub drct_dbt_dtls: Option<DirectDebitMandate7>,
    #[serde(rename = "Chq", skip_serializing_if = "Option::is_none")]
    pub chq: Option<YesNoIndicator>,
    #[serde(rename = "PmtCardDtls", skip_serializing_if = "Option::is_none")]
    pub pmt_card_dtls: Option<PaymentCard29>,
    #[serde(rename = "BkrsDrft", skip_serializing_if = "Option::is_none")]
    pub bkrs_drft: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentInstrument24Choice {
    #[serde(flatten)]
    pub value: PaymentInstrument24ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AccountStatusUpdateInstruction1Code {
    #[serde(rename = "CLOS")]
    Clos,
    #[serde(rename = "REAC")]
    Reac,
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
pub struct AccountIdentification4ChoiceEnum {
    #[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
    pub iban: Option<Iban2007Identifier>,
    #[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericAccountIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountIdentification4Choice {
    #[serde(flatten)]
    pub value: AccountIdentification4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ConductClassification1Code {
    #[serde(rename = "NSTA")]
    Nsta,
    #[serde(rename = "RCLT")]
    Rclt,
    #[serde(rename = "STAN")]
    Stan,
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
pub struct Intermediary46 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification177Choice,
    #[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
    pub lgl_ntty_idr: Option<LeiIdentifier>,
    #[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
    pub acct: Option<Account32>,
    #[serde(rename = "WvdTrlrComssnInd", skip_serializing_if = "Option::is_none")]
    pub wvd_trlr_comssn_ind: Option<YesNoIndicator>,
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<PartyRole2Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PmryComAdr", default)]
    pub pmry_com_adr: Vec<CommunicationAddress6>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ScndryComAdr", default)]
    pub scndry_com_adr: Vec<CommunicationAddress6>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MiFidClassification1 {
    #[serde(rename = "Clssfctn")]
    pub clssfctn: OrderOriginatorEligibility1Code,
    #[serde(rename = "Nrrtv", skip_serializing_if = "Option::is_none")]
    pub nrrtv: Option<Max350Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CrsStatus1Code {
    #[serde(rename = "C101")]
    C101,
    #[serde(rename = "C102")]
    C102,
    #[serde(rename = "C103")]
    C103,
    #[serde(rename = "C104")]
    C104,
    #[serde(rename = "C105")]
    C105,
    #[serde(rename = "C106")]
    C106,
    #[serde(rename = "C107")]
    C107,
    #[serde(rename = "C108")]
    C108,
    #[serde(rename = "C109")]
    C109,
    #[serde(rename = "C110")]
    C110,
    #[serde(rename = "C111")]
    C111,
    #[serde(rename = "C112")]
    C112,
    #[serde(rename = "C113")]
    C113,
    #[serde(rename = "C114")]
    C114,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CrsSourceStatus1Code {
    #[serde(rename = "CALC")]
    Calc,
    #[serde(rename = "DECL")]
    Decl,
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
pub struct ExtensiveBranchNetworkIdentifier {
    #[validate(regex = "EXTENSIVE_BRANCH_NETWORK_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EventFrequency9Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "SEMI")]
    Semi,
    #[serde(rename = "QUTR")]
    Qutr,
    #[serde(rename = "TOMN")]
    Tomn,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "TWMN")]
    Twmn,
    #[serde(rename = "TOWK")]
    Towk,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "ADHO")]
    Adho,
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "OVNG")]
    Ovng,
    #[serde(rename = "ONDE")]
    Onde,
    #[serde(rename = "NONE")]
    None,
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
pub struct GenericIdentification82 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp")]
    pub tp: OtherIdentification3Choice,
    #[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
    #[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
    pub isse_dt: Option<IsoDate>,
    #[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
    pub xpry_dt: Option<IsoDate>,
    #[serde(rename = "Stat", skip_serializing_if = "Option::is_none")]
    pub stat: Option<Max70Text>,
    #[serde(rename = "IssrCtry", skip_serializing_if = "Option::is_none")]
    pub issr_ctry: Option<CountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Max3Text {
    #[validate(length(min = 1, max = 3,))]
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
pub struct DocumentToSend4 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: Max140Text,
    #[serde(rename = "Rcpt")]
    pub rcpt: PartyIdentification125Choice,
    #[serde(rename = "MtdOfTrnsmssn")]
    pub mtd_of_trnsmssn: CommunicationMethod3Choice,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BranchData4 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress1>,
}
#[derive(
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
pub struct IsoYearMonth {
    #[validate(regex = "ISO_YEAR_MONTH_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestmentAccountCategory1Code {
    #[serde(rename = "MAND")]
    Mand,
    #[serde(rename = "RETA")]
    Reta,
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
pub struct SedolIdentifier {
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DataModification2Code {
    #[serde(rename = "INSE")]
    Inse,
    #[serde(rename = "DELT")]
    Delt,
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
pub struct CusipIdentifier {
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
pub struct MailType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MailType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MailType1Choice {
    #[serde(flatten)]
    pub value: MailType1ChoiceEnum,
}
#[derive(
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
pub enum FatcaFormType1Code {
    #[serde(rename = "CER5")]
    Cer5,
    #[serde(rename = "CER7")]
    Cer7,
    #[serde(rename = "CER1")]
    Cer1,
    #[serde(rename = "CER2")]
    Cer2,
    #[serde(rename = "CER3")]
    Cer3,
    #[serde(rename = "CER4")]
    Cer4,
    #[serde(rename = "CER6")]
    Cer6,
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
pub struct TaxReporting3 {
    #[serde(rename = "TaxtnCtry")]
    pub taxtn_ctry: CountryCode,
    #[serde(rename = "TaxRate", skip_serializing_if = "Option::is_none")]
    pub tax_rate: Option<PercentageRate>,
    #[serde(rename = "TaxPyer", skip_serializing_if = "Option::is_none")]
    pub tax_pyer: Option<PartyIdentification125Choice>,
    #[serde(rename = "TaxRcpt", skip_serializing_if = "Option::is_none")]
    pub tax_rcpt: Option<PartyIdentification125Choice>,
    #[serde(rename = "CshAcctDtls", skip_serializing_if = "Option::is_none")]
    pub csh_acct_dtls: Option<CashAccount204>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementInstructionReason1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementInstructionReason1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementInstructionReason1Choice {
    #[serde(flatten)]
    pub value: SettlementInstructionReason1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountOwner3ChoiceEnum {
    #[serde(rename = "IndvOwnrId", skip_serializing_if = "Option::is_none")]
    pub indv_ownr_id: Option<IndividualPersonIdentification3Choice>,
    #[serde(rename = "OrgOwnrId", skip_serializing_if = "Option::is_none")]
    pub org_ownr_id: Option<PartyIdentification220>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountOwner3Choice {
    #[serde(flatten)]
    pub value: AccountOwner3ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum KnowYourCustomerCheckType1Code {
    #[serde(rename = "ENHA")]
    Enha,
    #[serde(rename = "ORDN")]
    Ordn,
    #[serde(rename = "SIMP")]
    Simp,
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
pub struct ModificationScope44 {
    #[serde(rename = "ModScpIndctn")]
    pub mod_scp_indctn: DataModification1Code,
    #[validate]
    #[serde(rename = "SvcLvlAgrmt")]
    pub svc_lvl_agrmt: DocumentToSend4,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EventFrequency8Code {
    #[serde(rename = "ADHO")]
    Adho,
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "FOMN")]
    Fomn,
    #[serde(rename = "TOMN")]
    Tomn,
    #[serde(rename = "TOWK")]
    Towk,
    #[serde(rename = "TYEA")]
    Tyea,
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "ONDE")]
    Onde,
    #[serde(rename = "OVNG")]
    Ovng,
    #[serde(rename = "QUTR")]
    Qutr,
    #[serde(rename = "SEMI")]
    Semi,
    #[serde(rename = "TWMN")]
    Twmn,
    #[serde(rename = "WEEK")]
    Week,
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
pub struct SicovamIdentifier {
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
pub struct MessageIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[validate]
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: IsoDateTime,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestorProfile2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<ProfileType1Choice>,
    #[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
    pub sts: Option<InvestorProfileStatus1Choice>,
    #[serde(rename = "Trsr", skip_serializing_if = "Option::is_none")]
    pub trsr: Option<TreasuryProfile1>,
    #[serde(rename = "HghFrqcyTradg", skip_serializing_if = "Option::is_none")]
    pub hgh_frqcy_tradg: Option<HighFrequencyTradingProfile1>,
    #[serde(rename = "MktMakr", skip_serializing_if = "Option::is_none")]
    pub mkt_makr: Option<MarketMakerProfile2>,
}
#[derive(
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
pub struct NewZealandNccIdentifier {
    #[validate(regex = "NEW_ZEALAND_NCC_IDENTIFIER_REGEX")]
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
pub struct ModificationScope21 {
    #[serde(rename = "ModScpIndctn")]
    pub mod_scp_indctn: DataModification1Code,
    #[validate]
    #[serde(rename = "IsseAllcn")]
    pub isse_allcn: NewIssueAllocation2,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxExemptionReason2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxExemptReason3Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxExemptionReason2Choice {
    #[serde(flatten)]
    pub value: TaxExemptionReason2ChoiceEnum,
}
#[derive(
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
pub struct ClearingSystemMemberIdentification4ChoiceEnum {
    #[serde(rename = "ITNCC", skip_serializing_if = "Option::is_none")]
    pub itncc: Option<ItalianDomesticIdentifier>,
    #[serde(rename = "ESNCC", skip_serializing_if = "Option::is_none")]
    pub esncc: Option<SpanishDomesticInterbankingIdentifier>,
    #[serde(rename = "ATBLZ", skip_serializing_if = "Option::is_none")]
    pub atblz: Option<AustrianBankleitzahlIdentifier>,
    #[serde(rename = "GBSC", skip_serializing_if = "Option::is_none")]
    pub gbsc: Option<UkDomesticSortCodeIdentifier>,
    #[serde(rename = "IENSC", skip_serializing_if = "Option::is_none")]
    pub iensc: Option<IrishNscIdentifier>,
    #[serde(rename = "CHBC", skip_serializing_if = "Option::is_none")]
    pub chbc: Option<SwissBcIdentifier>,
    #[serde(rename = "PTNCC", skip_serializing_if = "Option::is_none")]
    pub ptncc: Option<PortugueseNccIdentifier>,
    #[serde(rename = "RUCB", skip_serializing_if = "Option::is_none")]
    pub rucb: Option<RussianCentralBankIdentificationCodeIdentifier>,
    #[serde(rename = "USCHU", skip_serializing_if = "Option::is_none")]
    pub uschu: Option<ChipsUniversalIdentifier>,
    #[serde(rename = "DEBLZ", skip_serializing_if = "Option::is_none")]
    pub deblz: Option<GermanBankleitzahlIdentifier>,
    #[serde(rename = "USCH", skip_serializing_if = "Option::is_none")]
    pub usch: Option<ChipsParticipantIdentifier>,
    #[serde(rename = "HKNCC", skip_serializing_if = "Option::is_none")]
    pub hkncc: Option<HongKongBankIdentifier>,
    #[serde(rename = "CHSIC", skip_serializing_if = "Option::is_none")]
    pub chsic: Option<SwissSicIdentifier>,
    #[serde(rename = "NZNCC", skip_serializing_if = "Option::is_none")]
    pub nzncc: Option<NewZealandNccIdentifier>,
    #[serde(rename = "ZANCC", skip_serializing_if = "Option::is_none")]
    pub zancc: Option<SouthAfricanNccIdentifier>,
    #[serde(rename = "USFW", skip_serializing_if = "Option::is_none")]
    pub usfw: Option<FedwireRoutingNumberIdentifier>,
    #[serde(rename = "CACPA", skip_serializing_if = "Option::is_none")]
    pub cacpa: Option<CanadianPaymentsArnIdentifier>,
    #[serde(rename = "AUBSBx", skip_serializing_if = "Option::is_none")]
    pub aubs_bx: Option<ExtensiveBranchNetworkIdentifier>,
    #[serde(rename = "AUBSBs", skip_serializing_if = "Option::is_none")]
    pub aubs_bs: Option<SmallNetworkIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClearingSystemMemberIdentification4Choice {
    #[serde(flatten)]
    pub value: ClearingSystemMemberIdentification4ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountStatusUpdateInstruction1 {
    #[serde(rename = "UpdInstr")]
    pub upd_instr: AccountStatusUpdateInstruction1Choice,
    #[serde(rename = "UpdInstrRsn", skip_serializing_if = "Option::is_none")]
    pub upd_instr_rsn: Option<AccountStatusUpdateInstructionReason1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GenericIdentification81 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "IdTp")]
    pub id_tp: OtherIdentification3Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CashAccountType5Code {
    #[serde(rename = "LEND")]
    Lend,
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "SETT")]
    Sett,
    #[serde(rename = "MARR")]
    Marr,
    #[serde(rename = "SEGT")]
    Segt,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum MoneyLaunderingCheck1Code {
    #[serde(rename = "PASS")]
    Pass,
    #[serde(rename = "NOTC")]
    Notc,
    #[serde(rename = "EXEM")]
    Exem,
    #[serde(rename = "CLMO")]
    Clmo,
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "POEP")]
    Poep,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CertificateType2Code {
    #[serde(rename = "AMLC")]
    Amlc,
    #[serde(rename = "DVLC")]
    Dvlc,
    #[serde(rename = "DFOR")]
    Dfor,
    #[serde(rename = "GOST")]
    Gost,
    #[serde(rename = "IDEN")]
    Iden,
    #[serde(rename = "INCU")]
    Incu,
    #[serde(rename = "LREF")]
    Lref,
    #[serde(rename = "PASS")]
    Pass,
    #[serde(rename = "PRAD")]
    Prad,
    #[serde(rename = "PKIC")]
    Pkic,
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
pub struct PortugueseNccIdentifier {
    #[validate(regex = "PORTUGUESE_NCC_IDENTIFIER_REGEX")]
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
pub struct PartyIdentification177ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
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
pub struct PartyIdentification177Choice {
    #[serde(flatten)]
    pub value: PartyIdentification177ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct EuroclearClearstreamIdentifier {
    #[validate(length(min = 1, max = 12,))]
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
pub struct NationalityCode {
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
pub struct FiscalYear1ChoiceEnum {
    #[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
    pub start_dt: Option<IsoDate>,
    #[serde(rename = "EndDt", skip_serializing_if = "Option::is_none")]
    pub end_dt: Option<IsoDate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FiscalYear1Choice {
    #[serde(flatten)]
    pub value: FiscalYear1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountStatusUpdateInstructionReason1ChoiceEnum {
    #[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
    pub no_spcfd_rsn: Option<NoReasonCode>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<AccountStatusUpdateInstructionReason1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountStatusUpdateInstructionReason1Choice {
    #[serde(flatten)]
    pub value: AccountStatusUpdateInstructionReason1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DateTimePeriod2 {
    #[validate]
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm: IsoDateTime,
    #[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
    pub to_dt_tm: Option<IsoDateTime>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ItalianDomesticIdentifier {
    #[validate(regex = "ITALIAN_DOMESTIC_IDENTIFIER_REGEX")]
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
pub struct RussianCentralBankIdentificationCodeIdentifier {
    #[validate(regex = "RUSSIAN_CENTRAL_BANK_IDENTIFICATION_CODE_IDENTIFIER_REGEX")]
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
pub struct NamePrefix1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<NamePrefix1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct NamePrefix1Choice {
    #[serde(flatten)]
    pub value: NamePrefix1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModificationScope27 {
    #[serde(rename = "ModScpIndctn")]
    pub mod_scp_indctn: DataModification2Code,
    #[validate]
    #[serde(rename = "InvstrPrflVldtn")]
    pub invstr_prfl_vldtn: PartyProfileInformation5,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModificationScope40 {
    #[serde(rename = "ModScpIndctn")]
    pub mod_scp_indctn: DataModification1Code,
    #[validate]
    #[serde(rename = "Intrmy")]
    pub intrmy: Intermediary46,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Gender1Code {
    #[serde(rename = "FEMA")]
    Fema,
    #[serde(rename = "MALE")]
    Male,
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
pub struct RegulatoryInformation1 {
    #[serde(rename = "Sctr", skip_serializing_if = "Option::is_none")]
    pub sctr: Option<Max35Text>,
    #[serde(rename = "Brnch", skip_serializing_if = "Option::is_none")]
    pub brnch: Option<Max35Text>,
    #[serde(rename = "Grp", skip_serializing_if = "Option::is_none")]
    pub grp: Option<Max35Text>,
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
pub struct WertpapierIdentifier {
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
pub struct CustomerConductClassification1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ConductClassification1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CustomerConductClassification1Choice {
    #[serde(flatten)]
    pub value: CustomerConductClassification1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InitialAmount1ChoiceEnum {
    #[serde(rename = "InitlNbOfInstlmts", skip_serializing_if = "Option::is_none")]
    pub initl_nb_of_instlmts: Option<Number>,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InitialAmount1Choice {
    #[serde(flatten)]
    pub value: InitialAmount1ChoiceEnum,
}
#[derive(
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
pub struct CitizenshipInformation2 {
    #[serde(rename = "Ntlty")]
    pub ntlty: NationalityCode,
    #[validate]
    #[serde(rename = "MnrInd")]
    pub mnr_ind: YesNoIndicator,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationSource1ChoiceEnum {
    #[serde(rename = "Dmst", skip_serializing_if = "Option::is_none")]
    pub dmst: Option<CountryCode>,
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
pub struct IdentificationSource1Choice {
    #[serde(flatten)]
    pub value: IdentificationSource1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OwnershipType2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AccountOwnershipType4Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OwnershipType2Choice {
    #[serde(flatten)]
    pub value: OwnershipType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SmallNetworkIdentifier {
    #[validate(regex = "SMALL_NETWORK_IDENTIFIER_REGEX")]
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
pub struct AccountParties18 {
    #[serde(rename = "ModScpIndctn")]
    pub mod_scp_indctn: DataModification1Code,
    #[serde(rename = "PrncplAcctPty", skip_serializing_if = "Option::is_none")]
    pub prncpl_acct_pty: Option<AccountParties13Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ScndryOwnr", default)]
    pub scndry_ownr: Vec<InvestmentAccountOwnershipInformation17>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Bnfcry", default)]
    pub bnfcry: Vec<InvestmentAccountOwnershipInformation17>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PwrOfAttny", default)]
    pub pwr_of_attny: Vec<InvestmentAccountOwnershipInformation17>,
    #[validate(length(min = 0,))]
    #[serde(rename = "LglGuardn", default)]
    pub lgl_guardn: Vec<InvestmentAccountOwnershipInformation17>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CtdnForMnr", default)]
    pub ctdn_for_mnr: Vec<InvestmentAccountOwnershipInformation17>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SucssrOnDth", default)]
    pub sucssr_on_dth: Vec<InvestmentAccountOwnershipInformation17>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Admstr", default)]
    pub admstr: Vec<InvestmentAccountOwnershipInformation17>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrPty", default)]
    pub othr_pty: Vec<ExtendedParty15>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Grntr", default)]
    pub grntr: Vec<InvestmentAccountOwnershipInformation17>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Sttlr", default)]
    pub sttlr: Vec<InvestmentAccountOwnershipInformation17>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SnrMggOffcl", default)]
    pub snr_mgg_offcl: Vec<InvestmentAccountOwnershipInformation17>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Prtctr", default)]
    pub prtctr: Vec<InvestmentAccountOwnershipInformation17>,
    #[serde(rename = "RegdShrhldrNm", skip_serializing_if = "Option::is_none")]
    pub regd_shrhldr_nm: Option<RegisteredShareholderName1Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AddressType1Code {
    #[serde(rename = "HOME")]
    Home,
    #[serde(rename = "BIZZ")]
    Bizz,
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
pub struct FatcaStatus2 {
    #[serde(rename = "Tp")]
    pub tp: FatcaStatus2Choice,
    #[serde(rename = "Src", skip_serializing_if = "Option::is_none")]
    pub src: Option<FatcaSource1Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Insurance1Code {
    #[serde(rename = "LIFE")]
    Life,
    #[serde(rename = "PDIS")]
    Pdis,
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
pub struct GenericAccountIdentification1 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max34Text,
    #[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<AccountSchemeName1Choice>,
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
pub struct Organisation23 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[validate(length(min = 1, max = 5,))]
    #[serde(rename = "PstlAdr", default)]
    pub pstl_adr: Vec<PostalAddress21>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CrsForm1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CrsFormType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CrsForm1Choice {
    #[serde(flatten)]
    pub value: CrsForm1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FatcaSource1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FatcaSourceStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FatcaSource1Choice {
    #[serde(flatten)]
    pub value: FatcaSource1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RestrictionStatus1Code {
    #[serde(rename = "ACTV")]
    Actv,
    #[serde(rename = "INAC")]
    Inac,
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
pub struct TransactionChannelType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TransactionChannel2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionChannelType1Choice {
    #[serde(flatten)]
    pub value: TransactionChannelType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModificationScope42 {
    #[serde(rename = "ModScpIndctn")]
    pub mod_scp_indctn: DataModification2Code,
    #[validate]
    #[serde(rename = "FinInstrmDtls")]
    pub fin_instrm_dtls: FinancialInstrument87,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PersonalInformation1 {
    #[serde(rename = "NmOfFthr", skip_serializing_if = "Option::is_none")]
    pub nm_of_fthr: Option<Max35Text>,
    #[serde(rename = "MdnNmOfMthr", skip_serializing_if = "Option::is_none")]
    pub mdn_nm_of_mthr: Option<Max35Text>,
    #[serde(rename = "NmOfPrtnr", skip_serializing_if = "Option::is_none")]
    pub nm_of_prtnr: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentInstrument17 {
    #[serde(rename = "SttlmCcy")]
    pub sttlm_ccy: ActiveCurrencyCode,
    #[serde(rename = "DvddPctg", skip_serializing_if = "Option::is_none")]
    pub dvdd_pctg: Option<PercentageBoundedRate>,
    #[serde(rename = "SbcptPmtInstrm", skip_serializing_if = "Option::is_none")]
    pub sbcpt_pmt_instrm: Option<PaymentInstrument24Choice>,
    #[serde(rename = "RedPmtInstrm", skip_serializing_if = "Option::is_none")]
    pub red_pmt_instrm: Option<PaymentInstrument19Choice>,
    #[serde(rename = "DvddPmtInstrm", skip_serializing_if = "Option::is_none")]
    pub dvdd_pmt_instrm: Option<PaymentInstrument19Choice>,
    #[serde(rename = "SvgsPlanPmtInstrm", skip_serializing_if = "Option::is_none")]
    pub svgs_plan_pmt_instrm: Option<PaymentInstrument24Choice>,
    #[serde(rename = "IntrstPmtInstrm", skip_serializing_if = "Option::is_none")]
    pub intrst_pmt_instrm: Option<PaymentInstrument19Choice>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Eligible1Code {
    #[serde(rename = "ELIG")]
    Elig,
    #[serde(rename = "NELI")]
    Neli,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AccountingStatus1Code {
    #[serde(rename = "YDOM")]
    Ydom,
    #[serde(rename = "NDOM")]
    Ndom,
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
pub struct AustrianBankleitzahlIdentifier {
    #[validate(regex = "AUSTRIAN_BANKLEITZAHL_IDENTIFIER_REGEX")]
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
pub struct Extension1 {
    #[validate]
    #[serde(rename = "PlcAndNm")]
    pub plc_and_nm: Max350Text,
    #[validate]
    #[serde(rename = "Txt")]
    pub txt: Max350Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FatcaStatus2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FatcaStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FatcaStatus2Choice {
    #[serde(flatten)]
    pub value: FatcaStatus2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CivilStatus1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CivilStatus1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CivilStatus1Choice {
    #[serde(flatten)]
    pub value: CivilStatus1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestmentAccountOwnershipInformation17 {
    #[serde(rename = "Pty")]
    pub pty: Party48Choice,
    #[serde(rename = "MnyLndrgChck", skip_serializing_if = "Option::is_none")]
    pub mny_lndrg_chck: Option<MoneyLaunderingCheck1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ModfdInvstrPrflVldtn", default)]
    pub modfd_invstr_prfl_vldtn: Vec<ModificationScope27>,
    #[serde(rename = "OwnrshBnfcryRate", skip_serializing_if = "Option::is_none")]
    pub ownrsh_bnfcry_rate: Option<OwnershipBeneficiaryRate1>,
    #[serde(rename = "ClntId", skip_serializing_if = "Option::is_none")]
    pub clnt_id: Option<Max35Text>,
    #[serde(rename = "FsclXmptn", skip_serializing_if = "Option::is_none")]
    pub fscl_xmptn: Option<YesNoIndicator>,
    #[serde(rename = "SgntryRghtInd", skip_serializing_if = "Option::is_none")]
    pub sgntry_rght_ind: Option<YesNoIndicator>,
    #[serde(rename = "MiFIDClssfctn", skip_serializing_if = "Option::is_none")]
    pub mi_fid_clssfctn: Option<MiFidClassification1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Ntfctn", default)]
    pub ntfctn: Vec<Notification2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FATCAFormTp", default)]
    pub fatca_form_tp: Vec<FatcaForm1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FATCASts", default)]
    pub fatca_sts: Vec<FatcaStatus2>,
    #[serde(rename = "FATCARptgDt", skip_serializing_if = "Option::is_none")]
    pub fatca_rptg_dt: Option<IsoDate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CRSFormTp", default)]
    pub crs_form_tp: Vec<CrsForm1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CRSSts", default)]
    pub crs_sts: Vec<CrsStatus4>,
    #[serde(rename = "CRSRptgDt", skip_serializing_if = "Option::is_none")]
    pub crs_rptg_dt: Option<IsoDate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrId", default)]
    pub othr_id: Vec<GenericIdentification82>,
    #[serde(rename = "TaxXmptn", skip_serializing_if = "Option::is_none")]
    pub tax_xmptn: Option<TaxExemptionReason2Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxRptg", default)]
    pub tax_rptg: Vec<TaxReporting3>,
    #[serde(rename = "Lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageCode>,
    #[serde(rename = "MailTp", skip_serializing_if = "Option::is_none")]
    pub mail_tp: Option<MailType1Choice>,
    #[serde(rename = "CtryAndResdtlSts", skip_serializing_if = "Option::is_none")]
    pub ctry_and_resdtl_sts: Option<CountryAndResidentialStatusType2>,
    #[serde(rename = "MntryWlth", skip_serializing_if = "Option::is_none")]
    pub mntry_wlth: Option<DateAndAmount1>,
    #[serde(rename = "EqtyVal", skip_serializing_if = "Option::is_none")]
    pub eqty_val: Option<DateAndAmount1>,
    #[serde(rename = "WorkgCptl", skip_serializing_if = "Option::is_none")]
    pub workg_cptl: Option<DateAndAmount1>,
    #[serde(rename = "CpnyLk", skip_serializing_if = "Option::is_none")]
    pub cpny_lk: Option<CompanyLink1Choice>,
    #[serde(rename = "ElctrncMlngSvcRef", skip_serializing_if = "Option::is_none")]
    pub elctrnc_mlng_svc_ref: Option<Max350Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PmryComAdr", default)]
    pub pmry_com_adr: Vec<CommunicationAddress6>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ScndryComAdr", default)]
    pub scndry_com_adr: Vec<CommunicationAddress6>,
    #[serde(rename = "AddtlRgltryInf", skip_serializing_if = "Option::is_none")]
    pub addtl_rgltry_inf: Option<RegulatoryInformation1>,
    #[serde(rename = "AcctgSts", skip_serializing_if = "Option::is_none")]
    pub acctg_sts: Option<AccountingStatus1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditiononalInformation13>,
    #[serde(rename = "CtrlgPty", skip_serializing_if = "Option::is_none")]
    pub ctrlg_pty: Option<YesNoIndicator>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestmentFundTransactionType1Code {
    #[serde(rename = "ALLL")]
    Alll,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "BUYI")]
    Buyi,
    #[serde(rename = "SWIO")]
    Swio,
    #[serde(rename = "TRIN")]
    Trin,
    #[serde(rename = "TOUT")]
    Tout,
    #[serde(rename = "SUBS")]
    Subs,
    #[serde(rename = "REDM")]
    Redm,
    #[serde(rename = "CDEP")]
    Cdep,
    #[serde(rename = "CWIT")]
    Cwit,
    #[serde(rename = "DIVP")]
    Divp,
    #[serde(rename = "CAEV")]
    Caev,
    #[serde(rename = "CROI")]
    Croi,
    #[serde(rename = "CROO")]
    Croo,
    #[serde(rename = "DIVI")]
    Divi,
    #[serde(rename = "INSP")]
    Insp,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "REAA")]
    Reaa,
    #[serde(rename = "RWPL")]
    Rwpl,
    #[serde(rename = "RDIV")]
    Rdiv,
    #[serde(rename = "SSPL")]
    Sspl,
    #[serde(rename = "SUAA")]
    Suaa,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BlockedReason2Code {
    #[serde(rename = "BKRP")]
    Bkrp,
    #[serde(rename = "CMMT")]
    Cmmt,
    #[serde(rename = "CNFS")]
    Cnfs,
    #[serde(rename = "MORT")]
    Mort,
    #[serde(rename = "PCOM")]
    Pcom,
    #[serde(rename = "PLDG")]
    Pldg,
    #[serde(rename = "TRPE")]
    Trpe,
    #[serde(rename = "SANC")]
    Sanc,
    #[serde(rename = "TRAN")]
    Tran,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum FundOwnership1Code {
    #[serde(rename = "YALL")]
    Yall,
    #[serde(rename = "NALL")]
    Nall,
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
pub struct Iban2007Identifier {
    #[validate(regex = "IBAN_2007_IDENTIFIER_REGEX")]
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
pub struct OwnershipBeneficiaryRate1 {
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "Frctn", skip_serializing_if = "Option::is_none")]
    pub frctn: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PartyRole1Code {
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "INVS")]
    Invs,
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
pub struct ModificationScope34 {
    #[serde(rename = "ModScpIndctn")]
    pub mod_scp_indctn: DataModification1Code,
    #[validate]
    #[serde(rename = "PstlAdr")]
    pub pstl_adr: PostalAddress21,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountSelection3ChoiceEnum {
    #[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<Max35Text>,
    #[serde(rename = "OthrAcctSelctnData", skip_serializing_if = "Option::is_none")]
    pub othr_acct_selctn_data: Option<InvestmentAccount76>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountSelection3Choice {
    #[serde(flatten)]
    pub value: AccountSelection3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GdprDataConsent1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<GdprDataConsent1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GdprDataConsent1Choice {
    #[serde(flatten)]
    pub value: GdprDataConsent1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BicfiDec2014Identifier {
    #[validate(regex = "BICFI_DEC_2014_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OrganisationType1Code {
    #[serde(rename = "IFUN")]
    Ifun,
    #[serde(rename = "PRIV")]
    Priv,
    #[serde(rename = "PUBL")]
    Publ,
    #[serde(rename = "PFUN")]
    Pfun,
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
pub struct SouthAfricanNccIdentifier {
    #[validate(regex = "SOUTH_AFRICAN_NCC_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestmentFundRole7Code {
    #[serde(rename = "CONC")]
    Conc,
    #[serde(rename = "DIST")]
    Dist,
    #[serde(rename = "FMCO")]
    Fmco,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "PAYI")]
    Payi,
    #[serde(rename = "TRAG")]
    Trag,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "CACO")]
    Caco,
    #[serde(rename = "FACT")]
    Fact,
    #[serde(rename = "INVE")]
    Inve,
    #[serde(rename = "INVS")]
    Invs,
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
pub struct PartyIdentification220 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<PartyIdentification182Choice>,
    #[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
    pub lgl_ntty_idr: Option<LeiIdentifier>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PoliticalExposureType2Code {
    #[serde(rename = "NPEX")]
    Npex,
    #[serde(rename = "YPEX")]
    Ypex,
    #[serde(rename = "PEXD")]
    Pexd,
    #[serde(rename = "PEXF")]
    Pexf,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Provided1Code {
    #[serde(rename = "NPRO")]
    Npro,
    #[serde(rename = "PROV")]
    Prov,
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
pub struct AccountSchemeName1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalAccountIdentification1Code>,
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
pub struct AccountSchemeName1Choice {
    #[serde(flatten)]
    pub value: AccountSchemeName1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentInstrument19ChoiceEnum {
    #[serde(rename = "BkrsDrftDtls", skip_serializing_if = "Option::is_none")]
    pub bkrs_drft_dtls: Option<Cheque4>,
    #[serde(rename = "ChqDtls", skip_serializing_if = "Option::is_none")]
    pub chq_dtls: Option<Cheque4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PaymentInstrument19Choice {
    #[serde(flatten)]
    pub value: PaymentInstrument19ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SwissSicIdentifier {
    #[validate(regex = "SWISS_SIC_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AccountStatusUpdateRequestReason1Code {
    #[serde(rename = "CLOE")]
    Cloe,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CardType1Code {
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "DBIT")]
    Dbit,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DistributionPolicy1Code {
    #[serde(rename = "DIST")]
    Dist,
    #[serde(rename = "ACCU")]
    Accu,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyRole2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestmentFundRole6Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyRole2Choice {
    #[serde(flatten)]
    pub value: PartyRole2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IndividualPerson29 {
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix1Choice>,
    #[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
    pub gvn_nm: Option<Max35Text>,
    #[serde(rename = "MddlNm", skip_serializing_if = "Option::is_none")]
    pub mddl_nm: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[validate(length(min = 1, max = 5,))]
    #[serde(rename = "PstlAdr", default)]
    pub pstl_adr: Vec<PostalAddress21>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketMakerProfile2 {
    #[serde(rename = "CtrctPrd", skip_serializing_if = "Option::is_none")]
    pub ctrct_prd: Option<DateTimePeriod2>,
    #[serde(rename = "Cmplc", skip_serializing_if = "Option::is_none")]
    pub cmplc: Option<YesNoIndicator>,
    #[serde(rename = "MaxSprd", skip_serializing_if = "Option::is_none")]
    pub max_sprd: Option<PercentageRate>,
    #[serde(rename = "Dscnt", skip_serializing_if = "Option::is_none")]
    pub dscnt: Option<PercentageRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ModificationScope41 {
    #[serde(rename = "ModScpIndctn")]
    pub mod_scp_indctn: DataModification1Code,
    #[validate]
    #[serde(rename = "InvstmtPlan")]
    pub invstmt_plan: InvestmentPlan16,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Frequency20ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<EventFrequency8Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Frequency20Choice {
    #[serde(flatten)]
    pub value: Frequency20ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Notification2 {
    #[validate]
    #[serde(rename = "NtfctnTp")]
    pub ntfctn_tp: Max35Text,
    #[validate]
    #[serde(rename = "Reqrd")]
    pub reqrd: YesNoIndicator,
    #[serde(rename = "DstrbtnTp", skip_serializing_if = "Option::is_none")]
    pub dstrbtn_tp: Option<InformationDistribution1Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification25ChoiceEnum {
    #[serde(rename = "SCVM", skip_serializing_if = "Option::is_none")]
    pub scvm: Option<SicovamIdentifier>,
    #[serde(rename = "SEDOL", skip_serializing_if = "Option::is_none")]
    pub sedol: Option<SedolIdentifier>,
    #[serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none")]
    pub tckr_symb: Option<TickerIdentifier>,
    #[serde(rename = "Vlrn", skip_serializing_if = "Option::is_none")]
    pub vlrn: Option<ValorenIdentifier>,
    #[serde(rename = "OthrPrtryId", skip_serializing_if = "Option::is_none")]
    pub othr_prtry_id: Option<AlternateSecurityIdentification7>,
    #[serde(rename = "RIC", skip_serializing_if = "Option::is_none")]
    pub ric: Option<RicIdentifier>,
    #[serde(rename = "Wrtppr", skip_serializing_if = "Option::is_none")]
    pub wrtppr: Option<WertpapierIdentifier>,
    #[serde(rename = "Dtch", skip_serializing_if = "Option::is_none")]
    pub dtch: Option<DutchIdentifier>,
    #[serde(rename = "Belgn", skip_serializing_if = "Option::is_none")]
    pub belgn: Option<BelgianIdentifier>,
    #[serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none")]
    pub blmbrg: Option<Bloomberg2Identifier>,
    #[serde(rename = "CTA", skip_serializing_if = "Option::is_none")]
    pub cta: Option<ConsolidatedTapeAssociationIdentifier>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "CUSIP", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<CusipIdentifier>,
    #[serde(rename = "QUICK", skip_serializing_if = "Option::is_none")]
    pub quick: Option<QuickIdentifier>,
    #[serde(rename = "Cmon", skip_serializing_if = "Option::is_none")]
    pub cmon: Option<EuroclearClearstreamIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SecurityIdentification25Choice {
    #[serde(flatten)]
    pub value: SecurityIdentification25ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification182ChoiceEnum {
    #[serde(rename = "NtlRegnNb", skip_serializing_if = "Option::is_none")]
    pub ntl_regn_nb: Option<Max35Text>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "TaxIdNb", skip_serializing_if = "Option::is_none")]
    pub tax_id_nb: Option<Max35Text>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress15>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification182Choice {
    #[serde(flatten)]
    pub value: PartyIdentification182ChoiceEnum,
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
pub struct CrsStatus4 {
    #[serde(rename = "Tp")]
    pub tp: CrsStatus3Choice,
    #[serde(rename = "Src", skip_serializing_if = "Option::is_none")]
    pub src: Option<CrsSource1Choice>,
    #[serde(rename = "XcptnlRptgCtry", skip_serializing_if = "Option::is_none")]
    pub xcptnl_rptg_ctry: Option<CountryCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SimpleIdentificationInformation4 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CrsStatus3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CrsStatus1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CrsStatus3Choice {
    #[serde(flatten)]
    pub value: CrsStatus3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PlanStatus2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PlanStatus1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PlanStatus2Choice {
    #[serde(flatten)]
    pub value: PlanStatus2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstitutionIdentification11ChoiceEnum {
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification4Choice>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<SimpleIdentificationInformation4>,
    #[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<BicfiDec2014Identifier>,
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
pub struct FinancialInstitutionIdentification11Choice {
    #[serde(flatten)]
    pub value: FinancialInstitutionIdentification11ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MarketPracticeVersion1 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max35Text,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoYearMonth>,
    #[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
    pub nb: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionType5ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestmentFundTransactionType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TransactionType5Choice {
    #[serde(flatten)]
    pub value: TransactionType5ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PoliticallyExposedPersonStatus1Code {
    #[serde(rename = "PE03")]
    Pe03,
    #[serde(rename = "PE01")]
    Pe01,
    #[serde(rename = "PE02")]
    Pe02,
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
pub struct NameAndAddress15 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max350Text,
    #[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress21>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountParties13ChoiceEnum {
    #[serde(rename = "Trstee", skip_serializing_if = "Option::is_none")]
    pub trstee: Option<InvestmentAccountOwnershipInformation17>,
    #[serde(rename = "PmryOwnr", skip_serializing_if = "Option::is_none")]
    pub pmry_ownr: Option<InvestmentAccountOwnershipInformation17>,
    #[serde(rename = "JntOwnr", skip_serializing_if = "Option::is_none")]
    pub jnt_ownr: Option<InvestmentAccountOwnershipInformation17>,
    #[serde(rename = "Nmnee", skip_serializing_if = "Option::is_none")]
    pub nmnee: Option<InvestmentAccountOwnershipInformation17>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountParties13Choice {
    #[serde(flatten)]
    pub value: AccountParties13ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AddressType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AddressType1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AddressType1Choice {
    #[serde(flatten)]
    pub value: AddressType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountDesignation1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<Rank1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountDesignation1Choice {
    #[serde(flatten)]
    pub value: AccountDesignation1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ChipsUniversalIdentifier {
    #[validate(regex = "CHIPS_UNIVERSAL_IDENTIFIER_REGEX")]
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
pub struct Cheque4 {
    #[validate]
    #[serde(rename = "PyeeId")]
    pub pyee_id: NameAndAddress5,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ChipsParticipantIdentifier {
    #[validate(regex = "CHIPS_PARTICIPANT_IDENTIFIER_REGEX")]
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
pub struct DeMinimusApplicable1 {
    #[validate]
    #[serde(rename = "NewIssePrmssn")]
    pub new_isse_prmssn: YesNoIndicator,
    #[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
    pub pctg: Option<PercentageRate>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountUsageType2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AccountUsageType2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountUsageType2Choice {
    #[serde(flatten)]
    pub value: AccountUsageType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AlternateSecurityIdentification7 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "IdSrc")]
    pub id_src: IdentificationSource1Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum EventFrequency10Code {
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "ADHO")]
    Adho,
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
pub struct InvestmentAccountCategory1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestmentAccountCategory1Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestmentAccountCategory1Choice {
    #[serde(flatten)]
    pub value: InvestmentAccountCategory1ChoiceEnum,
}
#[derive(
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
pub struct InvestmentPlan16 {
    #[serde(rename = "Frqcy")]
    pub frqcy: Frequency20Choice,
    #[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
    pub start_dt: Option<IsoDate>,
    #[serde(rename = "EndDt", skip_serializing_if = "Option::is_none")]
    pub end_dt: Option<IsoDate>,
    #[serde(rename = "Qty")]
    pub qty: UnitsOrAmount1Choice,
    #[serde(rename = "GrssAmtInd", skip_serializing_if = "Option::is_none")]
    pub grss_amt_ind: Option<YesNoIndicator>,
    #[serde(rename = "IncmPref", skip_serializing_if = "Option::is_none")]
    pub incm_pref: Option<IncomePreference2Code>,
    #[serde(rename = "InitlAmt", skip_serializing_if = "Option::is_none")]
    pub initl_amt: Option<InitialAmount1Choice>,
    #[serde(rename = "TtlNbOfInstlmts", skip_serializing_if = "Option::is_none")]
    pub ttl_nb_of_instlmts: Option<Number>,
    #[serde(rename = "RndgDrctn", skip_serializing_if = "Option::is_none")]
    pub rndg_drctn: Option<RoundingDirection1Code>,
    #[validate(length(min = 1, max = 50,))]
    #[serde(rename = "SctyDtls", default)]
    pub scty_dtls: Vec<Repartition6>,
    #[validate(length(min = 0, max = 8,))]
    #[serde(rename = "ModfdCshSttlm", default)]
    pub modfd_csh_sttlm: Vec<CashSettlement4>,
    #[serde(rename = "CtrctRef", skip_serializing_if = "Option::is_none")]
    pub ctrct_ref: Option<Max35Text>,
    #[serde(rename = "RltdCtrctRef", skip_serializing_if = "Option::is_none")]
    pub rltd_ctrct_ref: Option<Max35Text>,
    #[serde(rename = "PdctId", skip_serializing_if = "Option::is_none")]
    pub pdct_id: Option<Max35Text>,
    #[serde(
        rename = "SLAChrgAndComssnRef",
        skip_serializing_if = "Option::is_none"
    )]
    pub sla_chrg_and_comssn_ref: Option<Max35Text>,
    #[serde(rename = "InsrncCover", skip_serializing_if = "Option::is_none")]
    pub insrnc_cover: Option<InsuranceType2Choice>,
    #[serde(rename = "PlanSts", skip_serializing_if = "Option::is_none")]
    pub plan_sts: Option<PlanStatus2Choice>,
    #[serde(rename = "InstlmtMgrRole", skip_serializing_if = "Option::is_none")]
    pub instlmt_mgr_role: Option<PartyRole4Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MoneyLaunderingCheck1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<MoneyLaunderingCheck1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MoneyLaunderingCheck1Choice {
    #[serde(flatten)]
    pub value: MoneyLaunderingCheck1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CountryAndResidentialStatusType2 {
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
    #[serde(rename = "ResdtlSts")]
    pub resdtl_sts: ResidentialStatus1Code,
}
#[derive(
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
    #[validate]
    #[serde(rename = "AcctModInstr")]
    pub acct_mod_instr: AccountModificationInstructionV08,
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
pub struct FatcaForm1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<FatcaFormType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FatcaForm1Choice {
    #[serde(flatten)]
    pub value: FatcaForm1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Extended350Code {
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
pub struct ModificationScope43 {
    #[serde(rename = "ModScpIndctn")]
    pub mod_scp_indctn: DataModification1Code,
    #[serde(rename = "Plcmnt")]
    pub plcmnt: ReferredAgent3,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Collateral1Code {
    #[serde(rename = "COLL")]
    Coll,
    #[serde(rename = "NCOL")]
    Ncol,
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
pub struct Max10Text {
    #[validate(length(min = 1, max = 10,))]
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
pub struct AnyBicDec2014Identifier {
    #[validate(regex = "ANY_BIC_DEC_2014_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PositionEffect3Code {
    #[serde(rename = "FIFO")]
    Fifo,
    #[serde(rename = "LIFO")]
    Lifo,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum Rank1Code {
    #[serde(rename = "PRIM")]
    Prim,
    #[serde(rename = "SECO")]
    Seco,
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
pub struct BlockedHoldingDetails2 {
    #[serde(rename = "BlckdHldg")]
    pub blckd_hldg: Holding1Code,
    #[serde(rename = "PrtlHldgUnits", skip_serializing_if = "Option::is_none")]
    pub prtl_hldg_units: Option<DecimalNumber>,
    #[serde(rename = "HldgCertNb", skip_serializing_if = "Option::is_none")]
    pub hldg_cert_nb: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestmentAccount76 {
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[serde(rename = "Dsgnt", skip_serializing_if = "Option::is_none")]
    pub dsgnt: Option<Max35Text>,
    #[serde(rename = "FndTp", skip_serializing_if = "Option::is_none")]
    pub fnd_tp: Option<Max35Text>,
    #[serde(rename = "FndFmlyNm", skip_serializing_if = "Option::is_none")]
    pub fnd_fmly_nm: Option<Max350Text>,
    #[serde(rename = "SctyDtls", skip_serializing_if = "Option::is_none")]
    pub scty_dtls: Option<FinancialInstrument55>,
    #[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
    pub acct_ownr: Option<AccountOwner3Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Intrmy", default)]
    pub intrmy: Vec<Intermediary47>,
    #[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
    pub acct_svcr: Option<PartyIdentification125Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DirectDebitMandate7 {
    #[validate]
    #[serde(rename = "DbtrAcct")]
    pub dbtr_acct: AccountIdentificationAndName5,
    #[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<PartyIdentification125Choice>,
    #[serde(rename = "DbtrTaxIdNb", skip_serializing_if = "Option::is_none")]
    pub dbtr_tax_id_nb: Option<Max35Text>,
    #[serde(rename = "DbtrNtlRegnNb", skip_serializing_if = "Option::is_none")]
    pub dbtr_ntl_regn_nb: Option<Max35Text>,
    #[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<PartyIdentification125Choice>,
    #[serde(rename = "DbtrAgt")]
    pub dbtr_agt: FinancialInstitutionIdentification11Choice,
    #[serde(rename = "DbtrAgtBrnch", skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_brnch: Option<BranchData4>,
    #[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<FinancialInstitutionIdentification11Choice>,
    #[serde(rename = "CdtrAgtBrnch", skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_brnch: Option<BranchData4>,
    #[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
    pub regn_id: Option<Max35Text>,
    #[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
    pub mndt_id: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InvestmentAccountModification4 {
    #[serde(rename = "ModRsn", skip_serializing_if = "Option::is_none")]
    pub mod_rsn: Option<Max350Text>,
    #[serde(rename = "AcctApplId", skip_serializing_if = "Option::is_none")]
    pub acct_appl_id: Option<Max35Text>,
    #[serde(rename = "ClntRef", skip_serializing_if = "Option::is_none")]
    pub clnt_ref: Option<Max35Text>,
    #[serde(rename = "CtrPtyRef", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_ref: Option<AdditionalReference13>,
    #[validate(length(min = 0,))]
    #[serde(rename = "ExstgAcctId", default)]
    pub exstg_acct_id: Vec<Account23>,
}
#[derive(
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
pub struct DeMinimusNotApplicable1 {
    #[validate]
    #[serde(rename = "RstrctdPrsnRsn")]
    pub rstrctd_prsn_rsn: Max350Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditiononalInformation13 {
    #[serde(rename = "Lmttn", skip_serializing_if = "Option::is_none")]
    pub lmttn: Option<Max350Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max350Text>,
    #[serde(rename = "AcctVldtn", skip_serializing_if = "Option::is_none")]
    pub acct_vldtn: Option<Max350Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Rgltr", skip_serializing_if = "Option::is_none")]
    pub rgltr: Option<PartyIdentification125Choice>,
    #[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
    pub sts: Option<RestrictionStatus1Choice>,
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<DateTimePeriod2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ConsolidationType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<ConsolidationType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ConsolidationType1Choice {
    #[serde(flatten)]
    pub value: ConsolidationType1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ConsolidationType1Code {
    #[serde(rename = "GENL")]
    Genl,
    #[serde(rename = "PART")]
    Part,
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
pub struct CashAccountType3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CashAccountType5Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAccountType3Choice {
    #[serde(flatten)]
    pub value: CashAccountType3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct RicIdentifier {
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
pub struct ModificationScope39 {
    #[serde(rename = "ModScpIndctn")]
    pub mod_scp_indctn: DataModification2Code,
    #[validate]
    #[serde(rename = "Ctznsh")]
    pub ctznsh: CitizenshipInformation2,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BlockedReason2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<BlockedReason2Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BlockedReason2Choice {
    #[serde(flatten)]
    pub value: BlockedReason2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct LanguageCode {
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
pub struct PartyRole4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestmentFundRole7Code>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyRole4Choice {
    #[serde(flatten)]
    pub value: PartyRole4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AccountUsageType2Code {
    #[serde(rename = "INVE")]
    Inve,
    #[serde(rename = "ISSP")]
    Issp,
    #[serde(rename = "SETP")]
    Setp,
    #[serde(rename = "TRDP")]
    Trdp,
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
pub struct Reinvestment4 {
    #[validate]
    #[serde(rename = "FinInstrmDtls")]
    pub fin_instrm_dtls: FinancialInstrument87,
    #[serde(rename = "ReqdNAVCcy", skip_serializing_if = "Option::is_none")]
    pub reqd_nav_ccy: Option<ActiveCurrencyCode>,
    #[validate]
    #[serde(rename = "RinvstmtPctg")]
    pub rinvstmt_pctg: PercentageRate,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CommunicationAddress6 {
    #[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<AddressType1Choice>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Max256Text>,
    #[serde(rename = "Phne", skip_serializing_if = "Option::is_none")]
    pub phne: Option<PhoneNumber>,
    #[serde(rename = "Mob", skip_serializing_if = "Option::is_none")]
    pub mob: Option<PhoneNumber>,
    #[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<PhoneNumber>,
    #[serde(rename = "TlxAdr", skip_serializing_if = "Option::is_none")]
    pub tlx_adr: Option<Max35Text>,
    #[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max256Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum ResidentialStatus1Code {
    #[serde(rename = "RESI")]
    Resi,
    #[serde(rename = "PRES")]
    Pres,
    #[serde(rename = "NRES")]
    Nres,
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
pub struct AddressType2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<AddressType2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AddressType2Choice {
    #[serde(flatten)]
    pub value: AddressType2ChoiceEnum,
}
