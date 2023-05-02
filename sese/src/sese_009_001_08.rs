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
    static ref ACTIVE_CURRENCY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref POLISH_NATIONAL_CLEARING_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"PL[0-9]{8,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SWISS_SIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SW[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref IRISH_NSC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"IE[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref LEI_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{18,18}[0-9]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CANADIAN_PAYMENTS_ARN_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CA[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref RUSSIAN_CENTRAL_BANK_IDENTIFICATION_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"RU[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref UK_DOMESTIC_SORT_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SC[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CHIPS_UNIVERSAL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CH[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SMALL_NETWORK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AU[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref NEW_ZEALAND_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"NZ[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref HONG_KONG_BANK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"HK[0-9]{3,3}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref BLOOMBERG_2_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"(BBG)[BCDFGHJKLMNPQRSTVWXYZ\d]{8}\d"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref HELLENIC_BANK_IDENTIFICATION_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"GR[0-9]{7,7}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PHONE_NUMBER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"\+[0-9]{1,3}-[0-9()+\-]{1,30}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SPANISH_DOMESTIC_INTERBANKING_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"ES[0-9]{8,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISO_YEAR_MONTH_REGEX: ::regex::Regex = ::regex::Regex::new(r#"^-?\d{4}-(0[1-9]|1[0-2])([+-]\d{2}:\d{2}|Z)?$"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SWISS_BC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"SW[0-9]{3,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ISIN_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref PORTUGUESE_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"PT[0-9]{8,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref COUNTRY_CODE_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{2,2}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ANY_BIC_DEC_2014_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref AUSTRIAN_BANKLEITZAHL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AT[0-9]{5,5}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXTENSIVE_BRANCH_NETWORK_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"AU[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref FEDWIRE_ROUTING_NUMBER_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"FW[0-9]{9,9}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref EXACT_4_ALPHA_NUMERIC_TEXT_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[a-zA-Z0-9]{4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref INDIAN_FINANCIAL_SYSTEM_CODE_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"IN[a-zA-Z0-9]{11,11}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref SOUTH_AFRICAN_NCC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"ZA[0-9]{6,6}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CHIPS_PARTICIPANT_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"CP[0-9]{4,4}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref ITALIAN_DOMESTIC_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"IT[0-9]{10,10}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref GERMAN_BANKLEITZAHL_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"BL[0-9]{8,8}"#).unwrap();
}

::lazy_static::lazy_static! {
    static ref CFI_OCT_2015_IDENTIFIER_REGEX: ::regex::Regex = ::regex::Regex::new(r#"[A-Z]{6,6}"#).unwrap();
}

/// Returns the namespace of the schema
pub fn namespace() -> String {
    "urn:iso:std:iso:20022:tech:xsd:sese.009.001.08".to_string()
}

#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum RejectedStatusReason12Code {
    #[serde(rename = "BLCA")]
    Blca,
    #[serde(rename = "DOCC")]
    Docc,
    #[serde(rename = "IAQD")]
    Iaqd,
    #[serde(rename = "ICTN")]
    Ictn,
    #[serde(rename = "CYPA")]
    Cypa,
    #[serde(rename = "TREF")]
    Tref,
    #[serde(rename = "DSEC")]
    Dsec,
    #[serde(rename = "IDNA")]
    Idna,
    #[serde(rename = "DQUA")]
    Dqua,
    #[serde(rename = "FTAX")]
    Ftax,
    #[serde(rename = "INID")]
    Inid,
    #[serde(rename = "INAC")]
    Inac,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "INPM")]
    Inpm,
    #[serde(rename = "INNA")]
    Inna,
    #[serde(rename = "SAFE")]
    Safe,
    #[serde(rename = "INUK")]
    Inuk,
    #[serde(rename = "LEGL")]
    Legl,
    #[serde(rename = "NSLA")]
    Nsla,
    #[serde(rename = "SECU")]
    Secu,
    #[serde(rename = "PTNS")]
    Ptns,
    #[serde(rename = "DLVY")]
    Dlvy,
    #[serde(rename = "DDAT")]
    Ddat,
    #[serde(rename = "ISTP")]
    Istp,
    #[serde(rename = "DEPT")]
    Dept,
    #[serde(rename = "ISAT")]
    Isat,
    #[serde(rename = "OTHR")]
    Othr,
    #[serde(rename = "NCON")]
    Ncon,
    #[serde(rename = "ACLO")]
    Aclo,
    #[serde(rename = "NASS")]
    Nass,
    #[serde(rename = "NQTY")]
    Nqty,
    #[serde(rename = "BLTR")]
    Bltr,
    #[serde(rename = "COSE")]
    Cose,
    #[serde(rename = "ILLI")]
    Illi,
    #[serde(rename = "BMRV")]
    Bmrv,
    #[serde(rename = "DINV")]
    Dinv,
    #[serde(rename = "ICAG")]
    Icag,
    #[serde(rename = "IPAC")]
    Ipac,
    #[serde(rename = "INTE")]
    Inte,
    #[serde(rename = "DMON")]
    Dmon,
    #[serde(rename = "PRCT")]
    Prct,
    #[serde(rename = "IVAG")]
    Ivag,
    #[serde(rename = "NCRR")]
    Ncrr,
    #[serde(rename = "DTRD")]
    Dtrd,
    #[serde(rename = "UPAY")]
    Upay,
    #[serde(rename = "URSC")]
    Ursc,
    #[default]
    Unknown,
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
pub struct SwissSicIdentifier {
    #[validate(regex = "SWISS_SIC_IDENTIFIER_REGEX")]
    #[serde(rename = "$text")]
    pub value: String,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum TypeOfRequest1Code {
    #[serde(rename = "INFO")]
    Info,
    #[serde(rename = "STAT")]
    Stat,
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
pub struct Account33 {
    #[serde(rename = "OwnrId", skip_serializing_if = "Option::is_none")]
    pub ownr_id: Option<PartyIdentification132>,
    #[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<Max35Text>,
    #[serde(rename = "AcctDsgnt", skip_serializing_if = "Option::is_none")]
    pub acct_dsgnt: Option<Max35Text>,
    #[serde(rename = "AcctNm", skip_serializing_if = "Option::is_none")]
    pub acct_nm: Option<Max35Text>,
    #[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification30>,
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
pub struct MarketPracticeVersion1 {
    #[validate]
    #[serde(rename = "Nm")]
    pub nm: Max35Text,
    #[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
    pub dt: Option<IsoYearMonth>,
    #[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
    pub nb: Option<Max35Text>,
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
pub struct TypeOfRequest1ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<TypeOfRequest1Code>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct TypeOfRequest1Choice {
    #[serde(flatten)]
    pub value: TypeOfRequest1ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "GRHEBIC", skip_serializing_if = "Option::is_none")]
    pub grhebic: Option<HellenicBankIdentificationCodeIdentifier>,
    #[serde(rename = "CHBC", skip_serializing_if = "Option::is_none")]
    pub chbc: Option<SwissBcIdentifier>,
    #[serde(rename = "ZANCC", skip_serializing_if = "Option::is_none")]
    pub zancc: Option<SouthAfricanNccIdentifier>,
    #[serde(rename = "AUBSBs", skip_serializing_if = "Option::is_none")]
    pub aubs_bs: Option<SmallNetworkIdentifier>,
    #[serde(rename = "USFW", skip_serializing_if = "Option::is_none")]
    pub usfw: Option<FedwireRoutingNumberIdentifier>,
    #[serde(rename = "USCHU", skip_serializing_if = "Option::is_none")]
    pub uschu: Option<ChipsUniversalIdentifier>,
    #[serde(rename = "GBSC", skip_serializing_if = "Option::is_none")]
    pub gbsc: Option<UkDomesticSortCodeIdentifier>,
    #[serde(rename = "DEBLZ", skip_serializing_if = "Option::is_none")]
    pub deblz: Option<GermanBankleitzahlIdentifier>,
    #[serde(rename = "RUCB", skip_serializing_if = "Option::is_none")]
    pub rucb: Option<RussianCentralBankIdentificationCodeIdentifier>,
    #[serde(rename = "PLKNR", skip_serializing_if = "Option::is_none")]
    pub plknr: Option<PolishNationalClearingCodeIdentifier>,
    #[serde(rename = "AUBSBx", skip_serializing_if = "Option::is_none")]
    pub aubs_bx: Option<ExtensiveBranchNetworkIdentifier>,
    #[serde(rename = "OthrClrCdId", skip_serializing_if = "Option::is_none")]
    pub othr_clr_cd_id: Option<Max35Text>,
    #[serde(rename = "CHSIC", skip_serializing_if = "Option::is_none")]
    pub chsic: Option<SwissSicIdentifier>,
    #[serde(rename = "CACPA", skip_serializing_if = "Option::is_none")]
    pub cacpa: Option<CanadianPaymentsArnIdentifier>,
    #[serde(rename = "IENSC", skip_serializing_if = "Option::is_none")]
    pub iensc: Option<IrishNscIdentifier>,
    #[serde(rename = "ITNCC", skip_serializing_if = "Option::is_none")]
    pub itncc: Option<ItalianDomesticIdentifier>,
    #[serde(rename = "PTNCC", skip_serializing_if = "Option::is_none")]
    pub ptncc: Option<PortugueseNccIdentifier>,
    #[serde(rename = "ATBLZ", skip_serializing_if = "Option::is_none")]
    pub atblz: Option<AustrianBankleitzahlIdentifier>,
    #[serde(rename = "INIFSC", skip_serializing_if = "Option::is_none")]
    pub inifsc: Option<IndianFinancialSystemCodeIdentifier>,
    #[serde(rename = "ESNCC", skip_serializing_if = "Option::is_none")]
    pub esncc: Option<SpanishDomesticInterbankingIdentifier>,
    #[serde(rename = "USCH", skip_serializing_if = "Option::is_none")]
    pub usch: Option<ChipsParticipantIdentifier>,
    #[serde(rename = "HKNCC", skip_serializing_if = "Option::is_none")]
    pub hkncc: Option<HongKongBankIdentifier>,
    #[serde(rename = "NZNCC", skip_serializing_if = "Option::is_none")]
    pub nzncc: Option<NewZealandNccIdentifier>,
}
#[derive(
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
pub struct Role8ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<InvestmentFundRole8Code>,
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
pub struct Role8Choice {
    #[serde(flatten)]
    pub value: Role8ChoiceEnum,
}
#[derive(
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
pub struct MessageAndBusinessReference13 {
    #[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<References68Choice>,
    #[serde(rename = "TpOfReq", skip_serializing_if = "Option::is_none")]
    pub tp_of_req: Option<TypeOfRequest1Choice>,
    #[serde(rename = "MstrRef", skip_serializing_if = "Option::is_none")]
    pub mstr_ref: Option<Max35Text>,
    #[validate]
    #[serde(rename = "TrfRef")]
    pub trf_ref: Max35Text,
    #[serde(rename = "ClntRef", skip_serializing_if = "Option::is_none")]
    pub clnt_ref: Option<AdditionalReference10>,
    #[serde(rename = "CxlRef", skip_serializing_if = "Option::is_none")]
    pub cxl_ref: Option<Max35Text>,
    #[serde(rename = "Instrm", skip_serializing_if = "Option::is_none")]
    pub instrm: Option<FinancialInstrument63Choice>,
    #[serde(rename = "InvstmtAcctDtls", skip_serializing_if = "Option::is_none")]
    pub invstmt_acct_dtls: Option<Account33>,
    #[validate(length(min = 0,))]
    #[serde(rename = "IntrmyInf", default)]
    pub intrmy_inf: Vec<Intermediary48>,
    #[validate(length(min = 0,))]
    #[serde(rename = "QryInf", default)]
    pub qry_inf: Vec<AdditionalInformation25>,
    #[serde(rename = "ReqIssr", skip_serializing_if = "Option::is_none")]
    pub req_issr: Option<PartyIdentification139>,
    #[serde(rename = "ReqRcpt", skip_serializing_if = "Option::is_none")]
    pub req_rcpt: Option<PartyIdentification139>,
}
#[derive(
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
pub struct Intermediary48 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: PartyIdentification132,
    #[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
    pub acct: Option<Account34>,
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role8Choice>,
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
pub struct FinancialInstrumentIdentification2 {
    #[serde(rename = "Id")]
    pub id: SecurityIdentification25Choice,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max350Text>,
    #[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
    #[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
    pub clssfctn_tp: Option<ClassificationType32Choice>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct AdditionalInformation25 {
    #[serde(rename = "QryTp", skip_serializing_if = "Option::is_none")]
    pub qry_tp: Option<GenericIdentification36>,
    #[validate]
    #[serde(rename = "Qry")]
    pub qry: Max350Text,
    #[serde(rename = "QryRsn", skip_serializing_if = "Option::is_none")]
    pub qry_rsn: Option<Max350Text>,
    #[serde(rename = "RjctnRsn", skip_serializing_if = "Option::is_none")]
    pub rjctn_rsn: Option<RejectedReason33Choice>,
}
#[derive(
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
pub struct References68ChoiceEnum {
    #[serde(rename = "OthrRef", skip_serializing_if = "Option::is_none")]
    pub othr_ref: Option<AdditionalReference10>,
    #[serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none")]
    pub prvs_ref: Option<AdditionalReference10>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct References68Choice {
    #[serde(flatten)]
    pub value: References68ChoiceEnum,
}
#[derive(
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
#[derive(
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
pub struct RequestForTransferStatusReportV08 {
    #[validate]
    #[serde(rename = "MsgId")]
    pub msg_id: MessageIdentification1,
    #[validate(length(min = 1,))]
    #[serde(rename = "ReqDtls", default)]
    pub req_dtls: Vec<MessageAndBusinessReference13>,
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
pub struct Account34 {
    #[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<Max35Text>,
    #[serde(rename = "AcctDsgnt", skip_serializing_if = "Option::is_none")]
    pub acct_dsgnt: Option<Max35Text>,
    #[serde(rename = "AcctNm", skip_serializing_if = "Option::is_none")]
    pub acct_nm: Option<Max35Text>,
    #[validate]
    #[serde(rename = "AcctSvcr")]
    pub acct_svcr: PartyIdentification132,
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
pub struct FinancialInstrument63ChoiceEnum {
    #[serde(rename = "OthrAsst", skip_serializing_if = "Option::is_none")]
    pub othr_asst: Option<OtherAsset2>,
    #[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
    pub scty: Option<FinancialInstrumentIdentification2>,
    #[serde(rename = "CshAsst", skip_serializing_if = "Option::is_none")]
    pub csh_asst: Option<CashAsset3>,
}
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::derive_builder::Builder,
    ::validator::Validate,
)]
pub struct FinancialInstrument63Choice {
    #[serde(flatten)]
    pub value: FinancialInstrument63ChoiceEnum,
}
#[derive(
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
pub struct SubAccount5 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max35Text>,
    #[serde(rename = "Chrtc", skip_serializing_if = "Option::is_none")]
    pub chrtc: Option<Max35Text>,
}
#[derive(
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
pub struct AlternateSecurityIdentification7 {
    #[validate]
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "IdSrc")]
    pub id_src: IdentificationSource1Choice,
}
#[derive(Debug, Default, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub enum InvestmentFundRole8Code {
    #[serde(rename = "CUST")]
    Cust,
    #[serde(rename = "DIST")]
    Dist,
    #[serde(rename = "FMCO")]
    Fmco,
    #[serde(rename = "INTR")]
    Intr,
    #[serde(rename = "INVE")]
    Inve,
    #[serde(rename = "INVS")]
    Invs,
    #[serde(rename = "TRAG")]
    Trag,
    #[serde(rename = "TRAN")]
    Tran,
    #[serde(rename = "UCL1")]
    Ucl1,
    #[serde(rename = "UCL2")]
    Ucl2,
    #[serde(rename = "REGI")]
    Regi,
    #[serde(rename = "CACO")]
    Caco,
    #[serde(rename = "CONC")]
    Conc,
    #[serde(rename = "DATP")]
    Datp,
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
pub struct CashAsset3 {
    #[serde(rename = "CshAsstTp")]
    pub csh_asst_tp: CashAssetType1Choice,
    #[serde(rename = "HldgCcy")]
    pub hldg_ccy: ActiveCurrencyCode,
    #[serde(rename = "TrfCcy", skip_serializing_if = "Option::is_none")]
    pub trf_ccy: Option<ActiveCurrencyCode>,
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
#[serde(rename = "Document")]
pub struct Document {
    #[validate]
    #[serde(rename = "ReqForTrfStsRpt")]
    pub req_for_trf_sts_rpt: RequestForTransferStatusReportV08,
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
pub struct OtherAsset2ChoiceEnum {
    #[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
    pub prtry: Option<GenericIdentification36>,
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<OtherAsset2Code>,
}
#[derive(
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
pub struct RejectedReason33ChoiceEnum {
    #[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
    pub cd: Option<RejectedStatusReason12Code>,
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
pub struct RejectedReason33Choice {
    #[serde(flatten)]
    pub value: RejectedReason33ChoiceEnum,
}
#[derive(
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
    #[serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none")]
    pub tckr_symb: Option<TickerIdentifier>,
    #[serde(rename = "RIC", skip_serializing_if = "Option::is_none")]
    pub ric: Option<RicIdentifier>,
    #[serde(rename = "Belgn", skip_serializing_if = "Option::is_none")]
    pub belgn: Option<BelgianIdentifier>,
    #[serde(rename = "Cmon", skip_serializing_if = "Option::is_none")]
    pub cmon: Option<EuroclearClearstreamIdentifier>,
    #[serde(rename = "Wrtppr", skip_serializing_if = "Option::is_none")]
    pub wrtppr: Option<WertpapierIdentifier>,
    #[serde(rename = "SEDOL", skip_serializing_if = "Option::is_none")]
    pub sedol: Option<SedolIdentifier>,
    #[serde(rename = "OthrPrtryId", skip_serializing_if = "Option::is_none")]
    pub othr_prtry_id: Option<AlternateSecurityIdentification7>,
    #[serde(rename = "CTA", skip_serializing_if = "Option::is_none")]
    pub cta: Option<ConsolidatedTapeAssociationIdentifier>,
    #[serde(rename = "CUSIP", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<CusipIdentifier>,
    #[serde(rename = "QUICK", skip_serializing_if = "Option::is_none")]
    pub quick: Option<QuickIdentifier>,
    #[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
    pub isin: Option<IsinOct2015Identifier>,
    #[serde(rename = "Vlrn", skip_serializing_if = "Option::is_none")]
    pub vlrn: Option<ValorenIdentifier>,
    #[serde(rename = "Dtch", skip_serializing_if = "Option::is_none")]
    pub dtch: Option<DutchIdentifier>,
    #[serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none")]
    pub blmbrg: Option<Bloomberg2Identifier>,
}
#[derive(
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
pub struct PartyIdentification125ChoiceEnum {
    #[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
    pub prtry_id: Option<GenericIdentification1>,
    #[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBicDec2014Identifier>,
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
pub struct GermanBankleitzahlIdentifier {
    #[validate(regex = "GERMAN_BANKLEITZAHL_IDENTIFIER_REGEX")]
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
pub struct ClassificationType32ChoiceEnum {
    #[serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none")]
    pub clssfctn_fin_instrm: Option<CfiOct2015Identifier>,
    #[serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none")]
    pub altrn_clssfctn: Option<GenericIdentification36>,
}
#[derive(
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
#[derive(
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
