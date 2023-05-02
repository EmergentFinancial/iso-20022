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
    static ref IRISH_NSC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"IE[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ITALIAN_DOMESTIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"IT[0-9]{10,10}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PORTUGUESE_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"PT[0-9]{8,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SMALL_NETWORK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AU[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SOUTH_AFRICAN_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"ZA[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_YEAR_REGEX: ::regex::Regex = ::regex::Regex::new(r#"^-?\d{4}([+-]\d{2}:\d{2}|Z)?$"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref HELLENIC_BANK_IDENTIFICATION_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"GR[0-9]{7,7}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXTENSIVE_BRANCH_NETWORK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AU[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref NEW_ZEALAND_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"NZ[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_OR_HISTORIC_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref AUSTRIAN_BANKLEITZAHL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AT[0-9]{5,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref INDIAN_FINANCIAL_SYSTEM_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"IN[a-zA-Z0-9]{11,11}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_YEAR_MONTH_REGEX: ::regex::Regex = ::regex::Regex::new(r#"^-?\d{4}-(0[1-9]|1[0-2])([+-]\d{2}:\d{2}|Z)?$"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BLOOMBERG_2_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"(BBG)[BCDFGHJKLMNPQRSTVWXYZ\d]{8}\d"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RUSSIAN_CENTRAL_BANK_IDENTIFICATION_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"RU[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SPANISH_DOMESTIC_INTERBANKING_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"ES[0-9]{8,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SWISS_BC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SW[0-9]{3,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CHIPS_UNIVERSAL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CH[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref MAX_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{1,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SWISS_SIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SW[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref UK_DOMESTIC_SORT_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SC[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CANADIAN_PAYMENTS_ARN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CA[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref FEDWIRE_ROUTING_NUMBER_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"FW[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref HONG_KONG_BANK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"HK[0-9]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref POLISH_NATIONAL_CLEARING_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"PL[0-9]{8,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref GERMAN_BANKLEITZAHL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"BL[0-9]{8,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PREVIOUS_ALL_REGEX: ::regex::Regex = ::regex::Regex::new(r#"ALL"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CHIPS_PARTICIPANT_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CP[0-9]{4,4}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:sese.018.001.09".to_string()
}

#[derive(
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
    #[serde(rename = "TpAndId", skip_serializing_if = "Option::is_none")]
    pub tp_and_id: Option<SafekeepingPlaceTypeAndIdentification1>,
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification78>,
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
pub struct IrishNscIdentifier {
    #[validate(regex = "IRISH_NSC_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum WaivingInstruction1Code {
    #[serde(rename = "WICA")]
    Wica,
    #[serde(rename = "WIUN")]
    Wiun,
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
pub struct WaivingInstruction2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<WaivingInstruction1Code>,
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
pub struct WaivingInstruction2Choice {
    #[serde(flatten)]
    pub value: WaivingInstruction2ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
    pub dt_tm: Option<IsoDateTime>,
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
pub struct DateAndDateTime2Choice {
    #[serde(flatten)]
    pub value: DateAndDateTime2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PriceMethod1Code {
    #[serde(rename = "FORW")]
    Forw,
    #[serde(rename = "HIST")]
    Hist,
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
pub struct Intermediary43 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: PartyIdentification139,
    #[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
    pub acct: Option<Account27>,
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role4Choice>,
    #[serde(rename = "CtctPrsn", skip_serializing_if = "Option::is_none")]
    pub ctct_prsn: Option<ContactIdentification2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Tax35 {
    #[serde(rename = "Tp")]
    pub tp: TaxType3Choice,
    #[validate]
    #[serde(rename = "ApldAmt")]
    pub apld_amt: ActiveCurrencyAndAmount,
    #[serde(rename = "ApldRate", skip_serializing_if = "Option::is_none")]
    pub apld_rate: Option<PercentageRate>,
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "RcptId", skip_serializing_if = "Option::is_none")]
    pub rcpt_id: Option<PartyIdentification139>,
    #[serde(rename = "TaxClctnDtls", skip_serializing_if = "Option::is_none")]
    pub tax_clctn_dtls: Option<TaxCalculationInformation10>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeneralInvestment2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GeneralInvestmentAccountType2Choice>,
    #[serde(rename = "OwnrshTp", skip_serializing_if = "Option::is_none")]
    pub ownrsh_tp: Option<AccountOwnershipType6Code>,
    #[serde(rename = "CurInvstmtAmt", skip_serializing_if = "Option::is_none")]
    pub cur_invstmt_amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "EstmtdVal", skip_serializing_if = "Option::is_none")]
    pub estmtd_val: Option<DateAndAmount2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PensionOrder1 {
    #[serde(rename = "Tp")]
    pub tp: PensionOrderType1Choice,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
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
pub struct AlternateSecurityIdentification7 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "IdSrc")]
    pub id_src: IdentificationSource1Choice,
}
#[derive(
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
pub struct Unit11 {
    #[validate]
    #[serde(rename = "UnitsNb")]
    pub units_nb: DecimalNumber,
    #[serde(rename = "OrdrDt", skip_serializing_if = "Option::is_none")]
    pub ordr_dt: Option<IsoDate>,
    #[serde(rename = "AcqstnDt", skip_serializing_if = "Option::is_none")]
    pub acqstn_dt: Option<IsoDate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CertNb", default)]
    pub cert_nb: Vec<Max35Text>,
    #[serde(rename = "Grp1Or2Units", skip_serializing_if = "Option::is_none")]
    pub grp_1_or_2_units: Option<UkTaxGroupUnit1Code>,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<Max35Text>,
    #[serde(rename = "PricDtls", skip_serializing_if = "Option::is_none")]
    pub pric_dtls: Option<UnitPrice23>,
    #[serde(rename = "TxOvrhd", skip_serializing_if = "Option::is_none")]
    pub tx_ovrhd: Option<TotalFeesAndTaxes41>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrAmt", default)]
    pub othr_amt: Vec<OtherAmount1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationType42ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification30>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfIdentification1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IdentificationType42Choice {
    #[serde(flatten)]
    pub value: IdentificationType42ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxationBasis5Code {
    #[serde(rename = "FLAT")]
    Flat,
    #[serde(rename = "GRAM")]
    Gram,
    #[serde(rename = "NEAM")]
    Neam,
    #[serde(rename = "NAVP")]
    Navp,
    #[serde(rename = "PERU")]
    Peru,
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
pub struct DrawdownStatus1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DrawdownStatus1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DrawdownStatus1Choice {
    #[serde(flatten)]
    pub value: DrawdownStatus1ChoiceEnum,
}
#[derive(
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
pub struct PartyIdentification122ChoiceEnum {
    #[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
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
pub struct PartyIdentification122Choice {
    #[serde(flatten)]
    pub value: PartyIdentification122ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentificationAndAccount194 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification123Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification7>,
    #[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount19>,
    #[serde(rename = "PrcgDt", skip_serializing_if = "Option::is_none")]
    pub prcg_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation6>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PortfolioWithdrawalReason1Code {
    #[serde(rename = "FTRS")]
    Ftrs,
    #[default]
    Unknown,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum SettlementTransactionCondition11Code {
    #[serde(rename = "NOMC")]
    Nomc,
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
pub struct TaxCalculationInformation10 {
    #[serde(rename = "Bsis", skip_serializing_if = "Option::is_none")]
    pub bsis: Option<TaxBasis1Choice>,
    #[validate]
    #[serde(rename = "TaxblAmt")]
    pub taxbl_amt: ActiveCurrencyAndAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification126ChoiceEnum {
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
pub struct PartyIdentification126Choice {
    #[serde(flatten)]
    pub value: PartyIdentification126ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrument62ChoiceEnum {
    #[serde(rename = "CshAsst", skip_serializing_if = "Option::is_none")]
    pub csh_asst: Option<CashAsset2>,
    #[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
    pub scty: Option<FinancialInstrumentIdentification3>,
    #[serde(rename = "OthrAsst", skip_serializing_if = "Option::is_none")]
    pub othr_asst: Option<OtherAsset2>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrument62Choice {
    #[serde(flatten)]
    pub value: FinancialInstrument62ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum UkTaxGroupUnit1Code {
    #[serde(rename = "GRP1")]
    Grp1,
    #[serde(rename = "GRP2")]
    Grp2,
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
pub struct YesNoIndicator {
    #[serde(rename = "$text")]
    pub value: bool,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum AccountOwnershipType6Code {
    #[serde(rename = "BOWN")]
    Bown,
    #[serde(rename = "CORP")]
    Corp,
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "ENTR")]
    Entr,
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
    #[serde(rename = "LIPA")]
    Lipa,
    #[serde(rename = "NOMI")]
    Nomi,
    #[serde(rename = "NFPO")]
    Nfpo,
    #[serde(rename = "ONIS")]
    Onis,
    #[serde(rename = "OWNR")]
    Ownr,
    #[serde(rename = "RGIC")]
    Rgic,
    #[serde(rename = "SIGL")]
    Sigl,
    #[serde(rename = "UNCO")]
    Unco,
    #[serde(rename = "USUF")]
    Usuf,
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
pub struct UnitPrice23 {
    #[serde(rename = "Tp")]
    pub tp: TypeOfPrice46Choice,
    #[validate]
    #[serde(rename = "Val")]
    pub val: PriceValue1,
    #[serde(rename = "PricMtd", skip_serializing_if = "Option::is_none")]
    pub pric_mtd: Option<PriceMethod1Code>,
    #[serde(rename = "AcrdIntrstNAV", skip_serializing_if = "Option::is_none")]
    pub acrd_intrst_nav: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "NbOfDaysAcrd", skip_serializing_if = "Option::is_none")]
    pub nb_of_days_acrd: Option<Number>,
    #[serde(rename = "TaxblIncmPerShr", skip_serializing_if = "Option::is_none")]
    pub taxbl_incm_per_shr: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(
        rename = "TaxblIncmPerShrClctd",
        skip_serializing_if = "Option::is_none"
    )]
    pub taxbl_incm_per_shr_clctd: Option<TaxableIncomePerShareCalculated2Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PensionPolicy1 {
    #[validate]
    #[serde(rename = "Idr")]
    pub idr: Max35Text,
    #[serde(rename = "SubIdr", skip_serializing_if = "Option::is_none")]
    pub sub_idr: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<AdditionalInformation15>,
}
#[derive(
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxType3ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxType17Code>,
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
pub struct TaxType3Choice {
    #[serde(flatten)]
    pub value: TaxType3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalReference11 {
    #[validate]
    #[serde(rename = "Ref")]
    pub r#ref: Max35Text,
    #[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
    pub ref_issr: Option<PartyIdentification139>,
    #[serde(rename = "MsgNm", skip_serializing_if = "Option::is_none")]
    pub msg_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ChargeBasis2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxationBasis5Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ChargeBasis2Choice {
    #[serde(flatten)]
    pub value: ChargeBasis2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TypeOfAmount1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxWrapperAmountType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TypeOfAmount1Choice {
    #[serde(flatten)]
    pub value: TypeOfAmount1ChoiceEnum,
}
#[derive(
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
pub enum BusinessFlowDirectionType1Code {
    #[serde(rename = "ADLV")]
    Adlv,
    #[serde(rename = "ARCV")]
    Arcv,
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
pub struct ChargeOrCommissionDiscount1 {
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "Bsis", skip_serializing_if = "Option::is_none")]
    pub bsis: Option<WaivingInstruction2Choice>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum GeneralInvestmentAccountType2Code {
    #[serde(rename = "ANYY")]
    Anyy,
    #[serde(rename = "EQUI")]
    Equi,
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
pub struct ClassificationType32ChoiceEnum {
    #[serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none")]
    pub altrn_clssfctn: Option<GenericIdentification36>,
    #[serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none")]
    pub clssfctn_fin_instrm: Option<CfiOct2015Identifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClassificationType32Choice {
    #[serde(flatten)]
    pub value: ClassificationType32ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PensionSchemeType3Code {
    #[serde(rename = "AAVC")]
    Aavc,
    #[serde(rename = "DBEN")]
    Dben,
    #[serde(rename = "EXPP")]
    Expp,
    #[serde(rename = "FAVC")]
    Favc,
    #[serde(rename = "GPPS")]
    Gpps,
    #[serde(rename = "SIPG")]
    Sipg,
    #[serde(rename = "STKG")]
    Stkg,
    #[serde(rename = "IPST")]
    Ipst,
    #[serde(rename = "STKI")]
    Stki,
    #[serde(rename = "OTPM")]
    Otpm,
    #[serde(rename = "OCDC")]
    Ocdc,
    #[serde(rename = "PPNS")]
    Ppns,
    #[serde(rename = "EPKA")]
    Epka,
    #[serde(rename = "ITPO")]
    Itpo,
    #[serde(rename = "REAN")]
    Rean,
    #[serde(rename = "SC32")]
    Sc32,
    #[serde(rename = "S32A")]
    S32A,
    #[serde(rename = "SIPP")]
    Sipp,
    #[serde(rename = "SSAS")]
    Ssas,
    #[serde(rename = "NWRP")]
    Nwrp,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IsoYear {
    #[validate(regex = "ISO_YEAR_REGEX")]
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
pub struct OtherAmountType1ChoiceEnum {
    #[serde(rename = "PrtryCd", skip_serializing_if = "Option::is_none")]
    pub prtry_cd: Option<GenericIdentification1>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OtherAmountType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherAmountType1Choice {
    #[serde(flatten)]
    pub value: OtherAmountType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PreviousYear1ChoiceEnum {
    #[serde(rename = "SpcfcPrvsYrs", skip_serializing_if = "Option::is_none")]
    pub spcfc_prvs_yrs: Option<IsoYear>,
    #[serde(rename = "AllPrvsYrs", skip_serializing_if = "Option::is_none")]
    pub all_prvs_yrs: Option<PreviousAll>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PreviousYear1Choice {
    #[serde(flatten)]
    pub value: PreviousYear1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SubAccount5 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[serde(rename = "Chrtc", skip_serializing_if = "Option::is_none")]
    pub chrtc: Option<Max35Text>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxableIncomePerShareCalculated2Code {
    #[serde(rename = "TSIY")]
    Tsiy,
    #[serde(rename = "TSIN")]
    Tsin,
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
pub struct ContactIdentification2 {
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix1Code>,
    #[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
    pub gvn_nm: Option<Max35Text>,
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max35Text,
    #[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
    #[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
    pub mob_nb: Option<PhoneNumber>,
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
pub struct Role4ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestmentFundRole2Code>,
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
pub struct Role4Choice {
    #[serde(flatten)]
    pub value: Role4ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PensionTransferScope1Code {
    #[serde(rename = "CRYS")]
    Crys,
    #[serde(rename = "SDDT")]
    Sddt,
    #[serde(rename = "FULP")]
    Fulp,
    #[serde(rename = "UCRY")]
    Ucry,
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
pub struct HellenicBankIdentificationCodeIdentifier {
    #[validate(regex = "HELLENIC_BANK_IDENTIFICATION_CODE_IDENTIFIER_REGEX")]
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
pub struct TaxReferenceParty1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<HolderType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxReferenceParty1Choice {
    #[serde(flatten)]
    pub value: TaxReferenceParty1ChoiceEnum,
}
#[derive(
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
#[derive(
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
pub struct Pension5 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<PensionPolicy1>,
    #[serde(rename = "Tp")]
    pub tp: PensionSchemeType3Choice,
    #[serde(rename = "TrfScp", skip_serializing_if = "Option::is_none")]
    pub trf_scp: Option<PensionTransferScope1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TaxRef", default)]
    pub tax_ref: Vec<TaxReference1>,
    #[serde(rename = "DrwdwnTrchId", skip_serializing_if = "Option::is_none")]
    pub drwdwn_trch_id: Option<Max35Text>,
    #[serde(rename = "DrwdwnSts", skip_serializing_if = "Option::is_none")]
    pub drwdwn_sts: Option<DrawdownStatus1Choice>,
    #[serde(rename = "DrwdwnTp", skip_serializing_if = "Option::is_none")]
    pub drwdwn_tp: Option<DrawdownType2Choice>,
    #[serde(rename = "NbOfDrwdwnTrnchs", skip_serializing_if = "Option::is_none")]
    pub nb_of_drwdwn_trnchs: Option<Number>,
    #[serde(rename = "EstmtdVal", skip_serializing_if = "Option::is_none")]
    pub estmtd_val: Option<DateAndAmount2>,
    #[serde(rename = "BlckTrf", skip_serializing_if = "Option::is_none")]
    pub blck_trf: Option<YesNoIndicator>,
    #[serde(rename = "BlckTrfRef", skip_serializing_if = "Option::is_none")]
    pub blck_trf_ref: Option<Max35Text>,
    #[serde(rename = "TaxFreeCshPrtcn", skip_serializing_if = "Option::is_none")]
    pub tax_free_csh_prtcn: Option<YesNoIndicator>,
    #[serde(rename = "TaxFreeCshAmt", skip_serializing_if = "Option::is_none")]
    pub tax_free_csh_amt: Option<DateAndAmount2>,
    #[serde(
        rename = "ValOfPnsnPlcyOrPlanOrSchme",
        skip_serializing_if = "Option::is_none"
    )]
    pub val_of_pnsn_plcy_or_plan_or_schme: Option<DateAndAmount2>,
    #[serde(rename = "RtrmntAgePrtcn", skip_serializing_if = "Option::is_none")]
    pub rtrmnt_age_prtcn: Option<YesNoIndicator>,
    #[serde(rename = "RtrmntAge", skip_serializing_if = "Option::is_none")]
    pub rtrmnt_age: Option<DecimalNumber>,
    #[serde(rename = "Shrg", skip_serializing_if = "Option::is_none")]
    pub shrg: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "LumpSumTp", default)]
    pub lump_sum_tp: Vec<LumpSumType1Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "PnsnOrdr", default)]
    pub pnsn_ordr: Vec<PensionOrder1>,
    #[serde(
        rename = "RingFncdDrwdwnAssts",
        skip_serializing_if = "Option::is_none"
    )]
    pub ring_fncd_drwdwn_assts: Option<YesNoIndicator>,
    #[serde(rename = "MnyPurchsAnlAllwnc", skip_serializing_if = "Option::is_none")]
    pub mny_purchs_anl_allwnc: Option<MoneyPurchaseAnnualAllowance1>,
    #[serde(rename = "SfgrdBnft", skip_serializing_if = "Option::is_none")]
    pub sfgrd_bnft: Option<YesNoIndicator>,
    #[serde(
        rename = "NonSfgrddGrntedBnfts",
        skip_serializing_if = "Option::is_none"
    )]
    pub non_sfgrdd_grnted_bnfts: Option<YesNoIndicator>,
    #[serde(rename = "LftmAllwncPrtcn", skip_serializing_if = "Option::is_none")]
    pub lftm_allwnc_prtcn: Option<YesNoIndicator>,
    #[serde(
        rename = "ClntLftmAllwncPrtcn",
        skip_serializing_if = "Option::is_none"
    )]
    pub clnt_lftm_allwnc_prtcn: Option<YesNoIndicator>,
    #[serde(
        rename = "BnftCrstllstnEvtOcrd",
        skip_serializing_if = "Option::is_none"
    )]
    pub bnft_crstllstn_evt_ocrd: Option<YesNoIndicator>,
    #[serde(rename = "NonWrpprTrf", skip_serializing_if = "Option::is_none")]
    pub non_wrppr_trf: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
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
pub struct CfiOct2015Identifier {
    #[validate(regex = "CFI_OCT_2015_IDENTIFIER_REGEX")]
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
pub struct ClearingSystemMemberIdentification2ChoiceEnum {
    #[serde(rename = "HKNCC", skip_serializing_if = "Option::is_none")]
    pub hkncc: Option<HongKongBankIdentifier>,
    #[serde(rename = "DEBLZ", skip_serializing_if = "Option::is_none")]
    pub deblz: Option<GermanBankleitzahlIdentifier>,
    #[serde(rename = "GRHEBIC", skip_serializing_if = "Option::is_none")]
    pub grhebic: Option<HellenicBankIdentificationCodeIdentifier>,
    #[serde(rename = "USCH", skip_serializing_if = "Option::is_none")]
    pub usch: Option<ChipsParticipantIdentifier>,
    #[serde(rename = "USCHU", skip_serializing_if = "Option::is_none")]
    pub uschu: Option<ChipsUniversalIdentifier>,
    #[serde(rename = "RUCB", skip_serializing_if = "Option::is_none")]
    pub rucb: Option<RussianCentralBankIdentificationCodeIdentifier>,
    #[serde(rename = "AUBSBs", skip_serializing_if = "Option::is_none")]
    pub aubs_bs: Option<SmallNetworkIdentifier>,
    #[serde(rename = "NZNCC", skip_serializing_if = "Option::is_none")]
    pub nzncc: Option<NewZealandNccIdentifier>,
    #[serde(rename = "ATBLZ", skip_serializing_if = "Option::is_none")]
    pub atblz: Option<AustrianBankleitzahlIdentifier>,
    #[serde(rename = "ITNCC", skip_serializing_if = "Option::is_none")]
    pub itncc: Option<ItalianDomesticIdentifier>,
    #[serde(rename = "CACPA", skip_serializing_if = "Option::is_none")]
    pub cacpa: Option<CanadianPaymentsArnIdentifier>,
    #[serde(rename = "AUBSBx", skip_serializing_if = "Option::is_none")]
    pub aubs_bx: Option<ExtensiveBranchNetworkIdentifier>,
    #[serde(rename = "INIFSC", skip_serializing_if = "Option::is_none")]
    pub inifsc: Option<IndianFinancialSystemCodeIdentifier>,
    #[serde(rename = "CHSIC", skip_serializing_if = "Option::is_none")]
    pub chsic: Option<SwissSicIdentifier>,
    #[serde(rename = "GBSC", skip_serializing_if = "Option::is_none")]
    pub gbsc: Option<UkDomesticSortCodeIdentifier>,
    #[serde(rename = "USFW", skip_serializing_if = "Option::is_none")]
    pub usfw: Option<FedwireRoutingNumberIdentifier>,
    #[serde(rename = "PTNCC", skip_serializing_if = "Option::is_none")]
    pub ptncc: Option<PortugueseNccIdentifier>,
    #[serde(rename = "ESNCC", skip_serializing_if = "Option::is_none")]
    pub esncc: Option<SpanishDomesticInterbankingIdentifier>,
    #[serde(rename = "OthrClrCdId", skip_serializing_if = "Option::is_none")]
    pub othr_clr_cd_id: Option<Max35Text>,
    #[serde(rename = "CHBC", skip_serializing_if = "Option::is_none")]
    pub chbc: Option<SwissBcIdentifier>,
    #[serde(rename = "IENSC", skip_serializing_if = "Option::is_none")]
    pub iensc: Option<IrishNscIdentifier>,
    #[serde(rename = "PLKNR", skip_serializing_if = "Option::is_none")]
    pub plknr: Option<PolishNationalClearingCodeIdentifier>,
    #[serde(rename = "ZANCC", skip_serializing_if = "Option::is_none")]
    pub zancc: Option<SouthAfricanNccIdentifier>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ClearingSystemMemberIdentification2Choice {
    #[serde(flatten)]
    pub value: ClearingSystemMemberIdentification2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyIdentification139 {
    #[serde(rename = "Pty")]
    pub pty: PartyIdentification125Choice,
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
pub struct IndividualPerson8 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max35Text,
    #[validate]
    #[serde(rename = "GvnNm")]
    pub gvn_nm: Max35Text,
    #[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix1Code>,
    #[serde(rename = "NmSfx", skip_serializing_if = "Option::is_none")]
    pub nm_sfx: Option<Max35Text>,
    #[serde(rename = "Gndr", skip_serializing_if = "Option::is_none")]
    pub gndr: Option<GenderCode>,
    #[serde(rename = "BirthDt", skip_serializing_if = "Option::is_none")]
    pub birth_dt: Option<IsoDate>,
    #[serde(rename = "SclSctyNb", skip_serializing_if = "Option::is_none")]
    pub scl_scty_nb: Option<Max35Text>,
    #[validate]
    #[serde(rename = "IndvInvstrAdr")]
    pub indv_invstr_adr: PostalAddress1,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestmentFundFee1Code {
    #[serde(rename = "BEND")]
    Bend,
    #[serde(rename = "BRKF")]
    Brkf,
    #[serde(rename = "COMM")]
    Comm,
    #[serde(rename = "CDPL")]
    Cdpl,
    #[serde(rename = "CDSC")]
    Cdsc,
    #[serde(rename = "CBCH")]
    Cbch,
    #[serde(rename = "DLEV")]
    Dlev,
    #[serde(rename = "FEND")]
    Fend,
    #[serde(rename = "INIT")]
    Init,
    #[serde(rename = "ADDF")]
    Addf,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PREM")]
    Prem,
    #[serde(rename = "CHAR")]
    Char,
    #[serde(rename = "SHIP")]
    Ship,
    #[serde(rename = "SWIT")]
    Swit,
    #[serde(rename = "UCIC")]
    Ucic,
    #[serde(rename = "REGF")]
    Regf,
    #[serde(rename = "PENA")]
    Pena,
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
pub struct ImpliedCurrencyAndAmount {
    #[validate(range(min = 0,))]
    #[serde(rename = "$text")]
    pub value: f64,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OtherAsset2Code {
    #[serde(rename = "DIMA")]
    Dima,
    #[serde(rename = "EXIA")]
    Exia,
    #[serde(rename = "MOVE")]
    Move,
    #[serde(rename = "PROP")]
    Prop,
    #[serde(rename = "TIPP")]
    Tipp,
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
pub struct PensionOrderType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PensionOrderType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PensionOrderType1Choice {
    #[serde(flatten)]
    pub value: PensionOrderType1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxType17Code {
    #[serde(rename = "PROV")]
    Prov,
    #[serde(rename = "NATI")]
    Nati,
    #[serde(rename = "STAT")]
    Stat,
    #[serde(rename = "WITH")]
    With,
    #[serde(rename = "KAPA")]
    Kapa,
    #[serde(rename = "NKAP")]
    Nkap,
    #[serde(rename = "INPO")]
    Inpo,
    #[serde(rename = "STAM")]
    Stam,
    #[serde(rename = "WTAX")]
    Wtax,
    #[serde(rename = "INHT")]
    Inht,
    #[serde(rename = "SOSU")]
    Sosu,
    #[serde(rename = "CTAX")]
    Ctax,
    #[serde(rename = "GIFT")]
    Gift,
    #[serde(rename = "COAX")]
    Coax,
    #[serde(rename = "EUTR")]
    Eutr,
    #[serde(rename = "AKT1")]
    Akt1,
    #[serde(rename = "AKT2")]
    Akt2,
    #[serde(rename = "ZWIS")]
    Zwis,
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
pub struct DateAndAmount2 {
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoDate>,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalReference10 {
    #[validate]
    #[serde(rename = "Ref")]
    pub r#ref: Max35Text,
    #[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
    pub ref_issr: Option<PartyIdentification139>,
    #[serde(rename = "MsgNm", skip_serializing_if = "Option::is_none")]
    pub msg_nm: Option<Max35Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrument99 {
    #[serde(rename = "LineId", skip_serializing_if = "Option::is_none")]
    pub line_id: Option<Max35Text>,
    #[serde(rename = "Instrm")]
    pub instrm: FinancialInstrument62Choice,
    #[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<Quantity47>,
    #[validate(length(min = 0,))]
    #[serde(rename = "UnitsDtls", default)]
    pub units_dtls: Vec<Unit11>,
    #[serde(rename = "ClntRef", skip_serializing_if = "Option::is_none")]
    pub clnt_ref: Option<AdditionalReference10>,
    #[serde(rename = "CtrPtyRef", skip_serializing_if = "Option::is_none")]
    pub ctr_pty_ref: Option<AdditionalReference10>,
    #[serde(rename = "BizFlowTp", skip_serializing_if = "Option::is_none")]
    pub biz_flow_tp: Option<BusinessFlowType1Code>,
    #[serde(rename = "AvrgAcqstnPric", skip_serializing_if = "Option::is_none")]
    pub avrg_acqstn_pric: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TrfCcy", skip_serializing_if = "Option::is_none")]
    pub trf_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "TtlBookVal", skip_serializing_if = "Option::is_none")]
    pub ttl_book_val: Option<DateAndAmount2>,
    #[serde(rename = "LatstValtn", skip_serializing_if = "Option::is_none")]
    pub latst_valtn: Option<DateAndAmount2>,
    #[serde(rename = "OrgnlCost", skip_serializing_if = "Option::is_none")]
    pub orgnl_cost: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "TrfeeAcct", skip_serializing_if = "Option::is_none")]
    pub trfee_acct: Option<Account28>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Trfr", default)]
    pub trfr: Vec<Account28>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IntrmyInf", default)]
    pub intrmy_inf: Vec<Intermediary43>,
    #[validate(length(min = 0,))]
    #[serde(rename = "CrstllstnDtls", default)]
    pub crstllstn_dtls: Vec<Crystallisation2>,
    #[serde(rename = "TaxValtnPt", skip_serializing_if = "Option::is_none")]
    pub tax_valtn_pt: Option<Tax36>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SttlmPtiesDtls", default)]
    pub sttlm_pties_dtls: Vec<FundSettlementParameters17>,
    #[serde(rename = "AsstsHeldInOwnNm", skip_serializing_if = "Option::is_none")]
    pub assts_held_in_own_nm: Option<YesNoIndicator>,
    #[serde(
        rename = "TrfRsltsInChngOfBnfclOwnr",
        skip_serializing_if = "Option::is_none"
    )]
    pub trf_rslts_in_chng_of_bnfcl_ownr: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct IndianFinancialSystemCodeIdentifier {
    #[validate(regex = "INDIAN_FINANCIAL_SYSTEM_CODE_IDENTIFIER_REGEX")]
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
pub struct ChargeType5ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestmentFundFee1Code>,
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
pub struct ChargeType5Choice {
    #[serde(flatten)]
    pub value: ChargeType5ChoiceEnum,
}
#[derive(
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
pub struct PartyIdentification125ChoiceEnum {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
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
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct InnovativeFinance1 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: GenericIdentification36,
    #[validate]
    #[serde(rename = "Inf")]
    pub inf: Max350Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyTextInformation1 {
    #[serde(rename = "DclrtnDtls", skip_serializing_if = "Option::is_none")]
    pub dclrtn_dtls: Option<Max350Text>,
    #[serde(rename = "PtyCtctDtls", skip_serializing_if = "Option::is_none")]
    pub pty_ctct_dtls: Option<Max140Text>,
    #[serde(rename = "RegnDtls", skip_serializing_if = "Option::is_none")]
    pub regn_dtls: Option<Max350Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
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
pub struct OtherAsset2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OtherAsset2Code>,
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
pub struct OtherAsset2Choice {
    #[serde(flatten)]
    pub value: OtherAsset2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PensionTransferScope1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PensionTransferScope1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PensionTransferScope1Choice {
    #[serde(flatten)]
    pub value: PensionTransferScope1ChoiceEnum,
}
#[derive(
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
pub struct PriceValue1 {
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAnd13DecimalAmount,
}
#[derive(
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
pub struct OtherAmount1 {
    #[serde(rename = "Tp")]
    pub tp: OtherAmountType1Choice,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PensionSchemeType3ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PensionSchemeType3Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PensionSchemeType3Choice {
    #[serde(flatten)]
    pub value: PensionSchemeType3ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FundPortfolio7ChoiceEnum {
    #[serde(rename = "GnlInvstmt", skip_serializing_if = "Option::is_none")]
    pub gnl_invstmt: Option<GeneralInvestment2>,
    #[serde(rename = "TaxEffcntPdct", skip_serializing_if = "Option::is_none")]
    pub tax_effcnt_pdct: Option<TaxEfficientProduct7>,
    #[serde(rename = "Pnsn", skip_serializing_if = "Option::is_none")]
    pub pnsn: Option<Pension5>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FundPortfolio7Choice {
    #[serde(flatten)]
    pub value: FundPortfolio7ChoiceEnum,
}
#[derive(
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
#[derive(
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
pub struct ActiveOrHistoricCurrencyAndAmount {
    #[serde(rename = "ActiveOrHistoricCurrencyAndAmount")]
    pub value: ActiveOrHistoricCurrencyAndAmountSimpleType,
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
pub struct DrawdownType2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<DrawdownType2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct DrawdownType2Choice {
    #[serde(flatten)]
    pub value: DrawdownType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct OtherAmount3 {
    #[validate]
    #[serde(rename = "Tp")]
    pub tp: GenericIdentification1,
    #[validate]
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
}
#[derive(
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
pub enum GenderCode {
    #[serde(rename = "MALE")]
    Male,
    #[serde(rename = "FEMA")]
    Fema,
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
pub struct PartyIdentification132 {
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
    #[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2Choice>,
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
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
pub struct Quantity47 {
    #[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<DecimalNumber>,
    #[serde(rename = "PctgRate", skip_serializing_if = "Option::is_none")]
    pub pctg_rate: Option<PercentageRate>,
    #[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
    pub face_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
    pub amtsd_val: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "CshAmt", skip_serializing_if = "Option::is_none")]
    pub csh_amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "OthrAsst", skip_serializing_if = "Option::is_none")]
    pub othr_asst: Option<Max35Text>,
}
#[derive(
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
pub struct PartyIdentification123ChoiceEnum {
    #[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
    pub nm_and_adr: Option<NameAndAddress5>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
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
pub struct PartyIdentification123Choice {
    #[serde(flatten)]
    pub value: PartyIdentification123ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum CashAssetType1Code {
    #[serde(rename = "CSH2")]
    Csh2,
    #[serde(rename = "CSH1")]
    Csh1,
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
pub struct OtherAsset2 {
    #[serde(rename = "OthrAsstTp")]
    pub othr_asst_tp: OtherAsset2Choice,
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max35Text>,
    #[validate(length(min = 0, max = 5,))]
    #[serde(rename = "OthrId", default)]
    pub othr_id: Vec<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
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
#[derive(
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
pub struct DateQuarter1ChoiceEnum {
    #[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
    pub prd: Option<Max35Text>,
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
pub struct DateQuarter1Choice {
    #[serde(flatten)]
    pub value: DateQuarter1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxEfficientProductType2Code {
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CLIS")]
    Clis,
    #[serde(rename = "FISA")]
    Fisa,
    #[serde(rename = "GISK")]
    Gisk,
    #[serde(rename = "GASK")]
    Gask,
    #[serde(rename = "HISA")]
    Hisa,
    #[serde(rename = "INNF")]
    Innf,
    #[serde(rename = "JCSH")]
    Jcsh,
    #[serde(rename = "JISA")]
    Jisa,
    #[serde(rename = "LISA")]
    Lisa,
    #[serde(rename = "CCTF")]
    Cctf,
    #[serde(rename = "SCTF")]
    Sctf,
    #[serde(rename = "EQUI")]
    Equi,
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
#[serde(rename = "Document")]
pub struct Document {
    #[validate]
    #[serde(rename = "AcctHldgInf")]
    pub acct_hldg_inf: AccountHoldingInformationV09,
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
pub struct SettlementParties94 {
    #[serde(rename = "Dpstry", skip_serializing_if = "Option::is_none")]
    pub dpstry: Option<PartyIdentification141>,
    #[serde(rename = "Pty1", skip_serializing_if = "Option::is_none")]
    pub pty_1: Option<PartyIdentificationAndAccount194>,
    #[serde(rename = "Pty2", skip_serializing_if = "Option::is_none")]
    pub pty_2: Option<PartyIdentificationAndAccount194>,
    #[serde(rename = "Pty3", skip_serializing_if = "Option::is_none")]
    pub pty_3: Option<PartyIdentificationAndAccount194>,
    #[serde(rename = "Pty4", skip_serializing_if = "Option::is_none")]
    pub pty_4: Option<PartyIdentificationAndAccount194>,
    #[serde(rename = "Pty5", skip_serializing_if = "Option::is_none")]
    pub pty_5: Option<PartyIdentificationAndAccount194>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxReferenceType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PersonIdentificationType7Code>,
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
pub struct TaxReferenceType1Choice {
    #[serde(flatten)]
    pub value: TaxReferenceType1ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SettlementTransactionCondition30ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<SettlementTransactionCondition11Code>,
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
pub struct SettlementTransactionCondition30Choice {
    #[serde(flatten)]
    pub value: SettlementTransactionCondition30ChoiceEnum,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxWrapperAmountType1Code {
    #[serde(rename = "BONU")]
    Bonu,
    #[serde(rename = "WTHD")]
    Wthd,
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
pub struct PartyIdentification140 {
    #[serde(rename = "Pty")]
    pub pty: PartyIdentification126Choice,
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
pub struct TaxableIncomePerShareCalculated2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxableIncomePerShareCalculated2Code>,
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
pub struct TaxableIncomePerShareCalculated2Choice {
    #[serde(flatten)]
    pub value: TaxableIncomePerShareCalculated2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum BusinessFlowType1Code {
    #[serde(rename = "SLDP")]
    Sldp,
    #[serde(rename = "SLRP")]
    Slrp,
    #[serde(rename = "DLPR")]
    Dlpr,
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
pub struct IsoDate {
    #[serde(rename = "$text")]
    pub value: ::chrono::NaiveDate,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PensionOrderType1Code {
    #[serde(rename = "EARM")]
    Earm,
    #[serde(rename = "SHAR")]
    Shar,
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
pub struct SecuritiesAccount19 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification30>,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxReference2 {
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<TaxReferenceType1Choice>,
    #[validate]
    #[serde(rename = "Ref")]
    pub r#ref: Max35Text,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TotalFeesAndTaxes41 {
    #[serde(rename = "TtlOvrhdApld", skip_serializing_if = "Option::is_none")]
    pub ttl_ovrhd_apld: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TtlFees", skip_serializing_if = "Option::is_none")]
    pub ttl_fees: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "TtlTaxs", skip_serializing_if = "Option::is_none")]
    pub ttl_taxs: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ComrclAgrmtRef", skip_serializing_if = "Option::is_none")]
    pub comrcl_agrmt_ref: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IndvFee", default)]
    pub indv_fee: Vec<Fee5>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IndvTax", default)]
    pub indv_tax: Vec<Tax35>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct MoneyPurchaseAnnualAllowance1 {
    #[validate]
    #[serde(rename = "Trggrd")]
    pub trggrd: YesNoIndicator,
    #[serde(rename = "TrggrdDt", skip_serializing_if = "Option::is_none")]
    pub trggrd_dt: Option<IsoDate>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfPrice10Code {
    #[serde(rename = "BIDE")]
    Bide,
    #[serde(rename = "OFFR")]
    Offr,
    #[serde(rename = "NAVL")]
    Navl,
    #[serde(rename = "CREA")]
    Crea,
    #[serde(rename = "CANC")]
    Canc,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "SWNG")]
    Swng,
    #[serde(rename = "MIDD")]
    Midd,
    #[serde(rename = "RINV")]
    Rinv,
    #[serde(rename = "SWIC")]
    Swic,
    #[serde(rename = "DDVR")]
    Ddvr,
    #[serde(rename = "ACTU")]
    Actu,
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
pub struct LumpSumType1ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<LumpSumType1Code>,
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
pub struct LumpSumType1Choice {
    #[serde(flatten)]
    pub value: LumpSumType1ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum OtherAmountType1Code {
    #[serde(rename = "PINT")]
    Pint,
    #[serde(rename = "SINT")]
    Sint,
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
pub struct PortfolioTransfer9 {
    #[serde(rename = "MstrRef", skip_serializing_if = "Option::is_none")]
    pub mstr_ref: Option<Max35Text>,
    #[validate]
    #[serde(rename = "TrfId")]
    pub trf_id: Max35Text,
    #[serde(rename = "TrfConfId", skip_serializing_if = "Option::is_none")]
    pub trf_conf_id: Option<Max35Text>,
    #[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
    pub prtfl: Option<FundPortfolio7Choice>,
    #[serde(rename = "PrtlDscvry", skip_serializing_if = "Option::is_none")]
    pub prtl_dscvry: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "RsdlCsh", default)]
    pub rsdl_csh: Vec<ResidualCash1>,
    #[serde(rename = "TaxDt", skip_serializing_if = "Option::is_none")]
    pub tax_dt: Option<IsoDate>,
    #[validate(length(min = 0,))]
    #[serde(rename = "FinInstrmAsstForTrf", default)]
    pub fin_instrm_asst_for_trf: Vec<FinancialInstrument99>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
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
#[derive(
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
pub struct Account28 {
    #[serde(rename = "OwnrId", skip_serializing_if = "Option::is_none")]
    pub ownr_id: Option<PartyIdentification132>,
    #[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<Max35Text>,
    #[serde(rename = "Dsgnt", skip_serializing_if = "Option::is_none")]
    pub dsgnt: Option<Max35Text>,
    #[serde(rename = "AcctNm", skip_serializing_if = "Option::is_none")]
    pub acct_nm: Option<Max35Text>,
    #[serde(rename = "Svcr", skip_serializing_if = "Option::is_none")]
    pub svcr: Option<PartyIdentification132>,
    #[serde(rename = "SubAcctDtls", skip_serializing_if = "Option::is_none")]
    pub sub_acct_dtls: Option<SubAccount5>,
}
#[derive(
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
pub enum DrawdownType2Code {
    #[serde(rename = "BOTH")]
    Both,
    #[serde(rename = "CAPP")]
    Capp,
    #[serde(rename = "FLEX")]
    Flex,
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
pub struct InvestmentAccount69 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Dsgnt", skip_serializing_if = "Option::is_none")]
    pub dsgnt: Option<Max35Text>,
    #[serde(rename = "AcctNm", skip_serializing_if = "Option::is_none")]
    pub acct_nm: Option<Max35Text>,
    #[serde(rename = "Svcr", skip_serializing_if = "Option::is_none")]
    pub svcr: Option<PartyIdentification132>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PartyTextInformation6 {
    #[serde(rename = "DclrtnDtls", skip_serializing_if = "Option::is_none")]
    pub dclrtn_dtls: Option<Max350Text>,
    #[serde(rename = "PtyCtctDtls", skip_serializing_if = "Option::is_none")]
    pub pty_ctct_dtls: Option<Max140Text>,
    #[serde(rename = "RegnDtls", skip_serializing_if = "Option::is_none")]
    pub regn_dtls: Option<Max350Text>,
    #[serde(rename = "RegnAdr", skip_serializing_if = "Option::is_none")]
    pub regn_adr: Option<PostalAddress1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PreviousYear4 {
    #[serde(rename = "PrvsYrs")]
    pub prvs_yrs: PreviousYear1Choice,
    #[serde(rename = "CshCmpntInd", skip_serializing_if = "Option::is_none")]
    pub csh_cmpnt_ind: Option<YesNoIndicator>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct ResidualCash1 {
    #[validate]
    #[serde(rename = "RsdlCshInd")]
    pub rsdl_csh_ind: YesNoIndicator,
    #[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveCurrencyCode>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AccountHoldingInformationV09 {
    #[validate]
    #[serde(rename = "MsgRef")]
    pub msg_ref: MessageIdentification1,
    #[serde(rename = "PoolRef", skip_serializing_if = "Option::is_none")]
    pub pool_ref: Option<AdditionalReference11>,
    #[serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none")]
    pub prvs_ref: Option<AdditionalReference10>,
    #[serde(rename = "RltdRef", skip_serializing_if = "Option::is_none")]
    pub rltd_ref: Option<AdditionalReference10>,
    #[serde(rename = "BizFlowDrctnTp", skip_serializing_if = "Option::is_none")]
    pub biz_flow_drctn_tp: Option<BusinessFlowDirectionType1Code>,
    #[serde(rename = "PmryIndvInvstr", skip_serializing_if = "Option::is_none")]
    pub pmry_indv_invstr: Option<IndividualPerson8>,
    #[serde(rename = "ScndryIndvInvstr", skip_serializing_if = "Option::is_none")]
    pub scndry_indv_invstr: Option<IndividualPerson8>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrIndvInvstr", default)]
    pub othr_indv_invstr: Vec<IndividualPerson8>,
    #[serde(rename = "PmryCorpInvstr", skip_serializing_if = "Option::is_none")]
    pub pmry_corp_invstr: Option<Organisation36>,
    #[serde(rename = "ScndryCorpInvstr", skip_serializing_if = "Option::is_none")]
    pub scndry_corp_invstr: Option<Organisation36>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrCorpInvstr", default)]
    pub othr_corp_invstr: Vec<Organisation36>,
    #[validate]
    #[serde(rename = "TrfrAcct")]
    pub trfr_acct: InvestmentAccount69,
    #[serde(rename = "NmneeAcct", skip_serializing_if = "Option::is_none")]
    pub nmnee_acct: Option<InvestmentAccount69>,
    #[validate]
    #[serde(rename = "Trfee")]
    pub trfee: PartyIdentification132,
    #[validate(length(min = 1,))]
    #[serde(rename = "PdctTrf", default)]
    pub pdct_trf: Vec<PortfolioTransfer9>,
    #[serde(rename = "MktPrctcVrsn", skip_serializing_if = "Option::is_none")]
    pub mkt_prctc_vrsn: Option<MarketPracticeVersion1>,
    #[validate(length(min = 0,))]
    #[serde(rename = "Xtnsn", default)]
    pub xtnsn: Vec<Extension1>,
}
#[derive(
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
pub struct Max70Text {
    #[validate(length(min = 1, max = 70,))]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TradeTransactionCondition5Code {
    #[serde(rename = "XCPN")]
    Xcpn,
    #[serde(rename = "CCPN")]
    Ccpn,
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
pub struct PartyIdentification141 {
    #[serde(rename = "Id")]
    pub id: PartyIdentification122Choice,
    #[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
    pub lei: Option<LeiIdentifier>,
    #[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
    pub altrn_id: Option<AlternatePartyIdentification7>,
    #[serde(rename = "PrcgDt", skip_serializing_if = "Option::is_none")]
    pub prcg_dt: Option<DateAndDateTime2Choice>,
    #[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<PartyTextInformation1>,
}
#[derive(
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
pub struct UkDomesticSortCodeIdentifier {
    #[validate(regex = "UK_DOMESTIC_SORT_CODE_IDENTIFIER_REGEX")]
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
pub struct AlternatePartyIdentification7 {
    #[serde(rename = "IdTp")]
    pub id_tp: IdentificationType42Choice,
    #[serde(rename = "Ctry")]
    pub ctry: CountryCode,
    #[validate]
    #[serde(rename = "AltrnId")]
    pub altrn_id: Max35Text,
}
#[derive(
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
pub struct TaxEfficientProduct7 {
    #[serde(rename = "TaxEffcntPdctTp")]
    pub tax_effcnt_pdct_tp: TaxEfficientProductType2Choice,
    #[serde(rename = "CurYr", skip_serializing_if = "Option::is_none")]
    pub cur_yr: Option<YesNoIndicator>,
    #[serde(rename = "CshCmpntInd", skip_serializing_if = "Option::is_none")]
    pub csh_cmpnt_ind: Option<YesNoIndicator>,
    #[serde(rename = "PrvsYrs", skip_serializing_if = "Option::is_none")]
    pub prvs_yrs: Option<PreviousYear4>,
    #[serde(rename = "PrvsYrSbcptAmt", skip_serializing_if = "Option::is_none")]
    pub prvs_yr_sbcpt_amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "PrvsYrsSbcptAmt", skip_serializing_if = "Option::is_none")]
    pub prvs_yrs_sbcpt_amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "DtOfFrstSbcpt", skip_serializing_if = "Option::is_none")]
    pub dt_of_frst_sbcpt: Option<IsoDate>,
    #[serde(rename = "CurYrSbcptDtls", skip_serializing_if = "Option::is_none")]
    pub cur_yr_sbcpt_dtls: Option<SubscriptionInformation2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "BnsOrWdrwl", default)]
    pub bns_or_wdrwl: Vec<BonusWithdrawal2>,
    #[serde(
        rename = "WdrwlForResdtlPurchsPrgrs",
        skip_serializing_if = "Option::is_none"
    )]
    pub wdrwl_for_resdtl_purchs_prgrs: Option<YesNoIndicator>,
    #[serde(rename = "TrfrAltrnId", skip_serializing_if = "Option::is_none")]
    pub trfr_altrn_id: Option<Max35Text>,
    #[serde(rename = "TtlSbcptAmt", skip_serializing_if = "Option::is_none")]
    pub ttl_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "OthrAmt", default)]
    pub othr_amt: Vec<OtherAmount3>,
    #[serde(rename = "DtFrstQlfygAddtn", skip_serializing_if = "Option::is_none")]
    pub dt_frst_qlfyg_addtn: Option<IsoDate>,
    #[serde(rename = "InvstrTaxRef", skip_serializing_if = "Option::is_none")]
    pub invstr_tax_ref: Option<TaxReference2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InvstmtsToFllwVal", default)]
    pub invstmts_to_fllw_val: Vec<DateAndAmount2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "InnvtvFinc", default)]
    pub innvtv_finc: Vec<InnovativeFinance1>,
    #[serde(rename = "LwstInvstdAmtCurYr", skip_serializing_if = "Option::is_none")]
    pub lwst_invstd_amt_cur_yr: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "TaxClctnBase", skip_serializing_if = "Option::is_none")]
    pub tax_clctn_base: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "UusdTaxDdctn", skip_serializing_if = "Option::is_none")]
    pub uusd_tax_ddctn: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "CurInvstmtAmt", skip_serializing_if = "Option::is_none")]
    pub cur_invstmt_amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "EstmtdVal", skip_serializing_if = "Option::is_none")]
    pub estmtd_val: Option<DateAndAmount2>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
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
pub struct CanadianPaymentsArnIdentifier {
    #[validate(regex = "CANADIAN_PAYMENTS_ARN_IDENTIFIER_REGEX")]
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
pub struct CashAssetType1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<CashAssetType1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct CashAssetType1Choice {
    #[serde(flatten)]
    pub value: CashAssetType1ChoiceEnum,
}
#[derive(
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
pub enum HolderType1Code {
    #[serde(rename = "TFEE")]
    Tfee,
    #[serde(rename = "TFOR")]
    Tfor,
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
pub struct FinancialInstrumentIdentification4 {
    #[serde(rename = "Id")]
    pub id: SecurityIdentification25Choice,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<ClassificationType32Choice>,
    #[serde(rename = "AltrnSctyTp", skip_serializing_if = "Option::is_none")]
    pub altrn_scty_tp: Option<GenericIdentification1>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct Fee5 {
    #[serde(rename = "Tp")]
    pub tp: ChargeType5Choice,
    #[serde(rename = "Bsis", skip_serializing_if = "Option::is_none")]
    pub bsis: Option<ChargeBasis2Choice>,
    #[serde(rename = "StdAmt", skip_serializing_if = "Option::is_none")]
    pub std_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "StdRate", skip_serializing_if = "Option::is_none")]
    pub std_rate: Option<PercentageRate>,
    #[serde(rename = "DscntDtls", skip_serializing_if = "Option::is_none")]
    pub dscnt_dtls: Option<ChargeOrCommissionDiscount1>,
    #[serde(rename = "ApldAmt", skip_serializing_if = "Option::is_none")]
    pub apld_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "ApldRate", skip_serializing_if = "Option::is_none")]
    pub apld_rate: Option<PercentageRate>,
    #[serde(rename = "NonStdSLARef", skip_serializing_if = "Option::is_none")]
    pub non_std_sla_ref: Option<Max35Text>,
    #[serde(rename = "RcptId", skip_serializing_if = "Option::is_none")]
    pub rcpt_id: Option<PartyIdentification139>,
    #[validate]
    #[serde(rename = "InftvInd")]
    pub inftv_ind: YesNoIndicator,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeneralInvestmentAccountType2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<GeneralInvestmentAccountType2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct GeneralInvestmentAccountType2Choice {
    #[serde(flatten)]
    pub value: GeneralInvestmentAccountType2ChoiceEnum,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum PersonIdentificationType7Code {
    #[serde(rename = "ATIN")]
    Atin,
    #[serde(rename = "GTIN")]
    Gtin,
    #[serde(rename = "ITIN")]
    Itin,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestmentFundRole2Code {
    #[serde(rename = "FMCO")]
    Fmco,
    #[serde(rename = "REGI")]
    Regi,
    #[serde(rename = "TRAG")]
    Trag,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "DIST")]
    Dist,
    #[serde(rename = "CONC")]
    Conc,
    #[serde(rename = "UCL1")]
    Ucl1,
    #[serde(rename = "UCL2")]
    Ucl2,
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
pub struct SecurityIdentification25ChoiceEnum {
    #[serde(rename = "RIC", skip_serializing_if = "Option::is_none")]
    pub ric: Option<RicIdentifier>,
    #[serde(rename = "OthrPrtryId", skip_serializing_if = "Option::is_none")]
    pub othr_prtry_id: Option<AlternateSecurityIdentification7>,
    #[serde(rename = "QUICK", skip_serializing_if = "Option::is_none")]
    pub quick: Option<QuickIdentifier>,
    #[serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none")]
    pub tckr_symb: Option<TickerIdentifier>,
    #[serde(rename = "Vlrn", skip_serializing_if = "Option::is_none")]
    pub vlrn: Option<ValorenIdentifier>,
    #[serde(rename = "CUSIP", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<CusipIdentifier>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "Cmon", skip_serializing_if = "Option::is_none")]
    pub cmon: Option<EuroclearClearstreamIdentifier>,
    #[serde(rename = "Wrtppr", skip_serializing_if = "Option::is_none")]
    pub wrtppr: Option<WertpapierIdentifier>,
    #[serde(rename = "SEDOL", skip_serializing_if = "Option::is_none")]
    pub sedol: Option<SedolIdentifier>,
    #[serde(rename = "CTA", skip_serializing_if = "Option::is_none")]
    pub cta: Option<ConsolidatedTapeAssociationIdentifier>,
    #[serde(rename = "Dtch", skip_serializing_if = "Option::is_none")]
    pub dtch: Option<DutchIdentifier>,
    #[serde(rename = "SCVM", skip_serializing_if = "Option::is_none")]
    pub scvm: Option<SicovamIdentifier>,
    #[serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none")]
    pub blmbrg: Option<Bloomberg2Identifier>,
    #[serde(rename = "Belgn", skip_serializing_if = "Option::is_none")]
    pub belgn: Option<BelgianIdentifier>,
}
#[derive(
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
pub struct FinancialInstrumentIdentification3 {
    #[serde(rename = "Id")]
    pub id: SecurityIdentification25Choice,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<ClassificationType32Choice>,
    #[serde(rename = "RstrctdInd", skip_serializing_if = "Option::is_none")]
    pub rstrctd_ind: Option<YesNoIndicator>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AltrnScty", default)]
    pub altrn_scty: Vec<FinancialInstrumentIdentification4>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxReference1 {
    #[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
    pub tax_tp: Option<TaxReferenceType1Choice>,
    #[validate]
    #[serde(rename = "Ref")]
    pub r#ref: Max35Text,
    #[serde(rename = "HldrTp", skip_serializing_if = "Option::is_none")]
    pub hldr_tp: Option<TaxReferenceParty1Choice>,
}
#[derive(
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
pub struct TradeTransactionCondition8ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TradeTransactionCondition5Code>,
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
pub struct TradeTransactionCondition8Choice {
    #[serde(flatten)]
    pub value: TradeTransactionCondition8ChoiceEnum,
}
#[derive(
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
pub struct TaxEfficientProductType2ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxEfficientProductType2Code>,
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
pub struct TaxEfficientProductType2Choice {
    #[serde(flatten)]
    pub value: TaxEfficientProductType2ChoiceEnum,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TypeOfPrice46ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfPrice10Code>,
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
pub struct TypeOfPrice46Choice {
    #[serde(flatten)]
    pub value: TypeOfPrice46ChoiceEnum,
}
#[derive(
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
pub struct CashAsset2 {
    #[serde(rename = "CshAsstTp")]
    pub csh_asst_tp: CashAssetType1Choice,
    #[serde(rename = "HldgCcy")]
    pub hldg_ccy: ActiveCurrencyCode,
    #[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<AdditionalInformation15>,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum LumpSumType1Code {
    #[serde(rename = "DEAB")]
    Deab,
    #[serde(rename = "PCLS")]
    Pcls,
    #[serde(rename = "UFPL")]
    Ufpl,
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
pub struct Crystallisation2 {
    #[validate]
    #[serde(rename = "TrchId")]
    pub trch_id: Max35Text,
    #[serde(rename = "CrstllsdUnitsNb", skip_serializing_if = "Option::is_none")]
    pub crstllsd_units_nb: Option<DecimalNumber>,
    #[serde(rename = "UcrstllsdUnitsNb", skip_serializing_if = "Option::is_none")]
    pub ucrstllsd_units_nb: Option<DecimalNumber>,
    #[serde(rename = "CrstllsdAmt", skip_serializing_if = "Option::is_none")]
    pub crstllsd_amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "UcrstllsdAmt", skip_serializing_if = "Option::is_none")]
    pub ucrstllsd_amt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct BonusWithdrawal2 {
    #[serde(rename = "TpOfAmt")]
    pub tp_of_amt: TypeOfAmount1Choice,
    #[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
    #[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
    pub rsn: Option<WithdrawalReason1Choice>,
    #[serde(rename = "UclmdAmt", skip_serializing_if = "Option::is_none")]
    pub uclmd_amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
    #[serde(rename = "Outsdng", skip_serializing_if = "Option::is_none")]
    pub outsdng: Option<YesNoIndicator>,
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<Max35Text>,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
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
pub struct Tax36 {
    #[serde(rename = "DtOrPrd")]
    pub dt_or_prd: DateQuarter1Choice,
    #[validate(length(min = 0,))]
    #[serde(rename = "AddtlInf", default)]
    pub addtl_inf: Vec<AdditionalInformation15>,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TaxationBasis2Code {
    #[serde(rename = "FLAT")]
    Flat,
    #[serde(rename = "PERU")]
    Peru,
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
pub struct Account27 {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[validate]
    #[serde(rename = "AcctSvcr")]
    pub acct_svcr: PartyIdentification139,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct SubscriptionInformation2 {
    #[validate]
    #[serde(rename = "DtOfFrstSbcpt")]
    pub dt_of_frst_sbcpt: IsoDate,
    #[serde(rename = "EqtyCmpnt", skip_serializing_if = "Option::is_none")]
    pub eqty_cmpnt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[serde(rename = "CshCmpnt", skip_serializing_if = "Option::is_none")]
    pub csh_cmpnt: Option<ActiveCurrencyAnd13DecimalAmount>,
    #[validate]
    #[serde(rename = "TtlAmtYrToDt")]
    pub ttl_amt_yr_to_dt: ActiveCurrencyAnd13DecimalAmount,
}
#[derive(
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
pub struct TaxBasis1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification47>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TaxationBasis2Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TaxBasis1Choice {
    #[serde(flatten)]
    pub value: TaxBasis1ChoiceEnum,
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfIdentification1Code {
    #[serde(rename = "ARNU")]
    Arnu,
    #[serde(rename = "CCPT")]
    Ccpt,
    #[serde(rename = "CHTY")]
    Chty,
    #[serde(rename = "CORP")]
    Corp,
    #[serde(rename = "DRLC")]
    Drlc,
    #[serde(rename = "FIIN")]
    Fiin,
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
pub struct Organisation36 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max140Text,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<PartyIdentification140>,
    #[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
    pub purp: Option<Max35Text>,
    #[serde(rename = "TaxtnCtry", skip_serializing_if = "Option::is_none")]
    pub taxtn_ctry: Option<CountryCode>,
    #[serde(rename = "RegnCtry", skip_serializing_if = "Option::is_none")]
    pub regn_ctry: Option<CountryCode>,
    #[serde(rename = "RegnDt", skip_serializing_if = "Option::is_none")]
    pub regn_dt: Option<IsoDate>,
    #[serde(rename = "TaxIdNb", skip_serializing_if = "Option::is_none")]
    pub tax_id_nb: Option<Max35Text>,
    #[serde(rename = "NtlRegnNb", skip_serializing_if = "Option::is_none")]
    pub ntl_regn_nb: Option<Max35Text>,
    #[validate]
    #[serde(rename = "CorpInvstrAdr")]
    pub corp_invstr_adr: PostalAddress1,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct PolishNationalClearingCodeIdentifier {
    #[validate(regex = "POLISH_NATIONAL_CLEARING_CODE_IDENTIFIER_REGEX")]
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
pub struct WithdrawalReason1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<PortfolioWithdrawalReason1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct WithdrawalReason1Choice {
    #[serde(flatten)]
    pub value: WithdrawalReason1ChoiceEnum,
}
#[derive(
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
pub struct AdditionalInformation15 {
    #[validate]
    #[serde(rename = "InfTp")]
    pub inf_tp: GenericIdentification36,
    #[validate]
    #[serde(rename = "InfVal")]
    pub inf_val: Max350Text,
}
#[derive(
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
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum DrawdownStatus1Code {
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "NONE")]
    None,
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
pub struct FundSettlementParameters17 {
    #[serde(rename = "SfkpgPlc", skip_serializing_if = "Option::is_none")]
    pub sfkpg_plc: Option<SafekeepingPlaceFormat28Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "TradTxCond", default)]
    pub trad_tx_cond: Vec<TradeTransactionCondition8Choice>,
    #[validate(length(min = 0,))]
    #[serde(rename = "SttlmTxCond", default)]
    pub sttlm_tx_cond: Vec<SettlementTransactionCondition30Choice>,
    #[serde(rename = "SctiesSttlmSysId", skip_serializing_if = "Option::is_none")]
    pub scties_sttlm_sys_id: Option<Max35Text>,
    #[serde(rename = "RcvgSdDtls", skip_serializing_if = "Option::is_none")]
    pub rcvg_sd_dtls: Option<SettlementParties94>,
    #[serde(rename = "DlvrgSdDtls", skip_serializing_if = "Option::is_none")]
    pub dlvrg_sd_dtls: Option<SettlementParties94>,
}
#[derive(
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
pub struct PreviousAll {
    #[validate(regex = "PREVIOUS_ALL_REGEX")]
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
pub struct TickerIdentifier {
    #[validate(length(min = 1, max = 35,))]
    #[serde(rename = "$text")]
    pub value: String,
}
